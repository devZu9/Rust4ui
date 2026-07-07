use crate::border::{draw_border, get_border};
use crate::renderer::{attr_f64, attr_str, render_children, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let direction = attr_str(node, "direction").unwrap_or("vertical");

    let fill_str = attr_str(node, "fill");
    let fill = fill_str
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x1C, 0x1C, 0x22));

    let border = get_border(node, &ctx.theme, "IconBar");

    let width = attr_f64(node, "width").unwrap_or(48.0);
    let height = attr_f64(node, "height").unwrap_or(36.0);

    if direction == "horizontal" {
        let size = egui::vec2(ui.available_width(), height as f32);
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().rect_filled(rect, 0.0, fill);
        draw_border(ui, rect, egui::CornerRadius::ZERO, &border);

        let inner = rect.shrink(4.0);
        ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
            render_children(ui, node, ctx);
        });
    } else {
        let size = egui::vec2(width as f32, ui.available_height());
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().rect_filled(rect, 0.0, fill);
        draw_border(ui, rect, egui::CornerRadius::ZERO, &border);

        let inner = rect.shrink(4.0);
        ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
            render_children(ui, node, ctx);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_iconbar() {
        let json = serde_json::json!({"type": "IconBar", "direction": "vertical"});
        assert_eq!(attr_str(&json, "direction"), Some("vertical"));
    }
}
