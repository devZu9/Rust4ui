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
    pub inherited_bg: Option<egui::Color32>,
    pub inherited_color: Option<egui::Color32>,
    pub inherited_margin: Option<egui::Margin>,
    pub inherited_padding: Option<egui::Margin>,
    pub inherited_rounding: Option<egui::CornerRadius>,
}

impl RenderCtx {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            state: StateRegistry::new(),
            actions: ActionRegistry::new(),
            locale: LocaleRegistry::default(),
            icons: IconRegistry::new(),
            inherited_bg: None,
            inherited_color: None,
            inherited_margin: None,
            inherited_padding: None,
            inherited_rounding: None,
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
        "Label" => crate::widgets::label::render(ui, node, ctx),
        "Button" => crate::widgets::button::render(ui, node, ctx),
        "TextField" => crate::widgets::text_field::render(ui, node, ctx),
        "NumberField" => {
            let mut n = node.clone();
            n["mode"] = serde_json::json!("number");
            crate::widgets::text_field::render(ui, &n, ctx);
        }
        "Checkbox" => crate::widgets::checkbox::render(ui, node, ctx),
        "Separator" => crate::widgets::separator::render(ui, node, ctx),
        "Column" => crate::widgets::column::render(ui, node, ctx),
        "Row" => crate::widgets::row::render(ui, node, ctx),
        "RadioGroup" => crate::widgets::radio_group::render(ui, node, ctx),
        "Slider" => crate::widgets::slider::render(ui, node, ctx),
        "ComboBox" => crate::widgets::combo_box::render(ui, node, ctx),
        "Tabs" => crate::widgets::tabs::render(ui, node, ctx),
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
        "IconButton" => crate::widgets::icon_button::render(ui, node, ctx),
        "Caption" => crate::widgets::caption::render(ui, node, ctx),
        "Grid" => crate::widgets::grid::render(ui, node, ctx),
        "Hyperlink" => crate::widgets::hyperlink::render(ui, node, ctx),
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

pub fn get_padding(
    node: &serde_json::Value,
    theme: &crate::theme::Theme,
    widget: &str,
    default: egui::Margin,
) -> egui::Margin {
    node.get("padding")
        .and_then(parse_padding)
        .or_else(|| {
            theme.widget.get(widget)
                .and_then(|w| w.get("padding"))
                .and_then(parse_padding)
        })
        .unwrap_or(default)
}

pub fn get_margin(
    node: &serde_json::Value,
    theme: &crate::theme::Theme,
    widget: &str,
) -> egui::Margin {
    node.get("margin")
        .and_then(parse_padding)
        .or_else(|| {
            theme.widget.get(widget)
                .and_then(|w| w.get("margin"))
                .and_then(parse_padding)
        })
        .unwrap_or(egui::Margin::ZERO)
}

fn state_attr_lookup<T: Copy>(
    n: &serde_json::Value,
    t: &crate::theme::Theme,
    w: &str,
    k: &str,
    p: fn(&serde_json::Value) -> Option<T>,
) -> Option<T> {
    n.get(k).and_then(p)
        .or_else(|| t.widget.get(w).and_then(|x| x.get(k)).and_then(p))
}

pub fn get_state_attr<T: Copy>(
    node: &serde_json::Value,
    theme: &crate::theme::Theme,
    widget: &str,
    key: &str,
    resp: &egui::Response,
    enabled: bool,
    default: T,
    parse: fn(&serde_json::Value) -> Option<T>,
) -> T {
    let base = state_attr_lookup(node, theme, widget, key, parse).unwrap_or(default);
    if !enabled {
        return base;
    }
    if resp.is_pointer_button_down_on() {
        let ck = format!("{}_click", key);
        let fk = format!("{}_focus", key);
        state_attr_lookup(node, theme, widget, &ck, parse)
            .or_else(|| state_attr_lookup(node, theme, widget, &fk, parse))
            .unwrap_or(base)
    } else if resp.has_focus() {
        let fk = format!("{}_focus", key);
        state_attr_lookup(node, theme, widget, &fk, parse).unwrap_or(base)
    } else if resp.hovered() {
        let hk = format!("{}_hover", key);
        state_attr_lookup(node, theme, widget, &hk, parse).unwrap_or(base)
    } else {
        base
    }
}

pub fn get_state_background(
    node: &serde_json::Value,
    theme: &crate::theme::Theme,
    widget: &str,
    resp: &egui::Response,
    enabled: bool,
    default: egui::Color32,
) -> egui::Color32 {
    if !enabled {
        return egui::Color32::from_gray(60);
    }
    get_state_attr(node, theme, widget, "background", resp, true, default, crate::theme::parse_color_value)
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
                Some(egui::Margin {
                    left: l,
                    right: r,
                    top: t,
                    bottom: b,
                })
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
