use crate::border::{draw_border, get_state_border};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_margin, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {

    let raw_text = attr_str(node, "text").unwrap_or("");
    let icon_name = attr_str(node, "icon");

    if raw_text.is_empty() && icon_name.is_none() {
        log::warn!("Button: отсутствует атрибут 'text' и 'icon'");
    }

    let enabled = attr_bool(node, "enabled").unwrap_or(true);
    let min_width = attr_f64(node, "min_width")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "min_width", 100.0));
    let min_height = ctx.theme.w_f64("Button", "height", 28.0) as f32;

    let rounding = attr_f64(node, "rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "rounding", 6.0));

    let tooltip_text = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));
    let align = attr_str(node, "align").unwrap_or("center");

    let pad = get_padding(node, &ctx.theme, "Button", egui::Margin::symmetric(16, 4));
    let margin = get_margin(node, &ctx.theme, "Button");

    let color_text = node.get("color_text")
        .and_then(crate::theme::parse_color_value)
        .unwrap_or_else(|| ctx.theme.w_color("Button", "color_text", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let color_icon = node.get("color_icon")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Button", "color_icon"))
        .unwrap_or(color_text);

    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };
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

    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let content_w = icon_sz.x + gap + text_sz.x;
    let content_h = icon_sz.y.max(text_sz.y);

    let desired_w = (content_w + pad_l + pad_r).max(min_width as f32);
    let desired_h = (content_h + pad_t + pad_b).max(min_height);

    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let total_w = desired_w + m_l + m_r;
    let total_h = desired_h + m_t + m_b;

    let size = egui::vec2(total_w, total_h);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
    let border = get_state_border(node, &ctx.theme, "Button", &resp, enabled);

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
    let shadow = crate::border::get_shadow(node, &ctx.theme, "Button");
    crate::border::draw_shadow(ui, content_rect, rounding_cr, &shadow);
    ui.painter().rect_filled(content_rect, rounding_cr, actual_fill);
    draw_border(ui, content_rect, rounding_cr, &border);

    let inner = egui::Rect::from_min_max(
        egui::pos2(content_rect.left() + pad_l, content_rect.top() + pad_t),
        egui::pos2(content_rect.right() - pad_r, content_rect.bottom() - pad_b),
    );

    let start_x = halign.align_size_within_range(content_w, inner.x_range()).min;
    if let Some(ig) = &icon_galley {
        let y = valign.align_size_within_range(icon_sz.y, inner.y_range()).min;
        ui.painter().galley(egui::pos2(start_x, y), ig.clone(), color_icon);
    }
    if let Some(tg) = &text_galley {
        let y = valign.align_size_within_range(text_sz.y, inner.y_range()).min;
        ui.painter().galley_with_override_text_color(egui::pos2(start_x + icon_sz.x + gap, y), tg.clone(), actual_text);
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
