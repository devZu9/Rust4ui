use egui_kittest::kittest::Queryable;
use rust4ui::{RefResolver, RenderCtx, strip_json_comments};
use std::path::Path;

mod ui_helpers;

/// Клик по кнопке вызывает зарегистрированный action и меняет состояние.
#[test]
fn test_click_button_changes_state() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{"type": "Button", "text": "Старт", "action": "start"}"#,
    )
    .unwrap();

    let mut ctx = RenderCtx::new();
    ctx.actions.register("start", |action_ctx| {
        action_ctx.state.set_bool("started", true);
    });

    let mut harness = ui_helpers::harness_for_ctx(node, ctx);
    harness.get_by_label("Старт").click();
    harness.run();

    assert_eq!(
        harness.state().state.get_bool("started"),
        Some(true),
        "клик по кнопке должен вызвать action и изменить состояние"
    );
}

/// Клик по вкладке переключает активную вкладку (state binding).
#[test]
fn test_click_tab_switches_active() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{
            "type": "Tabs",
            "active": "active_tab",
            "children": [
                {"type": "Tab", "id": "one", "title": "Первая", "children": [{"type": "Label", "text": "Содержимое 1"}]},
                {"type": "Tab", "id": "two", "title": "Вторая", "children": [{"type": "Label", "text": "Содержимое 2"}]}
            ]
        }"#,
    )
    .unwrap();

    let mut harness = ui_helpers::harness_for_node(node);
    harness.run();
    assert_eq!(harness.state().state.get_string("active_tab"), Some("one"));

    harness.get_by_label("Вторая").click();
    harness.run();

    assert_eq!(
        harness.state().state.get_string("active_tab"),
        Some("two"),
        "клик по вкладке должен переключить active_tab"
    );
}

/// Наведение на кнопку рендерится без паники; снимок до/после hover.
#[test]
fn test_button_hover() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{"type": "Button", "text": "Наведи", "padding": [20, 10]}"#,
    )
    .unwrap();

    let mut harness = ui_helpers::harness_for_node(node);
    harness.run();

    let button = harness.get_by_label("Наведи");
    button.hover();
    harness.run();
    harness.snapshot("widget_button_hover");
}

/// Скриншот-эталоны ключевых виджетов.
/// При изменении стиля тест падает с .new.png / .diff.png — эталон обновлять осознанно.
#[test]
fn test_snapshot_button() {
    let node: serde_json::Value = serde_json::from_str(
        r##"{"type": "Button", "text": "Нажми", "padding": [20, 10], "background": "#3366FF"}"##,
    )
    .unwrap();
    let mut harness = ui_helpers::harness_for_node(node);
    harness.run();
    harness.snapshot("widget_button");
}

#[test]
fn test_snapshot_text_field() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{"type": "TextField", "binding": "name", "width": 200}"#,
    )
    .unwrap();
    let mut ctx = RenderCtx::new();
    ctx.state.set_string("name", "Привет мир".into());
    let mut harness = ui_helpers::harness_for_ctx(node, ctx);
    harness.run();
    harness.snapshot("widget_text_field");
}

#[test]
fn test_snapshot_tabs() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{
            "type": "Tabs",
            "active": "active_tab",
            "children": [
                {"type": "Tab", "id": "one", "title": "Первая", "children": [{"type": "Label", "text": "Один"}]},
                {"type": "Tab", "id": "two", "title": "Вторая", "children": [{"type": "Label", "text": "Два"}]}
            ]
        }"#,
    )
    .unwrap();
    let mut harness = ui_helpers::harness_for_node(node);
    harness.run();
    harness.snapshot("widget_tabs");
}

#[test]
fn test_snapshot_window() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{
            "type": "Window",
            "id": "info",
            "title": "Инфо",
            "open": "win_open",
            "default_width": 300,
            "default_height": 150,
            "children": [{"type": "Label", "text": "Содержимое окна"}]
        }"#,
    )
    .unwrap();
    let mut ctx = RenderCtx::new();
    ctx.state.set_bool("win_open", true);
    let mut harness = ui_helpers::harness_for_ctx(node, ctx);
    harness.run();
    harness.snapshot("widget_window");
}

#[test]
fn test_snapshot_color_picker() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{"type": "ColorPicker", "binding": "accent"}"#,
    )
    .unwrap();
    let mut ctx = RenderCtx::new();
    ctx.state.set_string("accent", "#66CCFF".into());
    let mut harness = ui_helpers::harness_for_ctx(node, ctx);
    harness.run();
    harness.snapshot("widget_color_picker");
}

#[test]
fn test_snapshot_slider() {
    let node: serde_json::Value = serde_json::from_str(
        r#"{"type": "Slider", "binding": "volume", "min": 0, "max": 100, "width": 160}"#,
    )
    .unwrap();
    let mut ctx = RenderCtx::new();
    ctx.state.set_f64("volume", 42.0);
    let mut harness = ui_helpers::harness_for_ctx(node, ctx);
    harness.run();
    harness.snapshot("widget_slider");
}

/// Полное демо-дерево (demo/ui.json после $ref-резолва) рендерится без паники.
#[test]
fn test_render_demo_tree() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("demo");
    let content = std::fs::read_to_string(base.join("ui.json")).unwrap();
    let root: serde_json::Value =
        serde_json::from_str(&strip_json_comments(&content)).unwrap();

    let mut resolver = RefResolver::new();
    let resolved = resolver.resolve(&root, &base).unwrap();

    let mut harness = ui_helpers::harness_for_node(resolved);
    harness.run();
    harness.snapshot("ui_demo_tree");
}
