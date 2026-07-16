use crate::border::{draw_border, draw_shadow_bg, draw_shadow_border, get_state_border, parse_shadow, Shadow};
use crate::renderer::{get_margin, get_padding, get_state_attr, parse_padding};

pub struct BaseOut {
    pub response: egui::Response,
    pub content_rect: egui::Rect,
    pub inner_rect: egui::Rect,
    pub rounding_cr: egui::CornerRadius,
}

fn lookup_bg(
    node: &serde_json::Value,
    inherited: Option<egui::Color32>,
    theme: &crate::theme::Theme,
    widget: &str,
    key: &str,
) -> Option<egui::Color32> {
    node.get(key).and_then(crate::theme::parse_color_value)
        .or_else(|| inherited)
        .or_else(|| theme.w_color_opt(widget, key))
}

fn get_bg(
    node: &serde_json::Value,
    inherited: Option<egui::Color32>,
    theme: &crate::theme::Theme,
    widget: &str,
    resp: &egui::Response,
    enabled: bool,
    default: egui::Color32,
) -> egui::Color32 {
    if !enabled { return egui::Color32::from_gray(60); }
    let base = lookup_bg(node, inherited, theme, widget, "background").unwrap_or(default);
    if resp.is_pointer_button_down_on() {
        lookup_bg(node, None, theme, widget, "background_click")
            .or_else(|| lookup_bg(node, None, theme, widget, "background_focus"))
            .unwrap_or(base)
    } else if resp.has_focus() {
        lookup_bg(node, None, theme, widget, "background_focus").unwrap_or(base)
    } else if resp.hovered() {
        lookup_bg(node, None, theme, widget, "background_hover").unwrap_or(base)
    } else {
        base
    }
}

pub fn widget_base(
    ui: &mut egui::Ui,
    node: &serde_json::Value,
    theme: &crate::theme::Theme,
    widget: &str,
    content_size: egui::Vec2,
    sense: egui::Sense,
    enabled: bool,
    default_bg: egui::Color32,
    default_rounding: f64,
    default_pad: egui::Margin,
    inherited_bg: Option<egui::Color32>,
) -> BaseOut {
    let pad = get_padding(node, theme, widget, default_pad);
    let margin = get_margin(node, theme, widget);

    let cw = content_size.x + pad.left as f32 + pad.right as f32;
    let ch = content_size.y + pad.top as f32 + pad.bottom as f32;
    let tw = cw + margin.left as f32 + margin.right as f32;
    let th = ch + margin.top as f32 + margin.bottom as f32;

    let size = egui::vec2(tw.max(0.0), th.max(0.0));
    let (rect, resp) = ui.allocate_exact_size(size, sense);

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + margin.left as f32, rect.min.y + margin.top as f32),
        egui::pos2(rect.max.x - margin.right as f32, rect.max.y - margin.bottom as f32),
    );

    let rounding = get_state_attr(node, theme, widget, "rounding", &resp, enabled, default_rounding, |v| v.as_f64());
    let rounding_cr = egui::CornerRadius::same(rounding as u8);

    let shadow_bg = get_state_attr(node, theme, widget, "shadow_background", &resp, enabled,
        Shadow::transparent(), parse_shadow);
    draw_shadow_bg(ui, content_rect, rounding_cr, &shadow_bg);

    let bg = get_bg(node, inherited_bg, theme, widget, &resp, enabled, default_bg);
    ui.painter().rect_filled(content_rect, rounding_cr, bg);

    let border = get_state_border(node, theme, widget, &resp, enabled);
    let shadow_border = get_state_attr(node, theme, widget, "shadow_border", &resp, enabled,
        Shadow::transparent(), parse_shadow);
    if shadow_border.z_order == crate::border::ShadowZOrder::Under {
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
        draw_border(ui, content_rect, rounding_cr, &border);
    } else {
        draw_border(ui, content_rect, rounding_cr, &border);
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
    }

    let pad = get_state_attr(node, theme, widget, "padding", &resp, enabled, pad, parse_padding);
    let inner_rect = egui::Rect::from_min_max(
        egui::pos2(content_rect.left() + pad.left as f32, content_rect.top() + pad.top as f32),
        egui::pos2(content_rect.right() - pad.right as f32, content_rect.bottom() - pad.bottom as f32),
    );

    BaseOut { response: resp, content_rect, inner_rect, rounding_cr }
}
