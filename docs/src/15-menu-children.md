# Универсальное наследование атрибутов (`_children`) и state-aware атрибуты (`_hover`/`_click`/`_focus`)

Система работает на двух универсальных механизмах:

1. **`_children`** — родитель передаёт атрибуты детям ровно на один уровень
2. **`_hover`/`_click`/`_focus`** — атрибут меняет значение в зависимости от состояния мыши/фокуса

Оба механизма **не требуют per-виджетного кода**. Любой атрибут, добавленный в JSON, работает автоматом.

---

## 1. Принцип `_children`

```
MenuBar
  ├── background_children   → → Menu (1 уровень)
  ├── border_children       → → Menu
  ├── icon_children         → → Menu
  └── ..._children          → → Menu
        └── (Menu НЕ передаёт дальше, если не указал _children)
              └── MenuItem — не получает
```

**Правила:**
- `_children` — **ровно один уровень** вниз
- Если ребёнок указал свой атрибут — он переопределяет наследованный
- Любой атрибут с суффиксом `_children` работает: хоть `background_hover_children`, хоть `icon_position_click_children`, хоть `border_focus_children`
- **`_children` из `theme.json`** — `*_children` можно задать в теме. Приоритет: **JSON-узел (inline) → тема**. Если атрибут не указан в JSON-узле, подхватывается из темы. Позволяет задать глобальные стили для всех Menu в одном месте.

  ```json
  // theme.json — глобальные настройки для всех Menu
  "Menu": {
    "rounding_children": 10,
    "padding_children": [20, 25]
  }

  // ui.json — переопределение для конкретного Menu
  {
    "type": "Menu",
    "text": "{{menu.file}}",
    "padding_children": [5, 5]  // свой padding, rounding из темы
  }
  ```

---

## 2. Принцип `_hover`/`_click`/`_focus`

Любой атрибут может иметь state-постфикс:

| Постфикс | Условие |
|----------|---------|
| (без) | Базовое состояние (inactive) |
| `_hover` | Курсор мыши наведён на элемент |
| `_click` | Кнопка мыши зажата на элементе |
| `_focus` | Элемент в фокусе (Tab-навигация) |

Приоритет: **click > focus > hover > base**.

**Как работает `get_attr_ctx()`:**

Единая функция, заменившая `resolve_state_attr`. Принимает `Option<&egui::Response>`:

```
// state-зависимый режим (c resp)
get_attr_ctx(ctx, node, Some(&resp), "key", parse, theme_lookup, default)

// базовый режим (без resp)
get_attr_ctx(ctx, node, None, "key", parse, theme_lookup, default)
```

В state-режиме цепочка:

```
node["key_click"] → inherited["key_click"] → theme["key_click"] → (state)
node["key_focus"] → inherited["key_focus"] → theme["key_focus"] → (state)
node["key_hover"] → inherited["key_hover"] → theme["key_hover"] → (state)
```

После state — base + _parent fallback:

```
node["key"] → inherited["key"] → theme["key"] → theme[_parent]["key_children"] → default
```

Шаг `_parent` — если у текущего виджета нет атрибута в node/inherited/theme, проверяется theme родителя с суффиксом `_children`. Например, `rounding` у MenuItem проверит `theme["Menu"]["rounding_children"]`.

---

## 3. Полный список атрибутов, работающих с `_children`

Все border-суб-атрибуты работают через `_children` благодаря `ctx.get_border()`, который обогащает node из `ctx.inherited`.

