# Тени

## Обзор

Тени поддерживаются только на `Button`. Рисуются как прямоугольник, сдвинутый относительно кнопки, с заданным цветом и прозрачностью.

## Атрибуты

```json
{
  "type": "Button",
  "text": "С тенью",
  "shadow_offset_x": 3,
  "shadow_offset_y": 3,
  "shadow_blur": 6,
  "shadow_color": "rgba(0,0,0,80)"
}
```

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `shadow_offset_x` | number | 2.0 | Смещение тени по X (вправо) |
| `shadow_offset_y` | number | 2.0 | Смещение тени по Y (вниз) |
| `shadow_blur` | number | 4.0 | Радиус размытия тени |
| `shadow_color` | string | rgba(0,0,0,40) | Цвет тени в HEX или rgba |

## Цвет тени

Цвет указывается как HEX-строка с альфа-каналом:

```json
"shadow_color": "#00000040"     // 6 + 2 hex = RGBA
"shadow_color": "rgba(0,0,0,64)" // десятичный rgba (пока не поддерживается напрямую — используйте 8-символьный HEX)
```

Рекомендуется использовать 8-символьный HEX: `#00000040` = чёрный с альфа 0.25 (64/255).

## Как это работает

```rust
let shadow = crate::border::get_shadow(node, &ctx.theme, "Button");
// (offset_x, offset_y, blur, color)

crate::border::draw_shadow(ui, rect, rounding_cr, &shadow);
// Рисует заполненный прямоугольник со скруглением,
// сдвинутый на (offset_x, offset_y) относительно кнопки
```

## Примеры

```json
{
  "type": "Button",
  "text": "Приподнятая",
  "shadow_offset_x": 2,
  "shadow_offset_y": 4,
  "shadow_blur": 8,
  "shadow_color": "#00000060"
}

{
  "type": "Button",
  "text": "Акцент справа",
  "shadow_offset_x": 5,
  "shadow_offset_y": 0,
  "shadow_blur": 3,
  "shadow_color": "#3366CC40"
}
```

## Примечания

- Тень рисуется **перед** кнопкой (в слое под ней)
- Если `shadow_color` имеет alpha = 0, тень не рисуется
- Параметр `shadow_blur` пока не используется для фактического размытия (рисуется сплошная тень)
- Тень не обрезается по границам родителя — может выходить за рамки
- В будущем планируется поддержка настоящего box-shadow с размытием
