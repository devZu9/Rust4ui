use rust4ui::{render_node, strip_json_comments, LocaleRegistry, RefResolver, RenderCtx, StateRegistry, Theme};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

struct DemoApp {
    ctx: RenderCtx,
    tree: serde_json::Value,
    prev_locale_idx: usize,
    files_changed: Arc<AtomicBool>,
    base: PathBuf,
}

fn load_theme(base: &Path) -> Theme {
    let mut theme = Theme::default();

    let theme_path = base.join("demo").join("theme.json");
    if theme_path.exists() {
        let content = match std::fs::read_to_string(&theme_path) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Не удалось прочитать theme.json: {e}");
                return theme;
            }
        };
        let parsed: serde_json::Value = match serde_json::from_str(&strip_json_comments(&content)) {
            Ok(v) => v,
            Err(e) => {
                log::error!("theme.json невалидный JSON: {e}");
                return theme;
            }
        };
        if let Some(obj) = parsed.as_object() {
            for (widget, attrs) in obj {
                theme.widget.insert(widget.clone(), attrs.clone());
            }
        }
        log::info!(
            "Тема загружена из demo/theme.json ({} секций)",
            parsed.as_object().map(|o| o.len()).unwrap_or(0)
        );
    } else {
        log::warn!("demo/theme.json не найден, использую дефолтную тёмную тему");
    }

    theme
}

fn load_phosphor_font(egui_ctx: &egui::Context) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("icons/phosphor.ttf");
    if let Ok(data) = std::fs::read(&path) {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert("phosphor".into(), Arc::new(egui::FontData::from_owned(data)));
        if let Some(proportional) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            proportional.push("phosphor".into());
        }
        egui_ctx.set_fonts(fonts);
        log::info!("Phosphor-шрифт загружен для иконок");
    } else {
        log::warn!("Phosphor-шрифт не найден: {}", path.display());
    }
}

fn start_file_watcher(demo_path: PathBuf, changed: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        use notify::{Config, EventKind, RecommendedWatcher, Watcher};
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(e) => {
                log::warn!("Файловый watcher не запущен: {e}");
                return;
            }
        };
        if let Err(e) = watcher.watch(&demo_path, notify::RecursiveMode::Recursive) {
            log::warn!("Не удалось начать слежение за demo/: {e}");
            return;
        }
        log::info!("Запущен watcher за demo/ ({})", demo_path.display());
        for event in rx {
            if let Ok(ev) = event {
                if matches!(ev.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    changed.store(true, Ordering::Relaxed);
                }
            }
        }
    });
}