| Атрибут `_children` | Наследуется как | Формат значения |
|---|---|---|
| `background_children` | `background` | `"#HEX"` или `["#HEX", opacity]` |
| `background_hover_children` | `background_hover` | `"#HEX"` или `["#HEX", opacity]` |
| `background_click_children` | `background_click` | `"#HEX"` или `["#HEX", opacity]` |
| `background_focus_children` | `background_focus` | `"#HEX"` или `["#HEX", opacity]` |
| `color_children` | `color` | `"#HEX"` |
| `color_hover_children` | `color_hover` | `"#HEX"` |
| `color_click_children` | `color_click` | `"#HEX"` |
| `color_focus_children` | `color_focus` | `"#HEX"` |
| `padding_children` | `padding` | `N`, `[V,H]`, `[T,R,B,L]` |
| `margin_children` | `margin` | `N`, `[V,H]`, `[T,R,B,L]` |
| `rounding_children` | `rounding` | число (см. особенности ниже) |
| `border_children` | `border` | массив `[width, color, ...]` |
| `border_hover_children` | `border_hover` | массив |
| `border_click_children` | `border_click` | массив |
| `border_focus_children` | `border_focus` | массив |
| `border_position_children` | `border_position` | `"inside"`, `"center"`, `"outside"` |
| `border_width_children` | `border_width` | число |
| `border_color_children` | `border_color` | `"#HEX"` |
| `border_type_children` | `border_type` | `"solid"`, `"dash"`, `"dot"` |
| `border_gap_children` | `border_gap` | число |
| `border_seg_len_children` | `border_seg_len` | число |
| `icon_children` | `icon` | имя иконки (Phosphor) |
| `icon_position_children` | `icon_position` | `"left"`, `"right"` |
| `icon_gap_children` | `icon_gap` | число |

---

## 4. Пример: MenuBar с полным наследованием

```json
{
  "type": "MenuBar",
  "background": ["$like", 1],
  "background_children": ["$pitch", 1],
  "background_hover_children": ["#87F", 1],
  "background_click_children": ["#77F", 1],
  "color_children": "#022",
  "color_hover_children": "#FFF",
  "color_click_children": "#FFF",
  "padding_children": [15, 25],
  "margin_children": [2],
  "rounding_children": 15,
  "icon_children": "check",
  "icon_position_children": "right",
  "icon_position_hover_children": "left",
  "border": [2, "#FF0", 1, "dot", 4, 1],
  "border_children": [3, "$pitch", 1, "dot", 2],
  "border_hover_children": [4, "$like", 1, "dash", 8],
  "border_position_children": "outside",
  "gap": 0,
  "children": [
    {
      "type": "Menu",
      "text": "{{menu.file}}",
      "children": [{"type": "MenuItem", "text": "{{menu.new}}"}]
    },
    {
      "type": "Menu",
      "text": "{{menu.edit}}",
      "background": ["#000", 1],
      "children": [{"type": "MenuItem", "text": "{{menu.undo}}"}]
    }
  ]
}
```

Что происходит:
- **Фон**: все Menu наследуют `background` → `["$pitch", 1]`. При наведении → `["#87F", 1]`, при клике → `["#77F", 1]`.
- **Цвет текста**: `#022` → hover `#FFF` → click `#FFF`.
- **Иконка**: все Menu получают иконку `check` справа. При наведении → слева.
- **Бордер**: все Menu получают бордер `[3, "$pitch", 1, "dot", 2]` с позицией `outside`. При наведении → `[4, "$like", 1, "dash", 8]`.
- **Скругление**: первый Menu: `nw=15, sw=15` (левые углы), последний: `ne=15, se=15` (правые), средние: 0.
- **Второй Menu (Edit)** переопределил `background` → не наследует фон.

**Наследование на MenuItem:**
- MenuItem внутри File НЕ видит `background_children` от MenuBar — только если Menu сам их укажет через `_children`.
- Если Menu напишет `"background_children": "#333"`, то его MenuItem получат фон `#333`.

---

## 5. Настройка контекстного меню (попап)

Каждый `Menu` состоит из двух частей:
- **Кнопка** — видимая часть на MenuBar (настраивается через `background`, `border`, `padding`, `icon` и т.д.)
- **Попап** — контейнер для MenuItem, открывается при клике/ховере (настраивается через `popup_*`)

Попап-атрибуты не влияют на кнопку и наоборот.

### 5.1 Полный список `popup_*` атрибутов

| Атрибут | Формат значения | Дефолт | Описание |
|---------|----------------|--------|----------|
| `popup_background` | `"#HEX"` или `["#HEX", opacity]` | `#1C1E24` | Фон площадки (Frame.fill) |
| `popup_rounding` | число | `4` | Скругление углов площадки |
| `popup_padding` | `N`, `[V,H]`, `[T,R,B,L]` | `0` | Внутренний отступ площадки (Frame.inner_margin) |
| `popup_gap` | число | `0` | Расстояние между MenuItem (item_spacing.y) |
| `popup_min_width` | число | `0` (ширина кнопки) | Минимальная ширина площадки |
| `popup_max_height` | число | `0` (безлимит) | Максимальная высота с прокруткой (ScrollArea) |
| `popup_border` | массив `[width, color, ...]` | нет | Бордер площадки |
| `popup_shadow` | массив `[opacity, z_order?, "color"?, x?, y?]` | нет | Тень площадки |

