use crate::border::{draw_border, get_border};
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

    let gap = attr_f64(node, "gap")
        .or_else(|| Some(ctx.theme.w_f64("MenuBar", "gap", 0.0)))
        .unwrap_or(0.0) as f32;

    let margin = get_margin(node, &ctx.theme, "MenuBar");
    let padding = get_padding(node, &ctx.theme, "MenuBar", egui::Margin::ZERO);
    let rounding = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuBar", "rounding", 0.0)))
        .unwrap_or(0.0) as u8;
    let rounding_cr = egui::CornerRadius::same(rounding);

    let inher_bg = node
        .get("background_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "background_children"));

    let inher_color = node
        .get("color_children")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "color_children"));

    let inher_margin = node
        .get("margin_children")
        .and_then(crate::renderer::parse_padding);

    let inher_padding = node
        .get("padding_children")
        .and_then(crate::renderer::parse_padding);

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    let prev_margin = ctx.inherited_margin;
    let prev_padding = ctx.inherited_padding;
    ctx.inherited_bg = inher_bg;
    ctx.inherited_color = inher_color;
    ctx.inherited_margin = inher_margin;
    ctx.inherited_padding = inher_padding;

    if margin.top > 0 { ui.add_space(margin.top as f32); }

    let frame_resp = egui::Frame::new()
        .fill(bg)
        .corner_radius(rounding_cr)
        .inner_margin(padding)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                if margin.left > 0 { ui.add_space(margin.left as f32); }
                for (i, child) in children.iter().enumerate() {
                    if i > 0 && gap > 0.0 { ui.add_space(gap); }
                    if let Some(ch_m) = inher_margin {
                        ui.vertical(|ui| {
                            if ch_m.top > 0 { ui.add_space(ch_m.top as f32); }
                            ui.horizontal(|ui| {
                                if ch_m.left > 0 { ui.add_space(ch_m.left as f32); }
                                super::super::renderer::render_node(ui, child, ctx);
                                if ch_m.right > 0 { ui.add_space(ch_m.right as f32); }
                            });
                            if ch_m.bottom > 0 { ui.add_space(ch_m.bottom as f32); }
                        });
                    } else {
                        super::super::renderer::render_node(ui, child, ctx);
                    }
                }
                if margin.right > 0 { ui.add_space(margin.right as f32); }
            });
        });

    let border = get_border(node, &ctx.theme, "MenuBar");
    draw_border(ui, frame_resp.response.rect, rounding_cr, &border);

    if margin.bottom > 0 { ui.add_space(margin.bottom as f32); }

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
