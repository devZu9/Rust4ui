use crate::renderer::{attr_f64, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let axis = attr_str(node, "axis").unwrap_or("vertical");
    let max_height = attr_f64(node, "max_height");
    let max_width = attr_f64(node, "max_width");

    let render_children = |ui: &mut egui::Ui| {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                super::super::renderer::render_node(ui, child, ctx);
            }
        }
    };

    match axis {
        "horizontal" => {
            let mut scroll = egui::ScrollArea::horizontal().id_salt("scroll_h");
            if let Some(h) = max_height {
                scroll = scroll.max_height(h as f32);
            }
            if let Some(w) = max_width {
                scroll = scroll.max_width(w as f32);
            }
            scroll.show(ui, render_children);
        }
        "both" => {
            let mut scroll = egui::ScrollArea::both().id_salt("scroll_b");
            if let Some(h) = max_height {
                scroll = scroll.max_height(h as f32);
            }
            if let Some(w) = max_width {
                scroll = scroll.max_width(w as f32);
            }
            scroll.show(ui, render_children);
        }
        _ => {
            let mut scroll = egui::ScrollArea::vertical().id_salt("scroll_v");
            if let Some(h) = max_height {
                scroll = scroll.max_height(h as f32);
            }
            if let Some(w) = max_width {
                scroll = scroll.max_width(w as f32);
            }
            scroll.show(ui, render_children);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_scroll() {
        let json = serde_json::json!({
            "type": "ScrollArea",
            "axis": "vertical",
            "max_height": 200
        });
        assert_eq!(attr_str(&json, "axis"), Some("vertical"));
    }
}
