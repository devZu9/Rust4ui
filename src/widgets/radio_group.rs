use crate::renderer::{attr_str, resolve_text, RenderCtx};

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

    let (_, _) = crate::widgets::base::widget_base_wrap(
        ui, node, &ctx.theme, "RadioGroup",
        egui::vec2(200.0, (options.len() as f32) * 24.0), egui::Sense::click(), true,
        egui::Color32::TRANSPARENT, 4.0, egui::Margin::ZERO, None,
        |ui| {
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
        },
    );

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
