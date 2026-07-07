use crate::renderer::{attr_bool, attr_f64, attr_str, get_padding, resolve_text, widget_margin, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    widget_margin(ui, &ctx.theme, "TextField");

    let binding = match attr_str(node, "binding") {
        Some(key) => key.to_string(),
        None => {
            log::warn!("TextField: отсутствует атрибут 'binding'");
            return;
        }
    };

    let mode = attr_str(node, "mode").unwrap_or("text");
    let password = attr_bool(node, "password").unwrap_or(false) || mode == "password";
    let multiline = attr_bool(node, "multiline").unwrap_or(false);
    let width = attr_f64(node, "width")
        .unwrap_or_else(|| ctx.theme.w_f64("TextField", "width", 200.0));
    let hint = attr_str(node, "hint")
        .map(|h| resolve_text(h, ctx))
        .unwrap_or_default();

    if mode == "number" {
        render_number(ui, &binding, node, ctx, width as f32, &hint);
        return;
    }

    let min_height = ctx.theme.w_f64("TextField", "height", 28.0) as f32;
    let bg = ctx.theme.w_color("TextField", "bg_fill", egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
    let rounding = ctx.theme.w_f64("TextField", "rounding", 4.0) as u8;
    let pad = get_padding(node, &ctx.theme, "TextField", egui::Margin::symmetric(0, 2));

    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let font_h = ui
        .painter()
        .layout_no_wrap(
            "A".into(),
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        )
        .size()
        .y;

    let field_w = (width as f32).max(20.0 + pad_l + pad_r);
    let field_h = min_height.max(font_h + pad_t + pad_b);

    let mut value = ctx.state.get_string(&binding).unwrap_or("").to_string();

    let mut text_edit: egui::TextEdit = if password {
        egui::TextEdit::singleline(&mut value)
            .password(true)
            .hint_text(hint)
            .desired_width(width as f32)
            .font(egui::TextStyle::Body)
    } else if multiline {
        egui::TextEdit::multiline(&mut value)
            .hint_text(hint)
            .desired_width(width as f32)
            .desired_rows(4)
            .font(egui::TextStyle::Body)
    } else {
        egui::TextEdit::singleline(&mut value)
            .hint_text(hint)
            .desired_width(width as f32)
            .font(egui::TextStyle::Body)
    };
    text_edit = text_edit.margin(pad).frame(true).background_color(bg);

    let radius = egui::CornerRadius::same(rounding);
    let w = &mut ui.style_mut().visuals.widgets;
    let prev = (w.inactive.corner_radius, w.hovered.corner_radius, w.active.corner_radius);
    w.inactive.corner_radius = radius;
    w.hovered.corner_radius = radius;
    w.active.corner_radius = radius;
    let resp = ui.add_sized(egui::vec2(field_w, field_h), text_edit);
    let w = &mut ui.style_mut().visuals.widgets;
    w.inactive.corner_radius = prev.0;
    w.hovered.corner_radius = prev.1;
    w.active.corner_radius = prev.2;

    if resp.changed() {
        ctx.state.set_string(&binding, value);
    }
}

fn render_number(
    ui: &mut egui::Ui,
    binding: &str,
    node: &serde_json::Value,
    ctx: &mut RenderCtx,
    width: f32,
    _hint: &str,
) {
    let min = attr_f64(node, "min").unwrap_or(f64::MIN);
    let max = attr_f64(node, "max").unwrap_or(f64::MAX);
    let step = attr_f64(node, "step").unwrap_or(1.0);
    let decimals = attr_f64(node, "decimals")
        .map(|d| d as usize)
        .or_else(|| {
            let s = format!("{step}");
            if let Some(dot) = s.find('.') {
                Some(s[dot + 1..].len())
            } else {
                Some(0)
            }
        })
        .unwrap_or(0);

    let min_height = ctx.theme.w_f64("TextField", "height", 28.0) as f32;
    let bg = ctx.theme.w_color("TextField", "bg_fill", egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
    let stepper_bg = ctx
        .theme
        .w_color("TextField", "stepper_bg", egui::Color32::from_rgb(0x33, 0x33, 0x44));
    let pad = get_padding(node, &ctx.theme, "TextField", egui::Margin::symmetric(0, 2));
    let rounding = ctx.theme.w_f64("TextField", "rounding", 4.0) as u8;

    let (pad_l, pad_r, pad_t, pad_b) = (pad.left as f32, pad.right as f32, pad.top as f32, pad.bottom as f32);

    let font_h = ui
        .painter()
        .layout_no_wrap(
            "A".into(),
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        )
        .size()
        .y;

    let field_w = width.max(40.0 + pad_l + pad_r);
    let field_h = min_height.max(font_h + pad_t + pad_b);

    let num_value = ctx.state.get_f64(binding).unwrap_or(0.0);
    let fmt_value = format!("{:.decimals$}", num_value, decimals = decimals);
    let mut text_value = fmt_value.clone();

    let (rect, rect_resp) = ui.allocate_exact_size(egui::vec2(field_w, field_h), egui::Sense::click());

    let fill = if rect_resp.hovered() {
        bg.linear_multiply(1.2)
    } else {
        bg
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(rounding as u8),
        fill,
        (1.0, egui::Color32::from_rgb(0x44, 0x44, 0x55)),
        egui::StrokeKind::Inside,
    );

    let content = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + pad_l, rect.min.y + pad_t),
        egui::pos2(rect.max.x - pad_r, rect.max.y - pad_b),
    );

    let text_changed;
    let area_hovered;

    {
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        let te = egui::TextEdit::singleline(&mut text_value)
            .font(egui::TextStyle::Body)
            .margin(pad)
            .frame(false);
        let edit_r = child_ui.add_sized(
            egui::vec2((content.size().x - 20.0).max(0.0), content.size().y),
            te,
        );
        text_changed = edit_r.changed();
        child_ui.vertical(|ui| {
            let up_btn = egui::Button::new("▲")
                .fill(stepper_bg)
                .min_size(egui::vec2(18.0, 0.0));
            if ui.add(up_btn).clicked() {
                let nv = (num_value + step).min(max);
                ctx.state.set_f64(binding, nv);
            }
            let down_btn = egui::Button::new("▼")
                .fill(stepper_bg)
                .min_size(egui::vec2(18.0, 0.0));
            if ui.add(down_btn).clicked() {
                let nv = (num_value - step).max(min);
                ctx.state.set_f64(binding, nv);
            }
        });
        drop(child_ui);

        area_hovered = rect_resp.rect.contains(
            ui.ctx()
                .pointer_interact_pos()
                .unwrap_or(egui::pos2(-1.0, -1.0)),
        );
    }

    if area_hovered {
        let sd = ui.input(|i| i.raw_scroll_delta.y);
        if sd != 0.0 {
            let nv = (num_value + step * sd.signum() as f64).max(min).min(max);
            ctx.state.set_f64(binding, nv);
        }
    }

    if text_changed {
        if let Ok(parsed) = text_value.parse::<f64>() {
            ctx.state.set_f64(binding, parsed.max(min).min(max));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_textfield() {
        let json = serde_json::json!({"type": "TextField", "binding": "name"});
        assert_eq!(attr_str(&json, "binding"), Some("name"));
    }
}
