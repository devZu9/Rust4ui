use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_else(|| "{{menu}}".to_string());

    let bg = node
        .get("background")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background"))
        .unwrap_or_else(|| egui::Color32::from_rgb(0x2A, 0x2A, 0x33));

    let bg_hover = node
        .get("background_hover")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg_hover)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_hover"))
        .unwrap_or(bg);

    let bg_click = node
        .get("background_click")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg_click)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_click"))
        .unwrap_or(bg_hover);

    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let color_hover = node
        .get("color_hover")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color_hover)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_hover"))
        .unwrap_or(color);

    let color_click = node
        .get("color_click")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color_click)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_click"))
        .unwrap_or(color_hover);

    let rounding_val = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("Menu", "rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let radius = ctx.inherited_rounding
        .unwrap_or_else(|| egui::CornerRadius::same(rounding_val));

    let margin = node
        .get("margin")
        .and_then(crate::renderer::parse_padding)
        .unwrap_or_default();

    let pad = node
        .get("padding")
        .and_then(crate::renderer::parse_padding)
        .or_else(|| ctx.inherited_padding)
        .unwrap_or(egui::Margin::ZERO);

    let inher_bg = node
        .get("background_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_children"));

    let inher_color = node
        .get("color_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_children"));

    let inher_bg_hover = node
        .get("background_hover_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_hover_children"));

    let inher_bg_click = node
        .get("background_click_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_click_children"));

    let inher_color_hover = node
        .get("color_hover_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_hover_children"));

    let inher_color_click = node
        .get("color_click_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_click_children"));

    let inher_margin = node
        .get("margin_children")
        .and_then(crate::renderer::parse_padding);

    let inher_padding = node
        .get("padding_children")
        .and_then(crate::renderer::parse_padding);

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    let prev_bg_hover = ctx.inherited_bg_hover;
    let prev_bg_click = ctx.inherited_bg_click;
    let prev_color_hover = ctx.inherited_color_hover;
    let prev_color_click = ctx.inherited_color_click;
    let prev_margin = ctx.inherited_margin;
    let prev_padding = ctx.inherited_padding;
    ctx.inherited_bg = inher_bg;
    ctx.inherited_color = inher_color;
    ctx.inherited_bg_hover = inher_bg_hover;
    ctx.inherited_bg_click = inher_bg_click;
    ctx.inherited_color_hover = inher_color_hover;
    ctx.inherited_color_click = inher_color_click;
    ctx.inherited_margin = inher_margin;
    ctx.inherited_padding = inher_padding;

    let (prev_inactive, prev_hovered, prev_active, prev_open, prev_window_fill, prev_button_pad) = {
        let style = &mut ui.style_mut();
        let prev = (style.visuals.widgets.inactive.clone(), style.visuals.widgets.hovered.clone(), style.visuals.widgets.active.clone(), style.visuals.widgets.open.clone(), style.visuals.window_fill, style.spacing.button_padding);
        style.visuals.widgets.inactive.weak_bg_fill = bg;
        style.visuals.widgets.inactive.corner_radius = radius;
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, color);
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.hovered.weak_bg_fill = bg_hover;
        style.visuals.widgets.hovered.corner_radius = radius;
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, color_hover);
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.active.weak_bg_fill = bg_click;
        style.visuals.widgets.active.corner_radius = radius;
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, color_click);
        style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.open.weak_bg_fill = bg;
        style.visuals.widgets.open.corner_radius = radius;
        style.visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0, color);
        style.visuals.widgets.open.bg_stroke = egui::Stroke::NONE;
        style.visuals.window_fill = bg;
        style.spacing.button_padding = egui::vec2(pad.left as f32, (pad.top as f32 - 1.0).max(0.0));
        prev
    };

    if margin.top > 0 { ui.add_space(margin.top as f32); }
    if margin.left > 0 { ui.add_space(margin.left as f32); }
    let menu_resp = ui.menu_button(egui::RichText::new(text), |ui| {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                super::super::renderer::render_node(ui, child, ctx);
            }
        }
    });
    if margin.right > 0 { ui.add_space(margin.right as f32); }
    if margin.bottom > 0 { ui.add_space(margin.bottom as f32); }

    let base_border = ctx.inherited_border
        .unwrap_or_else(|| crate::border::get_border(node, &ctx.theme, "Menu"));
    let resp = &menu_resp.response;

    let pick_border = |suffix: &str, inherited: Option<crate::border::BorderStyle>| {
        let key = format!("border_{}", suffix);
        if node.get(&key).is_some() {
            crate::border::apply_state_border(node, &ctx.theme, "Menu", suffix, &base_border)
        } else if let Some(b) = inherited {
            b
        } else {
            base_border
        }
    };

    let border = if resp.is_pointer_button_down_on() {
        pick_border("click", ctx.inherited_border_click)
    } else if resp.has_focus() {
        pick_border("focus", ctx.inherited_border_focus)
    } else if resp.hovered() {
        pick_border("hover", ctx.inherited_border_hover)
    } else {
        base_border
    };
    if border.is_visible() {
        ctx.pending_borders.push((menu_resp.response.rect.shrink(1.0), radius, border));
    }

    {
        let style = &mut ui.style_mut();
        style.visuals.widgets.inactive = prev_inactive;
        style.visuals.widgets.hovered = prev_hovered;
        style.visuals.widgets.active = prev_active;
        style.visuals.widgets.open = prev_open;
        style.visuals.window_fill = prev_window_fill;
        style.spacing.button_padding = prev_button_pad;
    }

    ctx.inherited_bg = prev_bg;
    ctx.inherited_color = prev_color;
    ctx.inherited_bg_hover = prev_bg_hover;
    ctx.inherited_bg_click = prev_bg_click;
    ctx.inherited_color_hover = prev_color_hover;
    ctx.inherited_color_click = prev_color_click;
    ctx.inherited_margin = prev_margin;
    ctx.inherited_padding = prev_padding;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_menu() {
        let json = serde_json::json!({"type": "Menu", "text": "File"});
        assert_eq!(attr_str(&json, "text"), Some("File"));
    }
}
