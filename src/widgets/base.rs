use crate::border::{draw_border, draw_shadow_bg, draw_shadow_border, get_state_border, parse_shadow, Shadow};
use crate::renderer::{get_attr_ctx, get_margin, get_padding, parse_padding, parse_rounding, RenderCtx};

pub struct PaintOut {
    pub response: egui::Response,
    pub content_rect: egui::Rect,
    pub inner_rect: egui::Rect,
    pub rounding_cr: egui::CornerRadius,
}

fn get_bg(
    ctx: &RenderCtx,
    node: &serde_json::Value,
    resp: &egui::Response,
    enabled: bool,
) -> egui::Color32 {
    if !enabled { return egui::Color32::from_gray(60); }
    let widget = node.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
    get_attr_ctx(
        ctx, node, Some(resp), "background",
        crate::theme::parse_color,
        |k| ctx.theme.w_color_opt(widget, k),
        egui::Color32::TRANSPARENT,
    )
}

pub fn widget_paint_custom(
    ui: &mut egui::Ui,
    node: &serde_json::Value,
    ctx: &RenderCtx,
    reserved_size: egui::Vec2,
    sense: egui::Sense,
    enabled: bool,
) -> PaintOut {
    let widget = node.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
    let base_padding = get_padding(node, &ctx.inherited, &ctx.theme, egui::Margin::ZERO);
    let base_margin = get_margin(node, &ctx.inherited, &ctx.theme);

    let content_width = reserved_size.x + base_padding.left as f32 + base_padding.right as f32;
    let content_height = reserved_size.y + base_padding.top as f32 + base_padding.bottom as f32;
    let total_width = content_width + base_margin.left as f32 + base_margin.right as f32;
    let total_height = content_height + base_margin.top as f32 + base_margin.bottom as f32;

    let size = egui::vec2(total_width.max(0.0), total_height.max(0.0));
    let (rect, resp) = reserve_exact_size(ui, size, sense);

    // State-зависимые padding/margin (padding_hover, margin_click и т.д.)
    let padding_theme_lookup = |k: &str| ctx.theme.widget.get(widget).and_then(|w| w.get(k)).and_then(parse_padding);
    let padding = get_attr_ctx(
        ctx, node, Some(&resp), "padding",
        parse_padding, &padding_theme_lookup, base_padding,
    );
    let margin = get_attr_ctx(
        ctx, node, Some(&resp), "margin",
        parse_padding, &padding_theme_lookup, base_margin,
    );

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + margin.left as f32, rect.min.y + margin.top as f32),
        egui::pos2(rect.max.x - margin.right as f32, rect.max.y - margin.bottom as f32),
    );

    let rounding_theme_lookup = |k: &str| ctx.theme.widget.get(widget).and_then(|w| w.get(k)).and_then(parse_rounding);
    let rounding_cr = get_attr_ctx(
        ctx, node, Some(&resp), "rounding",
        parse_rounding, &rounding_theme_lookup,
        egui::CornerRadius::same(ctx.theme.w_f64(widget, "rounding", 4.0) as u8),
    );

    let shadow_bg = get_attr_ctx(
        ctx, node, Some(&resp), "shadow_background",
        parse_shadow, |_k| None,
        Shadow::transparent(),
    );

    let inner_rect = egui::Rect::from_min_max(
        egui::pos2(content_rect.left() + padding.left as f32, content_rect.top() + padding.top as f32),
        egui::pos2(content_rect.right() - padding.right as f32, content_rect.bottom() - padding.bottom as f32),
    );

    draw_shadow_bg(ui, content_rect, rounding_cr, &shadow_bg);

    let bg = get_bg(ctx, node, &resp, enabled);
    ui.painter().rect_filled(content_rect, rounding_cr, bg);

    let border = get_state_border(node, &ctx.theme, widget, &resp, enabled);
    let shadow_border = get_attr_ctx(
        ctx, node, Some(&resp), "shadow_border",
        parse_shadow, |_k| None,
        Shadow::transparent(),
    );
    if shadow_border.z_order == crate::border::ShadowZOrder::Under {
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
        draw_border(ui, content_rect, rounding_cr, &border);
    } else {
        draw_border(ui, content_rect, rounding_cr, &border);
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
    }

    PaintOut { response: resp, content_rect, inner_rect, rounding_cr }
}

/// Зарезервировать ровно size.x × size.y места в ui.
fn reserve_exact_size(ui: &mut egui::Ui, size: egui::Vec2, sense: egui::Sense) -> (egui::Rect, egui::Response) {
    ui.allocate_exact_size(size, sense)
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

/// Wrap-режим: alloc + bg/border от widget_paint_custom + стилизованный child_ui для egui-виджета.
pub fn widget_paint_egui<R>(
    ui: &mut egui::Ui,
    node: &serde_json::Value,
    ctx: &RenderCtx,
    reserved_size: egui::Vec2,
    sense: egui::Sense,
    enabled: bool,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> (R, egui::Response) {
    let out = widget_paint_custom(ui, node, ctx, reserved_size, sense, enabled);

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



