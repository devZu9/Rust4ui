# План: дехардкодинг виджетов — всё через тему

## Проблема

Сейчас виджеты не читают свои атрибуты из `theme.json`. Все значения `inner_margin`, `width`, `height`, `fill`, `rounding` и т.д. захардкожены прямо в коде виджета. Причина: `load_theme()` в `main.rs` «сплющивает» секции виджетов из `theme.json` в плоские `colors`/`sizes` хеши, теряя структуру.

## Архитектурное решение

### 1. Добавить `widget: HashMap<String, serde_json::Value>` в Theme

```rust
// src/theme.rs
#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: HashMap<String, String>,
    pub sizes: HashMap<String, f32>,
    pub rounding: HashMap<String, f32>,
    pub widget: HashMap<String, serde_json::Value>,  // <-- по-виджетные JSON-секции
}
```

Каждый ключ — имя виджета (`"TextField"`, `"Button"`, `"ComboBox"`...), значение — исходный JSON-объект из `theme.json`:
```json
"TextField": { "width": 200, "height": 28, "bg_fill": "#1C1E24" }
```

### 2. Helper-функции для чтения из темы виджета

В `Theme` добавить:

```rust
impl Theme {
    /// Прочитать f64 атрибут из секции виджета, через точку в ключ
    pub fn w_f64(&self, widget: &str, key: &str, default: f64) -> f64 {
        self.widget.get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_f64())
            .unwrap_or(default)
    }
    pub fn w_str(&self, widget: &str, key: &str, default: &str) -> String {
        self.widget.get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| default.to_string())
    }
    pub fn w_bool(&self, widget: &str, key: &str, default: bool) -> bool {
        self.widget.get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_bool())
            .unwrap_or(default)
    }
    pub fn w_color(&self, widget: &str, key: &str, default: Color32) -> Color32 {
        self.widget.get(widget)
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
            .unwrap_or(default)
    }
}
```

Виджет читает: `ctx.theme.w_f64("TextField", "width", 200.0)` — если в theme.json есть, берёт оттуда, иначе fallback 200.

### 3. Переписать `load_theme()` в main.rs

Вместо сплющивания в плоские хеши — сохранять секции как есть:

```rust
let parsed: serde_json::Value = serde_json::from_str(&content)?;
if let Some(obj) = parsed.as_object() {
    for (widget_name, widget_attrs) in obj {
        theme.widget.insert(widget_name.clone(), widget_attrs.clone());
    }
}
```

### 4. Дефолты в `Theme::default()`

Заранее заполнить `widget` дефолтными значениями для каждого виджета:

```rust
let mut widget = HashMap::new();
widget.insert("TextField".into(), serde_json::json!({
    "width": 200.0, "height": 28.0, "bg_fill": "#1C1E24", "inner_margin": 4.0
}));
widget.insert("Button".into(), serde_json::json!({
    "min_width": 100.0, "fill": "#303030", "rounding": 6.0, "padding_h": 16.0, "padding_v": 4.0
}));
widget.insert("ComboBox".into(), serde_json::json!({
    "width": 200.0, "height": 32.0, "bg_fill": "#2A2A33", "inner_pad_h": 10.0
}));
// ... все виджеты
```

### 5. Виджеты читают тему — по каждому виджету

#### text_field.rs (обычный режим)
```rust
// БЫЛО (хардкод)
let width = attr_f64(node, "width").unwrap_or(200.0);
let bg = Color32::from_rgb(0x1C, 0x1E, 0x24);
let frame = Frame::new().fill(bg).corner_radius(4).inner_margin(symmetric(0, 2));

// СТАЛО (тема)
let width = attr_f64(node, "width").unwrap_or_else(|| ctx.theme.w_f64("TextField", "width", 200.0));
let bg = ctx.theme.w_color("TextField", "bg_fill", Color32::from_rgb(0x1C, 0x1E, 0x24));
let corner = ctx.theme.w_f64("TextField", "rounding", 4.0);
let inner_margin = ctx.theme.w_f64("TextField", "inner_margin", 4.0);
let frame = Frame::new().fill(bg).corner_radius(CornerRadius::same(corner as u8)).inner_margin(symmetric(0, inner_margin as i8));
```

#### text_field.rs (render_number)
Аналогично: `stepper_width`, `stepper_bg` и т.д. — все из темы.

#### button.rs
```rust
let fill = attr_str(node, "fill")
    .and_then(parse_hex_color)
    .unwrap_or_else(|| ctx.theme.w_color("Button", "fill", Color32::from_rgb(0x30, 0x30, 0x30)));
let min_width = attr_f64(node, "min_width").unwrap_or_else(|| ctx.theme.w_f64("Button", "min_width", 100.0));
```

#### combo_box.rs
```rust
let width = attr_f64(node, "width").unwrap_or_else(|| ctx.theme.w_f64("ComboBox", "width", 200.0));
let height = ctx.theme.w_f64("ComboBox", "height", 32.0);
let bg = ctx.theme.w_color("ComboBox", "bg_fill", Color32::from_rgb(0x2A, 0x2A, 0x33));
let pad_h = ctx.theme.w_f64("ComboBox", "inner_pad_h", 10.0);
let text_pos = pos2(rect.left() + pad_h as f32, rect.center().y);
```

#### tabs.rs, panel.rs, slider.rs, separator.rs — аналогично

### 6. demo/theme.json — добавить недостающие поля

Дополнить theme.json атрибутами для виджетов, где их нет:
```json
{
  "TextField": {
    "width": 200, "height": 28, "bg_fill": "#1C1E24",
    "inner_margin": 4, "rounding": 4
  },
  "Button": {
    "fill": "#303030", "rounding": 6, "min_width": 100,
    "padding_h": 16, "padding_v": 4
  },
  "ComboBox": {
    "width": 200, "bg_fill": "#2A2A33", "height": 32,
    "inner_pad_h": 10, "rounding": 4
  }
}
```

### 7. Заголовок окна Windows

Для тёмного заголовка — вызвать `ViewportBuilder::with_decorations(true)` + в `DemoApp::new()` после создания окна через unsafe Windows API:

```rust
// В main.rs после eframe::run_native:
fn apply_dark_titlebar() {
    let hwnd = ...; // получить HWND через winit/eframe
    // DWM API:
    // DwmSetWindowAttribute(hwnd, 20/*DWMWA_USE_IMMERSIVE_DARK_MODE*/, true);
}
```

Либо использовать `with_decorations(false)` и отрисовать кастомный заголовок прямо в egui — 100% контроль.

### 8. Phosphor иконки

(уже в плане раньше) Загрузить `phosphor.ttf` в `FontDefinitions`, использовать `\u{E266}`/`\u{E277}`.

## Порядок реализации

1. `theme.rs` — добавить `widget` поле, helper-функции, дефолты
2. `main.rs` — переписать `load_theme()`, загрузить Phosphor
3. `demo/theme.json` — дополнить недостающими атрибутами
4. Поочерёдно каждый виджет — заменить хардкод на `ctx.theme.w_f64(...)`:
   - text_field.rs
   - button.rs
   - combo_box.rs
   - tabs.rs
   - slider.rs
   - panel.rs
   - separator.rs
   - column.rs, row.rs
   - label.rs
   - checkbox.rs
   - остальные

## Ожидаемый результат

- `theme.json` полностью управляет внешним видом
- Изменения в JSON — мгновенно видны в UI после перезапуска
- Разработчик может заменить тему целиком, не трогая код
- Ни одного захардкоженного пикселя/цвета в коде виджетов
