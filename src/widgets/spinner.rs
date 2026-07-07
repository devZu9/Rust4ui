use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    let _size = attr_f64(node, "size").unwrap_or(24.0);
    let color = attr_str(node, "color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x66, 0xCC, 0xFF));

    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_default();

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.spinner();
        if !text.is_empty() {
            ui.add_space(8.0);
            ui.label(egui::RichText::new(text).color(color));
        }
    });
    ui.add_space(4.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_spinner() {
        let json = serde_json::json!({"type": "Spinner", "size": 24});
        assert_eq!(attr_f64(&json, "size"), Some(24.0));
    }
}
