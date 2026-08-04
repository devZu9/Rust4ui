// v1.0.0 (2026-08-04) menu_item.rs 
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

    let icon_glyph = icon_name.and_then(|n| ctx.icons.resolve(n)).unwrap_or("");
    let has_icon = !icon_glyph.is_empty();

    let icon_gap = get_attr_ctx(
        ctx, node, None,
        "icon_gap",
        |v| v.as_f64().map(|n| n as f32),
        |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_f64()).map(|n| n as f32),
        6.0_f32,
    );

    let icon_position = get_attr_ctx(
        ctx, node, None,
        "icon_position",
        |v| v.as_str().map(|s| s.to_string()),
        |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_str()).map(|s| s.to_string()),
        "left".to_string(),
    );

    let stretch = get_attr_ctx(
        ctx, node, None,
        "stretch",
        |v| v.as_bool(),
        |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_bool()),
        false,
    );

    let width = get_attr_ctx(
        ctx, node, None,
        "width",
        |v| v.as_f64().map(|n| n as f32),
        |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_f64()).map(|n| n as f32),
        0.0_f32,
    );

    let align = get_attr_ctx(
        ctx, node, None,
        "align",
        |v| v.as_str().map(|s| s.to_string()),
        |k| ctx.theme.widget.get("MenuItem").and_then(|w| w.get(k)).and_then(|v| v.as_str()).map(|s| s.to_string()),
        "left".to_string(),
    );

    let color = get_attr_ctx(
        ctx, node, None,
        "color",
        crate::theme::parse_color,
        |k| ctx.theme.w_color_opt("MenuItem", k),
        egui::Color32::from_gray(220),
    );

    let color_icon = get_attr_ctx(
        ctx, node, None,
        "color_icon",
        crate::theme::parse_color,
        |k| ctx.theme.w_color_opt("MenuItem", k),
        color,
    );

    let _base_rounding = attr_f64(node, "rounding")
        .or_else(|| Some(ctx.theme.w_f64("MenuItem", "rounding", 4.0)))
        .unwrap_or(4.0);

    let font_id = egui::FontId::proportional(size);
    let text_label = if let Some(sc) = shortcut {
        format!("{text}  {sc}")
    } else {
        text.clone()
    };
    let text_galley = ui.painter().layout_no_wrap(text_label, font_id.clone(), color);
    let icon_galley = has_icon.then(|| ui.painter().layout_no_wrap(icon_glyph.to_string(), font_id, color_icon));

    let icon_w = icon_galley.as_ref().map_or(0.0, |g| g.size().x);
    let text_w = text_galley.size().x;
    let content_h = icon_galley.as_ref().map_or(text_galley.size().y, |g| text_galley.size().y.max(g.size().y));
    let gap_w = if has_icon && text_w > 0.0 { icon_gap } else { 0.0 };
    let content_w = if icon_position == "right" {
        text_w + gap_w + icon_w
    } else {
        icon_w + gap_w + text_w
    };
    let content_size = egui::vec2(content_w, content_h);

    let margin = get_margin(node, &ctx.inherited, &ctx.theme);
    let padding = get_padding(node, &ctx.inherited, &ctx.theme, egui::Margin::ZERO);

    if margin.top > 0 { ui.add_space(margin.top as f32); }

    let reserved_size = if stretch {
        let pad_sum = padding.left as f32 + padding.right as f32 + margin.left as f32 + margin.right as f32;
        let popup_w = ctx.inherited.get("popup_content_width").and_then(|v| v.as_f64().map(|f| f as f32));
        let base_w = popup_w.unwrap_or_else(|| ui.available_width());
        let effective_w = if width > 0.0 { base_w.max(width) } else { base_w };
        let inner_w = (effective_w - pad_sum).max(1.0);
        egui::vec2(inner_w, content_size.y)
    } else {
        let w = if width > 0.0 { content_size.x.max(width) } else { content_size.x };
        egui::vec2(w, content_size.y)
    };

    let out = crate::widgets::base::widget_paint_custom(
        ui, node, ctx,
        reserved_size, egui::Sense::click(), enabled,
    );

    let block_x = match align.as_str() {
        "center" => egui::Align::Center.align_size_within_range(content_size.x, out.inner_rect.x_range()).min,
        "right"  => out.inner_rect.right() - content_size.x,
        _        => out.inner_rect.left(),
    };
    let text_y = egui::Align::Center.align_size_within_range(content_size.y, out.inner_rect.y_range()).min;

    if has_icon {
        let (icon_x, text_x) = if icon_position == "right" {
            (block_x + text_w + gap_w, block_x)
        } else {
            (block_x, block_x + icon_w + gap_w)
        };
        if let Some(ig) = &icon_galley {
            ui.painter().galley_with_override_text_color(egui::pos2(icon_x, text_y), ig.clone(), color_icon);
        }
        ui.painter().galley_with_override_text_color(egui::pos2(text_x, text_y), text_galley, color);
    } else {
        ui.painter().galley_with_override_text_color(egui::pos2(block_x, text_y), text_galley, color);
    }

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



