use crate::border::{apply_state_border, draw_border, get_border};
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
    let button_size = attr_f64(node, "button_size")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "button_size", 24.0)) as f32;

    let fill = attr_str(node, "fill")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or_else(|| ctx.theme.w_color("IconButton", "fill", egui::Color32::from_rgb(0x30, 0x30, 0x30)));

    let rounding = attr_f64(node, "rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "rounding", 6.0));

    let base_border = get_border(node, &ctx.theme, "IconButton");

    let tooltip_text = attr_str(node, "tooltip").map(|t| resolve_text(t, ctx));
    let align = attr_str(node, "align").unwrap_or("center");

    let pad = get_padding(node, &ctx.theme, "IconButton", egui::Margin::symmetric(0, 0));
    let margin = get_margin(node, &ctx.theme, "IconButton");

    let color = attr_str(node, "color")
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or_else(|| ctx.theme.w_color("IconButton", "color", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0)));

    let halign = match align {
        "left" => egui::Align::LEFT,
        "right" => egui::Align::RIGHT,
        _ => egui::Align::Center,
    };
    let valign = egui::Align::Center;

    let icon_size = attr_f64(node, "icon_size")
        .unwrap_or_else(|| ctx.theme.w_f64("IconButton", "icon_size", 14.0)) as f32;
    let maket = ui.painter().layout_no_wrap(
        text.clone(),
        egui::FontId::proportional(icon_size),
        color,
    );

    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let button_width = (maket.size().x + pad_l + pad_r).max(button_size);
    let button_height = (icon_size + pad_t + pad_b).max(button_size);

    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let total_w = button_width + m_l + m_r;
    let total_h = button_height + m_t + m_b;

    let size = egui::vec2(total_w, total_h);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + m_l, rect.min.y + m_t),
        egui::pos2(rect.max.x - m_r, rect.max.y - m_b),
    );

    let bg = if resp.hovered() && resp.is_pointer_button_down_on() {
        attr_str(node, "click_fill")
            .and_then(crate::theme::parse_hex_color)
            .or_else(|| ctx.theme.w_color_opt("IconButton", "click_fill"))
            .unwrap_or_else(|| ctx.theme.w_color("IconButton", "hover_fill", egui::Color32::from_rgb(0x44, 0x44, 0x55)))
    } else if resp.hovered() {
        attr_str(node, "hover_fill")
            .and_then(crate::theme::parse_hex_color)
            .unwrap_or_else(|| ctx.theme.w_color("IconButton", "hover_fill", egui::Color32::from_rgb(0x44, 0x44, 0x55)))
    } else if resp.has_focus() {
        ctx.theme.w_color("IconButton", "focus_fill", egui::Color32::from_rgb(0x33, 0x44, 0x66))
    } else {
        fill
    };

    let actual_fill = if enabled { bg } else { egui::Color32::from_gray(60) };
    let actual_text = if enabled {
        if resp.hovered() && resp.is_pointer_button_down_on() {
            attr_str(node, "click_color")
                .and_then(crate::theme::parse_hex_color)
                .or_else(|| ctx.theme.w_color_opt("IconButton", "click_color"))
                .unwrap_or(color)
        } else if resp.hovered() {
            attr_str(node, "hover_color")
                .and_then(crate::theme::parse_hex_color)
                .or_else(|| ctx.theme.w_color_opt("IconButton", "hover_color"))
                .unwrap_or(color)
        } else {
            color
        }
    } else {
        egui::Color32::from_gray(100)
    };

    let border = if enabled && resp.hovered() && resp.is_pointer_button_down_on() {
        apply_state_border(node, &ctx.theme, "IconButton", "click", &base_border)
    } else if enabled && resp.hovered() {
        apply_state_border(node, &ctx.theme, "IconButton", "hover", &base_border)
    } else {
        base_border
    };

    let rounding_cr = egui::CornerRadius::same(rounding as u8);
    let shadow = crate::border::get_shadow(node, &ctx.theme, "IconButton");
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

    ui.painter().galley_with_override_text_color(text_pos, maket, actual_text);

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
  