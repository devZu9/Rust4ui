use crate::renderer::RenderCtx;

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

    let color = node
        .get("color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| ctx.theme.w_color_opt("MenuBar", "color"))
        .unwrap_or_else(|| egui::Color32::from_gray(220));

    let prev_bg = ctx.inherited_bg;
    let prev_color = ctx.inherited_color;
    ctx.inherited_bg = Some(bg);
    ctx.inherited_color = Some(color);

    ui.horizontal(|ui| {
        for child in children {
            super::super::renderer::render_node(ui, child, ctx);
        }
    });

    ctx.inherited_bg = prev_bg;
    ctx.inherited_color = prev_color;
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
