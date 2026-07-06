---
name: rust-testing
description: Автоматическое написание тестов для Rust-проекта. Применять при создании новых функций, модулей, виджетов, bindings, locale-ключей. Используй, когда пользователь говорит «допиши тест», «проверь», «покрой тестами», или когда написан новый код без тестов.
---

# Rust — Тестирование

При появлении нового кода предлагать соответствующий уровень теста **сразу, не дожидаясь запроса**. Правило: новый код без теста — неполный код.

## Обязательное правило

- После каждой новой функции, модуля, виджета, binding или locale-ключа — автоматически предложить написать тест
- Если пользователь отказывается — не настаивать, но зафиксировать в TODO

## Сопоставление фича → уровень теста

| Фича | Уровень теста | Файл |
|------|--------------|------|
| Новая функция (чистая логика) | Unit | `tests/subsystems/<имя>.rs` |
| Новый виджет (Button, Slider, ...) | Smoke + Unit | `tests/smoke.rs` |
| Новый binding / state | Persistence | `tests/persistence.rs` |
| Новая locale / plural-форма | Locale | `tests/subsystems/locale.rs`, `tests/subsystems/plural.rs` |
| Новый файл с текстом (JSON, TOML, ...) | Encoding | `tests/encoding.rs` |
| Новый тип виджета в ui.json | Integration | `tests/integration.rs` |
| Новая тема / theme-атрибут | Theme | `tests/subsystems/theme.rs` |
| Новый action | Actions | `tests/subsystems/actions.rs` |

## Шаблоны тестов

### Unit-тест (чистая функция)

```rust
#[test]
fn test_parse_i18n_key() {
    assert_eq!(parse_i18n_key("{{btn.save}}"), Some("btn.save"));
    assert_eq!(parse_i18n_key("plain text"), None);
    assert_eq!(parse_i18n_key(""), None);
}
```

### StateRegistry-тест

```rust
#[test]
fn test_state_bind_unbind() {
    let mut state = State::new();
    let mut val: f64 = 0.0;
    state.bind("volume", &mut val);
    state.set_f64("volume", 75.0);
    assert_eq!(val, 75.0);
}
```

### Locale-тест (с fallback)

```rust
#[test]
fn test_locale_fallback_en() {
    let mut locales = LocaleRegistry::new("ru");
    locales.load(include_str!("../../locales/ru.json"));
    locales.load(include_str!("../../locales/en.json"));
    // ключ есть только в en
    assert_eq!(locales.resolve_raw("btn.cancel"), Some("Отмена"));
    // переключаем — ключ есть в en
    locales.switch("en");
    assert_eq!(locales.resolve_raw("btn.cancel"), Some("Cancel"));
}
```

### Plural-тест

```rust
#[test]
fn test_plural_ru_forms() {
    assert_eq!(plural_form("ru", 1.0), "one");   // 1 файл
    assert_eq!(plural_form("ru", 2.0), "few");   // 2 файла
    assert_eq!(plural_form("ru", 5.0), "many");  // 5 файлов
    assert_eq!(plural_form("ru", 0.0), "many");  // 0 файлов
    assert_eq!(plural_form("ru", 21.0), "one");  // 21 файл
}
```

### Smoke-тест (виджет без паники)

```rust
#[test]
fn test_smoke_button() {
    // Минимальный JSON-узел, не должно быть паники
    let json = r#"{"type": "Button", "text": "Test"}"#;
    let node: UiNode = serde_json::from_str(json).unwrap();
    let state = State::new();
    // Рендерим в фиктивный egui-контекст
    render_node(node, &state, &Actions::new()); // не panic!
}
```

### Persistence-тест

```rust
#[test]
fn test_persistence_roundtrip() {
    let mut state = State::new();
    let mut name = String::from("Test");
    let mut vol: f64 = 42.0;
    let mut gpu = true;
    state.bind("name", &mut name);
    state.bind("volume", &mut vol);
    state.bind("use_gpu", &mut gpu);

    let saved = state.to_json();
    let mut restored = State::from_json(&saved);

    assert_eq!(restored.get_str("name"), Some("Test"));
    assert_eq!(restored.get_f64("volume"), Some(42.0));
    assert_eq!(restored.get_bool("use_gpu"), Some(true));
}
```

### Encoding-тест

```rust
#[test]
fn test_file_utf8_no_bom() {
    let content = std::fs::read("locales/ru.json").unwrap();
    // проверяем отсутствие BOM
    assert!(!content.starts_with(&[0xEF, 0xBB, 0xBF]));
    // проверяем валидность UTF-8
    assert!(std::str::from_utf8(&content).is_ok());
}
```

## Edge-case чек-лист

При написании тестов обязательно покрывать:

- **Пустой ввод** — `""`, `None`, `0`
- **Граничные значения** — `min`, `max`, `max + 1`
- **Отсутствующий ключ** — binding/key не зарегистрирован → ожидаемое поведение (заглушка, а не panic)
- **Неверный тип** — f64 вместо usize, строка вместо числа → ошибка валидации
- **Смена состояния** — изменил → проверил изменилось, второй раз изменил → проверил снова
- **Fallback** — нет в ru → берём из en, нет нигде → заглушка `{{key}}`
- **Обратный порядок** — загрузить → переключить → переключить обратно → проверить что вернулось

## Именование тестовых файлов

- `tests/subsystems/<подсистема>.rs` — unit-тесты подсистемы
- `tests/integration.rs` — интеграционный тест всего проекта
- `tests/smoke.rs` — дымовые тесты виджетов
- `tests/persistence.rs` — тесты автосохранения
- `tests/encoding.rs` — тесты UTF-8/BOM
- `tests/validation.rs` — pre-flight валидатор

## После написания теста

- Запустить `cargo test -- имя_теста` — показать результат пользователю
- Если тест красный — разобрать причину, исправить код, перезапустить
- Если тест зелёный — сообщить пользователю: «Готово, test_xyz проходит»
