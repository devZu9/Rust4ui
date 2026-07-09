# Система действий (actions)

## ActionRegistry

`ActionRegistry` — реестр именованных действий (экшенов), которые вызываются из JSON по имени. Экшены пишутся на Rust и регистрируются при старте приложения.

```rust
pub struct ActionRegistry {
    actions: HashMap<String, ActionFn>,
}

type ActionFn = Box<dyn Fn(&mut ActionCtx) + Send + Sync>;
```

## ActionCtx

Контекст, передаваемый в экшен:

```rust
pub struct ActionCtx {
    pub target: String,    // строка цели (из атрибута target в JSON)
    pub state: StateRegistry, // состояние (можно читать и писать)
}
```

## Регистрация экшенов

```rust
let mut actions = ActionRegistry::new();

actions.register("click", |ctx| {
    let target = ctx.target.clone();
    log::info!("Нажата кнопка: {target}");
    ctx.state.set_string("status_text", format!("Нажата: {target}"));
});

actions.register("toggle_window", |ctx| {
    let current = ctx.state.get_bool("show_info_window").unwrap_or(false);
    ctx.state.set_bool("show_info_window", !current);
});

actions.register("save", |ctx| {
    // читаем состояние
    let name = ctx.state.get_string("name").unwrap_or("").to_string();
    log::info!("Сохраняем: {name}");
    ctx.state.set_string("status_text", "Сохранено ✓".into());
});
```

## Вызов из JSON

Экшен вызывается через атрибут `action` на кликабельном виджете:

```json
{ "type": "Button", "text": "Сохранить", "action": "save" }
{ "type": "Button", "text": "Удалить", "action": "delete", "target": "file_123" }
{ "type": "IconButton", "icon": "gear-six", "action": "open_settings" }
{ "type": "MenuItem", "text": "Выход", "action": "exit" }
```

## Атрибут target

Строка `target` передаётся в `ActionCtx.target`. Используется как идентификатор объекта действия:

```json
{ "type": "Button", "text": "Тег 1", "action": "click", "target": "tag_1" }
{ "type": "Button", "text": "Тег 2", "action": "click", "target": "tag_2" }
```

```rust
actions.register("click", |ctx| {
    match ctx.target.as_str() {
        "tag_1" => { /* обработать тег 1 */ }
        "tag_2" => { /* обработать тег 2 */ }
        _ => {}
    }
});
```

## Экшены в FileDrop

```json
{
  "type": "FileDrop",
  "action": "file_dropped",
  "multi": true,
  "children": [
    { "type": "Label", "text": "Бросьте файлы сюда" }
  ]
}
```

При броске файла в `target` помещается путь к файлу (или JSON-массив путей при `multi: true`).

```rust
actions.register("file_dropped", |ctx| {
    log::info!("Файл: {}", ctx.target);
    ctx.state.set_string("status_text", format!("Файл: {}", ctx.target));
});
```

## Экшены в Shortcut

```json
{ "type": "Shortcut", "key": "Ctrl+S", "action": "save" }
```

## Типовые паттерны экшенов

### Переключение bool

```rust
actions.register("toggle_window", |ctx| {
    let current = ctx.state.get_bool("show_info_window").unwrap_or(false);
    ctx.state.set_bool("show_info_window", !current);
});
```

### Сброс значений

```rust
actions.register("reset", |ctx| {
    ctx.state.set_f64("volume", 50.0);
    ctx.state.set_f64("font_size", 14.0);
    ctx.state.set_string("description", String::new());
    ctx.state.set_string("status_text", "Настройки сброшены".into());
});
```

### Подтверждение/отмена

```rust
actions.register("confirm_ok", |ctx| {
    ctx.state.set_bool("show_dialog", false);
    ctx.state.set_string("status_text", "Подтверждено ✓".into());
});

actions.register("confirm_cancel", |ctx| {
    ctx.state.set_bool("show_dialog", false);
    ctx.state.set_string("status_text", "Отменено".into());
});
```

## Проверка наличия экшена

```rust
if actions.has("save") {
    // экшен зарегистрирован
}
```

## Вызов из Rust кода

Экшены можно вызывать не только из JSON, но и из Rust:

```rust
let mut ctx = ActionCtx::new()
    .with_target("mic")
    .with_state(&state_registry);
actions.invoke("save", &mut ctx);
```
