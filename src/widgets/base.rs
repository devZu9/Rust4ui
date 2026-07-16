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

struct PrevWidgetStyle {
    inactive: egui::style::WidgetVisuals,
    hovered: egui::style::WidgetVisuals,
    active: egui::style::WidgetVisuals,
    open: egui::style::WidgetVisuals,
    window_fill: egui::Color32,
}

fn save_widget_style(ui: &egui::Ui) -> PrevWidgetStyle {
    let v = &ui.visuals();
    PrevWidgetStyle {
        inactive: v.widgets.inactive,
        hovered: v.widgets.hovered,
        active: v.widgets.active,
        open: v.widgets.open,
        window_fill: v.window_fill,
    }
}

fn restore_widget_style(ui: &mut egui::Ui, saved: PrevWidgetStyle) {
    let v = &mut ui.style_mut().visuals;
    v.widgets.inactive = saved.inactive;
    v.widgets.hovered = saved.hovered;
    v.widgets.active = saved.active;
    v.widgets.open = saved.open;
    v.window_fill = saved.window_fill;
}

/// Wrap-режим: alloc + bg/border от widget_base + стилизованный child_ui для egui-виджета.
pub fn widget_base_wrap<R>(
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
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> (R, egui::Response) {
    let out = widget_base(ui, node, theme, widget, content_size, sense, enabled,
        default_bg, default_rounding, default_pad, inherited_bg);

    let saved = save_widget_style(ui);
    let v = &mut ui.style_mut().visuals;
    v.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    v.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(0x3A, 0x3A, 0x44);
    v.widgets.active.weak_bg_fill = egui::Color32::from_rgb(0x4A, 0x4A, 0x54);
    v.widgets.open.weak_bg_fill = egui::Color32::TRANSPARENT;
    v.widgets.inactive.corner_radius = out.rounding_cr;
    v.widgets.hovered.corner_radius = out.rounding_cr;
    v.widgets.active.corner_radius = out.rounding_cr;
    v.widgets.open.corner_radius = out.rounding_cr;

    let mut child_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(out.inner_rect)
            .layout(*ui.layout()),
    );
    let result = add_contents(&mut child_ui);

    restore_widget_style(ui, saved);
    (result, out.response)
}
