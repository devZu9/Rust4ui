<!-- v1.0.3 (2026-08-05) CHANGELOG.md -->
# CHANGELOG

> Формат: сухие итоги по версиям. Подробности (проблема → решение → вывод) — в `SESSIONS.md`, ссылки на сессии указаны. О разведении ролей — канон `logs-md-files`.

## v0.5.1 (2026-07-20 00:14)

- Добавлено: Внедрена версионность файлов (`project-versioning`): все 110 текстовых git-файлов получили заголовки `v1.0.0` по формату скилла; создан `VERSION` (0.5.1) — единый источник правды; `Cargo.toml` выровнен 0.5.0 → 0.5.1 (журналы говорили 0.5.1).
- Изменено: Унификация команд opencode: `.opencode/command/` проекта — только языковые команды (review, test); универсальные команды — глобальные линки `~/.config/opencode/command/` (канон `_for_OpenCode` — единый источник, двухсторонний sync).

- Изменено: `get_attr_ctx` объединяет `resolve_state_attr` — единая функция с state + theme fallback.
- Изменено: Сигнатуры `widget_paint_custom` / `widget_paint_egui` — принимают `ctx: &RenderCtx`; параметр `widget: &str` удалён.
- Изменено: Separator: не наследует `_children` — `std::mem::take(ctx.inherited)` перед рендером.
- Изменено: Измерение детей после `inherit_children` — замер и рендер используют один inherited.
- Изменено: Нейминг без сокращений — menu.rs, renderer.rs (`icon_w→icon_width`, `cw→child_text_width`, и т.д.).
- Изменено: `ctx.state` borrow conflict — slider.rs, checkbox.rs, color_picker.rs: клонирование state перед вызовом.

- Удалено: `resolve_state_attr` — заменён на `get_attr_ctx`.
- Удалено: `empty_inherited` из separator.rs — зануление через `std::mem::take`.
- Удалено: `HashMap` импорт из base.rs — не используется.

- Исправлено: Popup width зависел от количества Separator — Separator не наследует padding.
- Исправлено: `get_attr_ctx` требовал `resp` — `Option<&egui::Response>` → `None` для базовых атрибутов.
- Исправлено: `theme_lookup` closure получал `&Value` вместо `&str` — добавлен `None` third param.

## v0.5.0 (2026-07-18 17:36)

- Изменено: `widget_base` → `widget_paint_custom`, `widget_base_wrap` → `widget_paint_egui`, `BaseOut` → `PaintOut` — прозрачный нейминг
- Изменено: `allocate_exact_size` → `reserve_exact_size` — обёртка с понятным названием.
- Изменено: Убраны `default_bg`, `default_rounding`, `default_pad` — читаются внутри из `theme`/`inherited`; сигнатура 12 → 8 параметров.

- Исправлено: `unused_imports` / `unused_variables` (кроме `dead_code`) в base.rs, renderer.rs.

## v0.4.6 (2026-07-18 13:30)

- Исправлено: Конфликт `padding_children` и `padding` на MenuItem — удалён лишний `resolve_state_attr("padding")` из `widget_paint_custom`; padding вычисляется один раз в `menu_item.rs`.

- Изменено: base.rs — удалён неиспользуемый импорт `parse_padding`.
- Изменено: Удалены временные debug-логи — `widget_paint_custom` логировал каждый виджет каждый кадр; оставлен только MenuItem при клике.

## v0.4.5 (2026-07-18 03:13)

- Добавлено: `popup_*` атрибуты — контекстное меню настраивается отдельно от кнопки Menu (background, rounding, padding, gap, min_width, max_height, border, shadow) через `_children`
- Добавлено: MenuItem: `stretch` / `align` — растяжение и выравнивание контента.
- Добавлено: Separator: динамическая ширина — `available_width()` вместо хардкода 200px (дефолт min_width 50).
- Добавлено: Попап: измерение детей — ширина по самому широкому MenuItem.
- Добавлено: `inherit_children`: theme fallback — `*_children` из `theme.json` как глобальные defaults.
- Добавлено: `always_on_top` — в `settings.json`, runtime-тогл через `ViewportCommand::WindowLevel`.

- Изменено: popup rendering — Area + Frame + `allocate_exact_size` + `allocate_ui_at_rect`; ширина фиксирована явно.
- Изменено: menu.rs — измерение детей ДО `inherit_children`.

