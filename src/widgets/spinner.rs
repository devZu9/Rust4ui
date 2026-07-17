use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_default();

    let (_, _) = crate::widgets::base::widget_base_wrap(
        ui, node, &ctx.theme, "Spinner",
        egui::vec2(200.0, 24.0), egui::Sense::hover(), true,
        egui::Color32::TRANSPARENT, 4.0, egui::Margin::symmetric(4, 0), &ctx.inherited,
        |ui| {
            ui.horizontal(|ui| {
                ui.spinner();
                if !text.is_empty() {
                    ui.add_space(8.0);
                    ui.label(&text);
                }
            });
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_spinner() {
        let json = serde_json::json!({"type": "Spinner", "size": 24});
        assert_eq!(attr_f64(&json, "size"), Some(24.0));
    }
}
