use crate::renderer::{attr_bool, attr_f64, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let columns = attr_f64(node, "columns").unwrap_or(2.0) as usize;
    let spacing_x = attr_f64(node, "spacing_x").unwrap_or(8.0) as f32;
    let spacing_y = attr_f64(node, "spacing_y").unwrap_or(4.0) as f32;
    let striped = attr_bool(node, "striped").unwrap_or(false);

    let children = match node.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    if children.is_empty() {
        return;
    }

    let mut grid = egui::Grid::new(node.get("id").and_then(|v| v.as_str()).unwrap_or("grid"))
        .num_columns(columns)
        .min_col_width(60.0)
        .spacing([spacing_x, spacing_y]);

    if striped {
        grid = grid.striped(true);
    }

    grid.show(ui, |ui| {
        for (i, child) in children.iter().enumerate() {
            super::super::renderer::render_node(ui, child, ctx);
            if (i + 1) % columns == 0 {
                ui.end_row();
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_grid() {
        let json = serde_json::json!({
            "type": "Grid",
            "columns": 3,
            "children": [
                {"type": "Label", "text": "A"},
                {"type": "Label", "text": "B"},
                {"type": "Label", "text": "C"}
            ]
        });
        assert_eq!(attr_f64(&json, "columns"), Some(3.0));
    }
}
