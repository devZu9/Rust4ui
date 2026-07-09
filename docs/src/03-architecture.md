# Архитектура

## Цикл рендеринга

```
main.rs: update()
  │
  ├─ hot-reload: check file changes → reload ui.json, theme.json
  │
  ├─ theme.apply_to_egui(ctx) — применяет тему к egui::Context
  │
  ├─ state → updates status_text (пример)
  │
  ├─ locale switching (active_locale → switch lang)
  │
  └─ egui::CentralPanel::default().show(ctx, |ui| {
       render_node(ui, &tree, &mut ctx)
     })
```

Каждый фрейм (`update`) приложение:
1. Проверяет флаг `files_changed` (выставляется watcher'ом)
2. Если файлы изменились — перезагружает `ui.json` и `theme.json`
3. Применяет тему к egui-контексту
4. Обновляет строку статуса из состояния
5. Проверяет переключение языка
6. Рендерит корневой узел, который рекурсивно обходит всё дерево

## RenderCtx

Главный контекст рендеринга, передаётся всем виджетам:

```rust
pub struct RenderCtx {
    pub theme: Theme,          // текущая тема
    pub state: StateRegistry,  // состояние (binding)
    pub actions: ActionRegistry, // зарегистрированные экшены
    pub locale: LocaleRegistry,  // локализация
    pub icons: IconRegistry,     // иконки Phosphor
}
```

Создаётся один раз при старте и живёт всё время работы приложения.

## Theme Registry

Хранит:
- `colors: HashMap<String, String>` — цвета (background, text_primary и т.д.)
- `sizes: HashMap<String, f32>` — размеры (text_size, gap)
- `rounding: HashMap<String, f32>` — скругления
- `widget: HashMap<String, Value>` — per-widget настройки

Приоритет: **атрибут на узле > widget-тема > дефолтная тема > egui default**

`Theme::merge()` накладывает одну тему поверх другой.

## StateRegistry

Хранит состояние в `HashMap<String, StateValue>`, где `StateValue` может быть:

- `String(String)`
- `F64(f64)`
- `I64(i64)`
- `Usize(usize)`
- `Bool(bool)`
- `VecString(Vec<String>)`

Binding — ключ в StateRegistry, указывается атрибутом `"binding"` на виджете.

## ActionRegistry

Реестр экшенов с методами `register(name, fn)` и `invoke(name, ctx)`.

ActionCtx содержит:
- `target: String` — строка цели (передаётся из атрибута `target` на виджете)
- `state: StateRegistry` — состояние (мутируется в экшене)

## LocaleRegistry

Загружает JSON-файлы локалей с поддержкой:

- `{{key}}` — подстановка перевода в тексте
- `{variable}` — интерполяция состояния в текст
- CLDR plural rules: `files.count.one`, `files.count.few`, `files.count.many`

Поддерживаемые языки: ru, en, de, fr, pl, uk, be, ja, zh, ko

## IconRegistry

Загружает `icons/icons.json` (1512 иконок Phosphor) через `include_str!` — иконки вкомпилированы в бинарник.

Методы:
- `resolve(name) -> Option<&str>` — получить глиф по имени
- `resolve_glyph(name) -> String` — получить глиф или "⬡" если не найден

Имена иконок — реальные Phosphor-имена: `"floppy-disk"`, `"trash-simple"`, `"gear-six"` и т.д.

## RefResolver

Обрабатывает `$ref` в JSON — загружает и подставляет фрагменты из других файлов:

```json
{ "$ref": "tabs/main.json" }
```

Поддерживает:
- Кэширование загруженных файлов
- Обнаружение циклов
- Переопределение атрибутов (слияние `$ref` с остальными ключами)
