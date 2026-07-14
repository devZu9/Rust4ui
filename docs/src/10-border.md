# Border — рамки и обводки

## BorderStyle

Рамка (border) настраивается через структуру `BorderStyle`:

```rust
pub struct BorderStyle {
    pub width: f32,           // толщина
    pub color: egui::Color32, // цвет
    pub border_type: BorderType, // solid / dash / dot
    pub gap: f32,             // расстояние между штрихами (dash/dot)
    pub seg_len: f32,         // длина штриха (dash)
    pub round_cap: bool,      // скруглённые концы штрихов
    pub position: BorderPosition, // inside / center / outside
}

pub enum BorderType { Solid, Dash, Dot }
pub enum BorderPosition { Inside, Center, Outside }
```

## Атрибуты в JSON

### Полные атрибуты

```json
{
  "type": "Button",
  "border_width": 2,
  "border_color": "#FF0000",
  "border_type": "dash",
  "border_gap": 6,
  "border_seg_len": 8,
  "border_seg_cap": true,
  "border_position": "inside"
}
```

### Сокращённый массив `border`

```json
// Только ширина
{ "border": 2 }

// Ширина и цвет
{ "border": [2, "#FF0000"] }

// Ширина, цвет, тип
{ "border": [2, "#FF0000", "dash"] }

// Полный формат с opacity: ширина, цвет, opacity, тип, gap, seg_len
{ "border": [2, "#FF0000", 1, "dash", 4, 8] }
{ "border": [5, "#000", 0, "dot", 3, 2] }     // opacity=0 → border невидим
```

Параметр `opacity` (0.0–1.0) указывается третьим элементом, перед типом. Если опущен — поведение как при `1` (полностью видим). Совместимость: если третий элемент строка — старый формат (без opacity).

## Border opacity (прозрачность границы)

Opacity в шортхенде умножает альфа-канал цвета:

```json
// Полностью видимый
"border": [5, "#FF0000", 1, "dot", 3, 2]

// Полупрозрачный
"border": [5, "#FF0000", 0.5, "dot", 3, 2]

// Полностью прозрачный — border невидим, но shadow_border может быть виден
"border": [5, "#000", 0, "dot", 3, 2]
```

## Shadow от границы (shadow_border)

Тень от рамки повторяет dash/dot/gap/seg_len рамки. Z-order: `"under"` (под рамкой, по умолчанию) или `"over"` (над рамкой).

Формат: `[opacity, "under"|"over"?, "#color"?, x?, y?]`

```json
// Невидимая (по умолчанию)
"shadow_border": [0]

// Только opacity + z-order (цвет #000)
"shadow_border": [0.5, "over"]

// Opacity + z-order + цвет + offset
"shadow_border": [0.4, "under", "#000", 2, 2]

// Яркое свечение ПОВЕРХ рамки
"shadow_border": [0.6, "over", "#0FF", 0, 0]
```

State-версии:

```json
"shadow_border": [0],
"shadow_border_hover": [0.4, "under", "#000", 2, 2],
"shadow_border_click": [0.2, "under", "#000", 1, 1]
```

Работает на: Button, IconButton.

## Shadow от фона (shadow_background)

Тень под прямоугольником кнопки. Формат тот же, что у shadow_border. Z-order не применяется (всегда под фоном).

```json
// Дефолт (rgba(0,0,0,40), under, offset 2,2)
"shadow_background": [0.16, "under", "#000", 2, 2]

// Полностью прозрачная
"shadow_background": [0]
```

State-версии:

```json
"shadow_background": [0.16, "under", "#000", 2, 2],
"shadow_background_hover": [0.3, "under", "#000", 4, 4],
"shadow_background_click": [0, "under", "#000", 0, 0],
"shadow_background_focus": [0.2, "under", "#000", 2, 2]
```

Работает на: Button, IconButton.

## Shadow от иконки (shadow_icon)

Тень по контуру глифа иконки/текста. Работает на Button и IconButton.
Z-order: `"under"` (под иконкой, по умолчанию) или `"over"` (над иконкой, glow).

Формат: `[opacity, "under"|"over"?, "#color"?, x?, y?]`

```json
// Тень под иконкой: 30% чёрная, offset 1,1
"shadow_icon": [0.3, "under", "#000", 1, 1]

// Свечение (glow) — тень поверх иконки
"shadow_icon": [0.5, "over", "#0FF", 0, 0]
```

## Типы границ

| Тип | JSON значение | Описание |
|-----|--------------|----------|
| Solid | `"solid"` | Сплошная линия |
| Dash | `"dash"` или `"dashed"` | Пунктирная линия |
| Dot | `"dot"` или `"dotted"` | Точечная линия |

## Позиция границы

| Позиция | JSON значение | Описание |
|---------|--------------|----------|
| Inside | `"inside"` | Внутри прямоугольника (по умолчанию) |
| Center | `"center"` | По центру границы |
| Outside | `"outside"` | Снаружи прямоугольника |

## Приоритет определения

1. Явные ключи на узле: `border_width`, `border_color`
2. Сокращённый `border` на узле
3. Ключи в теме: `theme.widget["Button"]["border_width"]`
4. Сокращённый `border` в теме
5. Дефолт (0, цвет #444455, solid, inside)

## В теме

```json
{
  "Panel": {
    "border": [1, "#333333"]
  },
  "Button": {
    "border_width": 2,
    "border_color": "#6666FF",
    "border_type": "solid"
  }
}
```

## Виджеты с поддержкой border

- `Button` — рамка вокруг кнопки
- `IconButton` — рамка вокруг кнопки-иконки
- `TextField` — рамка вокруг поля ввода
- `ComboBox` — рамка вокруг выпадающего списка
- `Panel` — рамка панели
- `Window` — рамка окна
- `FileDrop` — рамка области приёма файлов
- `Label` — если указан padding
- `Column` — если указан padding
- `Row` — если указан padding
- `Tabs` — рамка вокруг контента
- `Checkbox` — если указан padding
- `RadioGroup` — если указан padding

## State-зависимые границы (border_hover / border_click)

Граница может меняться при наведении и нажатии. Атрибуты `border_hover` и `border_click` используют тот же формат, что и `border`:

```json
{
  "type": "Button",
  "text": "Наведи",
  "border": [2, "#444466"],
  "border_hover": [2, "#88AAFF"],
  "border_click": [2, "#FF8844", "dash", 4]
}
```

Если атрибут не указан — используется базовый `border` для всех состояний.

Работает на: Button, IconButton, TextField, Column, Row, Label, Checkbox, RadioGroup, Tabs.

## Примеры

```json
{
  "type": "Button",
  "text": "Пунктир",
  "border": [2, "#66CCFF", "dash", 3, 6]
}

{
  "type": "Panel",
  "border": [1, "#444466"],
  "rounding": 8,
  "children": []
}

{
  "type": "TextField",
  "border": [1, "#3366CC"],
  "binding": "name"
}
```
