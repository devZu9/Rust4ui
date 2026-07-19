use crate::renderer::RenderCtx;

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, _ctx: &RenderCtx) {
    let width = node.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if width > 0.0 {
        ui.add_space(width as f32);
    } else {
        let avail = ui.available_size();
        ui.allocate_space(egui::vec2(avail.x, 0.0));
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_smoke_spacer() {
        let json = serde_json::json!({"type": "Spacer"});
        assert_eq!(json["type"], "Spacer");
    }
}
