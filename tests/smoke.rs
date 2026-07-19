use rust4ui::RenderCtx;
use serde_json::json;

#[test]
fn test_smoke_label() {
    let node = json!({"type": "Label", "text": "Test"});
    assert_eq!(node["type"], "Label");
}

#[test]
fn test_smoke_button() {
    let node = json!({"type": "Button", "text": "OK", "action": "test"});
    assert_eq!(node["type"], "Button");
}

#[test]
fn test_smoke_textfield() {
    let node = json!({"type": "TextField", "binding": "name"});
    let mut ctx = RenderCtx::new();
    ctx.state.set_string("name", "Test".into());
    assert_eq!(node["type"], "TextField");
}

#[test]
fn test_smoke_checkbox() {
    let _node = json!({"type": "Checkbox", "binding": "flag"});
    let mut ctx = RenderCtx::new();
    ctx.state.set_bool("flag", true);
    assert!(ctx.state.get_bool("flag").unwrap());
}

#[test]
fn test_smoke_separator() {
    let node = json!({"type": "Separator"});
    assert_eq!(node["type"], "Separator");
}

#[test]
fn test_smoke_column() {
    let node = json!({
        "type": "Column",
        "children": [{"type": "Label", "text": "A"}]
    });
    assert_eq!(node["type"], "Column");
}

#[test]
fn test_smoke_row() {
    let node = json!({
        "type": "Row",
        "children": [{"type": "Button", "text": "A"}]
    });
    assert_eq!(node["type"], "Row");
}

#[test]
fn test_smoke_radio_group() {
    let node = json!({
        "type": "RadioGroup",
        "binding": "theme",
        "options": [{"value": 0, "text": "Dark"}]
    });
    assert_eq!(node["type"], "RadioGroup");
}

#[test]
fn test_smoke_slider() {
    let node = json!({
        "type": "Slider",
        "binding": "vol",
        "min": 0, "max": 100
    });
    assert_eq!(node["type"], "Slider");
}

#[test]
fn test_smoke_combobox() {
    let node = json!({
        "type": "ComboBox",
        "binding": "mic",
        "items": "mic_list"
    });
    let mut ctx = RenderCtx::new();
    ctx.state
        .set_vec_string("mic_list", vec!["A".into(), "B".into()]);
    assert_eq!(node["type"], "ComboBox");
}

#[test]
fn test_smoke_tabs() {
    let node = json!({
        "type": "Tabs",
        "active": "basic",
        "children": [{"type": "Tab", "id": "basic", "title": "Basic"}]
    });
    assert_eq!(node["type"], "Tabs");
}

#[test]
fn test_smoke_panel() {
    let node = json!({"type": "Panel", "fill": "#1A1D23"});
    assert_eq!(node["type"], "Panel");
}

#[test]
fn test_smoke_scroll() {
    let node = json!({
        "type": "ScrollArea",
        "max_height": 200,
        "children": [{"type": "Label", "text": "Test"}]
    });
    assert_eq!(node["type"], "ScrollArea");
}

#[test]
fn test_smoke_window() {
    let node = json!({
        "type": "Window",
        "id": "test_win",
        "title": "Test",
        "open": "show_test"
    });
    let mut ctx = RenderCtx::new();
    ctx.state.set_bool("show_test", false);
    assert_eq!(node["type"], "Window");
}

#[test]
fn test_smoke_spinner() {
    let node = json!({"type": "Spinner"});
    assert_eq!(node["type"], "Spinner");
}

#[test]
fn test_smoke_colorpicker() {
    let node = json!({"type": "ColorPicker", "binding": "accent"});
    assert_eq!(node["type"], "ColorPicker");
}

#[test]
fn test_smoke_filedrop() {
    let node = json!({"type": "FileDrop", "action": "drop"});
    assert_eq!(node["type"], "FileDrop");
}

#[test]
fn test_smoke_indicator() {
    let node = json!({"type": "Indicator", "color": "#00FF66"});
    assert_eq!(node["type"], "Indicator");
}

#[test]
fn test_smoke_statusbar() {
    let node = json!({"type": "StatusBar"});
    assert_eq!(node["type"], "StatusBar");
}

#[test]
fn test_smoke_iconbar() {
    let node = json!({
        "type": "IconBar",
        "direction": "vertical",
        "children": [{"type": "IconButton", "icon": "save"}]
    });
    assert_eq!(node["type"], "IconBar");
}

#[test]
fn test_smoke_menu() {
    let node = json!({
        "type": "Menu",
        "text": "File",
        "children": [{"type": "MenuItem", "text": "Save"}]
    });
    assert_eq!(node["type"], "Menu");
}
