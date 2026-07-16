use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let action = attr_str(node, "action");
    let target = attr_str(node, "target");
    let enabled = node
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let raw_text = attr_str(node, "text").unwrap_or("");
    let text = resolve_text(raw_text, ctx);
    let icon_name = attr_str(node, "icon");
    let shortcut = attr_str(node, "shortcut");
    let size = attr_f64(node, "size")
        .unwrap_or_else(|| ctx.theme.w_f64("MenuItem", "size", 14.0)) as f32;

    let prefix = icon_name.and_then(|n| ctx.icons.resolve(n)).unwrap_or("");
    let label = if let Some(sc) = shortcut {
        format!("{prefix} {text}  {sc}")
    } else {
        format!("{prefix} {text}")
    };

    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color)
        .or_else(|| ctx.theme.w_color_opt("MenuItem", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let color_icon = node
        .get("color_icon")
        .and_then(crate::theme::parse_color_value)
        .unwrap_or(color);

    let base_rounding = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuItem", "rounding", 4.0)))
        .unwrap_or(4.0);

    let font_id = egui::FontId::proportional(size);
    let galley = ui.painter().layout_no_wrap(label.clone(), font_id, color_icon);
    let csize = galley.size();

    let out = crate::widgets::base::widget_base(
        ui, node, &ctx.theme, "MenuItem",
        csize, egui::Sense::click(), enabled,
        egui::Color32::TRANSPARENT, base_rounding,
        egui::Margin::ZERO,
        ctx.inherited_bg,
    );

    let text_pos = egui::pos2(out.inner_rect.left(), egui::Align::Center.align_size_within_range(csize.y, out.inner_rect.y_range()).min);
    ui.painter().galley_with_override_text_color(text_pos, galley, color_icon);

    if out.response.clicked() && enabled {
        if let Some(action_name) = action {
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target.unwrap_or(""))
                .with_state(&ctx.state);
            ctx.actions.invoke(action_name, &mut action_ctx);
            ctx.state = action_ctx.state;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_menuitem() {
        let json = serde_json::json!({"type": "MenuItem", "text": "Copy", "action": "copy"});
        assert_eq!(attr_str(&json, "text"), Some("Copy"));
    }
}
