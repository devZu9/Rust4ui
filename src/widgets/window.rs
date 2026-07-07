use crate::border::get_border;
use crate::renderer::{attr_bool, attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let open_binding = attr_str(node, "open");
    let title = attr_str(node, "title")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_default();

    let id = attr_str(node, "id").unwrap_or("window");
    let title_bar = attr_bool(node, "title_bar").unwrap_or(true);
    let default_w = attr_f64(node, "default_width").unwrap_or(400.0);
    let default_h = attr_f64(node, "default_height").unwrap_or(300.0);
    let movable = attr_bool(node, "movable").unwrap_or(true);
    let resizable = attr_bool(node, "resizable").unwrap_or(true);
    let collapsible = attr_bool(node, "collapsible").unwrap_or(true);
    let constrain = attr_bool(node, "constrain").unwrap_or(true);

    let min_w = attr_f64(node, "min_width").unwrap_or(100.0);
    let min_h = attr_f64(node, "min_height").unwrap_or(80.0);

    let fill_str = attr_str(node, "fill");
    let fill = fill_str
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0x1E, 0x1E, 0x24));

    let border = get_border(node, &ctx.theme, "Window");
    let padding = attr_f64(node, "padding").unwrap_or(8.0);

    let anchor_h = attr_str(node, "anchor_h");
    let auto_sized = attr_bool(node, "auto_sized").unwrap_or(false);
    let _show_close = attr_bool(node, "show_close").unwrap_or(true);
    let anchor_x = attr_f64(node, "anchor_x").unwrap_or(0.0);
    let anchor_y = attr_f64(node, "anchor_y").unwrap_or(0.0);

    if open_binding.is_some() && !ctx.state.get_bool(open_binding.unwrap()).unwrap_or(false) {
        return;
    }

    if open_binding.is_none() {
        log::warn!("Window: отсутствует атрибут 'open' (binding), окно не будет управляться");
    }

    let mut window = egui::Window::new(title)
        .id(egui::Id::new(id))
        .default_size([default_w as f32, default_h as f32])
        .min_width(min_w as f32)
        .min_height(min_h as f32)
        .resizable(resizable)
        .movable(movable)
        .collapsible(collapsible)
        .constrain(constrain);

    if auto_sized {
        window = window.auto_sized();
    }
    if !title_bar {
        window = window.title_bar(false);
    }

    if let Some(h) = anchor_h {
        match h {
            "left" => {
                window = window.anchor(egui::Align2::LEFT_TOP, [anchor_x as f32, anchor_y as f32])
            }
            "center" => {
                window = window.anchor(egui::Align2::CENTER_TOP, [anchor_x as f32, anchor_y as f32])
            }
            "right" => {
                window = window.anchor(egui::Align2::RIGHT_TOP, [anchor_x as f32, anchor_y as f32])
            }
            _ => {}
        }
    }

    let frame = egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(border.width, border.color))
        .inner_margin(egui::Margin::same(padding as i8));

    window.frame(frame).show(ui.ctx(), |ui| {
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
    fn test_smoke_window() {
        let json = serde_json::json!({
            "type": "Window",
            "id": "test_win",
            "title": "Test",
            "open": "show_test"
        });
        assert_eq!(attr_str(&json, "id"), Some("test_win"));
    }
}