Все атрибуты читаются по цепочке: **свой узел → `ctx.inherited` (от `_children` родителя) → тема → дефолт**.

Все атрибуты поддерживают `_children` суффикс: `popup_background_children`, `popup_padding_children`, `popup_border_children` и т.д.

Важно: `popup_*` читаются **до** `inherit_children()` на текущем Menu, чтобы `popup_*_children` от MenuBar были видны. Это особенность реализации — кнопка Menu читает свои атрибуты из inherited, а попап читает свои атрибуты ТОЖЕ из inherited, до того как Menu установит собственные `_children` для MenuItem.

Ширина попапа вычисляется автоматически: измеряется каждый MenuItem (текст + иконка + padding), берётся максимальная ширина. Если задан `popup_min_width` — используется он.

### Stretch, Align и Width в MenuItem

MenuItem внутри попапа поддерживает атрибуты:

| Атрибут | Тип | Дефолт | Описание |
|---------|-----|--------|----------|
| `stretch` | bool | `false` | Растянуть MenuItem на всю ширину попапа |
| `align` | string | `"left"` | Выравнивание контента (`"left"`, `"center"`, `"right"`) |
| `width` | number | `0` (auto) | Явная ширина пункта. Если шире самого широкого — попап расширяется |

Все три поддерживают `_children` наследование. `align_children` и `width_children` задаются на Menu и применяются ко всем его MenuItem:

```json
{
  "type": "Menu",
  "text": "{{menu.file}}",
  "stretch_children": true,
  "align_children": "center",
  "width_children": 250,
  "children": [
    { "type": "MenuItem", "text": "{{menu.new}}" },
    { "type": "Separator" },
    { "type": "MenuItem", "text": "{{menu.export}}" }
  ]
}
```

При `stretch: true` все MenuItem в попапе одинаковой ширины, даже если текст разной длины. Separator также растягивается на всю ширину попапа.

Если на одном из MenuItem указан `width`, и он больше самого широкого — попап расширяется под него. Если `width` на MenuItem меньше самой широкой кнопки — `stretch` дотягивает до размера попапа.

При `stretch: false` и указанном `width` — только этот пункт шире контента, остальные не меняются.

### 5.2 Пример: попап на одном Menu

```json
{
  "type": "Menu",
  "text": "{{menu.file}}",

  "popup_background": "#222",
  "popup_rounding": 12,
  "popup_padding": [8, 12],
  "popup_gap": 4,
  "popup_border": [1, "#555"],
  "popup_shadow": [0.3],
  "popup_min_width": 200,
  "popup_max_height": 400,

  "children": [
    { "type": "MenuItem", "text": "{{menu.new}}", "icon": "file-plus" },
    { "type": "MenuItem", "text": "{{menu.open}}", "icon": "folder" },
    { "type": "Separator" },
    { "type": "MenuItem", "text": "{{menu.export}}", "icon": "download" }
  ]
}
```

### 5.3 Пример: попап через `_children` на MenuBar (все Menu сразу)

```json
{
  "type": "MenuBar",
  "popup_background_children": "$color_bg_dark",
  "popup_rounding_children": 15,
  "popup_padding_children": [10, 15],
  "popup_gap_children": 2,
  "popup_border_children": [1, "#666", 1],
  "children": [
    {
      "type": "Menu",
      "text": "{{menu.file}}",
      "children": [
        { "type": "MenuItem", "text": "{{menu.new}}" }
      ]
    },
    {
      "type": "Menu",
      "text": "{{menu.help}}",
      "popup_background": "#111",
      "children": [
        { "type": "MenuItem", "text": "{{menu.about}}" }
      ]
    }
  ]
}
```

В этом примере:
- **File** наследует `popup_background` от MenuBar → `$color_bg_dark`
- **Help** переопределил `popup_background` → `#111` (свой, не наследует)

### 5.4 Попап в теме

Все `popup_*` можно задать в `theme.json` в секции `"Menu"`:

