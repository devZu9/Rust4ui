use crate::renderer::{attr_f64, get_margin, get_padding, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let children = match node.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    let bg = node
        .get("background")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "background"))
        .unwrap_or_else(|| egui::Color32::from_rgb(0x1E, 0x1E, 0x24));

    let bg_hover = node
        .get("background_hover")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "background_hover"))
        .unwrap_or(bg);

    let gap = attr_f64(node, "gap")
        .or_else(|| Some(ctx.theme.w_f64("MenuBar", "gap", 0.0)))
        .unwrap_or(0.0) as f32;

    let margin = get_margin(node, &ctx.theme, "MenuBar");
    let padding = get_padding(node, &ctx.theme, "MenuBar", egui::Margin::ZERO);
    let rounding = ctx.theme.w_f64("MenuBar", "rounding", 0.0) as u8;
    let rounding_cr = egui::CornerRadius::same(rounding);

    let inher_bg = node
        .get("background_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| node.get("background").and_then(crate::theme::parse_color_value))
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "background_children"))
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "background"));

    let inher_color = node
        .get("color_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| node.get("color").and_then(crate::theme::parse_color_value))
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "color_children"))
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "color"));

    let inher_margin = node
        .get("margin_children")
        .and_then(crate::renderer::parse_padding);

    let inher_padding = node
        .get("padding_children")
        .and_then(crate::renderer::parse_padding);

    // Измеряем реальный размер детей (invisible scope — shapes не рендерятся)
    let kids_rect = ui.scope(|ui| {
        ui.set_invisible();
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
            for (i, child) in children.iter().enumerate() {
                if i > 0 && gap > 0.0 { ui.add_space(gap); }
                super::super::renderer::render_node(ui, child, ctx);
            }
        });
    }).response.rect;

    let (p_l, p_r, p_t, p_b) = (padding.left as f32, padding.right as f32, padding.top as f32, padding.bottom as f32);
    let (m_l, m_r, m_t, m_b) = (margin.left as f32, margin.right as f32, margin.top as f32, margin.bottom as f32);
    let total_w = kids_rect.width() + p_l + p_r + m_l + m_r;
    let total_h = kids_rect.height() + p_t + p_b + m_t + m_b;

    if m_t > 0.0 { ui.add_space(m_t); }
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(total_w, total_h), egui::Sense::hover());
    if m_b > 0.0 { ui.add_space(m_b); }

    let actual_bg = if resp.hovered() { bg_hover } else { bg };
    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + m_l, rect.min.y + m_t),
        egui::pos2(rect.max.x - m_r, rect.max.y - m_b),
    );
    ui.painter().rect_filled(content_rect, rounding_cr, actual_bg);

    let inner_rect = egui::Rect::from_min_max(
        egui::pos2(content_rect.min.x + p_l, content_rect.min.y + p_t),
        egui::pos2(content_rect.max.x - p_r, content_rect.max.y - p_b),
    );

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    let prev_margin = ctx.inherited_margin;
    let prev_padding = ctx.inherited_padding;
    ctx.inherited_bg = inher_bg;
    ctx.inherited_color = inher_color;
    ctx.inherited_margin = inher_margin;
    ctx.inherited_padding = inher_padding;

    ui.scope_builder(egui::UiBuilder::new().max_rect(inner_rect), |ui| {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
            for (i, child) in children.iter().enumerate() {
                if i > 0 && gap > 0.0 { ui.add_space(gap); }
                super::super::renderer::render_node(ui, child, ctx);
            }
        });
    });

    ctx.inherited_bg = prev_bg;
    ctx.inherited_color = prev_color;
    ctx.inherited_margin = prev_margin;
    ctx.inherited_padding = prev_padding;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_menubar() {
        let json = serde_json::json!({
            "type": "MenuBar",
            "children": [{"type": "Menu", "text": "File"}]
        });
        assert_eq!(json["type"], "MenuBar");
    }
}
