# Theme JSON — темы и стилизация

## Структура theme.json

Файл темы — это JSON-объект, где каждый ключ — имя виджета или общий ключ, а значение — объект с настройками.

```json
{
  "Базовые цвета": {
    "background": "#14161B",
    "panel_fill": "#1A1D23",
    "window_fill": "#1E1E24",
    "text_primary": "#E0E0E0",
    "text_dim": "#888888",
    "link_color": "#66CCFF",
    "border_color": "#33333A",
    "accent": "#3366CC",
    "success": "#00AA66",
    "danger": "#CC3333",
    "warning": "#CC8800"
  },
  "global": {
    "item_spacing_x": 8,
    "item_spacing_y": 10
  },
  "Button": {
    "min_width": 100,
    "height": 28,
    "background": "#303030",
    "rounding": 6,
    "padding": [5, 10, 15, 5],
    "margin": 0,
    "color": "#E0E0E0",
    "background_hover": "#444455",
    "color_hover": "#FFFFFF",
    "background_click": "#555566",
    "color_click": "#FFFFFF",
    "focus_fill": "#334466",
    "border_hover": [3, "#88AAFF"]
  },
  "TextField": {
    "width": 200,
    "height": 28,
    "background": "#1C1E24",
    "inner_margin": 4,
    "rounding": 4,
    "color_text": "#E0E0E0",
    "margin": 0,
    "stepper_padding": 2,
    "stepper_background": ["#333344", 0.5],
    "stepper_rounding": 3,
    "valign": "center"
  },
  "Label": {
    "size": 14,
    "color": "#E0E0E0",
    "margin": 0
  },
  "ComboBox": {
    "width": 200,
    "height": 32,
    "background": "#2A2A33",
    "inner_pad_h": 10,
    "rounding": 4,
    "popup_bg": "#1C1E24",
    "color_text": "#E0E0E0",
    "sel_bg": "rgba(51,102,204,0.4)",
    "margin": 0
  },
  "Slider": {
    "width": 250,
    "height": 20,
    "margin": 0
  },
  "Separator": {
    "space": 6,
    "margin": 0
  },
  "Column": {
    "gap": 4,
    "padding": 8
  },
  "Row": {
    "gap": 4,
    "padding": 8
  },
  "Tabs": {
    "gap": 4,
    "active_color": "#66CCFF",
    "inactive_color": "#999999",
    "tab_height": 28,
    "margin": 0,
    "tab_padding": 10
  },
  "Panel": {
    "background": "#1A1D23",
    "rounding": 8,
    "padding": 12,
    "border": [1, "#333333"]
  },
  "Window": {
    "default_width": 400,
    "default_height": 300,
    "background": "#1E1E24",
    "border": [1, "#33333A"],
    "padding": 8
  },
  "Spinner": {
    "color": "#66CCFF",
    "size": 24
  },
  "FileDrop": {
    "background": "#1A1D23",
    "rounding": 8,
    "border": [1, "#333333"],
    "padding": 16
  },
  "ScrollArea": {
    "axis": "vertical"
  },
  "Checkbox": {
    "margin": 0
  },
  "RadioGroup": {
    "margin": 0
  },
  "IconButton": {
    "button_size": 20,
    "icon_size": 14,
    "color": "#CCCCCC",
    "background": "transparent",
    "background_hover": "rgba(68,68,85,0.25)",
    "color_hover": "#FFFFFF",
    "background_click": "rgba(85,85,102,0.4)",
    "color_click": "#888888",
    "border_hover": [2, "#88AAFF"]
  },
  "MenuItem": {
    "size": 12
  },
  "StatusBar": {
    "height": 26,
    "background": "#18181D"
  },
  "IconBar": {
    "background": "#1C1C22"
  }
}
```

## Переменные vars

В `theme.json` можно определить переменные и переиспользовать их в любых атрибутах. Синтаксис: `$имя_переменной`.

```json
{
  "vars": {
    "color_primary": "#3366CC",
    "color_text": "#E0E0E0",
    "color_bg": "#1A1D23",
    "radius": 6,
    "border_thin": [1, "$color_text"]
  },
  "Button": {
    "background": "$color_bg",
    "color_text": "$color_text",
    "rounding": "$radius",
    "border_hover": "$border_thin"
  }
}
```

