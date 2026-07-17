use crate::renderer::{attr_f64, attr_str, get_margin, get_padding, resolve_state_attr, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let raw_text = attr_str(node, "text").unwrap_or("");

    // Base icon attrs (no state — used for layout sizing before Response exists)
    let icon_name_base = attr_str(node, "icon")
        .or_else(|| ctx.inherited.get("icon").and_then(|v| v.as_str()))
        .map(|s| s.to_string());
    let icon_pos_base = ctx.inherited.get("icon_position")
        .and_then(|v| v.as_str().map(|s| s.to_owned()))
        .or_else(|| attr_str(node, "icon_position").map(|s| s.to_owned()))
        .unwrap_or_else(|| "left".to_owned());
    let icon_gap_base = ctx.inherited.get("icon_gap")
        .and_then(|v| v.as_f64())
        .or_else(|| attr_f64(node, "icon_gap"))
        .unwrap_or(6.0) as f32;

    let text = if raw_text.is_empty() && icon_name_base.is_some() {
        String::new()
    } else if raw_text.is_empty() {
        "{{menu}}".to_string()
    } else {
        resolve_text(raw_text, ctx)
    };

    let icon_glyph = icon_name_base.as_deref()
        .and_then(|n| ctx.icons.resolve(n))
        .map(|s| s.to_string());
    let has_icon = icon_glyph.is_some();

    let rounding_val = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("Menu", "rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let radius = ctx.inherited.get("rounding")
        .and_then(|v| {
            v.as_f64().map(|f| egui::CornerRadius::same(f as u8))
                .or_else(|| v.as_array().and_then(|a| {
                    if a.len() >= 4 {
                        Some(egui::CornerRadius {
                            nw: a[0].as_f64()? as u8,
                            ne: a[1].as_f64()? as u8,
                            sw: a[2].as_f64()? as u8,
                            se: a[3].as_f64()? as u8,
                        })
                    } else { None }
                }))
        })
        .unwrap_or_else(|| egui::CornerRadius::same(rounding_val));

    let margin = get_margin(node, &ctx.theme, "Menu");
    let inherited_pad = ctx.inherited.get("padding").and_then(crate::renderer::parse_padding);
    let inherited_pad_val = inherited_pad.unwrap_or(egui::Margin::ZERO);
    let pad = get_padding(node, &ctx.theme, "Menu", inherited_pad_val);

    // Inherit _children for children (save/restore around children rendering)
    let old = ctx.inherit_children(node);

    // Layout (placeholder color — actual color resolved after Response)
    let placeholder_color = egui::Color32::from_gray(220);
    let font_id = egui::FontId::proportional(14.0);
    let text_galley = ui.painter().layout_no_wrap(text.clone(), font_id.clone(), placeholder_color);
    let icon_galley = has_icon.then(|| ui.painter().layout_no_wrap(icon_glyph.unwrap().to_string(), font_id, placeholder_color));
    let icon_w = icon_galley.as_ref().map_or(0.0, |g| g.size().x);
    let text_w = text_galley.size().x;
    let gap_w = if has_icon && text_w > 0.0 { icon_gap_base } else { 0.0 };
    let content_w = if icon_pos_base == "right" { text_w + gap_w + icon_w } else { icon_w + gap_w + text_w };
    let content_h = icon_galley.as_ref().map_or(text_galley.size().y, |g| text_galley.size().y.max(g.size().y));
    let (p_l, p_r, p_t, p_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);
    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let total_w = content_w + p_l + p_r + m_l + m_r;
    let total_h = content_h + p_t + p_b + m_t + m_b;

    if m_t > 0.0 { ui.add_space(m_t); }
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(total_w, total_h), egui::Sense::click());
    if m_b > 0.0 { ui.add_space(m_b); }

    // State-dependent values with real Response
    let bg_actual = resolve_state_attr(
        node, &ctx.inherited, &resp, "background",
        crate::theme::parse_color_value,
        |k| ctx.theme.w_color_opt("Menu", k),
        egui::Color32::from_rgb(0x2A, 0x2A, 0x33),
    );
    let color_actual = resolve_state_attr(
        node, &ctx.inherited, &resp, "color",
        crate::theme::parse_color_value,
        |k| ctx.theme.w_color_opt("Menu", k),
        egui::Color32::from_gray(220),
    );
    let icon_pos = resolve_state_attr(
        node, &ctx.inherited, &resp, "icon_position",
        |v| Some(v.as_str().unwrap_or(&icon_pos_base).to_string()),
        |k| None,
        icon_pos_base.to_string(),
    );
    let icon_gap_actual = resolve_state_attr(
        node, &ctx.inherited, &resp, "icon_gap",
        |v| v.as_f64().map(|n| n as f32),
        |k| None,
        icon_gap_base,
    );
    let gap_w_actual = if has_icon && text_w > 0.0 { icon_gap_actual } else { 0.0 };

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
        let (ix, tx) = if icon_pos == "right" {
            (text_pos_x + text_w + gap_w_actual, text_pos_x)
        } else {
            (text_pos_x, text_pos_x + icon_w + gap_w_actual)
        };
        ui.painter().galley_with_override_text_color(egui::pos2(ix, text_y), ig.clone(), color_actual);
        ui.painter().galley_with_override_text_color(egui::pos2(tx, text_y), text_galley, color_actual);
    } else {
        ui.painter().galley_with_override_text_color(egui::pos2(text_pos_x, text_y), text_galley, color_actual);
    }

    // Border (deferred)
    let base_border = crate::border::get_border(node, &ctx.theme, "Menu");
    let border = resolve_state_attr(
        node, &ctx.inherited, &resp, "border",
        |v| Some(crate::border::get_border(
            &serde_json::json!({"border": v, "border_position": node.get("border_position").cloned().unwrap_or(serde_json::Value::Null)}),
            &ctx.theme, "Menu",
        )),
        |k| {
            if k == "border" { return Some(base_border); }
            ctx.theme.widget.get("Menu").and_then(|w| w.get(k)).map(|bv| {
                crate::border::get_border(
                    &serde_json::json!({"border": bv, "border_position": node.get("border_position").cloned().unwrap_or(serde_json::Value::Null)}),
                    &ctx.theme, "Menu",
                )
            })
        },
        crate::border::BorderStyle::default(),
    );
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

    ctx.restore_children(old);
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
