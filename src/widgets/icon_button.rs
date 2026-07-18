use crate::border::{parse_content_shadow, Shadow, ShadowZOrder};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let raw_text = attr_str(node, "text").unwrap_or("");
    let icon_name = attr_str(node, "icon");
    let text = if let Some(icon) = icon_name.and_then(|n| ctx.icons.resolve(n)) {
        format!("{}", icon)
    } else {
        resolve_text(raw_text, ctx)
    };

    if raw_text.is_empty() && icon_name.is_none() {
        log::warn!("IconButton: отсутствует атрибут 'text' и 'icon'");
    }

    let enabled = attr_bool(node, "enabled").unwrap_or(true);
    let base_button_size = attr_f64(node, "button_size")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "button_size", 24.0)) as f32;
    let base_rounding = attr_f64(node, "rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "rounding", 6.0));
    let tooltip_text = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));
    let align = attr_str(node, "align").unwrap_or("center");

    let base_pad = get_padding(node, &ctx.inherited, &ctx.theme, "IconButton", None, egui::Margin::ZERO);
    let color = node.get("color")
        .and_then(crate::theme::parse_color_value)
        .unwrap_or_else(|| ctx.theme.w_color("IconButton", "color", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };

    let base_icon_size = attr_f64(node, "icon_size")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "icon_size", 14.0)) as f32;
    let maket = ui.painter().layout_no_wrap(text.clone(), egui::FontId::proportional(base_icon_size), color);

    let (pad_l, pad_r, pad_t, pad_b) = (base_pad.left as f32, base_pad.right as f32, base_pad.top as f32, base_pad.bottom as f32);
    let content_w = (maket.size().x + pad_l + pad_r).max(base_button_size);
    let content_h = (base_icon_size + pad_t + pad_b).max(base_button_size);

    let out = crate::widgets::base::widget_paint_custom(
        ui, node, &ctx.theme, "IconButton",
        egui::vec2(content_w, content_h),
        egui::Sense::click_and_drag(), enabled,
        &ctx.inherited,
    );

    let actual_text = if enabled {
        crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "color", &out.response, true, color, crate::theme::parse_color_value)
    } else {
        egui::Color32::from_gray(100)
    };

    let icon_size = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "icon_size", &out.response, true, base_icon_size, |v| v.as_f64().map(|x| x as f32));
    let maket = ui.painter().layout_no_wrap(text.clone(), egui::FontId::proportional(icon_size), color);

    let text_x = halign.align_size_within_range(maket.size().x, out.inner_rect.x_range()).min;
    let text_y = egui::Align::Center.align_size_within_range(maket.size().y, out.inner_rect.y_range()).min;
    let text_pos = egui::pos2(text_x, text_y);

    let shadow_icon = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "shadow_icon", &out.response, true,
        Shadow { color: egui::Color32::TRANSPARENT, offset: egui::Vec2::ZERO, z_order: ShadowZOrder::Under }, parse_content_shadow);
    if shadow_icon.is_visible() {
        ui.painter().galley_with_override_text_color(text_pos + shadow_icon.offset, maket.clone(), shadow_icon.color);
    }
    ui.painter().galley_with_override_text_color(text_pos, maket, actual_text);

    if let Some(tip) = &tooltip_text {
        if !tip.is_empty() {
            out.response.clone().on_hover_text(tip.as_str());
        }
    }

    if out.response.clicked() && enabled {
        if let Some(action_name) = attr_str(node, "action") {
            let target = attr_str(node, "target").unwrap_or("");
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target)
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
    fn test_smoke_iconbutton() {
        let json = serde_json::json!({"type": "IconButton", "icon": "save", "action": "save"});
        assert_eq!(attr_str(&json, "icon"), Some("save"));
    }
}

