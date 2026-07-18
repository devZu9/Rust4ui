use crate::border::get_border;
use crate::renderer::{attr_f64, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let height = attr_f64(node, "height").unwrap_or(26.0);
    let padding = attr_f64(node, "padding").unwrap_or(4.0);

    let fill = node.get("background")
        .and_then(crate::theme::parse_color)
        .unwrap_or(egui::Color32::from_rgb(0x18, 0x18, 0x1D));

    let border = get_border(node, &ctx.theme, "StatusBar");

    let children = match node.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    let available = ui.available_size();

    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(available.x, height as f32), egui::Sense::hover());

    ui.painter().rect_filled(rect, 0.0, fill);
    if border.is_visible() {
        ui.painter().line_segment(
            [rect.left_top(), rect.right_top()],
            (border.width, border.color),
        );
    }

    let inner = rect.shrink(padding as f32);

    let mut start_items = Vec::new();
    let mut _center_items = Vec::new();
    let mut _end_items = Vec::new();

    for child in children {
        let anchor = attr_str(child, "anchor").unwrap_or("start");
        match anchor {
            "center" => _center_items.push(child.clone()),
            "end" => _end_items.push(child.clone()),
            _ => start_items.push(child.clone()),
        }
    }

    ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
        for child in &start_items {
            super::super::renderer::render_node(ui, child, ctx);
            ui.add_space(8.0);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_statusbar() {
        let json = serde_json::json!({"type": "StatusBar", "height": 26});
        assert_eq!(attr_f64(&json, "height"), Some(26.0));
    }
}