- Исправлено: popup open/close logic — три проблемы (свой клик, ховер-переключение, чужой ключ `open_popup_id`).
- Исправлено: stretch + Separator «лесенка» — явная фиксация ширины через `allocate_exact_size`.
- Исправлено: popup_padding — читал `"padding"` вместо `"popup_padding"`.

## v0.4.4 (2026-07-17 20:27)

- Добавлено: Универсальный механизм `_hover/_click/_focus` — `resolve_state_attr()` в renderer.rs
- Добавлено: Универсальный механизм `_children` — `inherit_children()` / `restore_children()` в RenderCtx.
- Добавлено: `RenderCtx.inherited: HashMap<String, Value>` — единое хранилище вместо 14 отдельных полей.
- Добавлено: border-суб-атрибуты с `_children` — через `ctx.get_border()`.
- Добавлено: `icon_children` и state-позиции иконки — через `ctx.inherited_icon` и `resolve_state_attr`.
- Добавлено: `Default` impl для `BorderStyle`, `BorderType`, `BorderPosition`.

- Изменено: `inherit_children()` — `drain` → `clear` → только `_children` узла; уровни изолированы.
- Изменено: `restore_children()` — `clear` + снапшот; ключи не протекают.
- Изменено: `menu.rs` — порядок: layout → resolve_state_attr → inherit_children → popup → restore_children.
- Изменено: `menu_bar.rs` — `rounding_children` как массив `[nw,ne,sw,se]`; ручная обработка 30+ строк заменена.
- Изменено: `widget_paint_custom/egui` — принимают `&HashMap<String, Value>`.
- Изменено: `main.rs`, `examples/demo.rs` — под новую структуру RenderCtx.

- Исправлено: `border_position_children` не работал — через `ctx.get_border()`.
- Исправлено: `inherit_children` протекал глубже одного уровня — полный `clear()` + снапшот.
- Исправлено: Menu не видел `background_children` от MenuBar — порядок вызовов.
- Исправлено: `rounding_children` делал углы одинаковыми — массив вместо f64.

- Исправлено: label.rs, panel.rs, spinner.rs, checkbox.rs, hyperlink.rs, slider.rs — неиспользуемые импорты/параметры.

## v0.4.3 (2026-07-17 02:13)

- Добавлено: MenuBar: система `_children` — background/color/padding/margin/rounding + hover/click для детей
- Добавлено: MenuBar: state-aware фон; Menu: state-aware цвет текста через `fg_stroke`; Menu: margin top/bottom.
- Добавлено: RenderCtx — `inherited_*` поля для state-атрибутов.
- Добавлено: `border.rs: rounded_rect_perimeter` — публичная функция.

- Исправлено: border.rs: draw_pattern — замыкание периметра; левая сторона dash/dot при `rounding=0`.
- Исправлено: border.rs: point_at_dist — выход за границы (паника при `rounding=0`).

## v0.4.2 (2026-07-16 16:18)

- Добавлено: `widget_paint_custom` — единый слой отрисовки custom-paint виджетов (alloc, фон, обводка, тени, padding/margin, rounding, state)
- Добавлено: MenuItem — переведён на custom-paint; state-aware стили.
- Добавлено: MenuBar — каскад `background`/`color` → Menu → MenuItem через `weak_bg_fill`.

- Изменено: RenderCtx — `inherited_bg`, `inherited_color`; menu.rs — `weak_bg_fill` + `window_fill` для попапа.
- Изменено: Темы — `color_text` → `color` для Menu/MenuItem/MenuBar + секции с state-фонами.

- Удалено: `fg_stroke` — цвет текста через `RichText::color()`.

## v0.4.1 (2026-07-16 03:36)

- Добавлено: NumberField — алиас TextField с `mode: "number"`
- Добавлено: Stepper overlay — степпер поверх поля (не привязан к высоте текста).
- Добавлено: `stepper_padding` / `stepper_background` / `stepper_rounding` — атрибуты степпера.

- Изменено: Иконки `▲/▼` → Phosphor-глифы `caret-up`/`caret-down`.
- Изменено: Stepper button — `ui.interact()` + painter (без фона/тени по умолчанию).
- Изменено: TextEdit — вся ширина контента.

- Удалено: `stepper_bg` — заменён на `stepper_background`.

## v0.4.0 (2026-07-14 20:45)

- Добавлено: Button: каскад теней — `shadow_content` (шорткат), `shadow_icon`/`shadow_text` (переопределения)
- Добавлено: `parse_content_shadow()` — парсер с offset (1,1).
- Добавлено: `ShadowZOrder` enum (`Under`/`Over`), `draw_shadow_content()`, конструкторы `Shadow::from_rgba()/transparent()`.

