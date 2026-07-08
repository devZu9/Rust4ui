# Changelog

<details open>
<summary><strong>[0.2.1] — multiline fixed</strong></summary>

### Исправлено
- **Multiline TextField с фиксированной высотой (fixed=true)** — долгая проблема, на решение которой ушло почти два дня многочисленных итераций. Поле multiline расширялось вниз при добавлении строк, не имея возможности зафиксировать высоту и включить прокрутку. Каждая попытка внедрить ScrollArea ломала визуал: фон и рамка «ехали» отдельно от текста, hover/focus переставали работать, ширина поля растягивалась на всё окно.

  **Как решили:**
  1. `allocate_exact_size(rect)` — резервирует ровно `field_w × field_h`, родитель не даёт больше
  2. `rect_filled(rect)` — фон рисуется ДО ScrollArea, строго внутри rect
  3. `allocate_ui_at_rect(rect, |ui| ScrollArea::vertical().max_height(field_h).show(...))` — ScrollArea привязан к тому же rect, не может вылезти
  4. TextEdit внутри с `frame(false).desired_width(field_w)` — без своей рамки, ширина фиксирована
  5. `draw_border(rect)` — кастомная рамка по внешнему rect, фон и бордюр едины
  6. Фокус — синяя рамка рисуется вручную через `inner_resp.has_focus()`

  Ключевое отличие от неудачных попыток: ScrollArea обёрнут в `allocate_ui_at_rect`, а не напрямую вызван после `allocate_exact_size`, что исключает разрыв между фоном и областью прокрутки.

</details>

<details>
<summary><strong>[0.2.0] — border-система, JSON-комментарии, valign</strong></summary>

### Добавлено
- **Border-система** — единый модуль `border.rs` с solid/dash/dot, gap, seg_len, border_seg_cap, border_position, shorthand-массивы, theme-поддержка
- **JSON-комментарии** — поддержка `//` и `/* */` во всех загрузчиках (`node.rs`, `main.rs`, `locale.rs`, `state.rs`, `ref_resolver.rs`, `tests/`)
- **valign для TextField** — top / center (дефолт) / bottom, читается из узла и темы
- **Вкладка «Меню и иконки»** — демо MenuBar, IconBar, IconButton в `demo/tabs/menus.json`
- **`background`** — переименован из `bg_fill` во всех файлах
- **`border_position`** — inside / center / outside (дефолт inside)

### Исправлено
- **`find_index`** — возвращала индекс вставки вместо индекса предыдущей точки → seg_max=324.6. Исправлено: `i.saturating_sub(1)`
- **Phosphor-шрифт** — перенесён из `insert(0)` в `push` (конец стека), латиница и пробел отображаются корректно
- **TextField rounding** — восстановлен override `style_mut().visuals.widgets.{inactive,hovered,active}.corner_radius` для singleline и multiline
- **Multiline TextField** — единый код с singleline, hover/focus работают
- **Dash/dot rounded corners** — `draw_pattern` использует `rounded_rect_perimeter` с 24 шагами на дугу
- **Равномерное распределение dash** — `floor()` вместо `round()` для `n`, фиксированный шаг

### Изменено
- **`bg_fill` → `background`** — во всех темах, конфигах и коде
- **`stroke_width` / `stroke_color` → `border` / `border_width` / `border_color`** — удалены полностью
- **Phosphor-шрифт** — отключён как основной (конец стека), только для PUA-иконок

</details>

<details>
<summary><strong>[0.1.0] — MVP</strong></summary>

### Исправлено
- **Button: устранён stair-step эффект** — переписан рендер с `egui::Button` на кастомный `painter().rect_filled()` + `galley`.
- **Row: убран лишний вертикальный отступ у первого элемента** — всегда `Align::TOP`.
- **TextField: padding не раздвигал поле** — `TextEdit::margin(pad).frame(true)`.
- **TextField: `rounding` не применялся** — переопределяются все три состояния.
- **TextField: `height` не влиял на поле** — формула `field_h = max(height, font_h + padding)`.
- **Button: padding не раздвигал кнопку** — размер вычисляется из `text + padding`.
- **Hot-reload не работал для табов/окон** — watcher на всю `demo/` директорию.
- **`[N]` не парсился** — добавлен match arm `1 => Margin::same(n)`.
- **margin работал только как `add_space(N)`** — переписан через `parse_padding`.
- **Spacer widget** — добавлен виджет для заполнения свободного места.

### Изменено
- **`parse_margin` → `parse_padding`** — переименовано.
- **`padding_h`/`padding_v` удалены** — единый формат `padding: N | [N] | [V,H] | [T,R,B,L]`.
- **Button height — динамическая** — height трактуется как минимальная.
- **TextField рендер** — упрощён с 70 строк до 1 строки.
- **Тема: bg_fill больше не теряется** — явные дефолты.

### Добавлено
- **`text_align` для TextField** — left / center / right.
- **Контур TextField** — 1px `#444455`.
- **Hover-подсветка TextField** — `bg.linear_multiply(1.2)`.
- **CHANGELOG.md** — этот файл.

</details>
