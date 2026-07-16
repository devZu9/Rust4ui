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

    let bg = node
        .get("background")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg)
        .or_else(|| ctx.theme.w_color_opt("MenuItem", "background"))
        .unwrap_or_else(|| egui::Color32::TRANSPARENT);

    let bg_hover = node
        .get("background_hover")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuItem", "background_hover"))
        .unwrap_or_else(|| egui::Color32::from_rgb(0x3A, 0x3A, 0x44));

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

    let rounding_val = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuItem", "rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let radius = egui::CornerRadius::same(rounding_val);

    let (prev_inactive, prev_hovered, prev_active) = {
        let w = &mut ui.style_mut().visuals.widgets;
        let prev = (w.inactive.clone(), w.hovered.clone(), w.active.clone());
        w.inactive.bg_fill = bg;
        w.inactive.corner_radius = radius;
        w.hovered.bg_fill = bg_hover;
        w.hovered.corner_radius = radius;
        w.active.bg_fill = bg_hover;
        w.active.corner_radius = radius;
        prev
    };

    if ui
        .add_enabled(
            enabled,
            egui::Button::new(
                egui::RichText::new(label).size(size).color(color_icon),
            ),
        )
        .clicked()
        && enabled
    {
        if let Some(action_name) = action {
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target.unwrap_or(""))
                .with_state(&ctx.state);
            ctx.actions.invoke(action_name, &mut action_ctx);
            ctx.state = action_ctx.state;
        }
    }

    {
        let w = &mut ui.style_mut().visuals.widgets;
        w.inactive = prev_inactive;
        w.hovered = prev_hovered;
        w.active = prev_active;
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