- Изменено: `parse_shadow()` — строгий формат `[opacity, "under"/"over"?, "#color"?, x?, y?]`.
- Изменено: Default offset: background/border → (2,2); content/icon/text → (1,1).
- Изменено: Button: state-aware align/padding/margin; приоритет state — click > focus > hover > base.
- Изменено: TextField: focus state — `border_focus`, `background_focus`, убрана синяя рамка egui.
- Изменено: Settings persistence — размер/позиция окна, вкладка, язык; `demo/settings.json`; hot-reload игнорит.
- Изменено: Vars в theme.json — `$имя`, авторезолв, `substitute_vars()` в ref_resolver.rs (5 тестов).

## v0.3.1 (2026-07-12 01:12)

- Добавлено: Shadow система — `Shadow`, `parse_shadow()`, `draw_shadow_bg/border/icon`, state-aware
- Добавлено: border opacity — `[width, color, opacity, type, gap, seg_len]`, обратная совместимость.
- Добавлено: `color_icon`, `parse_color_value()` (`["#HEX", opacity]`), `get_state_background()`, `get_margin()`, `gap_row`.

- Изменено: `fill` → `background`; `hover_fill` → `background_hover` и т.д.
- Изменено: `Sense::click()` → `click_and_drag()` — без таймаута удержания.
- Изменено: `get_state_border()` — чистый click без `hovered`.
- Изменено: `theme.json` — удалены псевдо-виджеты Hover/Focus/Disabled.
- Изменено: suffix naming — `text_color`→`color_text`, `hover_color`→`color_hover`; `parse_hex_color` поддерживает #RGB/#RGBA.
- Изменено: `galley()` → `galley_with_override_text_color()` — цвет иконки при hover/click.
- Изменено: ZhukMax → devZu9 — git config, Cargo.toml, docs, история переписана.

- Исправлено: Button: удержание клика; border_hover/click на всех виджетах; высота IconButton от `icon_size`; дефолтный padding `symmetric(0,0)`.

## v0.3.0 (2026-07-10 01:07)

- Добавлено: IconRegistry — 1512 иконок Phosphor, вкомпилирован в бинарь
- Добавлено: icon_size, Hover/Click-стейты, тени на Button, галерея иконок, документация mdBook (14 глав).

- Изменено: Официальный Phosphor TTF; icons.json перегенерирован; `icons/phosphor-icons/` в `.gitignore`.

- Исправлено: Ключ локали `tab.icons` в en.json; AGENTS.md — правило «Локали-ключи — во все файлы».

## v0.2.1 (2026-07-08 23:52)

- Исправлено: Multiline TextField с фиксированной высотой (fixed=true) — решение: `allocate_exact_size` + фон до ScrollArea + `allocate_ui_at_rect` + TextEdit `frame(false)` + кастомная рамка + ручной фокус (подробно, 6 шагов — SESSIONS, сессия 06.07).

## v0.2.0 (2026-07-08 22:54)

- Добавлено: Border-система — `border.rs` (solid/dash/dot, gap, seg_len, border_seg_cap, border_position, shorthand-массивы).
- Добавлено: JSON-комментарии — `//` и `/* */` во всех загрузчиках.
- Добавлено: valign для TextField — top/center/bottom; вкладка «Меню и иконки»; `background` из `bg_fill`; `border_position` inside/center/outside.

- Исправлено: `find_index` — индекс вставки вместо предыдущей точки (seg_max=324.6).
- Исправлено: Phosphor-шрифт — `push` вместо `insert(0)`.
- Исправлено: TextField rounding/override; multiline — единый код с singleline; dash/dot скругления; равномерное распределение dash (`floor()`).

- Изменено: `bg_fill` → `background`; `stroke_width`/`stroke_color` → `border`/`border_width`/`border_color`; Phosphor не основной шрифт.

## v0.1.0 (2026-07-06 06:14)

- Исправлено: Button stair-step — кастомный рендер `rect_filled()` + galley; Row Align::TOP; TextField padding/rounding/height; Button padding; hot-reload на `demo/`; `[N]`-парсинг margin; Spacer.

- Изменено: `parse_margin` → `parse_padding`; `padding_h/v` удалены (единый формат); высота Button динамическая; рендер TextField 70 → 1 строка.

- Добавлено: `text_align` для TextField; контур 1px `#444455`; hover-подсветка.


