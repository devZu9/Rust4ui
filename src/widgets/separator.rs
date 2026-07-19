use crate::renderer::{attr_f64, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let space = attr_f64(node, "space").unwrap_or(6.0);
    ui.add_space(space as f32);

    let sep_w = ui.available_width().max(50.0);

    // Separator не наследует _children (padding, margin, цвет — только свои настройки)
    let saved_inherited = std::mem::take(&mut ctx.inherited);

    let (_, _) = crate::widgets::base::widget_paint_egui(
        ui, node, ctx,
        egui::vec2(sep_w, 4.0), egui::Sense::hover(), true,
        |ui| { ui.separator(); },
    );

    ctx.inherited = saved_inherited;
    ui.add_space(space as f32);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_smoke_separator() {
        let json = serde_json::json!({"type": "Separator"});
        assert_eq!(json["type"], "Separator");
    }
}
