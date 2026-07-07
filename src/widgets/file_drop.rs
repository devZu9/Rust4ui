use crate::renderer::{attr_bool, attr_f64, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let action = attr_str(node, "action");
    let _accept = node.get("accept").and_then(|v| v.as_array());
    let multi = attr_bool(node, "multi").unwrap_or(false);

    let highlight_color = attr_str(node, "highlight_color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgba_unmultiplied(
            0x33, 0x66, 0xCC, 0x44,
        ));

    let fill_str = attr_str(node, "fill");
    let fill = fill_str
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x1A, 0x1D, 0x23));

    let rounding = attr_f64(node, "rounding").unwrap_or(8.0);
    let stroke_width = attr_f64(node, "stroke_width").unwrap_or(1.0);
    let stroke_color = attr_str(node, "stroke_color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x33, 0x33, 0x33));
    let padding = attr_f64(node, "padding").unwrap_or(16.0);

    let available = ui.available_size();
    let desired = egui::vec2(available.x, 120.0);
    let (rect, resp) = ui.allocate_at_least(desired, egui::Sense::hover());

    let is_hovering = resp.hovered();
    let bg = if is_hovering { highlight_color } else { fill };

    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(rounding as u8), bg);
    if stroke_width > 0.0 {
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(rounding as u8),
            egui::Stroke::new(stroke_width as f32, stroke_color),
            egui::StrokeKind::Inside,
        );
    }

    let dropped = ctx_dropped_files(ui);

    if !dropped.is_empty() && is_hovering {
        if let Some(action_name) = action {
            let target = if multi {
                serde_json::to_string(&dropped).unwrap_or_default()
            } else {
                dropped[0].clone()
            };
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(&target)
                .with_state(&ctx.state);
            ctx.actions.invoke(action_name, &mut action_ctx);
        }
    }

    let inner_rect = rect.shrink(padding as f32);
    ui.scope_builder(egui::UiBuilder::new().max_rect(inner_rect), |ui| {
        if let Some(children_arr) = node.get("children").and_then(|v| v.as_array()) {
            for child in children_arr {
                super::super::renderer::render_node(ui, child, ctx);
            }
        }
    });
}

fn ctx_dropped_files(ui: &egui::Ui) -> Vec<String> {
    let mut files = Vec::new();
    ui.input(|i| {
        for file in &i.raw.dropped_files {
            if let Some(path) = &file.path {
                files.push(path.to_string_lossy().to_string());
            }
        }
    });
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_filedrop() {
        let json = serde_json::json!({
            "type": "FileDrop",
            "action": "file_dropped",
            "accept": [".json"]
        });
        assert_eq!(attr_str(&json, "action"), Some("file_dropped"));
    }
}
