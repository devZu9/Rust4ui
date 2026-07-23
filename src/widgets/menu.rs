use crate::renderer::{attr_f64, attr_str, get_attr_ctx, get_margin, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let raw_text = attr_str(node, "text").unwrap_or("");

    // Base icon attrs (no state — used for layout sizing before Response exists)
    let icon_name_base = attr_str(node, "icon")
        .or_else(|| ctx.inherited.get("icon").and_then(|v| v.as_str()))
        .map(|s| s.to_string());
    let icon_position_base = ctx.inherited.get("icon_position")
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

    // get_margin/get_padding — для измерения детей (popup_min_w)
    // Фактический margin/padding для кнопки обрабатывает widget_paint_custom
    let _margin = get_margin(node, &ctx.inherited, &ctx.theme);
    let padding = get_padding(node, &ctx.inherited, &ctx.theme, egui::Margin::ZERO);

    // Layout (placeholder color — actual color resolved after Response)
    let placeholder_color = egui::Color32::from_gray(220);
    let font_id = egui::FontId::proportional(14.0);
    let text_galley = ui.painter().layout_no_wrap(text.clone(), font_id.clone(), placeholder_color);
    let icon_galley = has_icon.then(|| ui.painter().layout_no_wrap(icon_glyph.unwrap().to_string(), font_id, placeholder_color));
    let icon_width = icon_galley.as_ref().map_or(0.0, |g| g.size().x);
    let text_width = text_galley.size().x;
    let gap_width = if has_icon && text_width > 0.0 { icon_gap_base } else { 0.0 };
    let content_width = if icon_position_base == "right" { text_width + gap_width + icon_width } else { icon_width + gap_width + text_width };
    let content_height = icon_galley.as_ref().map_or(text_galley.size().y, |g| text_galley.size().y.max(g.size().y));
    let (padding_left, padding_right, _, _) = (padding.left as f32, padding.right as f32, padding.top as f32, padding.bottom as f32);
    let out = crate::widgets::base::widget_paint_custom(
        ui, node, ctx,
        egui::vec2(content_width, content_height),
        egui::Sense::click(), true,
    );
    let response = out.response;

    let color_actual = get_attr_ctx(
        ctx, node, Some(&response), "color",
        crate::theme::parse_color,
        |k| ctx.theme.w_color_opt("Menu", k),
        egui::Color32::from_gray(220),
    );
    let icon_position = get_attr_ctx(
        ctx, node, Some(&response), "icon_position",
        |v| Some(v.as_str().unwrap_or(&icon_position_base).to_string()),
        |_k| None,
        icon_position_base.to_string(),
    );
    let icon_gap_actual = get_attr_ctx(
        ctx, node, Some(&response), "icon_gap",
        |v| v.as_f64().map(|n| n as f32),
        |_k| None,
        icon_gap_base,
    );
    let gap_width_actual = if has_icon && text_width > 0.0 { icon_gap_actual } else { 0.0 };

    let text_pos_x = egui::Align::Center.align_size_within_range(content_width, out.inner_rect.x_range()).min;
    let text_y = egui::Align::Center.align_size_within_range(content_height, out.inner_rect.y_range()).min;
    if let Some(ig) = &icon_galley {
        let (icon_x, text_x) = if icon_position == "right" {
            (text_pos_x + text_width + gap_width_actual, text_pos_x)
        } else {
            (text_pos_x, text_pos_x + icon_width + gap_width_actual)
        };
        ui.painter().galley_with_override_text_color(egui::pos2(icon_x, text_y), ig.clone(), color_actual);
        ui.painter().galley_with_override_text_color(egui::pos2(text_x, text_y), text_galley, color_actual);
    } else {
        ui.painter().galley_with_override_text_color(egui::pos2(text_pos_x, text_y), text_galley, color_actual);
    }

    // Popup attrs (читаем ДО inherit_children, чтобы popup_*_children от MenuBar были видны)
    let popup_bg = crate::renderer::attr_str(node, "popup_background")
        .or_else(|| ctx.inherited.get("popup_background").and_then(|v| v.as_str()))
        .and_then(crate::theme::parse_color_hex)
        .or_else(|| ctx.theme.w_color_opt("Menu", "popup_background"))
        .unwrap_or_else(|| egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
    let popup_rounding = crate::renderer::attr_f64(node, "popup_rounding")
        .or_else(|| ctx.inherited.get("popup_rounding").and_then(|v| v.as_f64()))
        .or_else(|| Some(ctx.theme.w_f64("Menu", "popup_rounding", 4.0)))
        .unwrap_or(4.0) as u8;
    let popup_padding = ctx.inherited.get("popup_padding")
        .and_then(crate::renderer::parse_padding)
        .or_else(|| node.get("popup_padding").and_then(crate::renderer::parse_padding))
        .or_else(|| ctx.theme.widget.get("Menu")
            .and_then(|w| w.get("popup_padding"))
            .and_then(crate::renderer::parse_padding))
        .unwrap_or(egui::Margin::ZERO);
    let popup_gap = crate::renderer::attr_f64(node, "popup_gap")
        .or_else(|| ctx.inherited.get("popup_gap").and_then(|v| v.as_f64()))
        .unwrap_or(0.0) as f32;
    let popup_minimum_width = crate::renderer::attr_f64(node, "popup_min_width")
        .or_else(|| ctx.inherited.get("popup_min_width").and_then(|v| v.as_f64()))
        .unwrap_or(0.0) as f32;
    let popup_max_height = crate::renderer::attr_f64(node, "popup_max_height")
        .or_else(|| ctx.inherited.get("popup_max_height").and_then(|v| v.as_f64()))
        .unwrap_or(0.0) as f32;
    let popup_border = ctx.inherited.get("popup_border")
        .or_else(|| node.get("popup_border"))
        .map(|bv| crate::border::get_border(&serde_json::json!({"border": bv}), &ctx.theme, "Menu"))
        .unwrap_or_default();
    let popup_shadow = ctx.inherited.get("popup_shadow")
        .or_else(|| node.get("popup_shadow"))
        .and_then(crate::border::parse_shadow)
        .unwrap_or(crate::border::Shadow::transparent());

    // Измеряем детей заранее — определяем ширину попапа по самому широкому
    let children = node.get("children").and_then(|v| v.as_array()).cloned().unwrap_or_default();

    // Inherit _children ДО замера — чтобы get_padding видел тот же inherited, что и при рендере
    let old = ctx.inherit_children(node, Some("Menu"));

    let mut maximum_child_outer_width = content_width + padding_left + padding_right; // как минимум ширина кнопки
    let measurement_font = egui::FontId::proportional(14.0);
    for child in &children {
        let is_separator = child.get("type").and_then(|v| v.as_str()) == Some("Separator");
        if is_separator { continue; }
        let child_icon = crate::renderer::attr_str(child, "icon").and_then(|n| ctx.icons.resolve(n));
        let child_raw_text = crate::renderer::attr_str(child, "text").unwrap_or("");
        let child_text = crate::renderer::resolve_text(child_raw_text, ctx);
        let child_text_width = ui.painter().layout_no_wrap(child_text, measurement_font.clone(), egui::Color32::WHITE).size().x;
        let icon_width = if child_icon.is_some() { 16.0 } else { 0.0 };
        // padding из своего узла → inherited → темы
        let child_padding = crate::renderer::get_padding(child, &ctx.inherited, &ctx.theme, egui::Margin::ZERO);
        let child_width = crate::renderer::get_attr_ctx(
            ctx, child, None, "width",
            |v| v.as_f64().map(|n| n as f32),
            |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_f64()).map(|n| n as f32),
            0.0_f32,
        );
        let measured = child_text_width + icon_width + child_padding.left as f32 + child_padding.right as f32;
        let total_width = if child_width > 0.0 { measured.max(child_width) } else { measured };
        maximum_child_outer_width = maximum_child_outer_width.max(total_width);
    }
    let popup_width = if popup_minimum_width > 0.0 { popup_minimum_width } else { maximum_child_outer_width };

    ctx.inherited.insert("popup_content_width".to_string(), serde_json::json!(maximum_child_outer_width));

    // Popup
    let popup_key = format!("__menu_popup_{}", text);
    let mut is_open = ctx.state.get_bool(&popup_key).unwrap_or(false);
    let prev_open = ctx.open_popup_id.clone();

    if response.clicked() { is_open = !is_open; }
    if response.hovered() && !is_open && prev_open.is_some() && prev_open.as_deref() != Some(&popup_key) {
        if let Some(prev) = &prev_open {
            ctx.state.set_bool(prev, false);
        }
        is_open = true;
    }
    if is_open {
        ctx.open_popup_id = Some(popup_key.clone());
    } else if ctx.open_popup_id.as_deref() == Some(&popup_key) {
        ctx.open_popup_id = None;
    }

    ctx.state.set_bool(&popup_key, is_open);

    if is_open && !children.is_empty() {
        let popup_corner_radius = egui::CornerRadius::same(popup_rounding);

        let area_response: egui::InnerResponse<()> = egui::Area::new(egui::Id::new(&popup_key))
            .fixed_pos(egui::pos2(out.content_rect.left(), out.content_rect.bottom()))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(popup_bg)
                    .corner_radius(popup_corner_radius)
                    .inner_margin(popup_padding)
                    .show(ui, |ui| {
                        let (content_rect, _) = ui.allocate_exact_size(
                            egui::vec2(popup_width, 0.0),
                            egui::Sense::hover(),
                        );

                        ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                            ui.set_min_width(popup_width);
                            ui.set_max_width(popup_width);
                            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, popup_gap);
                            if popup_max_height > 0.0 {
                                egui::ScrollArea::vertical().max_height(popup_max_height).show(ui, |ui| {
                                    for child in &children {
                                        super::super::renderer::render_node(ui, child, ctx);
                                    }
                                });
                            } else {
                                for child in &children {
                                    super::super::renderer::render_node(ui, child, ctx);
                                }
                            }
                        });
                    });

                let popup_rect = ui.min_rect();
                crate::border::draw_shadow_bg(ui, popup_rect, popup_corner_radius, &popup_shadow);
                if popup_border.is_visible() {
                    crate::border::draw_border(ui, popup_rect, popup_corner_radius, &popup_border);
                }
            });

        // clicked_elsewhere — только если не было клика на этой же Menu (toggle уже обработал)
        if !response.clicked() && area_response.response.clicked_elsewhere() {
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





