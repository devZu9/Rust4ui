use crate::renderer::{attr_bool, attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let binding = match attr_str(node, "binding") {
        Some(key) => key.to_string(),
        None => return,
    };

    let alpha = attr_bool(node, "alpha").unwrap_or(false);

    let color_str = ctx
        .state
        .get_string(&binding)
        .unwrap_or("#FF6633")
        .to_string();
    let color = crate::theme::parse_color_hex(&color_str)
        .unwrap_or(egui::Color32::from_rgb(0xFF, 0x66, 0x33));

    let (_, _) = crate::widgets::base::widget_paint_egui(
        ui, node, &ctx.theme, "ColorPicker",
        egui::vec2(200.0, 24.0), egui::Sense::hover(), true,
        &ctx.inherited,
        |ui| {
            let new_color = if alpha {
                let mut rgba = [
                    color.r() as f32 / 255.0,
                    color.g() as f32 / 255.0,
                    color.b() as f32 / 255.0,
                    color.a() as f32 / 255.0,
                ];
                ui.color_edit_button_rgba_premultiplied(&mut rgba);
                egui::Color32::from_rgba_premultiplied(
                    (rgba[0] * 255.0) as u8,
                    (rgba[1] * 255.0) as u8,
                    (rgba[2] * 255.0) as u8,
                    (rgba[3] * 255.0) as u8,
                )
            } else {
                let mut srgb = [color.r(), color.g(), color.b()];
                ui.color_edit_button_srgb(&mut srgb);
                egui::Color32::from_rgb(srgb[0], srgb[1], srgb[2])
            };

            if new_color != color {
                let hex = format!("#{:02X}{:02X}{:02X}", new_color.r(), new_color.g(), new_color.b());
                ctx.state.set_string(&binding, hex);
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_colorpicker() {
        let json = serde_json::json!({"type": "ColorPicker", "binding": "accent", "alpha": false});
        assert_eq!(attr_str(&json, "binding"), Some("accent"));
    }
}

