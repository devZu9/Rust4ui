use eframe::egui;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: HashMap<String, String>,
    pub sizes: HashMap<String, f32>,
    pub rounding: HashMap<String, f32>,
    pub widget: HashMap<String, serde_json::Value>,
}

impl Theme {
    pub fn merge(&mut self, other: &Theme) {
        for (k, v) in &other.colors {
            self.colors.insert(k.clone(), v.clone());
        }
        for (k, v) in &other.sizes {
            self.sizes.insert(k.clone(), *v);
        }
        for (k, v) in &other.rounding {
            self.rounding.insert(k.clone(), *v);
        }
        for (k, v) in &other.widget {
            self.widget.insert(k.clone(), v.clone());
        }
    }

    pub fn color(&self, key: &str) -> Option<egui::Color32> {
        self.colors
            .get(key)
            .and_then(|s| parse_hex_color(s.as_str()))
    }

    pub fn color_or(&self, key: &str, default: egui::Color32) -> egui::Color32 {
        self.color(key).unwrap_or(default)
    }

    pub fn size(&self, key: &str) -> Option<f32> {
        self.sizes.get(key).copied()
    }

    pub fn size_or(&self, key: &str, default: f32) -> f32 {
        self.size(key).unwrap_or(default)
    }

    pub fn rounding_or(&self, key: &str, default: f32) -> f32 {
        self.rounding.get(key).copied().unwrap_or(default)
    }

    pub fn w_f64(&self, widget: &str, key: &str, default: f64) -> f64 {
        self.widget
            .get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_f64())
            .unwrap_or(default)
    }

    pub fn w_str(&self, widget: &str, key: &str, default: &str) -> String {
        self.widget
            .get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| default.to_string())
    }

    pub fn w_bool(&self, widget: &str, key: &str, default: bool) -> bool {
        self.widget
            .get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_bool())
            .unwrap_or(default)
    }

    pub fn w_color(&self, widget: &str, key: &str, default: egui::Color32) -> egui::Color32 {
        self.widget
            .get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
            .unwrap_or(default)
    }

    pub fn w_color_opt(&self, widget: &str, key: &str) -> Option<egui::Color32> {
        self.widget
            .get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
    }

    pub fn w_str2(&self, node: &serde_json::Value, widget: &str, key: &str) -> Option<String> {
        node.get(key)
            .and_then(|v| v.as_str())
            .map(String::from)
            .or_else(|| {
                self.widget
                    .get(widget)
                    .and_then(|v| v.get(key))
                    .and_then(|v| v.as_str())
                    .map(String::from)
            })
    }

    pub fn apply_to_egui(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;

        let bg = self.color_or("background", egui::Color32::from_rgb(0x14, 0x16, 0x1B));
        let panel = self.color_or("panel_fill", egui::Color32::from_rgb(0x1A, 0x1D, 0x23));
        let window = self.color_or("window_fill", egui::Color32::from_rgb(0x1E, 0x1E, 0x24));
        let text = self.color_or("text_primary", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0));
        let _text_dim = self.color_or("text_dim", egui::Color32::from_rgb(0x88, 0x88, 0x88));

        style.visuals.panel_fill = panel;
        style.visuals.window_fill = window;
        style.visuals.window_stroke.color =
            self.color_or("border_color", egui::Color32::from_rgb(0x33, 0x33, 0x3A));
        style.visuals.window_stroke.width = 1.0;
        style.visuals.faint_bg_color = bg;
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(0x0C, 0x0E, 0x12);
        style.visuals.code_bg_color = egui::Color32::from_rgb(0x0C, 0x0E, 0x12);
        style.visuals.warn_fg_color = egui::Color32::from_rgb(0xFF, 0x88, 0x00);

        let nonint = &mut style.visuals.widgets.noninteractive;
        nonint.fg_stroke.color = text;
        nonint.bg_fill = egui::Color32::from_rgb(0x55, 0x55, 0x55);
        nonint.corner_radius = egui::CornerRadius::same(4);

        let inactive = &mut style.visuals.widgets.inactive;
        inactive.fg_stroke.color = text;
        inactive.bg_fill = egui::Color32::from_rgb(0x2A, 0x2A, 0x33);
        inactive.corner_radius = egui::CornerRadius::same(4);

        let hovered = &mut style.visuals.widgets.hovered;
        hovered.fg_stroke.color = text;
        hovered.bg_fill = egui::Color32::from_rgb(0x3A, 0x3A, 0x44);
        hovered.corner_radius = egui::CornerRadius::same(4);

        let active = &mut style.visuals.widgets.active;
        active.fg_stroke.color = text;
        active.bg_fill = egui::Color32::from_rgb(0x44, 0x44, 0x55);
        active.corner_radius = egui::CornerRadius::same(4);

        let open = &mut style.visuals.widgets.open;
        open.fg_stroke.color = text;
        open.bg_fill = egui::Color32::from_rgb(0x33, 0x33, 0x44);
        open.corner_radius = egui::CornerRadius::same(4);

        style.visuals.selection.bg_fill =
            egui::Color32::from_rgba_unmultiplied(0x33, 0x66, 0xCC, 0x66);
        style.visuals.selection.stroke.color = egui::Color32::from_rgb(0x66, 0x99, 0xFF);
        style.visuals.hyperlink_color =
            self.color_or("link_color", egui::Color32::from_rgb(0x66, 0xCC, 0xFF));
        style.visuals.text_cursor.preview = true;
        style.visuals.text_cursor.stroke.color = egui::Color32::from_rgb(0xCC, 0xCC, 0xCC);

        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::proportional(14.0),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::proportional(14.0),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::proportional(20.0),
        );

        let margins = &mut style.spacing.item_spacing;
        margins.x = self.w_f64("global", "item_spacing_x", 8.0) as f32;
        margins.y = self.w_f64("global", "item_spacing_y", 10.0) as f32;

        // button_padding не влияет — используем кастомный painter
        style.spacing.button_padding = egui::vec2(12.0, 4.0);

        ctx.set_style(style);
    }

    pub fn apply_node_theme(&self, node: &Value) -> egui::Color32 {
        let key = node.get("theme_key").and_then(|v| v.as_str()).unwrap_or("");
        self.color(key)
            .unwrap_or(self.color_or("text_primary", egui::Color32::GRAY))
    }
}

