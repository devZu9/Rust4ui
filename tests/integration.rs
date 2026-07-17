use rust4ui::{strip_json_comments, LocaleRegistry, RefResolver, Validator};
use std::path::Path;

#[test]
fn test_load_demo_ui() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("demo");
    let ui_path = base.join("ui.json");
    assert!(ui_path.exists(), "demo/ui.json должен существовать");

    let content = std::fs::read_to_string(&ui_path).expect("Не удалось прочитать ui.json");
    let root: serde_json::Value =
        serde_json::from_str(&strip_json_comments(&content)).expect("ui.json должен быть валидным JSON");

    let mut resolver = RefResolver::new();
    let resolved = resolver
        .resolve(&root, &base)
        .expect("$ref резолвинг не должен падать");

    assert_eq!(resolved["type"], "Column");
    let children = resolved["children"]
        .as_array()
        .expect("Должны быть children");
    assert!(
        children.len() >= 4,
        "Ожидается минимум 4 дочерних элемента (табы + окна)"
    );
}

#[test]
fn test_load_demo_tabs() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("demo");
    let tabs_dir = base.join("tabs");
    assert!(tabs_dir.join("all.json").exists());

    for entry in std::fs::read_dir(&tabs_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |e| e == "json") {
            let content = std::fs::read_to_string(&path).unwrap();
            let _: serde_json::Value = serde_json::from_str(&strip_json_comments(&content))
                .unwrap_or_else(|e| panic!("{} должен быть валидным JSON: {e}", path.display()));
        }
    }
}

#[test]
fn test_load_demo_windows() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("demo")
        .join("windows");
    for entry in std::fs::read_dir(&base).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |e| e == "json") {
            let content = std::fs::read_to_string(&path).unwrap();
            let _: serde_json::Value = serde_json::from_str(&content)
                .unwrap_or_else(|e| panic!("{} должен быть валидным JSON: {e}", path.display()));
        }
    }
}

#[test]
fn test_load_theme() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("demo")
        .join("theme.json");
    assert!(path.exists());
    let content = std::fs::read_to_string(&path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&strip_json_comments(&content)).unwrap();
    let obj = parsed.as_object().expect("theme.json должен быть объектом");
    assert!(obj.len() > 10, "theme.json должен иметь > 10 секций виджетов");
    assert!(obj.contains_key("Label"), "Должна быть секция Label");
    assert!(obj.contains_key("Button"), "Должна быть секция Button");
}

#[test]
fn test_load_locales() {
    for lang in &["ru", "en"] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("locales")
            .join(format!("{lang}.json"));
        assert!(path.exists(), "locales/{lang}.json должен существовать");

        let mut locale = LocaleRegistry::new(lang);
        locale
            .load_file(lang, &path)
            .unwrap_or_else(|e| panic!("locales/{lang}.json должен загружаться: {e}"));

        let test_keys = ["btn.ok", "btn.cancel", "tab.basic"];
        for key in &test_keys {
            let resolved = locale.resolve(key);
            assert_ne!(
                resolved, *key,
                "Ключ {key} должен разрешаться в локали {lang}"
            );
        }
    }
}

#[test]
fn test_preflight_validate_demo() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("demo");
    let content = std::fs::read_to_string(base.join("ui.json")).unwrap();
    let root: serde_json::Value = serde_json::from_str(&content).unwrap();

    let mut resolver = RefResolver::new();
    let resolved = resolver.resolve(&root, &base).unwrap();

    let validator = Validator::new();
    let errors = validator.validate(&resolved);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("  {} — {}", e.path, e.message);
        }
    }
    assert!(
        errors.is_empty(),
        "Валидация demo/ui.json не должна выдавать ошибок"
    );
}
