use crate::border::{draw_border, get_border};
use crate::renderer::{attr_f64, attr_str, get_padding, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {

    let binding = match attr_str(node, "binding") {
        Some(key) => key.to_string(),
        None => {
            log::warn!("ComboBox: отсутствует атрибут 'binding'");
            return;
        }
    };

    let items_key = attr_str(node, "items").unwrap_or(&binding);
    let width = attr_f64(node, "width")
        .unwrap_or_else(|| ctx.theme.w_f64("ComboBox", "width", 200.0));

    let items: Vec<String> = ctx
        .state
        .get_vec_string(items_key)
        .cloned()
        .unwrap_or_else(|| {
            log::warn!("ComboBox: список '{}' не найден в StateRegistry", items_key);
            Vec::new()
        });

    if items.is_empty() {
        ui.label("⚠ ComboBox: список пуст");
        return;
    }

    let mut selected = ctx.state.get_usize(&binding).unwrap_or(0usize);
    selected = selected.min(items.len().saturating_sub(1));
    let current = items.get(selected).cloned().unwrap_or_default();

    let text_fg = ctx.theme.w_color("ComboBox", "text_color", egui::Color32::from_rgb(0xE0, 0xE0, 0xE0));
    let bg = ctx.theme.w_color("ComboBox", "background", egui::Color32::from_rgb(0x2A, 0x2A, 0x33));
    let sel_bg = ctx.theme.w_color("ComboBox", "sel_bg", egui::Color32::from_rgba_unmultiplied(0x33, 0x66, 0xCC, 0x66));
    let popup_bg = ctx.theme.w_color("ComboBox", "popup_bg", egui::Color32::from_rgb(0x1C, 0x1E, 0x24));
    let height = ctx.theme.w_f64("ComboBox", "height", 32.0) as f32;
    let rounding = ctx.theme.w_f64("ComboBox", "rounding", 4.0) as u8;
    let inner_pad = get_padding(node, &ctx.inherited, &ctx.theme, "ComboBox", egui::Margin::symmetric(10, 0));
    let (pad_l, pad_r, pad_t, pad_b) = (inner_pad.left as f32, inner_pad.right as f32, inner_pad.top as f32, inner_pad.bottom as f32);
    let border = get_border(node, &ctx.theme, "ComboBox");

    let font_h = ui
        .painter()
        .layout_no_wrap(
            "A".into(),
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        )
        .size()
        .y;

    let combo_w = (width as f32).max(20.0 + pad_l + pad_r);
    let combo_h = (height as f32).max(font_h + pad_t + pad_b);

    let open_key = format!("__combo_open_{binding}");
    let is_open = ctx.state.get_bool(&open_key).unwrap_or(false);

    let (rect, resp) = ui.allocate_exact_size(
        egui::vec2(combo_w, combo_h),
        egui::Sense::click(),
    );

    ui.painter().rect_filled(rect, egui::CornerRadius::same(rounding), bg);
    draw_border(ui, rect, egui::CornerRadius::same(rounding), &border);

    let text_pos = egui::pos2(rect.left() + pad_l, rect.center().y);
    ui.painter().text(text_pos, egui::Align2::LEFT_CENTER, &current, egui::FontId::proportional(14.0), text_fg);

    // Рисуем треугольник вместо иконки Phosphor (надежнее, чем шрифтовые codepoint'ы)
    let arrow_size = 4.0;
    let arrow_right = rect.right() - pad_r;
    let arrow_center = rect.center().y;
    if is_open {
        // Треугольник вверх: вершина вверх, основание внизу
        let p1 = egui::pos2(arrow_right, arrow_center - arrow_size);
        let p2 = egui::pos2(arrow_right - arrow_size, arrow_center + arrow_size);
        let p3 = egui::pos2(arrow_right + arrow_size, arrow_center + arrow_size);
        ui.painter().add(egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            text_fg,
            egui::Stroke::NONE,
        ));
    } else {
        // Треугольник вниз: вершина внизу, основание наверху
        let p1 = egui::pos2(arrow_right, arrow_center + arrow_size);
        let p2 = egui::pos2(arrow_right - arrow_size, arrow_center - arrow_size);
        let p3 = egui::pos2(arrow_right + arrow_size, arrow_center - arrow_size);
        ui.painter().add(egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            text_fg,
            egui::Stroke::NONE,
        ));
    }

    if resp.clicked() {
        ctx.state.set_bool(&open_key, !is_open);
    } else if resp.clicked_elsewhere() && is_open {
        ctx.state.set_bool(&open_key, false);
    }

    if is_open {
        let area = egui::Area::new(egui::Id::new(&format!("__combo_popup_{binding}")))
            .order(egui::Order::Foreground)
            .fixed_pos(rect.left_bottom())
            .constrain(true);

        area.show(ui.ctx(), |ui| {
            let popup_border = get_border(node, &ctx.theme, "ComboBox");
            let frame = egui::Frame::new()
                .fill(popup_bg)
                .corner_radius(egui::CornerRadius::same(rounding))
                .stroke(egui::Stroke::new(popup_border.width, popup_border.color));

            frame.show(ui, |ui| {
                ui.set_min_width(combo_w);
                for (i, item) in items.iter().enumerate() {
                    let is_sel = selected == i;
                    let item_color = if is_sel { egui::Color32::WHITE } else { text_fg };
                    let item_text = egui::RichText::new(item.as_str()).color(item_color);
                    let item_bg = if is_sel { sel_bg } else { bg };

                    let item_resp = ui.add(
                        egui::Button::new(item_text).fill(item_bg).min_size(egui::vec2(combo_w, 24.0)),
                    );

                    if item_resp.clicked() {
                        selected = i;
                        ctx.state.set_usize(&binding, selected);
                        ctx.state.set_bool(&open_key, false);
                    }
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_combobox() {
        let json = serde_json::json!({
            "type": "ComboBox",
            "binding": "mic_idx",
            "items": "mic_list"
        });
        assert_eq!(attr_str(&json, "binding"), Some("mic_idx"));
    }
}


