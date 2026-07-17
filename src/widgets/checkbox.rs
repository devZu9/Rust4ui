use crate::renderer::{attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let binding = match attr_str(node, "binding") {
        Some(key) => key.to_string(),
        None => {
            log::warn!("Checkbox: отсутствует атрибут 'binding'");
            return;
        }
    };

    let raw_text = attr_str(node, "text").unwrap_or("");
    let text = resolve_text(raw_text, ctx);

    let mut checked = ctx.state.get_bool(&binding).unwrap_or(false);

    let (_, resp) = crate::widgets::base::widget_base_wrap(
        ui, node, &ctx.theme, "Checkbox",
        egui::vec2(200.0, 24.0), egui::Sense::click(), true,
        egui::Color32::TRANSPARENT, 4.0, egui::Margin::ZERO, &ctx.inherited,
        |ui| {
            let r = ui.checkbox(&mut checked, text);
            if r.changed() {
                ctx.state.set_bool(&binding, checked);
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_checkbox() {
        let json = serde_json::json!({"type": "Checkbox", "binding": "gpu", "text": "GPU"});
        assert_eq!(attr_str(&json, "binding"), Some("gpu"));
    }
}
