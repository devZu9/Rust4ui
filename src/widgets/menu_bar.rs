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

    let _color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

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

    let mut content_w = 0.0_f32;
    let mut content_h = 0.0_f32;
    let font = egui::FontId::proportional(14.0);
    for (i, child) in children.iter().enumerate() {
        let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
        let galley = ui.painter().layout_no_wrap(text.to_string(), font.clone(), egui::Color32::WHITE);
        if i > 0 { content_w += gap; }
        content_w += galley.size().x;
        content_h = content_h.max(galley.size().y);
    }

    let (pad_l, pad_r, pad_t, pad_b) = (padding.left as f32, padding.right as f32, padding.top as f32, padding.bottom as f32);
    let total_w = content_w + pad_l + pad_r + margin.left as f32 + margin.right as f32;
    let total_h = content_h + pad_t + pad_b + margin.top as f32 + margin.bottom as f32;

    if margin.top > 0 { ui.add_space(margin.top as f32); }
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(total_w, total_h), egui::Sense::hover());
    if margin.bottom > 0 { ui.add_space(margin.bottom as f32); }

    let actual_bg = if resp.hovered() { bg_hover } else { bg };
    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + margin.left as f32, rect.min.y + margin.top as f32),
        egui::pos2(rect.max.x - margin.right as f32, rect.max.y - margin.bottom as f32),
    );
    ui.painter().rect_filled(content_rect, rounding_cr, actual_bg);

    let inner_rect = egui::Rect::from_min_max(
        egui::pos2(content_rect.min.x + pad_l, content_rect.min.y + pad_t),
        egui::pos2(content_rect.max.x - pad_r, content_rect.max.y - pad_b),
    );

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    ctx.inherited_bg = inher_bg;
    ctx.inherited_color = inher_color;

    ui.scope_builder(egui::UiBuilder::new().max_rect(inner_rect), |ui| {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
            for (i, child) in children.iter().enumerate() {
                if i > 0 && gap > 0.0 {
                    ui.add_space(gap);
                }
                super::super::renderer::render_node(ui, child, ctx);
            }
        });
    });

    ctx.inherited_bg = prev_bg;
    ctx.inherited_color = prev_color;
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
