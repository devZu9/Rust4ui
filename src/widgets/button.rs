use crate::border::{draw_border, get_border};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_margin, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {

    let raw_text = attr_str(node, "text").unwrap_or("");
    let icon_name = attr_str(node, "icon");
    let text = if let Some(icon) = icon_name.and_then(|n| ctx.icons.resolve(n)) {
        format!("{}  {}", icon, resolve_text(raw_text, ctx))
    } else {
        resolve_text(raw_text, ctx)
    };

    if raw_text.is_empty() && icon_name.is_none() {
        log::warn!("Button: отсутствует атрибут 'text' и 'icon'");
    }

    let enabled = attr_bool(node, "enabled").unwrap_or(true);
    let min_width = attr_f64(node, "min_width")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "min_width", 100.0));
    let min_height = ctx.theme.w_f64("Button", "height", 28.0) as f32;

    let fill = attr_str(node, "fill")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or_else(|| ctx.theme.w_color("Button", "fill", egui::Color32::from_rgb(0x30, 0x30, 0x30)));

    let rounding = attr_f64(node, "rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("Button", "rounding", 6.0));

    let border = get_border(node, &ctx.theme, "Button");

    let tooltip_text = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));
    let align = attr_str(node, "align").unwrap_or("center");

    let pad = get_padding(node, &ctx.theme, "Button", egui::Margin::symmetric(16, 4));
    let margin = get_margin(node, &ctx.theme, "Button");

    let text_color = attr_str(node, "text_color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or_else(|| ctx.theme.w_color("Button", "text_color", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };
    let valign = egui::Align::Center;

    let maket = ui.painter().layout_no_wrap(
        text.clone(),
        egui::FontId::proportional(14.0),
        text_color,
    );

    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let desired_w = (maket.size().x + pad_l + pad_r).max(min_width as f32);
    let desired_h = (maket.size().y + pad_t + pad_b).max(min_height);

    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let total_w = desired_w + m_l + m_r;
    let total_h = desired_h + m_t + m_b;

    let size = egui::vec2(total_w, total_h);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + m_l, rect.min.y + m_t),
        egui::pos2(rect.max.x - m_r, rect.max.y - m_b),
    );

    let bg = if resp.hovered() && resp.is_pointer_button_down_on() {
        attr_str(node, "click_fill")
            .and_then(crate::theme::parse_hex_color)
            .or_else(|| ctx.theme.w_color_opt("Button", "click_fill"))
            .unwrap_or_else(|| ctx.theme.w_color("Button", "hover_fill", egui::Color32::from_rgb(0x44, 0x44, 0x55)))
    } else if resp.hovered() {
        attr_str(node, "hover_fill")
            .and_then(crate::theme::parse_hex_color)
            .unwrap_or_else(|| ctx.theme.w_color("Button", "hover_fill", egui::Color32::from_rgb(0x44, 0x44, 0x55)))
    } else if resp.has_focus() {
        ctx.theme.w_color("Button", "focus_fill", egui::Color32::from_rgb(0x33, 0x44, 0x66))
    } else {
        fill
    };

    let actual_fill = if enabled { bg } else { egui::Color32::from_gray(60) };
    let actual_text = if enabled {
        if resp.hovered() && resp.is_pointer_button_down_on() {
            attr_str(node, "click_text_color")
                .and_then(crate::theme::parse_hex_color)
                .or_else(|| ctx.theme.w_color_opt("Button", "click_text_color"))
                .unwrap_or(text_color)
        } else if resp.hovered() {
            attr_str(node, "hover_text_color")
                .and_then(crate::theme::parse_hex_color)
                .unwrap_or(text_color)
        } else {
            text_color
        }
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
    let text_x = halign.align_size_within_range(maket.size().x, inner.x_range()).min;
    let text_y = valign.align_size_within_range(maket.size().y, inner.y_range()).min;
    let text_pos = egui::pos2(text_x, text_y);

    ui.painter().galley(text_pos, maket, actual_text);

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
