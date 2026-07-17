use rust4ui::{render_node, strip_json_comments, substitute_vars, LocaleRegistry, RefResolver, RenderCtx, StateRegistry, Theme};
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
    settings_path: PathBuf,
    last_saved_content: String,
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
            // 1. Вычитать vars
            if let Some(v) = obj.get("vars").and_then(|v| v.as_object()) {
                for (key, val) in v {
                    theme.vars.insert(key.clone(), val.clone());
                }
            }
            // 2. Резолвить vars внутри самой секции vars
            let resolved_vars = theme.vars.clone();
            for val in theme.vars.values_mut() {
                substitute_vars(val, &resolved_vars);
            }
            // 3. Загрузить и резолвить все остальные секции
            for (widget, attrs) in obj {
                if widget == "vars" { continue; }
                let mut resolved = attrs.clone();
                substitute_vars(&mut resolved, &theme.vars);
                theme.widget.insert(widget.clone(), resolved);
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
                    if !ev.paths.iter().any(|p| p.ends_with("settings.json")) {
                        changed.store(true, Ordering::Relaxed);
                    }
                }
            }
        }
    });
}

impl DemoApp {
    fn new() -> Self {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();

        let theme = load_theme(&base);

        log::info!("Загрузка UI из demo/ui.json...");
        let tree = {
            let ui_path = base.join("demo").join("ui.json");
            let content =
                std::fs::read_to_string(&ui_path).expect("Не удалось прочитать demo/ui.json");
            let root: serde_json::Value =
                serde_json::from_str(&strip_json_comments(&content)).expect("demo/ui.json невалидный JSON");
            let mut resolver = RefResolver::new();
            let mut resolved = resolver
                .resolve(&root, &base.join("demo"))
                .expect("Ошибка резолвинга $ref в demo/");
            substitute_vars(&mut resolved, &theme.vars);
            resolved
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
        state.set_string("active_tab", "basic".into());

        let settings_path = base.join("demo").join("settings.json");
        if settings_path.exists() {
            let settings = StateRegistry::load(&settings_path);
            if let Some(w) = settings.get_f64("window_size_width") { state.set_f64("window_size_width", w); }
            if let Some(h) = settings.get_f64("window_size_height") { state.set_f64("window_size_height", h); }
            if let Some(x) = settings.get_f64("window_position_x") { state.set_f64("window_position_x", x); }
            if let Some(y) = settings.get_f64("window_position_y") { state.set_f64("window_position_y", y); }
            if let Some(t) = settings.get_string("active_tab") { state.set_string("active_tab", t.to_string()); }
            if let Some(l) = settings.get_string("active_locale") {
                let idx = match l { "en" => 1, _ => 0 };
                state.set_usize("active_locale", idx);
            }
            log::info!("Настройки загружены из demo/settings.json");
        }

        log::info!("Загрузка локалей...");
        let mut locale = LocaleRegistry::new("ru");
        locale
            .load_file("ru", &base.join("locales").join("ru.json"))
            .expect("Не удалось загрузить locales/ru.json");
        locale
            .load_file("en", &base.join("locales").join("en.json"))
            .expect("Не удалось загрузить locales/en.json");

        rust4ui::contrast::check_theme_contrasts(&theme.colors, "demo");

        let mut ctx = RenderCtx {
            theme,
            state,
            actions: rust4ui::ActionRegistry::new(),
            locale,
            icons: rust4ui::IconRegistry::new(),
            inherited_bg: None,
            inherited_color: None,
            inherited_bg_hover: None,
            inherited_bg_click: None,
            inherited_color_hover: None,
            inherited_color_click: None,
            inherited_margin: None,
            inherited_padding: None,
            inherited_rounding: None,
            inherited_border: None,
            inherited_border_hover: None,
            inherited_border_click: None,
            inherited_border_focus: None,
            pending_borders: Vec::new(),
            open_popup_id: None,
            inherited_icon: None,
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
            settings_path,
            last_saved_content: String::new(),
        }
    }
}

impl DemoApp {
    fn reload_ui(&mut self) {
        let new_theme = load_theme(&self.base);
        let ui_path = self.base.join("demo").join("ui.json");
        match std::fs::read_to_string(&ui_path) {
            Ok(content) => match serde_json::from_str::<serde_json::Value>(&strip_json_comments(&content)) {
                Ok(root) => {
                    let mut resolver = RefResolver::new();
                    match resolver.resolve(&root, &self.base.join("demo")) {
                        Ok(mut resolved) => {
                            substitute_vars(&mut resolved, &new_theme.vars);
                            self.tree = resolved;
                            log::info!("UI-дерево и тема перезагружены с диска");
                        }
                        Err(e) => log::error!("Ошибка резолвинга UI: {e}"),
                    }
                }
                Err(e) => log::error!("Ошибка парсинга ui.json: {e}"),
            },
            Err(e) => log::error!("Ошибка чтения ui.json: {e}"),
        }
        rust4ui::contrast::check_theme_contrasts(&new_theme.colors, "demo");
        self.ctx.theme = new_theme;
    }

    fn save_settings_if_needed(&mut self, ctx: &egui::Context) {
        let mut settings = StateRegistry::new();
        if let Some(rect) = ctx.input(|i| i.viewport().inner_rect) {
            settings.set_f64("window_size_width", rect.width() as f64);
            settings.set_f64("window_size_height", rect.height() as f64);
            settings.set_f64("window_position_x", rect.min.x as f64);
            settings.set_f64("window_position_y", rect.min.y as f64);
        } else {
            let sr = ctx.screen_rect();
            settings.set_f64("window_size_width", sr.width() as f64);
            settings.set_f64("window_size_height", sr.height() as f64);
        }
        if let Some(t) = self.ctx.state.get_string("active_tab") {
            settings.set_string("active_tab", t.to_string());
        }
        let locale_idx = self.ctx.state.get_usize("active_locale").unwrap_or(0);
        let locale_code = match locale_idx { 1 => "en", _ => "ru" };
        settings.set_string("active_locale", locale_code.to_string());

        let content = settings.to_json();
        if content == self.last_saved_content {
            return;
        }
        if let Err(e) = settings.save(&self.settings_path) {
            log::error!("{e}");
        }
        self.last_saved_content = content;
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.files_changed.swap(false, Ordering::Relaxed) {
            log::info!("Обнаружено изменение файлов, hot-reload...");
            self.reload_ui();
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

        self.save_settings_if_needed(ctx);
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> Result<(), eframe::Error> {
    rust4ui::logger::init_logger();

    log::info!("Rust4ui v{} — запуск", env!("CARGO_PKG_VERSION"));

    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("demo");
    let saved = StateRegistry::load(&base.join("settings.json"));
    let sw = saved.get_f64("window_size_width").unwrap_or(900.0) as f32;
    let sh = saved.get_f64("window_size_height").unwrap_or(640.0) as f32;
    let sx = saved.get_f64("window_position_x");
    let sy = saved.get_f64("window_position_y");

    let mut vp = egui::ViewportBuilder::default()
        .with_title("Rust4ui — Demo")
        .with_inner_size([sw, sh])
        .with_min_inner_size([600.0, 400.0]);
    if let (Some(px), Some(py)) = (sx, sy) {
        vp = vp.with_position([px as f32, py as f32]);
    }

    let options = eframe::NativeOptions {
        viewport: vp,
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
