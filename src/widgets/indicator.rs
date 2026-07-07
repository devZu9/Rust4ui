use crate::renderer::{attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, _ctx: &mut RenderCtx) {
    let _color = attr_str(node, "color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x80, 0x80, 0x80));

    let size = attr_str(node, "size").unwrap_or("8");
    let _pulse = attr_str(node, "pulse").unwrap_or("false") == "true";

    let tooltip = attr_str(node, "tooltip");

    let color_str = attr_str(node, "color").unwrap_or("#888888");
    let color = crate::theme::parse_hex_color(color_str)
        .unwrap_or(egui::Color32::from_rgb(0x88, 0x88, 0x88));

    let diameter: f32 = size.parse().unwrap_or(8.0);
    let pos = ui.next_widget_position();
    let rect = egui::Rect::from_center_size(
        pos + egui::vec2(diameter / 2.0, diameter / 2.0),
        egui::vec2(diameter, diameter),
    );

    ui.painter()
        .circle_filled(rect.center(), diameter / 2.0, color);

    let resp = ui.allocate_rect(rect, egui::Sense::hover());

    if let Some(tip) = tooltip {
        resp.on_hover_text(tip);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_indicator() {
        let json = serde_json::json!({"type": "Indicator", "color": "#00FF66", "size": 8});
        assert_eq!(attr_str(&json, "color"), Some("#00FF66"));
    }
}
