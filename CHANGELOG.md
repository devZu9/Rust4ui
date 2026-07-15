# Changelog

## [0.4.1] — 2026-07-15

### Добавлено
- **NumberField** — алиас `"type": "NumberField"` для TextField с `mode: "number"`. Работает без указания `mode`.
- **Stepper overlay** — степпер вынесен из layout'а в overlay поверх поля. Не привязан к высоте текста.
- **`stepper_padding`** — атрибут, расширяющий каждую кнопку степпера равномерно (ширина + высота). Формула: `btn_dim = icon_size + 2 * pad`.
- **`stepper_background`** — атрибут, цвет/альфа фона кнопок степпера. Поддерживает `"#HEX"` и `["#HEX", opacity]`.
- **`stepper_rounding`** — атрибут скругления углов кнопок степпера.

### Изменено
- **Числовое поле (mode=number)**: иконки `▲/▼` заменены на Phosphor-глифы `caret-up`/`caret-down` (через IconRegistry).
- **Stepper button**: egui::Button заменён на `ui.interact()` + painter. Нет фона/тени/обводки по умолчанию.
- **TextEdit в number-поле**: занимает всю ширину контента (без резервации 20px под степпер).

### Удалено
- **`stepper_bg`** из темы — больше не читается (заменён на `stepper_background`).

## [0.4.0] — 2026-07-14

### Добавлено
- **Button: каскад теней** — `shadow_content` как шорткат, `shadow_icon` и `shadow_text` как переопределения. Приоритет: `shadow_icon ?? shadow_content`, `shadow_text ?? shadow_content`.
- **`parse_content_shadow()`** — парсер с default offset (1,1) для shadow_content/shadow_icon/shadow_text.
- **`ShadowZOrder` enum** — `Under` / `Over` в `border.rs`.
- **`draw_shadow_content()`** — утилита отрисовки тени для galley.
- **`Shadow::from_rgba()` / `Shadow::transparent()`** — удобные конструкторы.

### Изменено
- **`parse_shadow()`** — строгий формат: `[opacity, "under"/"over"?, "#color"?, x?, y?]`. Позиция 1 — z-order, позиция 2 — цвет. Старые форматы не поддерживаются.
- **Default offset**: `shadow_background`/`shadow_border` → (2,2); `shadow_content`/`shadow_icon`/`shadow_text` → (1,1).
- **IconButton**: `shadow_icon` парсится через `parse_content_shadow` (offset 1,1).
- **Shadow struct** — добавлено поле `z_order: ShadowZOrder`.
- **Button**: align_hover/click, padding_hover/click, margin_hover/click — state-aware через get_state_attr.
- **Приоритет state** — `click > focus > hover > base` в `get_state_border` и `get_state_attr`.
- **TextField: focus state** — `border_focus`, `background_focus`. Убрана синяя рамка egui (active.bg_stroke = NONE, frame(false) для multiline).
- **Settings persistence** — `StateRegistry::save()/load()`. Сохранение размера/позиции окна, активной вкладки (`active_tab`), языка (`active_locale`). Файл `demo/settings.json` читается при старте, пишется только при изменении. Hot-reload игнорирует settings.json.
- **Vars в theme.json** — секция `vars` с переменными вида `$имя`. Авторезолв внутри vars и во всех атрибутах темы + UI. `substitute_vars()` в `ref_resolver.rs`. 5 unit-тестов.

## [0.3.1] — 2026-07-11

### Добавлено
- **Shadow система** — `Shadow` struct, `parse_shadow()`, `draw_shadow_bg/border/icon`, state-aware через `get_state_attr`
- **border opacity** — `[width, color, opacity, type, gap, seg_len]`, обратная совместимость (если третий — строка, старый формат)
- **`color_icon`** — отдельный цвет иконки на Button (рендер иконки и текста разделён)
- **`parse_color_value()`** — цвет + opacity как `["#HEX", opacity]` (0.0–1.0)
- **`get_state_background()`** — универсальная функция выбора фона по hover/click/focus
- **`border_hover` / `border_click`** — state-зависимые границы на всех виджетах (IconButton, Button, TextField, Column, Row, Label, Checkbox, RadioGroup, Tabs)
- **`get_margin()`** — универсальная утилита чтения margin (атрибут → тема → 0)
- **`gap_row`** — вертикальный отступ между wrapped-строками в Row

### Изменено
- **`fill` → `background`** — переименован во всех виджетах, темах, UI-файлах и док-ции (`hover_fill` → `background_hover`, `click_fill` → `background_click`)
- **`Sense::click()` → `Sense::click_and_drag()`** — Button и IconButton (убрано таймаут удержания ~1-2 сек)
- **`get_state_border()`** — условие: `is_pointer_button_down_on` без `hovered` (зажатая кнопка — всегда click, даже если курсор ушёл)
- **`theme.json`** — удалены секции Hover/Focus/Disabled (псевдо-виджеты, никем не читались)
- **`border` → `get_state_border()`** — единая функция выбора hover/click/base border
- **`widget_border`** — добавлены параметры `resp` и `enabled` (state-зависимые границы на 6 виджетах)
- **`text_color` → `color_text`** — suffix naming (`color_text_hover`, `color_text_click`)
- **`hover_color`/`click_color` → `color_hover`/`color_click`** — suffix naming
- **`parse_hex_color`** — добавлена поддержка `#RGB` / `#RGBA`
- **Button** — добавлен `margin` (по аналогии с IconButton)
- **Row** — `item_spacing = ZERO` (только явный gap), добавлен `gap_row`
- **`galley()` → `galley_with_override_text_color()`** — починен цвет иконки при hover/click
- **ZhukMax → devZu9** — git config, Cargo.toml, docs, история git переписана

### Исправлено
- **Button: удержание клика** — egui-таймаут ~1-2 сек на Sense::click, заменён на click_and_drag (держи сколько хочешь)
- **`border_hover` / `border_click` не работали** — теперь задействуют `get_state_border()` на всех виджетах
- **`hover_color`/`click_color` из темы** — не работали из-за `galley()` (заменял только placeholder-цвета вместо всех)
- **Высота IconButton** — `maket.size().y` → `icon_size` (убран line-height бонус от шрифта)
- **Дефолтный padding IconButton** — `symmetric(16, 4)` → `symmetric(0, 0)`

## [0.3.0] — 2026-07-09
- **IconRegistry** — 1512 иконок Phosphor, `resolve()` / `resolve_glyph()`, вкомпилирован в бинарь
- **icon_size** — атрибут для IconButton, size для MenuItem (+ тема)
- **Hover/Click-стейты** — `hover_fill`, `click_fill`, `hover_text_color`, `click_text_color` на Button и IconButton
- **Тени** — `shadow_offset_x/y`, `shadow_blur`, `shadow_color` на Button
- **Галерея иконок** — отдельная вкладка со всеми 1512 иконками (через Label, без тормозов)
- **Документация mdBook** — 14 глав, `___docs.bat`, поиск, навигация

### Изменено
- `icons/phosphor.ttf` заменён на официальный из коллекции Phosphor Icons (2024)
- `icons/icons.json` перегенерирован — все 1512 Codepoint'ов соответствуют TTF
- `icons/phosphor-icons/` добавлен в `.gitignore`
- IconButton — упрощён рендер, hover/click-фон рисуется поверх кнопки

### Исправлено
- Ключ локали `tab.icons` отсутствовал в `en.json`
- `AGENTS.md` — добавлено правило «Локали-ключи — во все файлы»

## [0.2.1]

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

## [0.2.0]

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

## [0.1.0]

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
