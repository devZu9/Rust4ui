use crate::renderer::{attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_default();

    let children = match node.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    ui.menu_button(text, |ui| {
        for child in children {
            super::super::renderer::render_node(ui, child, ctx);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_submenu() {
        let json = serde_json::json!({
            "type": "SubMenu",
            "text": "Export",
            "children": [{"type": "MenuItem", "text": "JSON"}]
        });
        assert_eq!(attr_str(&json, "text"), Some("Export"));
    }
}
