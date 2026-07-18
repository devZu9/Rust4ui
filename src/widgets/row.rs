use crate::border::widget_border;
use crate::renderer::{attr_bool, attr_f64, get_padding, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let gap = attr_f64(node, "gap").unwrap_or(0.0);
    let gap_row = attr_f64(node, "gap_row").unwrap_or(0.0);
    let wrap = attr_bool(node, "wrap").unwrap_or(false);
    let columns = attr_f64(node, "columns").map(|c| c as usize);
    if let Some(ncols) = columns {
        if ncols < 2 {
            log::warn!("Row.columns: должно быть >= 2, указано {ncols}");
            return;
        }
        let children: Vec<&serde_json::Value> = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().collect())
            .unwrap_or_default();

        ui.columns(ncols, |cols| {
            for (i, child) in children.iter().enumerate() {
                if i >= ncols {
                    break;
                }
                super::super::renderer::render_node(&mut cols[i], child, ctx);
            }
        });
        return;
    }

    let layout = egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(wrap);
    let pad = get_padding(node, &ctx.inherited, &ctx.theme, "Row", egui::Margin::ZERO);

    let response = if pad == egui::Margin::ZERO {
        ui.scope_builder(
            egui::UiBuilder::new().layout(layout),
            |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, gap_row as f32);
                render_row_children(ui, node, ctx, gap as f32);
            },
        );
        None
    } else {
        Some(egui::Frame::new()
            .inner_margin(pad)
            .show(ui, |ui| {
                ui.scope_builder(
                    egui::UiBuilder::new().layout(layout),
                    |ui| {
                        ui.style_mut().spacing.item_spacing = egui::vec2(0.0, gap_row as f32);
                        render_row_children(ui, node, ctx, gap as f32);
                    },
                );
            }))
    };
    if let Some(r) = response {
        widget_border(ui, r.response.rect, node, &ctx.theme, "Row", egui::CornerRadius::same(4), Some(&r.response), true);
    }
}

fn render_row_children(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx, gap: f32) {
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
    fn test_smoke_row() {
        let json = serde_json::json!({
            "type": "Row",
            "gap": 4,
            "children": [{"type": "Button", "text": "A"}, {"type": "Button", "text": "B"}]
        });
        assert_eq!(json["type"], "Row");
        assert_eq!(json["children"].as_array().unwrap().len(), 2);
    }
}