### Правила работы

1. **Секция `vars` должна быть первой** в `theme.json` для читаемости (порядок не влияет на загрузку)
2. **`$var` работает везде** — в любом атрибуте любого виджета, включая массивы и вложенные объекты
3. **Авторезолв внутри vars** — переменные могут ссылаться друг на друга:
   ```json
   "vars": {
     "color_text": "#E0E0E0",
     "border_primary": [2, "$color_text"]
   }
   ```
4. **`$var` в UI-файлах** — работает не только в теме, но и в `ui.json`, `tabs/*.json`, `windows/*.json`
5. **При смене темы** — переменные перезагружаются вместе с темой, UI перерендеривается

### Примеры использования

```json
// Цвет в массиве
"vars": { "color_text": "#E0E0E0" },
"Button": { "border": [2, "$color_text", "dash"] }

// Целое значение (число, массив)
"vars": { "radius": 6, "border1": [4, "$color_text", 1, "dash", 5, 6] },
"Button": { "rounding": "$radius", "border_hover": "$border1" }

// В обычном тексте НЕ работает — только полное значение
// ✅ "$color_text"
// ❌ "color: $color_text" — не поддерживается
```

### Авторезолв (двухпроходный механизм)

1. **Проход 1** — переменные резолвятся внутри секции `vars`: `"border1": [4, "$color_text", ...]` → `[4, "#E0E0E0", ...]`
2. **Проход 2** — все остальные секции (`Button`, `Panel` и т.д.) резолвятся через готовый словарь переменных

Это позволяет строить цепочки: `color → border → widget_attr`.

## Приоритет правил

Атрибуты применяются по приоритету (от высшего к низшему):

1. **Явный атрибут на узле JSON** — `"background": "#FF0000"` в виджете
2. **Секция виджета в теме** — `"Button": { "background": "#303030" }`
3. **Дефолтная тема** (встроенная в код)
4. **egui default**

Пример: если в `theme.json` указан `"Button": { "background": "#FF0000" }`, а на кнопке в `ui.json` — `"background": "#00FF00"`, то применится `#00FF00`.

## Встроенные темы

### dark.json (дефолтная)

Тёмная тема, встроена в `Theme::default()`. Базовые цвета:

- `background: #14161B`
- `panel_fill: #1A1D23`
- `text_primary: #E0E0E0`
- `accent: #3366CC`

### light.json (план)

Светлая тема — в разработке. Будет содержать светлые цвета фона и тёмный текст.

### dj_green.json (план)

Зелёная тема в стиле DJ-софта — в разработке.

## Цветовые форматы

Цвета указываются в HEX:
- `#RRGGBB` — 6 символов
- `#RRGGBBAA` — 8 символов (с альфа-каналом)
- Можно без `#` — `"FF6600"`

## Секция 'global'

Содержит глобальные настройки отступов между виджетами:

- `item_spacing_x` — горизонтальный отступ
- `item_spacing_y` — вертикальный отступ

## Margin

Внешний отступ вокруг виджета со всех четырёх сторон. Указывается так же, как padding:

- число: `4` — 4px со всех сторон
- массив [2]: `[v, h]` — v вертикаль, h горизонталь
- массив [4]: `[top, right, bottom, left]`

```json
"IconButton": { "margin": 4 }
"Button": { "margin": [5, 10] }
"Label": { "margin": [2, 4, 6, 8] }
```

Margin суммируется с gap: `margin(right) + gap + margin(left)`.

## Padding

Указывается:
- числом: `8` — одинаково со всех сторон
- массивом [v, h]: `[4, 8]` — вертикальный и горизонтальный
- массивом [top, right, bottom, left]: `[5, 10, 15, 5]`

## Border в теме

Рамку можно задать сокращённым массивом:
```json
"Panel": { "border": [1, "#333333"] }
"Panel": { "border": [2, "#FF0000", "dash"] }
"Panel": { "border": [2, "#FF0000", "dash", 4, 6] }
```

Форматы массива:
- `[width]` — только толщина
- `[width, "#color"]` — толщина + цвет
- `[width, "#color", "type"]` — толщина + цвет + тип (solid/dash/dot)
- `[width, "#color", "type", gap, seg_len]` — полный формат
