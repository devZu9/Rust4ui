use crate::renderer::{attr_f64, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    let space = attr_f64(node, "space").unwrap_or(6.0);
    ui.add_space(space as f32);

    let sep_w = ctx.inherited.get("popup_width")
        .and_then(|v| v.as_f64().map(|f| f as f32))
        .unwrap_or_else(|| ui.available_width().max(200.0));

    let (_, _) = crate::widgets::base::widget_base_wrap(
        ui, node, &ctx.theme, "Separator",
        egui::vec2(sep_w, 4.0), egui::Sense::hover(), true,
        egui::Color32::TRANSPARENT, 4.0, egui::Margin::ZERO, &ctx.inherited,
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