impl Default for Theme {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert("background".into(), "#14161B".into());
        colors.insert("panel_fill".into(), "#1A1D23".into());
        colors.insert("window_fill".into(), "#1E1E24".into());
        colors.insert("text_primary".into(), "#E0E0E0".into());
        colors.insert("text_dim".into(), "#888888".into());
        colors.insert("link_color".into(), "#66CCFF".into());
        colors.insert("border_color".into(), "#33333A".into());
        colors.insert("accent".into(), "#3366CC".into());
        colors.insert("success".into(), "#00AA66".into());
        colors.insert("danger".into(), "#CC3333".into());
        colors.insert("warning".into(), "#CC8800".into());

        let mut sizes = HashMap::new();
        sizes.insert("text_size".into(), 13.0);
        sizes.insert("heading_size".into(), 20.0);
        sizes.insert("gap".into(), 4.0);

        let mut widget = HashMap::new();
        widget.insert("TextField".into(), serde_json::json!({
            "width": 200.0,
            "height": 28.0,
            "background": "#1C1E24",
            "inner_margin": 4.0,
            "rounding": 4.0,
            "text_color": "#E0E0E0",
            "margin": 0.0
        }));
        widget.insert("Button".into(), serde_json::json!({
            "min_width": 100.0,
            "fill": "#303030",
            "rounding": 6.0,
            "padding": [5, 10, 15, 5],
            "margin": 0.0
        }));
        widget.insert("ComboBox".into(), serde_json::json!({
            "width": 200.0,
            "height": 32.0,
            "background": "#2A2A33",
            "inner_pad_h": 10.0,
            "rounding": 4.0,
            "popup_bg": "#1C1E24",
            "text_color": "#E0E0E0",
            "margin": 0.0
        }));
        widget.insert("Slider".into(), serde_json::json!({
            "width": 250.0,
            "height": 20.0,
            "margin": 0.0
        }));
        widget.insert("Separator".into(), serde_json::json!({
            "space": 6.0,
            "margin": 0.0
        }));
        widget.insert("Column".into(), serde_json::json!({
            "gap": 4.0,
            "padding": 8.0
        }));
        widget.insert("Row".into(), serde_json::json!({
            "gap": 4.0,
            "padding": 8.0
        }));
        widget.insert("Tabs".into(), serde_json::json!({
            "gap": 4.0,
            "active_color": "#66CCFF",
            "inactive_color": "#999999",
            "tab_height": 28.0,
            "margin": 0.0,
            "tab_padding": 10.0
        }));
        widget.insert("Panel".into(), serde_json::json!({
            "fill": "#1A1D23",
            "rounding": 8.0,
            "padding": 12.0,
            "border": [1, "#333333"]
        }));
        widget.insert("Label".into(), serde_json::json!({
            "size": 14.0,
            "color": "#E0E0E0",
            "margin": 0.0
        }));
        widget.insert("Checkbox".into(), serde_json::json!({
            "margin": 0.0
        }));
        widget.insert("RadioGroup".into(), serde_json::json!({
            "margin": 0.0
        }));
        widget.insert("ScrollArea".into(), serde_json::json!({
            "axis": "vertical"
        }));
        widget.insert("Window".into(), serde_json::json!({
            "default_width": 400.0,
            "default_height": 300.0,
            "fill": "#1E1E24",
            "border": [1, "#33333A"],
            "padding": 8.0
        }));
        widget.insert("Spinner".into(), serde_json::json!({
            "color": "#66CCFF",
            "size": 24.0
        }));
        widget.insert("FileDrop".into(), serde_json::json!({
            "fill": "#1A1D23",
            "rounding": 8.0,
            "border": [1, "#333333"],
            "padding": 16.0
        }));

