# Тени

## Обзор

Тени поддерживаются на `Button` и `IconButton`:

- **shadow_background** — тень фона (под заливкой)
- **shadow_border** — тень рамки (под или над рамкой)
- **shadow_content** — шорткат тени для иконки+текста — **Button**
- **shadow_icon** — переопределяет `shadow_content` для иконки — **Button**
- **shadow_text** — переопределяет `shadow_content` для текста — **Button**
- **shadow_icon** — тень иконки — **IconButton**

Подробное описание форматов и примеры см. в разделе [Border → Shadow](10-border.md).

## Формат массива

```
[opacity, "under"|"over"?, "#color"?, x?, y?]
```

| Позиция | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| 0 | number | — | Непрозрачность 0.0–1.0 |
| 1 | string | `"under"` | Z-order: `"under"` / `"over"` |
| 2 | string | `"#000"` | Цвет в HEX |
| 3 | number | 2.0 | Смещение по X |
| 4 | number | 2.0 | Смещение по Y |

Порядок строгий: opacity → z-order → цвет → x → y. Если на позиции 1 не строка — ошибка.

## Z-order

- `"under"` (по умолчанию) — тень под элементом (классическая тень)
- `"over"` — тень поверх элемента (glow-эффект)

Работает для `shadow_border`, `shadow_content`, `shadow_icon`, `shadow_text`. `shadow_background` всегда под фоном. `shadow_icon` (IconButton) — всегда `"under"`.

### Default offset

| Атрибут | Default offset | Где используется |
|---------|---------------|-----------------|
| `shadow_background` | (2, 2) | Button, IconButton |
| `shadow_border` | (2, 2) | Button, IconButton |
| `shadow_content` | (1, 1) | Button (шорткат) |
| `shadow_icon` | (1, 1) | Button, IconButton |
| `shadow_text` | (1, 1) | Button |

## Атрибуты (устаревшие)

Атрибуты `shadow_offset_x`, `shadow_offset_y`, `shadow_blur`, `shadow_color` считаются устаревшими. Используйте единый формат массива.

## Примечания

- Если alpha = 0, тень не рисуется
- Тень не обрезается по границам родителя — может выходить за рамки
- `shadow_border` повторяет dash/dot/gap/seg_len границы
