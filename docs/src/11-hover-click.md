# Hover/Click/Focus — state-атрибуты

## Универсальный механизм

Любой виджет поддерживает state-атрибуты с постфиксами `_hover`, `_click`, `_focus`.   
Не только Button — а **любой**: Menu, MenuItem, Checkbox, TextField, Label, Column, Row и т.д.

```json
{
  "type": "Button",
  "text": "Наведи",
  "background": "#303030",
  "background_hover": "#505060",
  "background_click": "#606080",
  "background_focus": "#334466",
  "color_hover": "#FFFFFF",
  "color_click": "#CCCCFF",
  "border_hover": [3, "#FF0", "dot", 4, 1]
}
```

### Как это работает

Внутри один вызов `resolve_state_attr()` из `src/renderer.rs`:

```
node["key_hover"] → inherited["key_hover"] → theme["key_hover"] →
node["key"]       → inherited["key"]       → theme["key"]       → default
```

Тип атрибута может быть любым: цвет, число, строка, массив, Margin, Shadow, BorderStyle — `resolve_state_attr` принимает generic `parse`-замыкание.

### Приоритет state

**`click > focus > hover > base`** — нажатие перекрывает фокус, фокус перекрывает наведение.

### State и `_children`

State-атрибуты работают с наследованием `_children`:

```json
{
  "type": "MenuBar",
  "background_hover_children": "#87F",
  "background_click_children": "#77F",
  "color_hover_children": "#FFF"
}
```

Подробнее: [MenuBar наследование _children](15-menu-children.md).

---

## Атрибуты виджетов

### Button

```json
{
  "type": "Button",
  "text": "Наведи",
  "background": "#303030",
  "background_hover": "#505060",
  "color_hover": "#FFFFFF",
  "background_click": "#606080",
  "color_click": "#CCCCFF",
  "border_hover": [3, "#FF0", "dot", 4, 1]
}
```

| Атрибут | Дефолт | Описание |
|---------|--------|----------|
| `background` | #303030 | Фон в покое |
| `background_hover` | =background | Фон при наведении |
| `background_click` | =background_hover | Фон при нажатии |
| `background_focus` | =background | Фон при фокусе |
| `color` | #E0E0E0 | Цвет текста в покое |
| `color_hover` | =color | Цвет текста при наведении |
| `color_click` | =color | Цвет текста при нажатии |
| `color_text` | =color | Цвет текста (alias) |
| `color_text_hover` | =color_text | Цвет текста при наведении |
| `color_text_click` | =color_text | Цвет текста при нажатии |
| `border` | массив | Граница в покое |
| `border_hover` | массив | Граница при наведении |
| `border_click` | массив | Граница при нажатии |
| `border_focus` | массив | Граница при фокусе |
| `align` | "center" | Выравнивание в покое |
| `align_hover` | =align | Выравнивание при наведении |
| `align_click` | =align | Выравнивание при нажатии |
| `padding` | [16, 4] | Отступ в покое |
| `padding_hover` | =padding | Отступ при наведении |
| `padding_click` | =padding | Отступ при нажатии |
| `margin` | 0 | Внешний отступ |
| `margin_hover` | =margin | Отступ при наведении |
| `margin_click` | =margin | Отступ при нажатии |
| `shadow_background` | массив | Тень фона |
| `shadow_background_hover` | массив | Тень фона при наведении |
| `shadow_background_click` | массив | Тень фона при нажатии |
| `shadow_border` | массив | Тень рамки |
| `shadow_border_hover` | массив | Тень рамки при наведении |
| `shadow_border_click` | массив | Тень рамки при нажатии |
| `shadow_content` | массив | Тень контента |
| `shadow_content_hover` | массив | Тень контента при наведении |
| `shadow_content_click` | массив | Тень контента при нажатии |
| `shadow_icon` | массив | Тень иконки |
| `shadow_icon_hover` | массив | Тень иконки при наведении |
| `shadow_icon_click` | массив | Тень иконки при нажатии |
| `shadow_text` | массив | Тень текста |
| `shadow_text_hover` | массив | Тень текста при наведении |
| `shadow_text_click` | массив | Тень текста при нажатии |

### IconButton

