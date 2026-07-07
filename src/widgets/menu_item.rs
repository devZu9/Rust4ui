use crate::renderer::{attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let action = attr_str(node, "action");
    let target = attr_str(node, "target");
    let enabled = node
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let text = attr_str(node, "text").unwrap_or("");
    let shortcut = attr_str(node, "shortcut");

    let label = if let Some(sc) = shortcut {
        format!("{text}  {sc}")
    } else {
        text.to_string()
    };

    if ui
        .add_enabled(
            enabled,
            egui::Button::new(label).fill(egui::Color32::TRANSPARENT),
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
