use crate::renderer::{attr_bool, attr_str, resolve_text, widget_margin, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    widget_margin(ui, &ctx.theme, "IconButton");
    let action_name = attr_str(node, "action");
    let target = attr_str(node, "target").unwrap_or("");
    let enabled = attr_bool(node, "enabled").unwrap_or(true);

    let tooltip = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));

    let icon_text = attr_str(node, "icon").unwrap_or("⬡");
    let icon_color = attr_str(node, "icon_color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0xCC, 0xCC, 0xCC));

    let button = egui::Button::new(egui::RichText::new(icon_text).size(18.0).color(icon_color))
        .fill(egui::Color32::TRANSPARENT)
        .min_size(egui::vec2(36.0, 32.0));

    let resp = ui.add_enabled(enabled, button);

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
