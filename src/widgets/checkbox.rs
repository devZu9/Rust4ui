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

    let mut state = ctx.state.clone();

    let (_, _resp) = crate::widgets::base::widget_paint_egui(
        ui, node, ctx,
        egui::vec2(200.0, 24.0), egui::Sense::click(), true,
        |ui| {
            let r = ui.checkbox(&mut checked, text);
            if r.changed() {
                state.set_bool(&binding, checked);
            }
        },
    );
    ctx.state = state;
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
