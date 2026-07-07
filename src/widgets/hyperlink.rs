use crate::renderer::{attr_str, resolve_text, widget_margin, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    widget_margin(ui, &ctx.theme, "Hyperlink");
    let url = attr_str(node, "url").unwrap_or("");
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_else(|| url.to_string());

    let tooltip = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));

    let resp = ui.hyperlink_to(text, url);

    if let Some(tip) = &tooltip {
        if !tip.is_empty() {
            resp.on_hover_text(tip.as_str());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_hyperlink() {
        let json = serde_json::json!({
            "type": "Hyperlink",
            "url": "https://github.com",
            "text": "GitHub"
        });
        assert_eq!(attr_str(&json, "url"), Some("https://github.com"));
    }
}
