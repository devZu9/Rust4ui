use std::collections::HashMap;
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
    pub inherited: HashMap<String, serde_json::Value>,
    pub pending_borders: Vec<(egui::Rect, egui::CornerRadius, crate::border::BorderStyle)>,
    pub open_popup_id: Option<String>,
}

impl RenderCtx {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            state: StateRegistry::new(),
            actions: ActionRegistry::new(),
            locale: LocaleRegistry::default(),
            icons: IconRegistry::new(),
            inherited: HashMap::new(),
            pending_borders: Vec::new(),
            open_popup_id: None,
        }
    }

    /// Применить все _children-атрибуты из JSON-узла в self.inherited.
    /// Сохраняет полный снапшот всех текущих inherited, затем очищает HashMap
    /// и заполняет только _children-ключами из node. Гарантирует отсутствие
    /// протекания значений на уровень глубже.
    /// Сохраняет имя родителя как "_parent" для get_padding / get_margin (шаг 4).
    pub fn inherit_children(&mut self, node: &serde_json::Value, parent_name: Option<&str>) -> Vec<(String, Option<serde_json::Value>)> {
        let old: Vec<_> = self.inherited.drain().map(|(k, v)| (k, Some(v))).collect();
        if let Some(obj) = node.as_object() {
            for (key, val) in obj {
                if let Some(base) = key.strip_suffix("_children") {
                    self.inherited.insert(base.to_string(), val.clone());
                }
            }
        }
        if let Some(name) = parent_name {
            self.inherited.insert("_parent".to_string(), serde_json::json!(name));
        }
        old
    }

    /// Восстановить сохранённые inherit_children значения.
    /// Очищает всё и вставляет только старые значения (снапшот),
    /// чтобы ключи, добавленные inherit_children, не просочились мимо.
    pub fn restore_children(&mut self, old: Vec<(String, Option<serde_json::Value>)>) {
        self.inherited.clear();
        for (key, val) in old {
            if let Some(v) = val {
                self.inherited.insert(key, v);
            }
        }
    }

    /// Получить BorderStyle для node, обогащая border-суб-атрибутами из inherited.
    /// Позволяет border_position_children / border_width_children / border_color_children
    /// и любым другим border_*_children работать автоматом через inherit_children.
    pub fn get_border(&self, node: &serde_json::Value, widget: &str) -> crate::border::BorderStyle {
        let mut n = node.clone();
        for key in &["border_position", "border_width", "border_color",
                       "border_type", "border_gap", "border_seg_len", "border_seg_cap"] {
            if let Some(val) = self.inherited.get(*key) {
                n[(*key).to_string()] = val.clone();
            }
        }
        crate::border::get_border(&n, &self.theme, widget)
    }

    pub fn color_from_attr(
        &self,
        attr: &serde_json::Value,
        key: &str,
        default: egui::Color32,
    ) -> egui::Color32 {
        attr.get(key)
            .and_then(|v| v.as_str())
            .and_then(crate::theme::parse_color_hex)
            .unwrap_or(default)
    }
}

impl Default for RenderCtx {
    fn default() -> Self {
        Self::new()
    }
}


fn state_attr_lookup<T: Copy>(
    node: &serde_json::Value,
    theme: &crate::theme::Theme,
    widget_name: &str,
    key: &str,
    parse: fn(&serde_json::Value) -> Option<T>,
) -> Option<T> {
    node.get(key).and_then(parse)
        .or_else(|| theme.widget.get(widget_name).and_then(|w| w.get(key)).and_then(parse))
}

