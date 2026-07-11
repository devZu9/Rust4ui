use crate::border::widget_border;
use crate::renderer::{attr_str, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {

    let binding = match attr_str(node, "binding") {
        Some(key) => key.to_string(),
        None => return,
    };

    let direction = attr_str(node, "direction").unwrap_or("vertical");

    let mut selected = ctx.state.get_usize(&binding).unwrap_or(0usize);
    let options: Vec<(usize, String)> = match node.get("options").and_then(|v| v.as_array()) {
        Some(arr) => arr
            .iter()
            .filter_map(|opt| {
                let val = opt.get("value").and_then(|v| v.as_u64())? as usize;
                let label = opt
                    .get("text")
                    .and_then(|v| v.as_str())
                    .map(|t| resolve_text(t, ctx))
                    .unwrap_or_default();
                Some((val, label))
            })
            .collect(),
        None => return,
    };

    let pad = get_padding(node, &ctx.theme, "RadioGroup", egui::Margin::ZERO);

    let mut render_radios = |ui: &mut egui::Ui| {
        for (val, label) in &options {
            let is_current = selected == *val;
            if direction == "horizontal" {
                if ui.selectable_label(is_current, label).clicked() {
                    selected = *val;
                }
            } else {
                if ui.radio_value(&mut selected, *val, label.as_str()).clicked() {
                    selected = *val;
                }
            }
        }
    };

    let response = if pad != egui::Margin::ZERO {
        Some(egui::Frame::new()
            .inner_margin(pad)
            .show(ui, render_radios))
    } else {
        render_radios(ui);
        None
    };
    if let Some(r) = response {
        widget_border(ui, r.response.rect, node, &ctx.theme, "RadioGroup", egui::CornerRadius::same(4), Some(&r.response), true);
    }

    ctx.state.set_usize(&binding, selected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_radio() {
        let json = serde_json::json!({
            "type": "RadioGroup",
            "binding": "theme",
            "options": [
                {"value": 0, "text": "Dark"},
                {"value": 1, "text": "Light"}
            ]
        });
        assert_eq!(attr_str(&json, "binding"), Some("theme"));
    }
}
