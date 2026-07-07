# Changelog

## [Unreleased]

### Исправлено

- **Button: устранён stair-step эффект** — переписан рендер с `egui::Button` на кастомный `painter().rect_filled()` + `galley`. Sub-UI `allocate_ui_with_layout` создавал смещение базовой линии, из-за которого кнопки «ступались» ступеньками лесенкой.
- **Row: убран лишний вертикальный отступ у первого элемента** — `Align::Center` в `Layout::left_to_right(valign)` заставлял egui центрировать детей по вертикали внутри доступной высоты, из-за чего `min_rect()` включал сдвиг. Родитель (Column) резервировал лишнее место. Исправлено: всегда `Align::TOP`, параметр `align` у Row зарезервирован.
- **TextField: padding не раздвигал поле** — `Frame::inner_margin(pad)` работал как content-box, padding расширял поле наружу, а не отодвигал текст внутрь. Переписано на `TextEdit::margin(pad).frame(true)`, где egui сам рисует фон, контур и hover.
- **TextField: `rounding` не применялся** — переопределял `widgets.noninteractive.corner_radius`, а TextEdit использует `ui.style().interact(&response)`, который возвращает `widgets.inactive`/`hovered`/`active`. Исправлено: переопределяются все три состояния.
- **TextField: `height` не влиял на поле** — формула `max(line_h + padding, height)` игнорировала `height`, потому что `line_h + padding` всегда больше. Исправлено на `field_h = max(height, font_h + padding)`: height — полная высота поля, padding внутри, поле растёт только если не хватает места для шрифта.
- **Button: padding не раздвигал кнопку** — жёсткий фиксированный размер `(min_width, height)`, padding только ужимал текст внутри. Исправлено: размер вычисляется из `text + padding`, height — минимальная высота.
- **Hot-reload не работал для табов/окон** — watcher следил только за `demo/theme.json`. Исправлено: watcher на всю `demo/` директорию рекурсивно, добавлен метод `reload_ui_tree()`.
- **`[N]` не парсился** — `parse_padding` (бывший `parse_margin`) не поддерживал одноэлементный массив. Добавлен match arm `1 => Margin::same(n)`.
- **margin работал только как `add_space(N)`** — не поддерживал форматы `[N]`, `[V,H]`, `[T,R,B,L]`. Переписан через `parse_padding`.
- **Spacer widget** — добавлен виджет для заполнения свободного места в Row/Column.

### Изменено

- **`parse_margin` → `parse_padding`** — функция переименована, добавлен `pub use self::parse_padding as parse_margin` для обратной совместимости.
- **`padding_h`/`padding_v` удалены** — весь код и конфиги переведены на единый формат `padding: N | [N] | [V,H] | [T,R,B,L]`.
- **Button height — динамическая** — `height` в теме трактуется как минимальная, а не точная высота. Button растёт если текст + padding больше.
- **TextField рендер** — упрощён с 70 строк кастомного `painter` + `UiBuilder` до 1 строки `TextEdit::margin().frame(true).background_color(bg)`.
- **`Ui::child_ui` → `UiBuilder::new()`** — вместо deprecated `child_ui(rect, layout, None)` используется `ui.new_child(UiBuilder::new().max_rect(content).layout(layout))`.
- **Тема: bg_fill больше не теряется** — при загрузке `theme.json` секции виджетов перезаписывали дефолтную тему, удаляя не указанные в JSON поля. Код теперь явно указывает дефолты.

### Добавлено

- **`text_align` для TextField** — `"left"` (дефолт), `"center"`, `"right"`. Читается через `w_str2(node, "TextField", "text_align")`.
- **Контур TextField** — 1px `#444455` вокруг поля для визуального разделения.
- **Hover-подсветка TextField** — при наведении поле подсвечивается через `bg.linear_multiply(1.2)`.
- **CHANGELOG.md** — этот файл.
