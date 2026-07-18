use crate::renderer::{attr_f64, attr_str, get_attr_ctx, get_margin, get_padding, resolve_text, RenderCtx};

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

    let stretch = get_attr_ctx(
        ctx, node,
        "stretch",
        |v| v.as_bool(),
        |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_bool()),
        false,
    );

    let align = get_attr_ctx(
        ctx, node,
        "align",
        |v| v.as_str().map(|s| s.to_string()),
        |k| Some(ctx.theme.w_str("MenuItem", k, "left")),
        "left".to_string(),
    );

    let color = get_attr_ctx(
        ctx, node,
        "color",
        crate::theme::parse_color,
        |k| ctx.theme.w_color_opt("MenuItem", k),
        egui::Color32::from_gray(220),
    );

    let color_icon = get_attr_ctx(
        ctx, node,
        "color_icon",
        crate::theme::parse_color,
        |k| ctx.theme.w_color_opt("MenuItem", k),
        color,
    );

    let base_rounding = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuItem", "rounding", 4.0)))
        .unwrap_or(4.0);

    let font_id = egui::FontId::proportional(size);
    let content = ui.painter().layout_no_wrap(label.clone(), font_id, color_icon);
    let content_size = content.size();

    let margin = get_margin(node, &ctx.inherited, &ctx.theme);
    let padding = get_padding(node, &ctx.inherited, &ctx.theme, egui::Margin::ZERO);

    if margin.top > 0 { ui.add_space(margin.top as f32); }

    let reserved_size = if stretch {
        let pad_sum = padding.left as f32 + padding.right as f32 + margin.left as f32 + margin.right as f32;
        let stretch_w = ctx.inherited.get("popup_content_w").and_then(|v| v.as_f64().map(|f| f as f32));
        let inner_w = if let Some(w) = stretch_w {
            (w - pad_sum).max(1.0)
        } else {
            (ui.available_width() - pad_sum).max(1.0)
        };
        egui::vec2(inner_w, content_size.y)
    } else {
        content_size
    };

    let out = crate::widgets::base::widget_paint_custom(
        ui, node, &ctx.theme, "MenuItem",
        reserved_size, egui::Sense::click(), enabled,
        &ctx.inherited,
    );

    let text_x = match align.as_str() {
        "center" => egui::Align::Center.align_size_within_range(content_size.x, out.inner_rect.x_range()).min,
        "right"  => out.inner_rect.right() - content_size.x,
        _        => out.inner_rect.left(),
    };
    let text_y = egui::Align::Center.align_size_within_range(content_size.y, out.inner_rect.y_range()).min;
    ui.painter().galley_with_override_text_color(egui::pos2(text_x, text_y), content, color_icon);

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



