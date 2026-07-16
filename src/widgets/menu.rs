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

    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let rounding_val = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("Menu", "rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let radius = egui::CornerRadius::same(rounding_val);

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    ctx.inherited_bg = Some(bg);
    ctx.inherited_color = Some(color);

    let (prev_inactive, prev_hovered, prev_active, prev_open, prev_window_fill) = {
        let v = &mut ui.style_mut().visuals;
        let prev = (v.widgets.inactive.clone(), v.widgets.hovered.clone(), v.widgets.active.clone(), v.widgets.open.clone(), v.window_fill);
        v.widgets.inactive.weak_bg_fill = bg;
        v.widgets.inactive.corner_radius = radius;
        v.widgets.hovered.weak_bg_fill = bg;
        v.widgets.hovered.corner_radius = radius;
        v.widgets.active.weak_bg_fill = bg;
        v.widgets.active.corner_radius = radius;
        v.widgets.open.weak_bg_fill = bg;
        v.widgets.open.corner_radius = radius;
        v.window_fill = bg;
        prev
    };

    ui.menu_button(egui::RichText::new(text).color(color), |ui| {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                super::super::renderer::render_node(ui, child, ctx);
            }
        }
    });

    {
        let v = &mut ui.style_mut().visuals;
        v.widgets.inactive = prev_inactive;
        v.widgets.hovered = prev_hovered;
        v.widgets.active = prev_active;
        v.widgets.open = prev_open;
        v.window_fill = prev_window_fill;
    }

    ctx.inherited_bg = prev_bg;
    ctx.inherited_color = prev_color;
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