```json
{
  "type": "IconButton",
  "icon": "trash-simple",
  "color": "#CC4444",
  "color_hover": "#FF8888",
  "color_click": "#FF4444",
  "background_hover": "rgba(200, 50, 50, 0.3)",
  "background_click": "rgba(200, 50, 50, 0.5)",
  "border_hover": [3, "#FF0", "dot", 4, 1]
}
```

| Атрибут | Дефолт | Описание |
|---------|--------|----------|
| `background` | transparent | Фон в покое |
| `background_hover` | rgba(68,68,85,0.25) | Фон при наведении |
| `background_click` | =hover | Фон при нажатии |
| `background_focus` | =background | Фон при фокусе |
| `color` | #CCCCCC | Цвет иконки в покое |
| `color_hover` | =color | Цвет иконки при наведении |
| `color_click` | =color | Цвет иконки при нажатии |
| `color_focus` | =color | Цвет иконки при фокусе |
| `icon_size` | 14 | Размер иконки |
| `icon_size_hover` | =icon_size | Размер иконки при наведении |
| `icon_size_click` | =icon_size | Размер иконки при нажатии |
| `border` | массив | Граница в покое |
| `border_hover` | массив | Граница при наведении |
| `border_click` | массив | Граница при нажатии |
| `border_focus` | массив | Граница при фокусе |
| (тени) | | Все shadow_* варианты, как в Button |

### Другие виджеты

Все виджеты, проходящие через `widget_paint_custom()` (TextField, Checkbox, RadioGroup, Label, Slider, ComboBox, Separator, Spacer и т.д.), поддерживают те же базовые state-атрибуты:

| Атрибут | Описание |
|---------|----------|
| `background_hover` | Фон при наведении |
| `background_click` | Фон при нажатии |
| `background_focus` | Фон при фокусе |
| `border_hover` | Граница при наведении |
| `border_click` | Граница при нажатии |
| `border_focus` | Граница при фокусе |
| `padding_hover` | Отступ при наведении |
| `padding_click` | Отступ при нажатии |
| `shadow_background_hover` | Тень фона при наведении |
| `shadow_border_hover` | Тень рамки при наведении |

---

## Тематические defaults

В `theme.json` можно задать дефолтные значения для всех кнопок:

```json
{
  "Button": {
    "background": "#303030",
    "color": "#E0E0E0",
    "background_hover": "#444455",
    "color_hover": "#FFFFFF",
    "background_click": "#555566",
    "color_click": "#FFFFFF",
    "focus_fill": "#334466",
    "border_hover": [3, "#88AAFF"]
  },
  "IconButton": {
    "color": "#CCCCCC",
    "button_size": 24,
    "icon_size": 14,
    "background_hover": "rgba(68,68,85,0.25)",
    "background_click": "rgba(85,85,102,0.4)",
    "color_hover": "#FFFFFF",
    "color_click": "#888888",
    "border_hover": [2, "#88AAFF"]
  }
}
```

### Приоритет разрешения

1. Атрибут на JSON-узле (`"background_hover": "#FF0000"`)
2. `ctx.inherited` (от `_children` родителя)
3. Тема виджета (`theme.widget["Button"]["background_hover"]`)
4. Дефолт в коде

Все state-атрибуты опциональны — если не указаны, используется базовое значение.

---

## Отключённое состояние

Если `enabled: false`, виджет рендерится серым:
- Фон: `Color32::from_gray(60)`
- Текст/иконка: `Color32::from_gray(100)`
- State-атрибуты игнорируются

---

## Логика выбора цвета (для справки)

```rust
if resp.is_pointer_button_down_on() {
    // _click → _focus
} else if resp.has_focus() {
    // _focus
} else if resp.hovered() {
    // _hover
} else {
    // base
}
```

Этот код живёт в `resolve_state_attr()` в `src/renderer.rs` единожды, не дублируется в виджетах.

---

## Примеры

```json
{
  "type": "Button",
  "text": "Опасная кнопка",
  "background": "#882222",
  "background_hover": "#CC3333",
  "background_click": "#991111",
  "color": "#FFCCCC",
  "color_hover": "#FFFFFF"
}

{
  "type": "IconButton",
  "icon": "heart",
  "color": "#FF6699",
  "color_hover": "#FF88BB",
  "background_hover": "rgba(255, 102, 153, 0.2)",
  "tooltip": "Избранное"
}
```
