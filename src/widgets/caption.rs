use crate::renderer::{attr_bool, attr_f64, attr_str, resolve_text, widget_margin, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    widget_margin(ui, &ctx.theme, "Caption");
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_default();

    let color_str = attr_str(node, "color");
    let color = color_str
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x88, 0x88, 0x88));

    let size = attr_f64(node, "size").unwrap_or(11.0);
    let monospace = attr_bool(node, "monospace").unwrap_or(false);

    let mut rich = egui::RichText::new(text).size(size as f32).color(color);
    if monospace {
        rich = rich.monospace();
    }
    ui.label(rich);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_caption() {
        let json = serde_json::json!({"type": "Caption", "text": "v1.0"});
        assert_eq!(attr_str(&json, "text"), Some("v1.0"));
    }
}
