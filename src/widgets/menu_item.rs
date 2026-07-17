use crate::renderer::{attr_f64, attr_str, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let action = attr_str(node, "action");
    let target = attr_str(node, "target");
    let enabled = node
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let raw_text = attr_str(node, "text").unwrap_or("");
    let text = resolve_text(raw_text, ctx);
    let icon_name = attr_str(node, "icon");
    let shortcut = attr_str(node, "shortcut");
    let size = attr_f64(node, "size")
        .unwrap_or_else(|| ctx.theme.w_f64("MenuItem", "size", 14.0)) as f32;

    let prefix = icon_name.and_then(|n| ctx.icons.resolve(n)).unwrap_or("");
    let label = if let Some(sc) = shortcut {
        format!("{prefix} {text}  {sc}")
    } else {
        format!("{prefix} {text}")
    };

    let stretch = node.get("stretch")
        .or_else(|| ctx.inherited.get("stretch"))
        .and_then(|v| v.as_bool())
        .or_else(|| Some(ctx.theme.w_bool("MenuItem", "stretch", false)))
        .unwrap_or(false);

    let align = attr_str(node, "align")
        .or_else(|| ctx.inherited.get("align").and_then(|v| v.as_str()))
        .map(|s| s.to_string())
        .or_else(|| Some(ctx.theme.w_str("MenuItem", "align", "left")))
        .unwrap_or_else(|| "left".to_string());

    let inherited_color = ctx.inherited.get("color").and_then(crate::theme::parse_color_value);
    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| inherited_color)
        .or_else(|| ctx.theme.w_color_opt("MenuItem", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let color_icon = node
        .get("color_icon")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.inherited.get("color_icon").and_then(crate::theme::parse_color_value))
        .or_else(|| ctx.theme.w_color_opt("MenuItem", "color_icon"))
        .unwrap_or(color);

    let base_rounding = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuItem", "rounding", 4.0)))
        .unwrap_or(4.0);

    let font_id = egui::FontId::proportional(size);
    let galley = ui.painter().layout_no_wrap(label.clone(), font_id, color_icon);
    let csize = galley.size();

    let inherited_margin = ctx.inherited.get("margin").and_then(crate::renderer::parse_padding);
    let margin = inherited_margin.unwrap_or(egui::Margin::ZERO);
    let inherited_pad = ctx.inherited.get("padding").and_then(crate::renderer::parse_padding);
    let pad = inherited_pad.unwrap_or(egui::Margin::ZERO);

    if margin.top > 0 { ui.add_space(margin.top as f32); }

    let content_size = if stretch {
        let avail_w = ui.available_width().max(csize.x + pad.left as f32 + pad.right as f32);
        egui::vec2(avail_w, csize.y)
    } else {
        csize
    };

    let out = crate::widgets::base::widget_base(
        ui, node, &ctx.theme, "MenuItem",
        content_size, egui::Sense::click(), enabled,
        egui::Color32::TRANSPARENT, base_rounding,
        pad,
        &ctx.inherited,
    );

    let text_x = match align.as_str() {
        "center" => egui::Align::Center.align_size_within_range(csize.x, out.inner_rect.x_range()).min,
        "right"  => out.inner_rect.right() - csize.x,
        _        => out.inner_rect.left(),
    };
    let text_y = egui::Align::Center.align_size_within_range(csize.y, out.inner_rect.y_range()).min;
    ui.painter().galley_with_override_text_color(egui::pos2(text_x, text_y), galley, color_icon);

    if out.response.clicked() && enabled {
        if let Some(action_name) = action {
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target.unwrap_or(""))
                .with_state(&ctx.state);
            ctx.actions.invoke(action_name, &mut action_ctx);
            ctx.state = action_ctx.state;
        }
    }

    if margin.bottom > 0 { ui.add_space(margin.bottom as f32); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_menuitem() {
        let json = serde_json::json!({"type": "MenuItem", "text": "Copy", "action": "copy"});
        assert_eq!(attr_str(&json, "text"), Some("Copy"));
    }
}
