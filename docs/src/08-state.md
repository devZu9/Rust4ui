# Состояние (state) и binding

## StateRegistry

`StateRegistry` — центральное хранилище состояния приложения. Представляет собой `HashMap<String, StateValue>`.

```rust
pub enum StateValue {
    String(String),
    F64(f64),
    I64(i64),
    Usize(usize),
    Bool(bool),
    VecString(Vec<String>),
}
```

## Чтение и запись

```rust
let mut state = StateRegistry::new();

// Запись
state.set_string("name", "Анна".into());
state.set_f64("volume", 75.0);
state.set_bool("gpu", true);
state.set_usize("selected_device", 0);
state.set_vec_string("list", vec!["A".into(), "B".into()]);

// Чтение
let name = state.get_string("name");        // Option<&str>
let vol = state.get_f64("volume");          // Option<f64>
let gpu = state.get_bool("gpu");            // Option<bool>
let idx = state.get_usize("selected_device"); // Option<usize>
let list = state.get_vec_string("list");    // Option<&Vec<String>>

// Удаление
state.remove("temp_key");
```

## Binding в JSON

Виджеты привязываются к состоянию через атрибут `binding`:

```json
{ "type": "TextField", "binding": "name", "hint": "Ваше имя..." }
{ "type": "Checkbox", "binding": "use_gpu", "text": "GPU" }
{ "type": "Slider", "binding": "volume", "min": 0, "max": 100 }
{ "type": "ComboBox", "binding": "selected_device", "items": "device_list" }
{ "type": "RadioGroup", "binding": "app_theme", "options": [...] }
{ "type": "ColorPicker", "binding": "accent_color" }
{ "type": "Window", "open": "show_info_window" }
```

## Типы binding по виджетам

| Виджет | Тип StateValue | Атрибут |
|--------|---------------|---------|
| TextField (text) | `String` | `binding` |
| TextField (number) | `F64` | `binding` |
| Checkbox | `Bool` | `binding` |
| Slider | `F64` | `binding` |
| ComboBox (индекс) | `Usize` | `binding` |
| ComboBox (список) | `VecString` | `items` |
| RadioGroup | `Usize` | `binding` |
| ColorPicker | `String` (HEX) | `binding` |
| Window | `Bool` | `open` |
| Tabs | `String` | `active` |

## Инициализация состояния в Rust

```rust
let mut state = StateRegistry::new();
state.set_f64("volume", 50.0);
state.set_bool("use_gpu", true);
state.set_usize("selected_device", 0);
state.set_vec_string("device_list", vec![
    "Микрофон 1".into(),
    "Микрофон 2".into(),
]);
```

## Сериализация JSON

`StateRegistry` можно сериализовать и десериализовать:

```rust
// В JSON
let json = state.to_json();
// {"name":"Анна","volume":75.0,"gpu":true}

// Из JSON
let restored = StateRegistry::from_json(&json)?;
```

## Автосохранение

Планируется автоматическое сохранение состояния при изменении и восстановление после загрузки (см. навык `rust-autosave`).

## Интерполяция в локализации

Значения из состояния подставляются в локализованные строки через `{variable}`:

```json
// locales/ru.json
{ "greeting": "Привет, {name}!" }
```

```rust
state.set_string("name", "Мир".into());
let text = locale.i18n_text("{{greeting}}", &state);
// "Привет, Мир!"
```

## Сохранение настроек (Settings Persistence)

Некоторые ключи состояния автоматически сохраняются в `demo/settings.json` и восстанавливаются при перезапуске:

| Ключ | Тип | Описание |
|------|-----|----------|
| `window_size_width` | f64 | Ширина окна |
| `window_size_height` | f64 | Высота окна |
| `window_position_x` | f64 | Позиция окна по X |
| `window_position_y` | f64 | Позиция окна по Y |
| `active_tab` | string | Активная вкладка (id) |
| `active_locale` | string | Код языка (`"ru"` / `"en"`) |

Файл обновляется **только при изменении** значений — не чаще, чем нужно. Hot-reload его игнорирует.

```json
// demo/settings.json
{
  "window_size_width": 900.0,
  "window_size_height": 640.0,
  "window_position_x": 100.0,
  "window_position_y": 100.0,
  "active_tab": "basic",
  "active_locale": "ru"
}
```

## Служебные ключи

- `__tab_active` — активная вкладка в Tabs (по умолчанию)
- `__combo_open_{binding}` — состояние открытия ComboBox (автоматически)
- `__scroll_{binding}` — идентификатор скролла для TextField (автоматически)
