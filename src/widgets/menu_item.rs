use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let action = attr_str(node, "action");
    let target = attr_str(node, "target");
    let enabled = node
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let raw_text = attr_str(node, "text").unwrap_or("");
    let text = resolve_text(raw_text, ctx);
    let icon_name = attr_str(node, "icon");
    let shortcut = attr_str(node, "shortcut");
    let size = attr_f64(node, "size")
        .unwrap_or_else(|| ctx.theme.w_f64("MenuItem", "size", 14.0)) as f32;

    let prefix = icon_name.and_then(|n| ctx.icons.resolve(n)).unwrap_or("");
    let label = if let Some(sc) = shortcut {
        format!("{prefix} {text}  {sc}")
    } else {
        format!("{prefix} {text}")
    };

    if ui
        .add_enabled(
            enabled,
            egui::Button::new(
                egui::RichText::new(label).size(size)
            ).fill(egui::Color32::TRANSPARENT),
        )
        .clicked()
        && enabled
    {
        if let Some(action_name) = action {
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target.unwrap_or(""))
                .with_state(&ctx.state);
            ctx.actions.invoke(action_name, &mut action_ctx);
            ctx.state = action_ctx.state;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_menuitem() {
        let json = serde_json::json!({"type": "MenuItem", "text": "Copy", "action": "copy"});
        assert_eq!(attr_str(&json, "text"), Some("Copy"));
    }
}
