# Hover/Click — состояния

## Per-widget состояния

Кнопки (`Button`, `IconButton`) поддерживают разные цвета и границы для состояний покоя, наведения и нажатия.

## Button

### Атрибуты на виджете

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

| Атрибут | По умолчанию | Описание |
|---------|-------------|----------|
| `fill` | #303030 | Цвет фона в покое |
| `hover_fill` | #444455 | Цвет фона при наведении |
| `hover_color` | =color | Цвет текста при наведении |
| `click_fill` | =hover_fill | Цвет фона при нажатии |
| `click_color` | =color | Цвет текста при нажатии |
| `color` | #E0E0E0 | Цвет текста/иконки в покое |
| `border_hover` | массив | Граница при наведении (тот же формат, что `border`) |
| `border_click` | массив | Граница при нажатии |

### Логика выбора цвета

```rust
if hovered && pressed {
    // click_fill / click_color / border_click
} else if hovered {
    // hover_fill / hover_color / border_hover
} else if has_focus {
    // focus_fill
} else {
    // fill / color / border
}
```

Для текста/иконки и границы аналогично.

## IconButton

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

| Атрибут | По умолчанию | Описание |
|---------|-------------|----------|
| `hover_fill` | rgba(68,68,85,0.25) | Цвет фона при наведении |
| `click_fill` | =hover_fill | Цвет фона при нажатии |
| `hover_color` | =color | Цвет иконки при наведении |
| `click_color` | =color | Цвет иконки при нажатии |
| `border_hover` | массив | Граница при наведении |
| `border_click` | массив | Граница при нажатии |
| `shadow_background_hover` | массив | Тень фона при наведении |
| `shadow_background_click` | массив | Тень фона при нажатии |
| `shadow_border_hover` | массив | Тень рамки при наведении |
| `shadow_border_click` | массив | Тень рамки при нажатии |
| `shadow_content_hover` | массив | Тень контента при наведении |
| `shadow_content_click` | массив | Тень контента при нажатии |

У `IconButton` фон прозрачный (`TRANSPARENT`) в покое, и меняется только при наведении/нажатии. Иконка перерисовывается поверх фона.

## Тематические defaults

В `theme.json` можно задать дефолтные цвета для всех кнопок:

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

## Приоритет

1. Атрибут на узле (`"background_hover": "#FF0000"`)
2. Тема виджета (`theme.widget["Button"]["background_hover"]`)
3. Дефолт темы (встроенный)

Все state-атрибуты (`hover_fill`, `hover_color`, `border_hover`, `shadow_background_hover`, `click_*`) опциональны — если не указаны, используется значение покоя (`fill`, `color`, `border`, `shadow_background`).

State-тени (`shadow_background`, `shadow_border`, `shadow_content`) также поддерживают `_hover`, `_click`, `_focus`.

## Отключённое состояние

Если `enabled: false`, кнопка рендерится серым цветом:
- Фон: `Color32::from_gray(60)`
- Текст/иконка: `Color32::from_gray(100)`
- State-атрибуты игнорируются

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
