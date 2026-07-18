use crate::renderer::{attr_f64, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let binding = match attr_str(node, "binding") {
        Some(key) => key.to_string(),
        None => {
            log::warn!("Slider: отсутствует атрибут 'binding'");
            return;
        }
    };

    let min = attr_f64(node, "min").unwrap_or(0.0);
    let max = attr_f64(node, "max").unwrap_or(1.0);
    let step = if let Some(s) = attr_f64(node, "step") {
        if s == 0.0 { 0.01 } else { s }
    } else {
        0.01
    };
    let width = attr_f64(node, "width").unwrap_or(250.0);

    let mut value = ctx.state.get_f64(&binding).unwrap_or(min);

    let (_, resp) = crate::widgets::base::widget_paint_egui(
        ui, node, &ctx.theme, "Slider",
        egui::vec2(width as f32, 20.0), egui::Sense::click(), true,
        egui::Color32::TRANSPARENT, 4.0, egui::Margin::ZERO, &ctx.inherited,
        |ui| {
            let slider = egui::Slider::new(&mut value, min..=max)
                .step_by(step)
                .show_value(false)
                .trailing_fill(false);
            let r = ui.add_sized(egui::vec2(width as f32, 20.0), slider);
            if r.changed() {
                ctx.state.set_f64(&binding, value);
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_slider() {
        let json = serde_json::json!({
            "type": "Slider",
            "binding": "volume",
            "min": 0,
            "max": 100
        });
        assert_eq!(attr_str(&json, "binding"), Some("volume"));
        assert_eq!(attr_f64(&json, "min"), Some(0.0));
    }
}
