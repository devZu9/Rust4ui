use crate::border::widget_border;
use crate::renderer::{attr_bool, attr_f64, attr_str, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &RenderCtx) {

    let raw_text = attr_str(node, "text").unwrap_or("");
    let icon_name = attr_str(node, "icon");
    let text = if let Some(icon) = icon_name.and_then(|n| ctx.icons.resolve(n)) {
        format!("{} {}", icon, resolve_text(raw_text, ctx))
    } else {
        resolve_text(raw_text, ctx)
    };

    let size = attr_f64(node, "size").unwrap_or(13.0);
    let bold = attr_bool(node, "bold").unwrap_or(false);
    let italic = attr_bool(node, "italic").unwrap_or(false);
    let monospace = attr_bool(node, "monospace").unwrap_or(false);
    let wrap = attr_bool(node, "wrap").unwrap_or(false);

    let color_str = attr_str(node, "color");
    let color = color_str
        .and_then(crate::theme::parse_hex_color)
        .unwrap_or(egui::Color32::from_rgb(0xCC, 0xCC, 0xCC));

    let mut rich = egui::RichText::new(text).size(size as f32).color(color);

    if bold {
        rich = rich.strong();
    }
    if italic {
        rich = rich.italics();
    }
    if monospace {
        rich = rich.monospace();
    }

    let pad = get_padding(node, &ctx.theme, "Label", egui::Margin::ZERO);

    let render_label = |ui: &mut egui::Ui| {
        if wrap {
            ui.label(rich);
        } else if attr_bool(node, "heading").unwrap_or(false) {
            ui.heading(rich);
        } else {
            ui.label(rich);
        }
    };

    let response = if pad != egui::Margin::ZERO {
        Some(egui::Frame::new()
            .inner_margin(pad)
            .show(ui, render_label))
    } else {
        render_label(ui);
        None
    };
    if let Some(r) = response {
        widget_border(ui, r.response.rect, node, &ctx.theme, "Label", egui::CornerRadius::same(4));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::RenderCtx;

    #[test]
    fn test_smoke_label() {
        let json = serde_json::json!({"type": "Label", "text": "Hello"});
        let ctx = RenderCtx::new();
        // Проверяем что не паникует при получении атрибутов
        assert_eq!(attr_str(&json, "text"), Some("Hello"));
        assert_eq!(attr_f64(&json, "size"), None);
        assert!(!attr_bool(&json, "bold").unwrap_or(false));
    }
}
