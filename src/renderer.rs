use crate::actions::ActionRegistry;
use crate::icons::IconRegistry;
use crate::locale::LocaleRegistry;
use crate::state::StateRegistry;
use crate::theme::Theme;

pub struct RenderCtx {
    pub theme: Theme,
    pub state: StateRegistry,
    pub actions: ActionRegistry,
    pub locale: LocaleRegistry,
    pub icons: IconRegistry,
}

impl RenderCtx {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            state: StateRegistry::new(),
            actions: ActionRegistry::new(),
            locale: LocaleRegistry::default(),
            icons: IconRegistry::new(),
        }
    }

    pub fn color_from_attr(
        &self,
        attr: &serde_json::Value,
        key: &str,
        default: egui::Color32,
    ) -> egui::Color32 {
        attr.get(key)
            .and_then(|v| v.as_str())
            .and_then(crate::theme::parse_hex_color)
            .unwrap_or(default)
    }
}

impl Default for RenderCtx {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_node(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let obj = match node.as_object() {
        Some(o) => o,
        None => return,
    };

    let node_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    match node_type {
        "Label" => with_margin(ui, node, ctx, "Label", |ui, node, ctx| crate::widgets::label::render(ui, node, ctx)),
        "Button" => with_margin(ui, node, ctx, "Button", |ui, node, ctx| crate::widgets::button::render(ui, node, ctx)),
        "TextField" => with_margin(ui, node, ctx, "TextField", |ui, node, ctx| crate::widgets::text_field::render(ui, node, ctx)),
        "Checkbox" => with_margin(ui, node, ctx, "Checkbox", |ui, node, ctx| crate::widgets::checkbox::render(ui, node, ctx)),
        "Separator" => with_margin(ui, node, ctx, "Separator", |ui, node, ctx| crate::widgets::separator::render(ui, node, ctx)),
        "Column" => crate::widgets::column::render(ui, node, ctx),
        "Row" => crate::widgets::row::render(ui, node, ctx),
        "RadioGroup" => with_margin(ui, node, ctx, "RadioGroup", |ui, node, ctx| crate::widgets::radio_group::render(ui, node, ctx)),
        "Slider" => with_margin(ui, node, ctx, "Slider", |ui, node, ctx| crate::widgets::slider::render(ui, node, ctx)),
        "ComboBox" => with_margin(ui, node, ctx, "ComboBox", |ui, node, ctx| crate::widgets::combo_box::render(ui, node, ctx)),
        "Tabs" => with_margin(ui, node, ctx, "Tabs", |ui, node, ctx| crate::widgets::tabs::render(ui, node, ctx)),
        "Panel" => crate::widgets::panel::render(ui, node, ctx),
        "ScrollArea" => crate::widgets::scroll_area::render(ui, node, ctx),
        "Window" => crate::widgets::window::render(ui, node, ctx),
        "Spinner" => crate::widgets::spinner::render(ui, node, ctx),
        "Shortcut" => crate::widgets::shortcut::render(ui, node, ctx),
        "ColorPicker" => crate::widgets::color_picker::render(ui, node, ctx),
        "FileDrop" => crate::widgets::file_drop::render(ui, node, ctx),
        "Indicator" => crate::widgets::indicator::render(ui, node, ctx),
        "StatusBar" => crate::widgets::status_bar::render(ui, node, ctx),
        "IconBar" => crate::widgets::icon_bar::render(ui, node, ctx),
        "IconButton" => with_margin(ui, node, ctx, "IconButton", |ui, node, ctx| crate::widgets::icon_button::render(ui, node, ctx)),
        "Caption" => with_margin(ui, node, ctx, "Caption", |ui, node, ctx| crate::widgets::caption::render(ui, node, ctx)),
        "Grid" => crate::widgets::grid::render(ui, node, ctx),
        "Hyperlink" => with_margin(ui, node, ctx, "Hyperlink", |ui, node, ctx| crate::widgets::hyperlink::render(ui, node, ctx)),
        "Notifications" => crate::widgets::notifications::render(ui, node, ctx),
        "MenuItem" => crate::widgets::menu_item::render(ui, node, ctx),
        "SubMenu" => crate::widgets::sub_menu::render(ui, node, ctx),
        "MenuBar" => crate::widgets::menu_bar::render(ui, node, ctx),
        "Menu" => crate::widgets::menu::render(ui, node, ctx),
        "Spacer" => crate::widgets::spacer::render(ui, node, ctx),
        _ => {
            log::warn!(
                "Renderer: неизвестный тип виджета <{node_type}> (путь: {:?}, attrs: {:?})",
                obj.get("path")
                    .or_else(|| obj.get("id"))
                    .or_else(|| obj.get("text")),
                obj.keys().take(5).collect::<Vec<_>>()
            );
            ui.label(
                egui::RichText::new(format!("⚠ Неизвестный виджет: <{node_type}>"))
                    .color(egui::Color32::from_rgb(0xFF, 0x88, 0x00)),
            );
        }
    }
}

fn with_margin(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx, widget_type: &str, f: impl FnOnce(&mut egui::Ui, &serde_json::Value, &mut RenderCtx)) {
    let m = get_margin(node, &ctx.theme, widget_type);
    if m == egui::Margin::ZERO {
        f(ui, node, ctx);
    } else {
        egui::Frame::new().outer_margin(m).show(ui, |ui| {
            f(ui, node, ctx);
        });
    }
}

pub fn render_children(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
        for child in children {
            render_node(ui, child, ctx);
        }
    }
}

