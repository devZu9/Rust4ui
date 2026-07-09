use crate::renderer::{attr_bool, attr_f64, attr_str, resolve_text, widget_margin, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    widget_margin(ui, &ctx.theme, "IconButton");
    let action_name = attr_str(node, "action");
    let target = attr_str(node, "target").unwrap_or("");
    let enabled = attr_bool(node, "enabled").unwrap_or(true);

    let tooltip = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));

    let icon_name = attr_str(node, "icon").unwrap_or("");
    let icon_glyph = if !icon_name.is_empty() {
        ctx.icons.resolve_glyph(icon_name)
    } else {
        "⬡".to_string()
    };
    let icon_color = attr_str(node, "icon_color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0xCC, 0xCC, 0xCC));
    let icon_size = attr_f64(node, "icon_size")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "icon_size", 18.0)) as f32;

    let button = egui::Button::new(
        egui::RichText::new(&icon_glyph).size(icon_size).color(icon_color)
    )
        .fill(egui::Color32::TRANSPARENT)
        .min_size(egui::vec2(36.0, 32.0));

    let resp = ui.add_enabled(enabled, button);

    if resp.hovered() && !resp.is_pointer_button_down_on() {
        let hover_fill = attr_str(node, "hover_fill")
            .and_then(crate::theme::parse_hex_color)
            .or_else(|| ctx.theme.w_color_opt("IconButton", "hover_fill"))
            .unwrap_or(egui::Color32::from_rgba_premultiplied(0x44, 0x44, 0x55, 0x40));
        ui.painter().rect_filled(resp.rect, egui::CornerRadius::same(4), hover_fill);
    } else if resp.is_pointer_button_down_on() {
        let click_fill = attr_str(node, "click_fill")
            .and_then(crate::theme::parse_hex_color)
            .or_else(|| ctx.theme.w_color_opt("IconButton", "click_fill"))
            .unwrap_or(egui::Color32::from_rgba_premultiplied(0x33, 0x33, 0x44, 0x60));
        ui.painter().rect_filled(resp.rect, egui::CornerRadius::same(4), click_fill);
    }

    if let Some(tip) = &tooltip {
        resp.clone().on_hover_text(tip.as_str());
    }

    if resp.clicked() && enabled {
        if let Some(action) = action_name {
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target)
                .with_state(&ctx.state);
            ctx.actions.invoke(action, &mut action_ctx);
            ctx.state = action_ctx.state;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_iconbutton() {
        let json = serde_json::json!({"type": "IconButton", "icon": "save", "action": "save"});
        assert_eq!(attr_str(&json, "icon"), Some("save"));
    }
}