impl DemoApp {
    fn new() -> Self {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();

        log::info!("Загрузка UI из demo/ui.json...");
        let tree = {
            let ui_path = base.join("demo").join("ui.json");
            let content =
                std::fs::read_to_string(&ui_path).expect("Не удалось прочитать demo/ui.json");
            let root: serde_json::Value =
                serde_json::from_str(&strip_json_comments(&content)).expect("demo/ui.json невалидный JSON");
            let mut resolver = RefResolver::new();
            resolver
                .resolve(&root, &base.join("demo"))
                .expect("Ошибка резолвинга $ref в demo/")
        };

        let mut state = StateRegistry::new();
        state.set_f64("volume", 50.0);
        state.set_f64("font_size", 14.0);
        state.set_f64("count", 3.0);
        state.set_string("name", String::new());
        state.set_string("description", String::new());
        state.set_string("password", String::new());
        state.set_string("status_text", "Готов к работе".into());
        state.set_string("greeting_text", "Друг".into());
        state.set_bool("use_gpu", true);
        state.set_bool("auto_start", false);
        state.set_bool("show_info_window", false);
        state.set_bool("show_dialog", false);
        state.set_bool("show_custom_window", false);
        state.set_usize("active_locale", 0);
        state.set_usize("selected_device", 0);
        state.set_usize("app_theme", 0);
        state.set_vec_string(
            "device_list",
            vec![
                "Микрофон 1 (Realtek)".into(),
                "Микрофон 2 (USB)".into(),
                "Линейный вход".into(),
            ],
        );
        state.set_vec_string("locale_list", vec!["Русский".into(), "English".into()]);

        log::info!("Загрузка локалей...");
        let mut locale = LocaleRegistry::new("ru");
        locale
            .load_file("ru", &base.join("locales").join("ru.json"))
            .expect("Не удалось загрузить locales/ru.json");
        locale
            .load_file("en", &base.join("locales").join("en.json"))
            .expect("Не удалось загрузить locales/en.json");

        let theme = load_theme(&base);

        rust4ui::contrast::check_theme_contrasts(&theme.colors, "demo");

        let mut ctx = RenderCtx {
            theme,
            state,
            actions: rust4ui::ActionRegistry::new(),
            locale,
            icons: rust4ui::IconRegistry::new(),
        };

        log::info!("Регистрация действий...");
        let actions = &mut ctx.actions;
        actions.register("click", |c| {
            let target = c.target.clone();
            log::info!("Нажата кнопка: {target}");
            c.state
                .set_string("status_text", format!("Нажата кнопка: {target}"));
        });
        actions.register("greeting", |c| {
            let name = c.state.get_string("name").unwrap_or("").to_string();
            let greeting = if name.is_empty() {
                "Друг".to_string()
            } else {
                name
            };
            c.state.set_string("greeting_text", greeting.clone());
            c.state
                .set_string("status_text", "Приветствие обновлено".into());
            log::info!("Приветствие: {greeting}");
        });
        actions.register("apply", |c| {
            c.state
                .set_string("status_text", "Настройки применены ✓".into());
            log::info!("Настройки применены");
        });
        actions.register("reset", |c| {
            c.state.set_f64("volume", 50.0);
            c.state.set_f64("font_size", 14.0);
            c.state.set_string("description", String::new());
            c.state
                .set_string("status_text", "Настройки сброшены".into());
            log::info!("Настройки сброшены");
        });
        actions.register("toggle_window", |c| {
            let current = c.state.get_bool("show_info_window").unwrap_or(false);
            c.state.set_bool("show_info_window", !current);
            log::info!(
                "Окно: {}",
                if current {
                    "закрыто"
                } else {
                    "открыто"
                }
            );
        });
        actions.register("toggle_dialog", |c| {
            let current = c.state.get_bool("show_dialog").unwrap_or(false);
            c.state.set_bool("show_dialog", !current);
        });
        actions.register("toggle_custom_window", |c| {
            let current = c.state.get_bool("show_custom_window").unwrap_or(false);
            c.state.set_bool("show_custom_window", !current);
        });
        actions.register("confirm_ok", |c| {
            c.state.set_bool("show_dialog", false);
            c.state.set_string("status_text", "Подтверждено ✓".into());
            log::info!("Диалог подтверждён");
        });
        actions.register("confirm_cancel", |c| {
            c.state.set_bool("show_dialog", false);
            c.state.set_string("status_text", "Отменено".into());
            log::info!("Диалог отменён");
        });
        actions.register("file_dropped", |c| {
            c.state
                .set_string("status_text", format!("Файл: {}", c.target));
            log::info!("Файл брошен: {}", c.target);
        });

        log::info!("Запуск watcher за demo/ для hot-reload...");
        let files_changed = Arc::new(AtomicBool::new(false));
        let demo_path = base.join("demo");
        start_file_watcher(demo_path, files_changed.clone());

        log::info!("Демо-приложение готово");
        Self {
            ctx,
            tree,
            prev_locale_idx: 0,
            files_changed,
            base,
        }
    }
}

impl DemoApp {
    fn reload_ui_tree(&mut self) {
        let ui_path = self.base.join("demo").join("ui.json");
        match std::fs::read_to_string(&ui_path) {
            Ok(content) => match serde_json::from_str::<serde_json::Value>(&strip_json_comments(&content)) {
                Ok(root) => {
                    let mut resolver = RefResolver::new();
                    match resolver.resolve(&root, &self.base.join("demo")) {
                        Ok(resolved) => {
                            self.tree = resolved;
                            log::info!("UI-дерево перезагружено с диска");
                        }
                        Err(e) => log::error!("Ошибка резолвинга UI: {e}"),
                    }
                }
                Err(e) => log::error!("Ошибка парсинга ui.json: {e}"),
            },
            Err(e) => log::error!("Ошибка чтения ui.json: {e}"),
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.files_changed.swap(false, Ordering::Relaxed) {
            log::info!("Обнаружено изменение файлов, hot-reload...");
            self.reload_ui_tree();
            let new_theme = load_theme(&self.base);
            rust4ui::contrast::check_theme_contrasts(&new_theme.colors, "demo");
            self.ctx.theme = new_theme;
        }

        self.ctx.theme.apply_to_egui(ctx);

        let vol = self.ctx.state.get_f64("volume").unwrap_or(0.0) as i64;
        let fs = self.ctx.state.get_f64("font_size").unwrap_or(14.0) as i64;
        self.ctx
            .state
            .set_string("status_text", format!("Громкость: {vol}% | Шрифт: {fs}px"));

        if let Some(idx) = self.ctx.state.get_usize("active_locale") {
            if idx != self.prev_locale_idx {
                self.prev_locale_idx = idx;
                let lang = match idx {
                    0 => "ru",
                    1 => "en",
                    _ => "ru",
                };
                log::info!("Переключение языка на '{lang}'");
                self.ctx.locale.switch(lang);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            render_node(ui, &self.tree, &mut self.ctx);
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> Result<(), eframe::Error> {
    rust4ui::logger::init_logger();

    log::info!("Rust4ui v{} — запуск", env!("CARGO_PKG_VERSION"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Rust4ui — Demo")
            .with_inner_size([900.0, 640.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Rust4ui — Demo",
        options,
        Box::new(|cc| {
            load_phosphor_font(&cc.egui_ctx);
            Ok(Box::new(DemoApp::new()))
        }),
    );

    if let Err(e) = &result {
        log::error!("Ошибка приложения: {e}");
    }

    result
}