        Self {
            colors,
            sizes,
            rounding: HashMap::new(),
            widget,
        }
    }
}

pub fn parse_hex_color(hex: &str) -> Option<egui::Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(egui::Color32::from_rgb(r, g, b))
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_priority() {
        let mut theme = Theme::default();
        theme.colors.insert("bg".into(), "#111111".into());
        theme.colors.insert("text".into(), "#CCCCCC".into());

        let mut overrides = Theme::default();
        overrides.colors.insert("bg".into(), "#222222".into());
        theme.merge(&overrides);

        assert_eq!(theme.colors.get("bg").unwrap(), "#222222");
        assert_eq!(theme.colors.get("text").unwrap(), "#CCCCCC");
    }

    #[test]
    fn test_theme_missing() {
        let mut theme = Theme::default();
        theme.colors.insert("bg".into(), "#111111".into());
        assert!(theme.color("missing").is_none());
        assert_eq!(
            theme.color("bg"),
            Some(egui::Color32::from_rgb(0x11, 0x11, 0x11))
        );
    }

    #[test]
    fn test_default_theme_is_dark() {
        let theme = Theme::default();
        assert!(theme.colors.contains_key("background"));
        assert_eq!(
            theme.color("background"),
            Some(egui::Color32::from_rgb(0x14, 0x16, 0x1B))
        );
        assert!(theme.colors.contains_key("text_primary"));
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(
            parse_hex_color("#FF6600"),
            Some(egui::Color32::from_rgb(0xFF, 0x66, 0x00))
        );
        assert_eq!(
            parse_hex_color("FF6600"),
            Some(egui::Color32::from_rgb(0xFF, 0x66, 0x00))
        );
        assert_eq!(
            parse_hex_color("#3366CC44"),
            Some(egui::Color32::from_rgba_unmultiplied(
                0x33, 0x66, 0xCC, 0x44
            ))
        );
        assert_eq!(parse_hex_color("invalid"), None);
    }
}
