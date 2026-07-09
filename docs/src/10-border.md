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

// Полный формат: ширина, цвет, тип, gap, seg_len
{ "border": [2, "#FF0000", "dash", 4, 8] }
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
