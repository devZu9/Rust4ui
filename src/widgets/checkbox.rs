use crate::border::widget_border;
use crate::renderer::{attr_str, get_padding, resolve_text, RenderCtx};

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

    let pad = get_padding(node, &ctx.theme, "Checkbox", egui::Margin::ZERO);

    let mut checked = ctx.state.get_bool(&binding).unwrap_or(false);

    let do_checkbox = |ui: &mut egui::Ui| {
        let resp = ui.checkbox(&mut checked, text);
        if resp.changed() {
            ctx.state.set_bool(&binding, checked);
        }
    };

    let response = if pad != egui::Margin::ZERO {
        Some(egui::Frame::new()
            .inner_margin(pad)
            .show(ui, do_checkbox))
    } else {
        do_checkbox(ui);
        None
    };
    if let Some(r) = response {
        widget_border(ui, r.response.rect, node, &ctx.theme, "Checkbox", egui::CornerRadius::same(4));
    }
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
