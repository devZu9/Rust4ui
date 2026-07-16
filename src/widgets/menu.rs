use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_else(|| "{{menu}}".to_string());

    let bg = node
        .get("background")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background"))
        .unwrap_or_else(|| egui::Color32::from_rgb(0x2A, 0x2A, 0x33));

    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let rounding_val = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("Menu", "rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let radius = egui::CornerRadius::same(rounding_val);

    let margin = node
        .get("margin")
        .and_then(crate::renderer::parse_padding)
        .unwrap_or_default();

    let pad = node
        .get("padding")
        .and_then(crate::renderer::parse_padding)
        .unwrap_or(egui::Margin::ZERO);

    let inher_bg = node
        .get("background_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| node.get("background").and_then(crate::theme::parse_color_value))
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_children"))
        .or_else(|| ctx.theme.w_color_opt("Menu", "background"));

    let inher_color = node
        .get("color_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| node.get("color").and_then(crate::theme::parse_color_value))
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_children"))
        .or_else(|| ctx.theme.w_color_opt("Menu", "color"));

    let inher_margin = node
        .get("margin_children")
        .and_then(crate::renderer::parse_padding);

    let inher_padding = node
        .get("padding_children")
        .and_then(crate::renderer::parse_padding);

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    let prev_margin = ctx.inherited_margin;
    let prev_padding = ctx.inherited_padding;
    ctx.inherited_bg = inher_bg;
    ctx.inherited_color = inher_color;
    ctx.inherited_margin = inher_margin;
    ctx.inherited_padding = inher_padding;

    let (prev_inactive, prev_hovered, prev_active, prev_open, prev_window_fill, prev_button_pad) = {
        let style = &mut ui.style_mut();
        let prev = (style.visuals.widgets.inactive.clone(), style.visuals.widgets.hovered.clone(), style.visuals.widgets.active.clone(), style.visuals.widgets.open.clone(), style.visuals.window_fill, style.spacing.button_padding);
        style.visuals.widgets.inactive.weak_bg_fill = bg;
        style.visuals.widgets.inactive.corner_radius = radius;
        style.visuals.widgets.hovered.weak_bg_fill = bg;
        style.visuals.widgets.hovered.corner_radius = radius;
        style.visuals.widgets.active.weak_bg_fill = bg;
        style.visuals.widgets.active.corner_radius = radius;
        style.visuals.widgets.open.weak_bg_fill = bg;
        style.visuals.widgets.open.corner_radius = radius;
        style.visuals.window_fill = bg;
        style.spacing.button_padding = egui::vec2(pad.left as f32, pad.top as f32);
        prev
    };

    if margin.left > 0 { ui.add_space(margin.left as f32); }
    ui.menu_button(egui::RichText::new(text).color(color), |ui| {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                super::super::renderer::render_node(ui, child, ctx);
            }
        }
    });
    if margin.right > 0 { ui.add_space(margin.right as f32); }

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
