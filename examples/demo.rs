use rust4ui::{render_node, LocaleRegistry, RefResolver, RenderCtx, StateRegistry, Theme};
use std::path::Path;

struct DemoApp {
    ctx: RenderCtx,
    tree: serde_json::Value,
    prev_locale_idx: usize,
}

impl DemoApp {
    fn new() -> Self {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"));

        let tree = {
            let ui_path = base.join("demo").join("ui.json");
            let content =
                std::fs::read_to_string(&ui_path).expect("Не удалось прочитать demo/ui.json");
            let root: serde_json::Value =
                serde_json::from_str(&content).expect("demo/ui.json невалидный JSON");
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
        state.set_string("status_text", "Готов к работе".into());
        state.set_string("greeting_text", "Друг".into());
        state.set_bool("use_gpu", true);
        state.set_bool("auto_start", false);
        state.set_bool("show_info_window", false);
        state.set_bool("show_dialog", false);
        state.set_bool("show_custom_window", false);
        state.set_usize("active_locale", 0);
        state.set_usize("selected_device", 0);
        state.set_vec_string(
            "device_list",
            vec![
                "Микрофон 1 (Realtek)".into(),
                "Микрофон 2 (USB)".into(),
                "Линейный вход".into(),
            ],
        );
        state.set_vec_string("locale_list", vec!["Русский".into(), "English".into()]);

        let mut locale = LocaleRegistry::new("ru");
        locale
            .load_file("ru", &base.join("locales").join("ru.json"))
            .expect("Не удалось загрузить locales/ru.json");
        locale
            .load_file("en", &base.join("locales").join("en.json"))
            .expect("Не удалось загрузить locales/en.json");

        let mut ctx = RenderCtx {
            theme: Theme::default(),
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
        };

        let actions = &mut ctx.actions;
        actions.register("click", |c| {
            let target = c.target.clone();
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
            c.state.set_string("greeting_text", greeting);
            c.state
                .set_string("status_text", "Приветствие обновлено".into());
        });
        actions.register("apply", |c| {
            c.state
                .set_string("status_text", "Настройки применены ✓".into());
        });
        actions.register("reset", |c| {
            c.state.set_f64("volume", 50.0);
            c.state.set_f64("font_size", 14.0);
            c.state.set_string("description", String::new());
            c.state
                .set_string("status_text", "Настройки сброшены".into());
        });
        actions.register("toggle_window", |c| {
            let current = c.state.get_bool("show_info_window").unwrap_or(false);
            c.state.set_bool("show_info_window", !current);
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
        });
        actions.register("confirm_cancel", |c| {
            c.state.set_bool("show_dialog", false);
            c.state.set_string("status_text", "Отменено".into());
        });
        actions.register("file_dropped", |c| {
            c.state
                .set_string("status_text", format!("Файл: {}", c.target));
        });

        Self {
            ctx,
            tree,
            prev_locale_idx: 0,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ctx.theme.apply_to_egui(ctx);

        let vol = self.ctx.state.get_f64("volume").unwrap_or(0.0) as i64;
        let fs = self.ctx.state.get_f64("font_size").unwrap_or(14.0) as i64;
        self.ctx
            .state
            .set_string("status_text", format!("Громкость: {vol}% | Шрифт: {fs}px"));

        if let Some(idx) = self.ctx.state.get_usize("active_locale") {
            if idx != self.prev_locale_idx {
                self.prev_locale_idx = idx;
                match idx {
                    0 => self.ctx.locale.switch("ru"),
                    1 => self.ctx.locale.switch("en"),
                    _ => {}
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            render_node(ui, &self.tree, &mut self.ctx);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Rust4ui — Demo")
            .with_inner_size([900.0, 640.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust4ui — Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::new()))),
    )
}
