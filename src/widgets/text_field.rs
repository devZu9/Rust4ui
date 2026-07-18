use crate::border::{draw_border, get_state_border};
use crate::renderer::{attr_bool, attr_f64, attr_str, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {

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
    let base_width = attr_f64(node, "width")
        .unwrap_or_else(|| ctx.theme.w_f64("TextField", "width", 200.0));
    let hint = attr_str(node, "hint")
        .map(|h| resolve_text(h, ctx))
        .unwrap_or_default();

    if mode == "number" {
        render_number(ui, &binding, node, ctx, base_width as f32, &hint);
        return;
    }

    let min_height = ctx.theme.w_f64("TextField", "height", 28.0) as f32;
    let base_bg = ctx.theme.w_color("TextField", "background", egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
    let rounding = ctx.theme.w_f64("TextField", "rounding", 4.0) as u8;
    let base_pad = get_padding(node, &ctx.inherited, &ctx.theme, "TextField", egui::Margin::symmetric(0, 2));
    let _valign = ctx.theme.w_str2(node, "TextField", "valign")
        .unwrap_or_else(|| "center".to_string());

    let (base_pad_l, base_pad_r, base_pad_t, base_pad_b) = (base_pad.left as f32, base_pad.right as f32, base_pad.top as f32, base_pad.bottom as f32);

    let font_h = ui
        .painter()
        .layout_no_wrap(
            "A".into(),
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        )
        .size()
        .y;

    let field_w = (base_width as f32).max(20.0 + base_pad_l + base_pad_r);
    let field_h = if multiline {
        let rows = attr_f64(node, "desired_rows").unwrap_or(4.0);
        min_height.max(font_h * rows as f32 + base_pad_t + base_pad_b)
    } else {
        min_height.max(font_h + base_pad_t + base_pad_b)
    };

    let mut value = ctx.state.get_string(&binding).unwrap_or("").to_string();

    let text_edit: egui::TextEdit = if password {
        egui::TextEdit::singleline(&mut value)
            .password(true)
            .hint_text(hint)
            .font(egui::TextStyle::Body)
    } else if multiline {
        egui::TextEdit::multiline(&mut value)
            .hint_text(hint)
            .font(egui::TextStyle::Body)
    } else {
        egui::TextEdit::singleline(&mut value)
            .hint_text(hint)
            .font(egui::TextStyle::Body)
    };
    let fixed = attr_bool(node, "fixed").unwrap_or(true);
    let radius = egui::CornerRadius::same(rounding);
    let scroll_id = egui::Id::new(format!("__scroll_{binding}"));

    let w = &mut ui.style_mut().visuals.widgets;
    let prev = (w.inactive.corner_radius, w.hovered.corner_radius, w.active.corner_radius, w.active.bg_stroke);
    w.inactive.corner_radius = radius;
    w.hovered.corner_radius = radius;
    w.active.corner_radius = radius;
    w.active.bg_stroke = egui::Stroke::NONE;

    let (resp, border_rect) = if multiline && fixed {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(field_w, field_h), egui::Sense::click());
        ui.painter().rect_filled(rect, radius, base_bg);
        let inner_resp = ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            egui::ScrollArea::vertical()
                .id_salt(scroll_id)
                .max_height(field_h)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add(text_edit.frame(false).margin(base_pad).desired_width(field_w))
                })
                .inner
        }).inner;
        (inner_resp, rect)
    } else {
        let te = text_edit.frame(true).background_color(base_bg).margin(base_pad);
        let r = ui.add_sized(egui::vec2(field_w, field_h), te);
        let rr = r.rect;
        (r, rr)
    };
    let border = get_state_border(node, &ctx.theme, "TextField", &resp, true);

    (ui.style_mut().visuals.widgets.inactive.corner_radius,
     ui.style_mut().visuals.widgets.hovered.corner_radius,
     ui.style_mut().visuals.widgets.active.corner_radius,
     ui.style_mut().visuals.widgets.active.bg_stroke) = prev;
    draw_border(ui, border_rect, radius, &border);
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

    let stepper_pad = attr_f64(node, "stepper_padding")
        .unwrap_or_else(|| ctx.theme.w_f64("TextField", "stepper_padding", 2.0)) as f32;
    let stepper_bg = node.get("stepper_background")
        .and_then(crate::theme::parse_color_value)
        .unwrap_or_else(|| ctx.theme.w_color("TextField", "stepper_background", egui::Color32::TRANSPARENT));
    let stepper_round = attr_f64(node, "stepper_rounding")
        .unwrap_or_else(|| ctx.theme.w_f64("TextField", "stepper_rounding", 0.0)) as u8;
    let stepper_show = attr_str(node, "stepper_show").unwrap_or("always");
    let min_height = ctx.theme.w_f64("TextField", "height", 28.0) as f32;
    let base_bg = ctx.theme.w_color("TextField", "background", egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
    let base_pad = get_padding(node, &ctx.inherited, &ctx.theme, "TextField", egui::Margin::symmetric(0, 2));
    let rounding = ctx.theme.w_f64("TextField", "rounding", 4.0) as u8;
    let valign = ctx.theme.w_str2(node, "TextField", "valign")
        .unwrap_or_else(|| "center".to_string());

    let (base_pad_l, base_pad_r, base_pad_t, base_pad_b) = (base_pad.left as f32, base_pad.right as f32, base_pad.top as f32, base_pad.bottom as f32);

    let font_h = ui
        .painter()
        .layout_no_wrap(
            "A".into(),
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        )
        .size()
        .y;

    let field_w = width.max(40.0 + base_pad_l + base_pad_r);
    let field_h = min_height.max(font_h + base_pad_t + base_pad_b);

    let num_value = ctx.state.get_f64(binding).unwrap_or(0.0);
    let fmt_value = format!("{:.decimals$}", num_value, decimals = decimals);
    let mut text_value = fmt_value.clone();

    let (rect, rect_resp) = ui.allocate_exact_size(egui::vec2(field_w, field_h), egui::Sense::click());

    ui.painter().rect_filled(rect, egui::CornerRadius::same(rounding as u8), base_bg);

    let avail_h = field_h - base_pad_t - base_pad_b;
    let content_y = match valign.as_str() {
        "bottom" => rect.top() + base_pad_t + (avail_h - font_h),
        "center" => rect.top() + base_pad_t + (avail_h - font_h) / 2.0,
        _ => rect.top() + base_pad_t,
    };
    let content = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + base_pad_l, content_y),
        egui::pos2(rect.max.x - base_pad_r, content_y + font_h),
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
            .margin(egui::Margin::ZERO)
            .frame(false);
        let edit_r = child_ui.add_sized(
            egui::vec2(content.size().x, content.size().y),
            te,
        );
        text_changed = edit_r.changed();
        drop(child_ui);

        area_hovered = rect_resp.rect.contains(
            ui.ctx()
                .pointer_interact_pos()
                .unwrap_or(egui::pos2(-1.0, -1.0)),
        );

        // Stepper interaction (registered before border for correct state)
        let mut stepper_up_hover = false;
        let mut stepper_up_click = false;
        let mut stepper_down_hover = false;
        let mut stepper_down_click = false;
        let mut stepper_data = None;

        if stepper_show == "always" || (stepper_show == "hover" && area_hovered) {
            let up_glyph = ctx.icons.resolve_glyph("caret-up");
            let down_glyph = ctx.icons.resolve_glyph("caret-down");
            let icon_size = 14.0;
            let btn_dim = icon_size + 2.0 * stepper_pad;

            let up_maket = ui.painter().layout_no_wrap(
                up_glyph, egui::FontId::proportional(icon_size), egui::Color32::LIGHT_GRAY,
            );
            let down_maket = ui.painter().layout_no_wrap(
                down_glyph, egui::FontId::proportional(icon_size), egui::Color32::LIGHT_GRAY,
            );

            let stepper_x = rect.right() - base_pad_r - btn_dim;
            let stepper_center_y = rect.center().y;

            let up_btn_rect = egui::Rect::from_min_size(
                egui::pos2(stepper_x, stepper_center_y - btn_dim),
                egui::vec2(btn_dim, btn_dim),
            );
            let down_btn_rect = egui::Rect::from_min_size(
                egui::pos2(stepper_x, stepper_center_y),
                egui::vec2(btn_dim, btn_dim),
            );

            stepper_data = Some((up_maket, down_maket, up_btn_rect, down_btn_rect));

            let up_id = ui.auto_id_with("__num_up");
            let up_resp = ui.interact(up_btn_rect, up_id, egui::Sense::click());
            stepper_up_hover = up_resp.hovered();
            stepper_up_click = up_resp.is_pointer_button_down_on();
            if up_resp.clicked() {
                let nv = (num_value + step).min(max);
                ctx.state.set_f64(binding, nv);
            }

            let down_id = ui.auto_id_with("__num_down");
            let down_resp = ui.interact(down_btn_rect, down_id, egui::Sense::click());
            stepper_down_hover = down_resp.hovered();
            stepper_down_click = down_resp.is_pointer_button_down_on();
            if down_resp.clicked() {
                let nv = (num_value - step).max(min);
                ctx.state.set_f64(binding, nv);
            }
        }

        // Border with combined state (stepper interact already registered)
        let is_click = edit_r.is_pointer_button_down_on() || stepper_up_click || stepper_down_click;
        let is_focus = edit_r.has_focus();
        let is_hover = rect_resp.hovered() || edit_r.hovered() || stepper_up_hover || stepper_down_hover;

        let base_border = crate::border::get_border(node, &ctx.theme, "TextField");
        let state_border = if is_click {
            crate::border::apply_state_border(node, &ctx.theme, "TextField", "click", &base_border)
        } else if is_focus {
            crate::border::apply_state_border(node, &ctx.theme, "TextField", "focus", &base_border)
        } else if is_hover {
            crate::border::apply_state_border(node, &ctx.theme, "TextField", "hover", &base_border)
        } else {
            base_border
        };
        draw_border(ui, rect, egui::CornerRadius::same(rounding as u8), &state_border);

        // Stepper visuals (after border — overlap on top)
        if let Some((up_maket, down_maket, up_btn_rect, down_btn_rect)) = stepper_data {
            if stepper_bg.a() > 0 {
                ui.painter().rect_filled(up_btn_rect, egui::CornerRadius::same(stepper_round), stepper_bg);
            }
            ui.painter().galley_with_override_text_color(
                egui::pos2(
                    up_btn_rect.center().x - up_maket.size().x / 2.0,
                    up_btn_rect.center().y - up_maket.size().y / 2.0,
                ),
                up_maket,
                egui::Color32::LIGHT_GRAY,
            );

            if stepper_bg.a() > 0 {
                ui.painter().rect_filled(down_btn_rect, egui::CornerRadius::same(stepper_round), stepper_bg);
            }
            ui.painter().galley_with_override_text_color(
                egui::pos2(
                    down_btn_rect.center().x - down_maket.size().x / 2.0,
                    down_btn_rect.center().y - down_maket.size().y / 2.0,
                ),
                down_maket,
                egui::Color32::LIGHT_GRAY,
            );
        }
    }

    if area_hovered {
        let sd = ui.input(|i| i.raw_scroll_delta.y);
        if sd != 0.0 {
            let nv = (num_value + step * sd.signum() as f64).max(min).min(max);
            ctx.state.set_f64(binding, nv);
        }
        ui.input_mut(|i| i.smooth_scroll_delta = egui::Vec2::ZERO);
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


