use std::collections::HashMap;
use crate::border::{draw_border, draw_shadow_bg, draw_shadow_border, get_state_border, parse_shadow, Shadow};
use crate::renderer::{get_margin, get_padding, resolve_state_attr};

pub struct BaseOut {
    pub response: egui::Response,
    pub content_rect: egui::Rect,
    pub inner_rect: egui::Rect,
    pub rounding_cr: egui::CornerRadius,
}

fn get_bg(
    node: &serde_json::Value,
    inherited: &HashMap<String, serde_json::Value>,
    theme: &crate::theme::Theme,
    widget: &str,
    resp: &egui::Response,
    enabled: bool,
    default: egui::Color32,
) -> egui::Color32 {
    if !enabled { return egui::Color32::from_gray(60); }
    resolve_state_attr(
        node, inherited, resp, "background",
        crate::theme::parse_color_value,
        |k| theme.w_color_opt(widget, k),
        default,
    )
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
    inherited: &HashMap<String, serde_json::Value>,
) -> BaseOut {
    let pad = get_padding(node, theme, widget, default_pad);
    let margin = get_margin(node, theme, widget);

    let content_width = content_size.x + pad.left as f32 + pad.right as f32;
    let content_height = content_size.y + pad.top as f32 + pad.bottom as f32;
    let total_width = content_width + margin.left as f32 + margin.right as f32;
    let total_height = content_height + margin.top as f32 + margin.bottom as f32;

    let size = egui::vec2(total_width.max(0.0), total_height.max(0.0));
    let (rect, resp) = ui.allocate_exact_size(size, sense);

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + margin.left as f32, rect.min.y + margin.top as f32),
        egui::pos2(rect.max.x - margin.right as f32, rect.max.y - margin.bottom as f32),
    );

    let rounding = resolve_state_attr(
        node, inherited, &resp, "rounding",
        |v| v.as_f64(),
        |k| theme.widget.get(widget).and_then(|w| w.get(k)).and_then(|v| v.as_f64()),
        default_rounding,
    );
    let rounding_cr = egui::CornerRadius::same(rounding as u8);

    let shadow_bg = resolve_state_attr(
        node, inherited, &resp, "shadow_background",
        parse_shadow,
        |k| None,
        Shadow::transparent(),
    );
    draw_shadow_bg(ui, content_rect, rounding_cr, &shadow_bg);

    let bg = get_bg(node, inherited, theme, widget, &resp, enabled, default_bg);
    ui.painter().rect_filled(content_rect, rounding_cr, bg);

    let border = get_state_border(node, theme, widget, &resp, enabled);
    let shadow_border = resolve_state_attr(
        node, inherited, &resp, "shadow_border",
        parse_shadow,
        |k| None,
        Shadow::transparent(),
    );
    if shadow_border.z_order == crate::border::ShadowZOrder::Under {
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
        draw_border(ui, content_rect, rounding_cr, &border);
    } else {
        draw_border(ui, content_rect, rounding_cr, &border);
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
    }

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
    inherited: &HashMap<String, serde_json::Value>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> (R, egui::Response) {
    let out = widget_base(ui, node, theme, widget, content_size, sense, enabled,
        default_bg, default_rounding, default_pad, inherited);

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
