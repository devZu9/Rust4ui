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

**Как работает `resolve_state_attr()`:**

```
node["bg_click"] → inherited["bg_click"] → theme["bg_click"] →
node["bg"]       → inherited["bg"]       → theme["bg"]       → default
```

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
node["key_hover"] → inherited["key_hover"] → theme["key_hover"] →
node["key"]       → inherited["key"]       → theme["key"]       → default
```

Где:
- `node` — JSON-узел текущего виджета
- `inherited` — `ctx.inherited` HashMap (от `_children` родителя)
- `theme` — `theme.json → секция текущего виджета`
- `default` — хардкод в Rust-виджете

Для border дополнительно:
- `ctx.get_border()` обогащает node из `ctx.inherited` для ВСЕХ border-суб-атрибутов (`border_position`, `border_width`, `border_color`, `border_type`, `border_gap`, `border_seg_len`)

---

## 8. Как добавить новый атрибут с поддержкой `_children` и state

Достаточно написать `"foobar_children"` / `"foobar_hover_children"` / `"foobar_click_children"` в JSON — больше ничего не нужно. `inherit_children()` подхватит любой ключ с суффиксом `_children`, а `resolve_state_attr()` отработает любой state-постфикс.

Для border-суб-атрибутов дополнительно: если атрибут читается через `get_border()`, то `ctx.get_border()` сам подтянет все inherited-значения.
