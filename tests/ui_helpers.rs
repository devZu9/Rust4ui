use rust4ui::{render_node, RenderCtx};

/// Создаёт kittest-стенд для рендера одного JSON-узла без открытия окна.
/// Состояние (RenderCtx) — внутри Harness, доступно через `harness.state()`.
pub fn harness_for_node(node: serde_json::Value) -> egui_kittest::Harness<'static, RenderCtx> {
    egui_kittest::Harness::new_ui_state(
        move |ui, ctx| render_node(ui, &node, ctx),
        RenderCtx::new(),
    )
}

/// То же, но с заранее настроенным контекстом (зарегистрированные actions, state, theme).
pub fn harness_for_ctx(
    node: serde_json::Value,
    ctx: RenderCtx,
) -> egui_kittest::Harness<'static, RenderCtx> {
    egui_kittest::Harness::new_ui_state(
        move |ui, state| render_node(ui, &node, state),
        ctx,
    )
}
