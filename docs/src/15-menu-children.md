# MenuBar — наследование атрибутов через `_children`

Система `_children` позволяет родителю (MenuBar) передавать атрибуты детям (Menu, MenuItem) **на один уровень вглубь**. Без `_children` дети имеют собственные атрибуты из темы или JSON-узла.

---

## 1. Принцип

```
MenuBar
  ├── background          → только сама полоса
  ├── background_children → → Menu (1 уровень)
  ├── padding_children    → → Menu (1 уровень)
  └── color_children      → → Menu (1 уровень)
        └── (Menu НЕ передаёт дальше)
              └── MenuItem — не получает, если Menu не указал _children
```

**Правила:**
- `_children` — **ровно один уровень** вниз
- Без `_children` — дети **не наследуют** атрибуты родителя
- Если ребёнок указал свой атрибут — он переопределяет наследованный
- Если не указал — применяется наследованный от родителя (через `_children`)

---

## 2. Все атрибуты `_children`

| Атрибут | Что задаёт | Формат |
|---------|-----------|--------|
| `background_children` | Фон детей | `"#HEX"` или `["#HEX", opacity]` |
| `background_hover_children` | Фон детей при наведении | `"#HEX"` или `["#HEX", opacity]` |
| `background_click_children` | Фон детей при клике | `"#HEX"` или `["#HEX", opacity]` |
| `color_children` | Цвет текста детей | `"#HEX"` |
| `color_hover_children` | Цвет текста детей при наведении | `"#HEX"` |
| `color_click_children` | Цвет текста детей при клике | `"#HEX"` |
| `padding_children` | Внутренний отступ детей | `N`, `[V,H]`, `[T,R,B,L]` |
| `margin_children` | Внешний отступ вокруг детей | `N`, `[V,H]`, `[T,R,B,L]` |
| `rounding_children` | Скругление углов детей | `N` — первый и последний получают внешние углы |

---

## 3. Пример

```json
{
  "type": "MenuBar",
  "background": ["#87F", 1],
  "background_children": ["#87F", 1],
  "background_hover_children": ["#77F", 1],
  "background_click_children": ["#66F", 1],
  "color_children": "#022",
  "color_hover_children": "#FFF",
  "color_click_children": "#FFF",
  "padding_children": [15, 25],
  "margin_children": [2],
  "rounding_children": 15,
  "gap": 0,
  "border": [2, "#87F", 1, "dot", 4, 1],
  "children": [
    {
      "type": "Menu",
      "text": "{{menu.file}}",
      "children": [...]
    },
    {
      "type": "Menu",
      "text": "{{menu.edit}}",
      "background": ["#000", 1],
      "children": [...]
    }
  ]
}
```

Что происходит:
- Все Menu наследуют `background` от MenuBar → `#87F`
- При наведении → `#77F`, при клике → `#66F`
- Цвет текста `#022`, при hover/click → `#FFF`
- Первый и последний Menu имеют скруглённые внешние углы (15px)
- Второй Menu (правка) переопределил `background` на `#000` → не наследует

---

## 4. Собственные атрибуты MenuBar

```json
{
  "type": "MenuBar",
  "background": ["$like", 1],
  "background_hover": ["#87F", 1],
  "padding": 5,
  "margin": [10, 15],
  "rounding": 15,
  "gap": 0,
  "border": [2, "#FF0", 1, "dot", 4, 1],
  "border_position": "inside"
}
```

| Атрибут | Назначение | Формат |
|---------|-----------|--------|
| `background` | Фон полосы | `#HEX` или `["#HEX", opacity]` |
| `background_hover` | Фон полосы при наведении | `#HEX` или `["#HEX", opacity]` |
| `background_click` | Фон полосы при клике | `#HEX` или `["#HEX", opacity]` |
| `padding` | Внутренний отступ полосы | `N`, `[V,H]`, `[T,R,B,L]` |
| `margin` | Внешний отступ полосы | `N`, `[V,H]`, `[T,R,B,L]` |
| `rounding` | Скругление углов полосы | число |
| `gap` | Расстояние между пунктами | число |
| `border` | Обводка (solid/dash/dot) | массив |
| `border_position` | Положение обводки | `inside`, `center`, `outside` |

---

## 5. Приоритет разрешения атрибутов

Для любого атрибута на любом уровне:

```
свой узел → ctx.inherited (от _children родителя) → тема → дефолт
```

Пример для `background` на Menu:

```json
{
  "type": "Menu",
  "background": ["#000", 1]  // свой → берётся этот
}

// Без "background":
// → ctx.inherited_bg (от родительского background_children)
// → ctx.theme.w_color_opt("Menu", "background")
// → Color32::from_rgb(0x2A, 0x2A, 0x33)
```

---

## 6. Особенности rounding_children

`rounding_children` распределяется автоматически:

| Позиция | Скругление |
|---------|-----------|
| Первый ребёнок | `nw`=value, `sw`=value (левые углы) |
| Последний ребёнок | `ne`=value, `se`=value (правые углы) |
| Средние дети | Все 0 (прямоугольные) |

Это гарантирует, что крайние пункты меню скруглены только с внешней стороны, не вылезая за границы MenuBar.
