use crate::renderer::{attr_f64, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let fill_str = attr_str(node, "fill");
    let fill = fill_str
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x1A, 0x1D, 0x23));

    let rounding = attr_f64(node, "rounding").unwrap_or(8.0);
    let padding = attr_f64(node, "padding").unwrap_or(12.0);
    let stroke_width = attr_f64(node, "stroke_width").unwrap_or(1.0);

    let stroke_color = attr_str(node, "stroke_color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x33, 0x33, 0x33));

    let frame = egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(rounding as u8))
        .inner_margin(egui::Margin::same(padding as i8))
        .stroke(egui::Stroke::new(stroke_width as f32, stroke_color));

    frame.show(ui, |ui| {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                super::super::renderer::render_node(ui, child, ctx);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_panel() {
        let json = serde_json::json!({
            "type": "Panel",
            "fill": "#1A1D23",
            "padding": 12
        });
        assert_eq!(attr_str(&json, "fill"), Some("#1A1D23"));
    }
}
