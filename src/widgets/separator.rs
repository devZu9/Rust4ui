use crate::renderer::{attr_f64, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    let space = attr_f64(node, "space").unwrap_or(6.0);
    ui.add_space(space as f32);

    let sep_w = ui.available_width().max(50.0);
    let empty_inherited = std::collections::HashMap::new();

    let (_, _) = crate::widgets::base::widget_base_wrap(
        ui, node, &ctx.theme, "Separator",
        egui::vec2(sep_w, 4.0), egui::Sense::hover(), true,
        egui::Color32::TRANSPARENT, 4.0, egui::Margin::ZERO, &empty_inherited,
        |ui| {
            ui.separator();
        },
    );

    ui.add_space(space as f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_separator() {
        let json = serde_json::json!({"type": "Separator"});
        assert_eq!(json["type"], "Separator");
    }
}
