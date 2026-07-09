# Hover/Click — состояния

## Per-widget состояния

Кнопки (`Button`, `IconButton`) поддерживают разные цвета для состояний покоя, наведения и нажатия.

## Button

### Атрибуты на виджете

```json
{
  "type": "Button",
  "text": "Наведи",
  "fill": "#303030",
  "hover_fill": "#505060",
  "hover_text_color": "#FFFFFF",
  "click_fill": "#606080",
  "click_text_color": "#CCCCFF"
}
```

| Атрибут | По умолчанию | Описание |
|---------|-------------|----------|
| `fill` | #303030 | Цвет фона в покое |
| `hover_fill` | #444455 | Цвет фона при наведении |
| `hover_text_color` | =text_color | Цвет текста при наведении |
| `click_fill` | =hover_fill | Цвет фона при нажатии |
| `click_text_color` | =text_color | Цвет текста при нажатии |
| `text_color` | #E0E0E0 | Цвет текста в покое |

### Логика выбора цвета

```rust
if hovered && pressed {
    // click_fill (или hover_fill если не указан)
} else if hovered {
    // hover_fill
} else if has_focus {
    // focus_fill
} else {
    // fill
}
```

Для текста аналогично: если нажат — `click_text_color`, если наведён — `hover_text_color`, иначе `text_color`.

## IconButton

```json
{
  "type": "IconButton",
  "icon": "trash-simple",
  "icon_color": "#CC4444",
  "hover_fill": "rgba(200, 50, 50, 0.3)",
  "click_fill": "rgba(200, 50, 50, 0.5)"
}
```

| Атрибут | По умолчанию | Описание |
|---------|-------------|----------|
| `hover_fill` | rgba(68,68,85,0.25) | Цвет фона при наведении |
| `click_fill` | =hover_fill | Цвет фона при нажатии |

У `IconButton` фон прозрачный (`TRANSPARENT`) в покое, и меняется только при наведении/нажатии. Иконка перерисовывается поверх фона.

## Тематические defaults

В `theme.json` можно задать дефолтные цвета для всех кнопок:

```json
{
  "Button": {
    "fill": "#303030",
    "text_color": "#E0E0E0",
    "hover_fill": "#444455",
    "hover_text_color": "#FFFFFF",
    "click_fill": "#555566",
    "click_text_color": "#FFFFFF",
    "focus_fill": "#334466"
  },
  "IconButton": {
    "icon_size": 18,
    "hover_fill": "rgba(68,68,85,0.25)",
    "click_fill": "rgba(85,85,102,0.4)"
  }
}
```

## Приоритет

1. Атрибут на узле (`"hover_fill": "#FF0000"`)
2. Тема виджета (`theme.widget["Button"]["hover_fill"]`)
3. Дефолт темы (встроенный)

## Отключённое состояние

Если `enabled: false`, кнопка рендерится серым цветом:
- Фон: `Color32::from_gray(60)`
- Текст: `Color32::from_gray(100)`

## Примеры

```json
{
  "type": "Button",
  "text": "Опасная кнопка",
  "fill": "#882222",
  "hover_fill": "#CC3333",
  "click_fill": "#991111",
  "text_color": "#FFCCCC",
  "hover_text_color": "#FFFFFF"
}

{
  "type": "IconButton",
  "icon": "heart",
  "icon_color": "#FF6699",
  "hover_fill": "rgba(255, 102, 153, 0.2)",
  "tooltip": "Избранное"
}
```
