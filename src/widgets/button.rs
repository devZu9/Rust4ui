use crate::border::{draw_border, draw_shadow_bg, draw_shadow_border, draw_shadow_content, get_state_border, parse_content_shadow, parse_shadow, Shadow, ShadowZOrder};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_margin, get_padding, resolve_text, RenderCtx};
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
        crate::renderer::get_state_attr(node, theme, widget, key, resp, enabled, Shadow::transparent(), parse_content_shadow)
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
    let base_min_width = attr_f64(node, "min_width")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "min_width", 100.0));
    let base_min_height = ctx.theme.w_f64("Button", "height", 28.0) as f32;

    let base_rounding = attr_f64(node, "rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "rounding", 6.0));

    let tooltip_text = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));
    let base_align = attr_str(node, "align").unwrap_or("center");

    let base_pad = get_padding(node, &ctx.theme, "Button", egui::Margin::symmetric(16, 4));
    let base_margin = get_margin(node, &ctx.theme, "Button");

    let color_text = node.get("color_text")
        .and_then(crate::theme::parse_color_value)
        .unwrap_or_else(|| ctx.theme.w_color("Button", "color_text", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let color_icon = node.get("color_icon")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Button", "color_icon"))
        .unwrap_or(color_text);

    let valign = egui::Align::Center;

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

    let icon_sz = icon_galley.as_ref().map_or(egui::Vec2::ZERO, |g| g.size());
    let text_sz = text_galley.as_ref().map_or(egui::Vec2::ZERO, |g| g.size());
    let gap = if has_icon && has_text { 6.0 } else { 0.0 };

    let (base_pad_l, base_pad_r, base_pad_t, base_pad_b) = (base_pad.left as f32, base_pad.right as f32, base_pad.top as f32, base_pad.bottom as f32);

    let content_w = icon_sz.x + gap + text_sz.x;
    let content_h = icon_sz.y.max(text_sz.y);

    let desired_w = (content_w + base_pad_l + base_pad_r).max(base_min_width as f32);
    let desired_h = (content_h + base_pad_t + base_pad_b).max(base_min_height);

    let (base_m_l, base_m_r, base_m_t, base_m_b) = (base_margin.left as f32, base_margin.right as f32, base_margin.top as f32, base_margin.bottom as f32);
    let total_w = desired_w + base_m_l + base_m_r;
    let total_h = desired_h + base_m_t + base_m_b;

    let size = egui::vec2(total_w, total_h);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

    let align = if enabled {
        if resp.is_pointer_button_down_on() {
            attr_str(node, "align_click").unwrap_or(base_align)
        } else if resp.hovered() {
            attr_str(node, "align_hover").unwrap_or(base_align)
        } else {
            base_align
        }
    } else {
        base_align
    };
    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };

    let border = get_state_border(node, &ctx.theme, "Button", &resp, enabled);
    let rounding = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "rounding", &resp, true, base_rounding, |v| v.as_f64());

    let pad = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "padding", &resp, true, base_pad, crate::renderer::parse_padding);
    let margin = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "margin", &resp, true, base_margin, crate::renderer::parse_padding);

    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + m_l, rect.min.y + m_t),
        egui::pos2(rect.max.x - m_r, rect.max.y - m_b),
    );

    let actual_fill = crate::renderer::get_state_background(node, &ctx.theme, "Button", &resp, enabled,
        egui::Color32::from_rgb(0x30, 0x30, 0x30));
    let actual_text = if enabled {
        crate::renderer::get_state_attr(node, &ctx.theme, "Button", "color_text", &resp, true, color_text, crate::theme::parse_color_value)
    } else {
        egui::Color32::from_gray(100)
    };

    let rounding_cr = egui::CornerRadius::same(rounding as u8);
    let shadow_bg = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "shadow_background", &resp, true,
        Shadow::from_rgba(0, 0, 0, 40), parse_shadow);
    draw_shadow_bg(ui, content_rect, rounding_cr, &shadow_bg);
    ui.painter().rect_filled(content_rect, rounding_cr, actual_fill);
    let shadow_border = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "shadow_border", &resp, true,
        Shadow::transparent(), parse_shadow);
    if shadow_border.z_order == ShadowZOrder::Under {
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
        draw_border(ui, content_rect, rounding_cr, &border);
    } else {
        draw_border(ui, content_rect, rounding_cr, &border);
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
    }
    let shadow_content = crate::renderer::get_state_attr(node, &ctx.theme, "Button", "shadow_content", &resp, true,
        Shadow::transparent(), parse_content_shadow);
    let shadow_icon = cascade_shadow(node, &ctx.theme, "Button", "shadow_icon", &resp, enabled, shadow_content);
    let shadow_text = cascade_shadow(node, &ctx.theme, "Button", "shadow_text", &resp, enabled, shadow_content);

    let inner = egui::Rect::from_min_max(
        egui::pos2(content_rect.left() + pad_l, content_rect.top() + pad_t),
        egui::pos2(content_rect.right() - pad_r, content_rect.bottom() - pad_b),
    );

    let start_x = halign.align_size_within_range(content_w, inner.x_range()).min;
    if let Some(ig) = &icon_galley {
        let y = valign.align_size_within_range(icon_sz.y, inner.y_range()).min;
        let icon_pos = egui::pos2(start_x, y);
        if shadow_icon.z_order == ShadowZOrder::Under {
            draw_shadow_content(ui, icon_pos, ig.clone(), &shadow_icon);
            ui.painter().galley(icon_pos, ig.clone(), color_icon);
        } else {
            ui.painter().galley(icon_pos, ig.clone(), color_icon);
            draw_shadow_content(ui, icon_pos, ig.clone(), &shadow_icon);
        }
    }
    if let Some(tg) = &text_galley {
        let y = valign.align_size_within_range(text_sz.y, inner.y_range()).min;
        let text_pos = egui::pos2(start_x + icon_sz.x + gap, y);
        if shadow_text.z_order == ShadowZOrder::Under {
            draw_shadow_content(ui, text_pos, tg.clone(), &shadow_text);
            ui.painter().galley_with_override_text_color(text_pos, tg.clone(), actual_text);
        } else {
            ui.painter().galley_with_override_text_color(text_pos, tg.clone(), actual_text);
            draw_shadow_content(ui, text_pos, tg.clone(), &shadow_text);
        }
    }

    if let Some(tip) = &tooltip_text {
        if !tip.is_empty() {
            resp.clone().on_hover_text(tip.as_str());
        }
    }

    if resp.clicked() && enabled {
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
    use crate::renderer::RenderCtx;

    #[test]
    fn test_smoke_button() {
        let json = serde_json::json!({"type": "Button", "text": "OK"});
        let mut ctx = RenderCtx::new();
        assert_eq!(attr_str(&json, "text"), Some("OK"));
        assert!(attr_bool(&json, "enabled").unwrap_or(true));
    }
}
