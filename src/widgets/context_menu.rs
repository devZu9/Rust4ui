use crate::renderer::RenderCtx;

#[allow(dead_code)]
pub fn render_context_menu(
    _ui: &mut egui::Ui,
    node: &serde_json::Value,
    ctx: &mut RenderCtx,
    response: &egui::Response,
) {
    if let Some(menu) = node.get("context_menu") {
        response.context_menu(|ui| {
            if let Some(children) = menu.get("children").and_then(|v| v.as_array()) {
                for child in children {
                    super::super::renderer::render_node(ui, child, ctx);
                }
            }
        });
    }
}