```json
{
  "Menu": {
    "popup_background": "$color_bg_dark",
    "popup_rounding": 8,
    "popup_padding": [8, 4],
    "popup_gap": 2,
    "popup_border": [1, "#555", 0.5],
    "popup_shadow": [0.2]
  }
}
```

---

## 6. Особенности `rounding_children`

`rounding_children` распределяется автоматически на **первый** и **последний** дочерний элемент (работает только в `menu_bar.rs`):

| Позиция | Скругление |
|---------|-----------|
| Первый ребёнок | `nw`=value, `sw`=value (левые углы) |
| Последний ребёнок | `ne`=value, `se`=value (правые углы) |
| Средние дети | Все 0 (прямоугольные) |

Хранится как массив `[nw, ne, sw, se]`, читается per-corner.

---

## 7. Приоритет разрешения атрибутов (полная цепочка)

```
// State-зависимая часть (если есть resp)
node["key_click"] → inherited["key_click"] → theme["key_click"]
node["key_focus"] → inherited["key_focus"] → theme["key_focus"]
node["key_hover"] → inherited["key_hover"] → theme["key_hover"]

// Base + _parent fallback
node["key"] → inherited["key"] → theme["key"] → theme[_parent]["key_children"] → default
```

Где:
- `node` — JSON-узел текущего виджета
- `inherited` — `ctx.inherited` HashMap (от `_children` родителя)
- `theme` — `theme.json → секция текущего виджета`
- `_parent` — имя родителя, установленное `inherit_children(node, Some("ParentName"))`
- `key_children` — ключ для поиска в теме родителя (например, `rounding_children`, `padding_children`)
- `default` — хардкод в Rust-виджете

Шаг `_parent` работает для всех атрибутов, читаемых через `get_attr_ctx`, `get_padding`, `get_margin`. Например, если MenuItem не указал `rounding` и в теме `MenuItem` его нет, `get_attr_ctx` проверит `theme["Menu"]["rounding_children"]`.

Для border дополнительно:
- `ctx.get_border()` обогащает node из `ctx.inherited` для ВСЕХ border-суб-атрибутов (`border_position`, `border_width`, `border_color`, `border_type`, `border_gap`, `border_seg_len`)

### Важное замечание: `get_attr_ctx` в `widget_paint_custom`

`widget_paint_custom` (общая функция отрисовки для всех custom-paint виджетов) использует `get_attr_ctx` для атрибутов `background`, `rounding`, `shadow_background`, `shadow_border`. **Для `padding` — не использует.**

`padding` вычисляется вызывающим виджетом (например, `menu_item.rs`) через `get_padding()` и передаётся в `widget_paint_custom` как `reserved_size`. Если бы `widget_paint_custom` перепроверял padding через `get_attr_ctx`, возник бы конфликт:

| Уровень | Цепочка | Результат |
|---------|---------|-----------|
| `menu_item.rs: get_padding()` | `node → theme → default` | `[15, 80]` из MenuItem ✅ |
| `widget_paint_custom: get_attr_ctx("padding")` | `node → inherited → theme → default` | `[10, 40]` из inherited ❌ |

Из-за того, что `get_attr_ctx` проверяет inherited ДО theme, `padding_children` выигрывал у собственного `padding` MenuItem. **Правило:** если атрибут уже корректно разрешён вызывающим кодом — не запускай для него `get_attr_ctx` повторно.

---

## 8. Как добавить новый атрибут с поддержкой `_children` и state

Достаточно написать `"foobar_children"` / `"foobar_hover_children"` / `"foobar_click_children"` в JSON — больше ничего не нужно. `inherit_children()` подхватит любой ключ с суффиксом `_children`, а `get_attr_ctx()` отработает любой state-постфикс.

Для border-суб-атрибутов дополнительно: если атрибут читается через `get_border()`, то `ctx.get_border()` сам подтянет все inherited-значения.

### Обратная совместимость

`get_attr_ctx` принимает `Option<&egui::Response>`:
- `Some(&resp)` — включена state-зависимость (hover/click/focus)
- `None` — только base + _parent fallback, state не проверяется

Это позволяет использовать единую функцию как для атрибутов, где Response уже есть (например, `color` после `widget_paint_custom`), так и для атрибутов, где Response ещё нет (например, `stretch`/`align`/`color` в menu_item.rs до получения `out.response`).