/// Упрощённая версия resolve_state_attr без inherited (только node + theme).
/// Сохранена для обратной совместимости виджетов, не использующих _children.
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
    if !enabled { return base; }
    if resp.is_pointer_button_down_on() {
        let click_key = format!("{}_click", key);
        let focus_key = format!("{}_focus", key);
        state_attr_lookup(node, theme, widget, &click_key, parse)
            .or_else(|| state_attr_lookup(node, theme, widget, &focus_key, parse))
            .unwrap_or(base)
    } else if resp.has_focus() {
        let focus_key = format!("{}_focus", key);
        state_attr_lookup(node, theme, widget, &focus_key, parse).unwrap_or(base)
    } else if resp.hovered() {
        let hover_key = format!("{}_hover", key);
        state_attr_lookup(node, theme, widget, &hover_key, parse).unwrap_or(base)
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
    if !enabled { return egui::Color32::from_gray(60); }
    get_state_attr(node, theme, widget, "background", resp, true, default, crate::theme::parse_color)
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
    inherited: &HashMap<String, serde_json::Value>,
    theme: &crate::theme::Theme,
    default: egui::Margin,
) -> egui::Margin {
    let widget = node.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
    get_attr(
        node, inherited, theme,
        "padding",
        parse_padding,
        |k| theme.widget.get(widget).and_then(|w| w.get(k)).and_then(parse_padding),
        "padding_children",
        default,
    )
}

pub fn get_margin(
    node: &serde_json::Value,
    inherited: &HashMap<String, serde_json::Value>,
    theme: &crate::theme::Theme,
) -> egui::Margin {
    let widget = node.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
    get_attr(
        node, inherited, theme,
        "margin",
        parse_padding,
        |k| theme.widget.get(widget).and_then(|w| w.get(k)).and_then(parse_padding),
        "margin_children",
        egui::Margin::ZERO,
    )
}

/// Универсальное чтение атрибута с полной цепочкой:
///   node → inherited → theme[widget][key] → theme[_parent][parent_key] → default
pub fn get_attr<T: Clone>(
    node: &serde_json::Value,
    inherited: &HashMap<String, serde_json::Value>,
    theme: &crate::theme::Theme,
    key: &str,
    parse: impl Fn(&serde_json::Value) -> Option<T>,
    theme_lookup: impl Fn(&str) -> Option<T>,
    parent_key: &str,
    default: T,
) -> T {
    node.get(key).and_then(&parse)
        .or_else(|| inherited.get(key).and_then(|j| parse(j)))
        .or_else(|| theme_lookup(key))
        .or_else(|| {
            let parent_name = inherited.get("_parent").and_then(|v| v.as_str())?;
            theme.widget.get(parent_name)
                .and_then(|w| w.get(parent_key))
                .and_then(&parse)
        })
        .unwrap_or(default)
}

/// Универсальное чтение атрибута с state (hover/click/focus), inherited, theme и _parent fallback.
/// Цепочка приоритета:
///   1–3: node/inherited/theme[ключ_click/focus/hover]
///   4–7: node → inherited → theme[widget][key] → theme[_parent][key_children] → default
pub fn get_attr_ctx<T: Clone>(
    ctx: &RenderCtx,
    node: &serde_json::Value,
    resp: Option<&egui::Response>,
    key: &str,
    parse: impl Fn(&serde_json::Value) -> Option<T>,
    theme_lookup: impl Fn(&str) -> Option<T>,
    default: T,
) -> T {
    if let Some(resp) = resp {
        let state = if resp.is_pointer_button_down_on() { Some("_click") }
            else if resp.has_focus() { Some("_focus") }
            else if resp.hovered() { Some("_hover") }
            else { None };

        if let Some(suffix) = state {
            let state_key = format!("{key}{suffix}");
            if let Some(v) = node.get(&state_key).and_then(&parse) { return v; }
            if let Some(v) = ctx.inherited.get(&state_key).and_then(|j| parse(j)) { return v; }
            if let Some(v) = theme_lookup(&state_key) { return v; }
        }
    }

    let parent_key = format!("{}_children", key);
    get_attr(node, &ctx.inherited, &ctx.theme, key, parse, theme_lookup, &parent_key, default)
}

/// Парсит скругление: число → 4 одинаковых угла, массив [nw, ne, sw, se] → per-corner
pub fn parse_rounding(val: &serde_json::Value) -> Option<egui::CornerRadius> {
    match val {
        serde_json::Value::Number(n) => Some(egui::CornerRadius::same(n.as_f64()? as u8)),
        serde_json::Value::Array(a) if a.len() >= 4 => Some(egui::CornerRadius {
            nw: a[0].as_f64()? as u8,
            ne: a[1].as_f64()? as u8,
            sw: a[2].as_f64()? as u8,
            se: a[3].as_f64()? as u8,
        }),
        _ => None,
    }
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

