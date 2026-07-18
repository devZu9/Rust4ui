use crate::border::widget_border;
use crate::renderer::{attr_bool, attr_f64, attr_str, get_padding, resolve_text, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let active_binding = attr_str(node, "active").unwrap_or("__tab_active");
    let gap = attr_f64(node, "gap")
        .unwrap_or_else(|| ctx.theme.w_f64("Tabs", "gap", 4.0));

    let children = match node.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    let mut active_id = ctx
        .state
        .get_string(active_binding)
        .unwrap_or("")
        .to_string();

    let tabs: Vec<(&serde_json::Value, String, String, bool)> = children
        .iter()
        .map(|tab| {
            let id = attr_str(tab, "id").unwrap_or("");
            let title = attr_str(tab, "title")
                .map(|t| resolve_text(t, ctx))
                .unwrap_or_else(|| id.to_string());
            let enabled = attr_bool(tab, "enabled").unwrap_or(true);
            (tab, id.to_string(), title, enabled)
        })
        .collect();

    if active_id.is_empty() && !tabs.is_empty() {
        active_id = tabs[0].1.clone();
        ctx.state.set_string(active_binding, active_id.clone());
    }

    let active_color = ctx.theme.w_color("Tabs", "active_color", egui::Color32::from_rgb(0x66, 0xCC, 0xFF));
    let inactive_color = ctx.theme.w_color("Tabs", "inactive_color", egui::Color32::from_rgb(0x99, 0x99, 0x99));
    let tab_pad = ctx.theme.w_f64("Tabs", "tab_padding", 8.0) as f32;

    ui.horizontal(|ui| {
        for (_, id, title, enabled) in &tabs {
            let selected = active_id == *id;
            let text_color = if selected { active_color } else { inactive_color };
            let text = egui::RichText::new(title.as_str()).color(text_color).size(14.0);
            let resp = ui.add_enabled(*enabled, egui::Button::new(text).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(tab_pad * 2.0, 28.0)));
            if resp.clicked() && *enabled {
                active_id = id.clone();
                ctx.state.set_string(active_binding, active_id.clone());
            }
            if selected {
                let line_y = resp.rect.bottom();
                ui.painter().line_segment(
                    [
                        egui::pos2(resp.rect.left() + 4.0, line_y - 1.0),
                        egui::pos2(resp.rect.right() - 4.0, line_y - 1.0),
                    ],
                    (2.0, active_color),
                );
            }
            if gap > 0.0 {
                ui.add_space(gap as f32);
            }
        }
    });

    ui.separator();

    let pad = get_padding(node, &ctx.inherited, &ctx.theme, "Tabs", None, egui::Margin::symmetric(0, 4));

    let mut render_content = |ui: &mut egui::Ui| {
        for (tab, id, _, _) in &tabs {
            if *id == active_id {
                if let Some(tab_children) = tab.get("children").and_then(|v| v.as_array()) {
                    for child in tab_children {
                        super::super::renderer::render_node(ui, child, ctx);
                    }
                }
            }
        }
    };

    let tab_response = if pad != egui::Margin::ZERO {
        Some(egui::Frame::new()
            .inner_margin(pad)
            .show(ui, render_content))
    } else {
        render_content(ui);
        None
    };
    if let Some(r) = tab_response {
        widget_border(ui, r.response.rect, node, &ctx.theme, "Tabs", egui::CornerRadius::same(4), Some(&r.response), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_tabs() {
        let json = serde_json::json!({
            "type": "Tabs",
            "active": "basic",
            "children": [
                {"type": "Tab", "id": "basic", "title": "Basic", "children": []}
            ]
        });
        assert_eq!(attr_str(&json, "active"), Some("basic"));
    }
}

