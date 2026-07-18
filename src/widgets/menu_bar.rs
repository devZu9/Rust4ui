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

    let margin = get_margin(node, &ctx.inherited, &ctx.theme, "MenuBar");
    let padding = get_padding(node, &ctx.inherited, &ctx.theme, "MenuBar", egui::Margin::ZERO);
    let rounding = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuBar", "rounding", 0.0)))
        .unwrap_or(0.0) as u8;
    let rounding_cr = egui::CornerRadius::same(rounding);

    // Universal _children inheritance (JSON → theme)
    let old = ctx.inherit_children(node, Some("MenuBar"));

    let inher_rounding_val = attr_f64(node, "rounding_children")
        .or_else(|| attr_f64(node, "rounding"))
        .or_else(|| Some(ctx.theme.w_f64("MenuBar", "rounding", 0.0)))
        .unwrap_or(0.0) as u8;

    let frame_resp = egui::Frame::new()
        .fill(bg)
        .corner_radius(rounding_cr)
        .inner_margin(padding)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                if margin.top > 0 { ui.add_space(margin.top as f32); }
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                    if margin.left > 0 { ui.add_space(margin.left as f32); }
                    for (i, child) in children.iter().enumerate() {
                        if i > 0 && gap > 0.0 { ui.add_space(gap); }
                        let child_cr = match i {
                            0 => egui::CornerRadius { nw: inher_rounding_val, sw: inher_rounding_val, ..egui::CornerRadius::ZERO },
                            i if i == children.len() - 1 => egui::CornerRadius { ne: inher_rounding_val, se: inher_rounding_val, ..egui::CornerRadius::ZERO },
                            _ => egui::CornerRadius::ZERO,
                        };
                        ctx.inherited.insert("rounding".to_string(), serde_json::json!([child_cr.nw, child_cr.ne, child_cr.sw, child_cr.se]));

                        let ch_m = ctx.inherited.get("margin").and_then(crate::renderer::parse_padding);
                        if let Some(ch_m) = ch_m {
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
                if margin.bottom > 0 { ui.add_space(margin.bottom as f32); }
            });
        });

    ctx.restore_children(old);

    let border = get_border(node, &ctx.theme, "MenuBar");
    draw_border(ui, frame_resp.response.rect, rounding_cr, &border);

    for (brect, bradius, bstyle) in ctx.pending_borders.drain(..) {
        crate::border::draw_border(ui, brect, bradius, &bstyle);
    }
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


