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

## 5. Особенности `rounding_children`

`rounding_children` распределяется автоматически на **первый** и **последний** дочерний элемент (работает только в `menu_bar.rs`):

| Позиция | Скругление |
|---------|-----------|
| Первый ребёнок | `nw`=value, `sw`=value (левые углы) |
| Последний ребёнок | `ne`=value, `se`=value (правые углы) |
| Средние дети | Все 0 (прямоугольные) |

Хранится как массив `[nw, ne, sw, se]`, читается per-corner.

---

## 6. Приоритет разрешения атрибутов (полная цепочка)

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

## 7. Как добавить новый атрибут с поддержкой `_children` и state

Достаточно написать `"foobar_children"` / `"foobar_hover_children"` / `"foobar_click_children"` в JSON — больше ничего не нужно. `inherit_children()` подхватит любой ключ с суффиксом `_children`, а `resolve_state_attr()` отработает любой state-постфикс.

Для border-суб-атрибутов дополнительно: если атрибут читается через `get_border()`, то `ctx.get_border()` сам подтянет все inherited-значения.
