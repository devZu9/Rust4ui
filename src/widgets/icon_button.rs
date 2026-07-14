use crate::border::{draw_border, draw_shadow_bg, draw_shadow_border, draw_shadow_icon, get_state_border, parse_shadow, Shadow, ShadowZOrder};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_margin, get_padding, resolve_text, RenderCtx};

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

    let pad = get_padding(node, &ctx.theme, "IconButton", egui::Margin::symmetric(0, 0));
    let margin = get_margin(node, &ctx.theme, "IconButton");
    let base_pad = pad;
    let base_margin = margin;

    let color = node.get("color")
        .and_then(crate::theme::parse_color_value)
        .unwrap_or_else(|| ctx.theme.w_color("IconButton", "color", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };
    let valign = egui::Align::Center;

    let base_icon_size = attr_f64(node, "icon_size")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "icon_size", 14.0)) as f32;
    let maket = ui.painter().layout_no_wrap(
        text.clone(),
        egui::FontId::proportional(base_icon_size),
        color,
    );

    let base_pad_l = base_pad.left as f32;
    let base_pad_r = base_pad.right as f32;
    let base_pad_t = base_pad.top as f32;
    let base_pad_b = base_pad.bottom as f32;

    let button_width = (maket.size().x + base_pad_l + base_pad_r).max(base_button_size);
    let button_height = (base_icon_size + base_pad_t + base_pad_b).max(base_button_size);

    let base_m_l = base_margin.left as f32;
    let base_m_r = base_margin.right as f32;
    let base_m_t = base_margin.top as f32;
    let base_m_b = base_margin.bottom as f32;

    let total_w = button_width + base_m_l + base_m_r;
    let total_h = button_height + base_m_t + base_m_b;

    let size = egui::vec2(total_w, total_h);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

    let pad = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "padding", &resp, true, base_pad, crate::renderer::parse_padding);
    let margin = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "margin", &resp, true, base_margin, crate::renderer::parse_padding);

    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + m_l, rect.min.y + m_t),
        egui::pos2(rect.max.x - m_r, rect.max.y - m_b),
    );

    let actual_fill = crate::renderer::get_state_background(node, &ctx.theme, "IconButton", &resp, enabled,
        egui::Color32::from_rgb(0x30, 0x30, 0x30));
    let actual_text = if enabled {
        crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "color", &resp, true, color, crate::theme::parse_color_value)
    } else {
        egui::Color32::from_gray(100)
    };

    let border = get_state_border(node, &ctx.theme, "IconButton", &resp, enabled);

    let rounding = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "rounding", &resp, true, base_rounding, |v| v.as_f64());
    let icon_size = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "icon_size", &resp, true, base_icon_size, |v| v.as_f64().map(|x| x as f32));
    let maket = ui.painter().layout_no_wrap(text.clone(), egui::FontId::proportional(icon_size), color);

    let rounding_cr = egui::CornerRadius::same(rounding as u8);
    let shadow_bg = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "shadow_background", &resp, true,
        Shadow::from_rgba(0, 0, 0, 40), parse_shadow);
    draw_shadow_bg(ui, content_rect, rounding_cr, &shadow_bg);
    ui.painter().rect_filled(content_rect, rounding_cr, actual_fill);
    let shadow_border = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "shadow_border", &resp, true,
        Shadow::transparent(), parse_shadow);
    if shadow_border.z_order == ShadowZOrder::Under {
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
        draw_border(ui, content_rect, rounding_cr, &border);
    } else {
        draw_border(ui, content_rect, rounding_cr, &border);
        draw_shadow_border(ui, content_rect, rounding_cr, &border, &shadow_border);
    }

    let inner = egui::Rect::from_min_max(
        egui::pos2(content_rect.left() + pad_l, content_rect.top() + pad_t),
        egui::pos2(content_rect.right() - pad_r, content_rect.bottom() - pad_b),
    );
    let text_x = halign.align_size_within_range(maket.size().x, inner.x_range()).min;
    let text_y = valign.align_size_within_range(maket.size().y, inner.y_range()).min;
    let text_pos = egui::pos2(text_x, text_y);

    let shadow_icon = crate::renderer::get_state_attr(node, &ctx.theme, "IconButton", "shadow_icon", &resp, true,
        Shadow::transparent(), parse_shadow);
    if shadow_icon.z_order == ShadowZOrder::Under {
        draw_shadow_icon(ui, text_pos, maket.clone(), &shadow_icon);
        ui.painter().galley_with_override_text_color(text_pos, maket, actual_text);
    } else {
        ui.painter().galley_with_override_text_color(text_pos, maket.clone(), actual_text);
        draw_shadow_icon(ui, text_pos, maket, &shadow_icon);
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

    #[test]
    fn test_smoke_iconbutton() {
        let json = serde_json::json!({"type": "IconButton", "icon": "save", "action": "save"});
        assert_eq!(attr_str(&json, "icon"), Some("save"));
    }
}
  