pub fn resolve_text(text: &str, ctx: &RenderCtx) -> String {
    ctx.locale.i18n_text(text, &ctx.state)
}

/// Применяет margin виджета из темы: добавляет `margin`px сверху/снизу
/// и возвращает отступ по X для `ui.add_space`. Если margin > 0 и это вертикальный контейнер,
/// добавляет пробел сверху, а также слева/справа через `indent`.
/// Читает padding из атрибута узла (число, [2], [4]) или из темы
pub fn get_padding(node: &serde_json::Value, theme: &crate::theme::Theme, widget: &str, default: egui::Margin) -> egui::Margin {
    node.get("padding")
        .and_then(parse_padding)
        .or_else(|| {
            theme.widget.get(widget)
                .and_then(|w| w.get("padding"))
                .and_then(parse_padding)
        })
        .unwrap_or(default)
}

pub fn get_margin(node: &serde_json::Value, theme: &crate::theme::Theme, widget: &str) -> egui::Margin {
    node.get("margin")
        .and_then(parse_padding)
        .or_else(|| {
            theme.widget.get(widget)
                .and_then(|w| w.get("margin"))
                .and_then(parse_padding)
        })
        .unwrap_or(egui::Margin::ZERO)
}

pub fn parse_padding(val: &serde_json::Value) -> Option<egui::Margin> {
    match val {
        serde_json::Value::Number(n) => {
            let s = n.as_f64()? as i8;
            Some(egui::Margin::same(s))
        }
        serde_json::Value::Array(arr) => match arr.len() {
            1 => {
                let n = arr[0].as_f64()? as i8;
                Some(egui::Margin::same(n))
            }
            2 => {
                let v = arr[0].as_f64()? as i8;
                let h = arr[1].as_f64()? as i8;
                Some(egui::Margin::symmetric(h, v))
            }
            4 => {
                let t = arr[0].as_f64()? as i8;
                let r = arr[1].as_f64()? as i8;
                let b = arr[2].as_f64()? as i8;
                let l = arr[3].as_f64()? as i8;
                Some(egui::Margin { left: l, right: r, top: t, bottom: b })
            }
            _ => None,
        },
        _ => None,
    }
}

pub use self::parse_padding as parse_margin;

pub fn attr_str<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    node.get(key).and_then(|v| v.as_str())
}

pub fn attr_f64(node: &serde_json::Value, key: &str) -> Option<f64> {
    node.get(key).and_then(|v| v.as_f64())
}

pub fn attr_bool(node: &serde_json::Value, key: &str) -> Option<bool> {
    node.get(key).and_then(|v| v.as_bool())
}
