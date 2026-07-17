use crate::renderer::{attr_f64, attr_str, get_margin, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let text = attr_str(node, "text")
        .map(|t| resolve_text(t, ctx))
        .unwrap_or_else(|| "{{menu}}".to_string());

    let icon_name = attr_str(node, "icon");
    let icon_position = attr_str(node, "icon_position").unwrap_or("left");
    let icon_gap = attr_f64(node, "icon_gap").unwrap_or(6.0) as f32;
    let icon_glyph = icon_name.and_then(|n| ctx.icons.resolve(n));
    let has_icon = icon_glyph.is_some();

    let bg = node
        .get("background")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background"))
        .unwrap_or_else(|| egui::Color32::from_rgb(0x2A, 0x2A, 0x33));

    let bg_hover = node
        .get("background_hover")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg_hover)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_hover"))
        .unwrap_or(bg);

    let bg_click = node
        .get("background_click")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_bg_click)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_click"))
        .unwrap_or(bg_hover);

    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let color_hover = node
        .get("color_hover")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color_hover)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_hover"))
        .unwrap_or(color);

    let color_click = node
        .get("color_click")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited_color_click)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_click"))
        .unwrap_or(color_hover);

    let rounding_val = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("Menu", "rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let radius = ctx.inherited_rounding
        .unwrap_or_else(|| egui::CornerRadius::same(rounding_val));

    let margin = get_margin(node, &ctx.theme, "Menu");
    let pad = get_padding(node, &ctx.theme, "Menu", ctx.inherited_padding.unwrap_or(egui::Margin::ZERO));

    // inher_* для children
    let inher_bg = node.get("background_children").and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_children"));
    let inher_color = node.get("color_children").and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_children"));
    let inher_bg_hover = node.get("background_hover_children").and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_hover_children"));
    let inher_bg_click = node.get("background_click_children").and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "background_click_children"));
    let inher_color_hover = node.get("color_hover_children").and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_hover_children"));
    let inher_color_click = node.get("color_click_children").and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("Menu", "color_click_children"));
    let inher_margin = node.get("margin_children").and_then(crate::renderer::parse_padding);
    let inher_padding = node.get("padding_children").and_then(crate::renderer::parse_padding);
    let inher_border = node.get("border_children").map(|bv| crate::border::get_border(&serde_json::json!({"border": bv}), &ctx.theme, "Menu")).or_else(|| ctx.inherited_border);
    let inher_border_hover = node.get("border_hover_children").map(|bv| crate::border::get_border(&serde_json::json!({"border": bv}), &ctx.theme, "Menu")).or_else(|| ctx.inherited_border_hover);
    let inher_border_click = node.get("border_click_children").map(|bv| crate::border::get_border(&serde_json::json!({"border": bv}), &ctx.theme, "Menu")).or_else(|| ctx.inherited_border_click);
    let inher_border_focus = node.get("border_focus_children").map(|bv| crate::border::get_border(&serde_json::json!({"border": bv}), &ctx.theme, "Menu")).or_else(|| ctx.inherited_border_focus);

    // Save/restore ctx
    let prev = (
        ctx.inherited_bg.take(), ctx.inherited_color.take(),
        ctx.inherited_bg_hover.take(), ctx.inherited_bg_click.take(),
        ctx.inherited_color_hover.take(), ctx.inherited_color_click.take(),
        ctx.inherited_margin.take(), ctx.inherited_padding.take(),
        ctx.inherited_border.take(), ctx.inherited_border_hover.take(),
        ctx.inherited_border_click.take(), ctx.inherited_border_focus.take(),
    );
    ctx.inherited_bg = inher_bg; ctx.inherited_color = inher_color;
    ctx.inherited_bg_hover = inher_bg_hover; ctx.inherited_bg_click = inher_bg_click;
    ctx.inherited_color_hover = inher_color_hover; ctx.inherited_color_click = inher_color_click;
    ctx.inherited_margin = inher_margin; ctx.inherited_padding = inher_padding;
    ctx.inherited_border = inher_border; ctx.inherited_border_hover = inher_border_hover;
    ctx.inherited_border_click = inher_border_click; ctx.inherited_border_focus = inher_border_focus;

    let font_id = egui::FontId::proportional(14.0);
    let text_galley = ui.painter().layout_no_wrap(text.clone(), font_id.clone(), color);
    let icon_galley = has_icon.then(|| ui.painter().layout_no_wrap(icon_glyph.unwrap().to_string(), font_id, color));
    let icon_w = icon_galley.as_ref().map_or(0.0, |g| g.size().x);
    let text_w = text_galley.size().x;
    let gap_w = if has_icon { icon_gap } else { 0.0 };
    let content_w = if icon_position == "right" { text_w + gap_w + icon_w } else { icon_w + gap_w + text_w };
    let content_h = icon_galley.as_ref().map_or(text_galley.size().y, |g| text_galley.size().y.max(g.size().y));
    let (p_l, p_r, p_t, p_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);
    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let total_w = content_w + p_l + p_r + m_l + m_r;
    let total_h = content_h + p_t + p_b + m_t + m_b;

    if m_t > 0.0 { ui.add_space(m_t); }
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(total_w, total_h), egui::Sense::click());
    if m_b > 0.0 { ui.add_space(m_b); }

    let bg_actual = if resp.is_pointer_button_down_on() { bg_click }
        else if resp.hovered() { bg_hover } else { bg };
    let color_actual = if resp.is_pointer_button_down_on() { color_click }
        else if resp.hovered() { color_hover } else { color };
    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + m_l, rect.min.y + m_t),
        egui::pos2(rect.max.x - m_r, rect.max.y - m_b),
    );
    ui.painter().rect_filled(content_rect, radius, bg_actual);

    let inner_rect = egui::Rect::from_min_max(
        egui::pos2(content_rect.min.x + p_l, content_rect.min.y + p_t),
        egui::pos2(content_rect.max.x - p_r, content_rect.max.y - p_b),
    );
    let text_pos_x = egui::Align::Center.align_size_within_range(content_w, inner_rect.x_range()).min;
    let text_y = egui::Align::Center.align_size_within_range(content_h, inner_rect.y_range()).min;
    if let Some(ig) = &icon_galley {
        let (ix, tx) = if icon_position == "right" {
            (text_pos_x + text_w + gap_w, text_pos_x)
        } else {
            (text_pos_x, text_pos_x + icon_w + gap_w)
        };
        ui.painter().galley_with_override_text_color(egui::pos2(ix, text_y), ig.clone(), color_actual);
        ui.painter().galley_with_override_text_color(egui::pos2(tx, text_y), text_galley, color_actual);
    } else {
        ui.painter().galley_with_override_text_color(egui::pos2(text_pos_x, text_y), text_galley, color_actual);
    }

    // Border (deferred)
    let base_border = ctx.inherited_border.unwrap_or_else(|| crate::border::get_border(node, &ctx.theme, "Menu"));
    let p = |s: &str, inh: Option<crate::border::BorderStyle>| {
        if node.get(&format!("border_{}", s)).is_some() {
            crate::border::apply_state_border(node, &ctx.theme, "Menu", s, &base_border)
        } else if let Some(b) = inh { b } else { base_border }
    };
    let border = if resp.is_pointer_button_down_on() { p("click", ctx.inherited_border_click) }
        else if resp.has_focus() { p("focus", ctx.inherited_border_focus) }
        else if resp.hovered() { p("hover", ctx.inherited_border_hover) } else { base_border };
    if border.is_visible() {
        ctx.pending_borders.push((content_rect, radius, border));
    }

    // Popup
    let popup_key = format!("__menu_popup_{}", text);
    let mut is_open = ctx.state.get_bool(&popup_key).unwrap_or(false);
    let prev_open = ctx.open_popup_id.clone();

    if resp.clicked() { is_open = !is_open; }
    if resp.hovered() && !is_open && prev_open.is_some() && prev_open.as_deref() != Some(&popup_key) {
        is_open = true;
    }
    if is_open {
        ctx.open_popup_id = Some(popup_key.clone());
    }

    ctx.state.set_bool(&popup_key, is_open);

    let children = node.get("children").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    if is_open && !children.is_empty() {
        let popup_bg = ctx.theme.w_color("Menu", "background", egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
        let popup_r = ctx.theme.w_f64("Menu", "rounding", 4.0) as u8;

        let ar: egui::InnerResponse<()> = egui::Area::new(egui::Id::new(&popup_key))
            .fixed_pos(egui::pos2(content_rect.left(), content_rect.bottom()))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(popup_bg)
                    .corner_radius(egui::CornerRadius::same(popup_r))
                    .show(ui, |ui| {
                        ui.set_min_width(content_rect.width().max(content_w + p_l + p_r));
                        for child in &children {
                            super::super::renderer::render_node(ui, child, ctx);
                        }
                    });
            });

        if ar.response.clicked_elsewhere() {
            ctx.state.set_bool(&popup_key, false);
            ctx.open_popup_id = None;
        }
    }

    // Restore ctx
    ctx.inherited_bg = prev.0; ctx.inherited_color = prev.1;
    ctx.inherited_bg_hover = prev.2; ctx.inherited_bg_click = prev.3;
    ctx.inherited_color_hover = prev.4; ctx.inherited_color_click = prev.5;
    ctx.inherited_margin = prev.6; ctx.inherited_padding = prev.7;
    ctx.inherited_border = prev.8; ctx.inherited_border_hover = prev.9;
    ctx.inherited_border_click = prev.10; ctx.inherited_border_focus = prev.11;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_menu() {
        let json = serde_json::json!({"type": "Menu", "text": "File"});
        assert_eq!(attr_str(&json, "text"), Some("File"));
    }
}
