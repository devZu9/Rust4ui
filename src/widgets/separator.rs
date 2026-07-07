use crate::renderer::{attr_f64, widget_margin, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    widget_margin(ui, &ctx.theme, "Separator");
    let space = attr_f64(node, "space").unwrap_or(6.0);
    ui.add_space(space as f32);
    ui.separator();
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
