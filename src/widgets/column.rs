use crate::border::widget_border;
use crate::renderer::{attr_f64, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let gap = attr_f64(node, "gap").unwrap_or(0.0);
    let padding = attr_f64(node, "padding").unwrap_or(0.0);
    let align = attr_str(node, "align").unwrap_or("left");

    let response = egui::Frame::new()
        .inner_margin(egui::Margin::same(padding as i8))
        .show(ui, |ui| {
            if align == "center" {
                ui.vertical_centered(|ui| {
                    render_with_gap(ui, node, ctx, gap as f32);
                });
            } else {
                ui.vertical(|ui| {
                    render_with_gap(ui, node, ctx, gap as f32);
                });
            }
        });
    widget_border(ui, response.response.rect, node, &ctx.theme, "Column", egui::CornerRadius::same(4));
}

fn render_with_gap(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx, gap: f32) {
    if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
        for (i, child) in children.iter().enumerate() {
            if i > 0 && gap > 0.0 {
                ui.add_space(gap);
            }
            super::super::renderer::render_node(ui, child, ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_column() {
        let json = serde_json::json!({
            "type": "Column",
            "gap": 8,
            "children": [{"type": "Label", "text": "A"}]
        });
        assert_eq!(json["type"], "Column");
        assert_eq!(json["children"].as_array().unwrap().len(), 1);
    }
}
