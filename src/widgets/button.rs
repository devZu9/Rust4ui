use crate::border::{draw_shadow_content, parse_shadow_content, Shadow, ShadowZOrder};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_padding, resolve_text, RenderCtx};
use crate::theme::Theme;

fn cascade_shadow(node: &serde_json::Value, theme: &Theme, widget: &str, key: &str, resp: &egui::Response, enabled: bool, fallback: Shadow) -> Shadow {
    let has = node.get(key).is_some()
        || node.get(&format!("{}_hover", key)).is_some()
        || node.get(&format!("{}_click", key)).is_some()
        || node.get(&format!("{}_focus", key)).is_some()
        || theme.widget.get(widget).and_then(|w| w.get(key)).is_some()
        || theme.widget.get(widget).and_then(|w| w.get(&format!("{}_hover", key))).is_some()
        || theme.widget.get(widget).and_then(|w| w.get(&format!("{}_click", key))).is_some()
        || theme.widget.get(widget).and_then(|w| w.get(&format!("{}_focus", key))).is_some();
    if has {
        crate::renderer::get_state_attr(node, theme, widget, key, resp, enabled, Shadow::transparent(), parse_shadow_content)
    } else {
        fallback
    }
}

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let raw_text = attr_str(node, "text").unwrap_or("");
    let icon_name = attr_str(node, "icon");

    if raw_text.is_empty() && icon_name.is_none() {
        log::warn!("Button: отсутствует атрибут 'text' и 'icon'");
    }

    let enabled = attr_bool(node, "enabled").unwrap_or(true);
    let min_width = attr_f64(node, "min_width")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "min_width", 100.0)) as f32;
    let min_height = ctx.theme.w_f64("Button", "height", 28.0) as f32;
    let _rounding = attr_f64(node, "rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "rounding", 6.0));

    let tooltip_text = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));
    let align_default = attr_str(node, "align").unwrap_or("center");

    let padding = get_padding(node, &ctx.inherited, &ctx.theme, egui::Margin::symmetric(16, 4));
    let color_text = node.get("color_text")
        .and_then(crate::theme::parse_color)
        .unwrap_or_else(|| ctx.theme.w_color("Button", "color_text", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let color_icon = node.get("color_icon")
        .and_then(crate::theme::parse_color)
        .or_else(|| ctx.theme.w_color_opt("Button", "color_icon"))
        .unwrap_or(color_text);

    let font_id = egui::FontId::proportional(14.0);
    let icon_glyph = icon_name.and_then(|n| ctx.icons.resolve(n));
    let has_icon = icon_glyph.is_some();
    let has_text = !raw_text.is_empty();

    let icon_galley = icon_glyph.map(|g|
        ui.painter().layout_no_wrap(g.to_string(), font_id.clone(), color_icon));
    let text_galley = if has_text {
        Some(ui.painter().layout_no_wrap(resolve_text(raw_text, ctx), font_id, color_text))
    } else {
        None
    };

    let icon_size = icon_galley.as_ref().map_or(egui::Vec2::ZERO, |g| g.size());
    let text_size = text_galley.as_ref().map_or(egui::Vec2::ZERO, |g| g.size());
    let gap = if has_icon && has_text { 6.0 } else { 0.0 };

    let (padding_left, padding_right, padding_top, padding_bottom) = (padding.left as f32, padding.right as f32, padding.top as f32, padding.bottom as f32);
    let min_width_content = (min_width - padding_left - padding_right).max(0.0);
    let min_height_content = (min_height - padding_top - padding_bottom).max(0.0);
    let content_width = (icon_size.x + gap + text_size.x).max(min_width_content);
    let content_height = (icon_size.y.max(text_size.y)).max(min_height_content);

    let out = crate::widgets::base::widget_paint_custom(
        ui, node, ctx,
        egui::vec2(content_width, content_height),
        egui::Sense::click_and_drag(), enabled,
    );

    let align = if enabled {
        if out.response.is_pointer_button_down_on() {
            attr_str(node, "align_click").unwrap_or(align_default)
        } else if out.response.hovered() {
            attr_str(node, "align_hover").unwrap_or(align_default)
        } else {
            align_default
        }
    } else {
        align_default
    };
    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };

    let actual_text = if enabled {
        crate::renderer::get_state_attr(node, &ctx.theme, "Button", "color_text", &out.response, true, color_text, crate::theme::parse_color)
    } else {
        egui::Color32::from_gray(100)
    };

    let shadow_content = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "shadow_content", &out.response, true,
        Shadow::transparent(), parse_shadow_content);
    let shadow_icon = cascade_shadow(node, &ctx.theme, "Button", "shadow_icon", &out.response, enabled, shadow_content);
    let shadow_text = cascade_shadow(node, &ctx.theme, "Button", "shadow_text", &out.response, enabled, shadow_content);

    let start_x = halign.align_size_within_range(icon_size.x + gap + text_size.x, out.inner_rect.x_range()).min;
    if let Some(ig) = &icon_galley {
        let y = egui::Align::Center.align_size_within_range(icon_size.y, out.inner_rect.y_range()).min;
        let pos = egui::pos2(start_x, y);
        if shadow_icon.z_order == ShadowZOrder::Under {
            draw_shadow_content(ui, pos, ig.clone(), &shadow_icon);
            ui.painter().galley(pos, ig.clone(), color_icon);
        } else {
            ui.painter().galley(pos, ig.clone(), color_icon);
            draw_shadow_content(ui, pos, ig.clone(), &shadow_icon);
        }
    }
    if let Some(tg) = &text_galley {
        let y = egui::Align::Center.align_size_within_range(text_size.y, out.inner_rect.y_range()).min;
        let pos = egui::pos2(start_x + icon_size.x + gap, y);
        if shadow_text.z_order == ShadowZOrder::Under {
            draw_shadow_content(ui, pos, tg.clone(), &shadow_text);
            ui.painter().galley_with_override_text_color(pos, tg.clone(), actual_text);
        } else {
            ui.painter().galley_with_override_text_color(pos, tg.clone(), actual_text);
            draw_shadow_content(ui, pos, tg.clone(), &shadow_text);
        }
    }

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
    fn test_smoke_button() {
        let json = serde_json::json!({"type": "Button", "text": "OK"});
        assert_eq!(attr_str(&json, "text"), Some("OK"));
        assert!(attr_bool(&json, "enabled").unwrap_or(true));
    }
}





