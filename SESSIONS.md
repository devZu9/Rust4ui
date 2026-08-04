<!-- v1.0.7 (2026-08-05) SESSIONS.md -->
# SESSIONS — журнал сессий

## Сессия 04.08 — Внедрение версионности (project-versioning)

- 2026-08-04 (19:18) - начата
- 2026-08-05 (02:55) - завершена

---

- 🟢 INBOX-тест (почта между сессиями): получена [инструкция] от канона, выведено сообщение-подтверждение в чат, ответное [опыт]-сообщение дописано в INBOX канона (тест замкнут); запись удалена из INBOX проекта по завершении реакции (05.08.2026 02:49)

- 🟢 Корень проекта очищен: автопилот-система перенесена в `_archive/AUTOPILOT.md` (git mv, история сохранена; идея жива — возрождение по запросу); `run_final.log` (лог запуска 06.07) и `autopilot_state.json` (все шаги done — артефакт завершённой сборки) удалены; AGENTS.md — правила автопилота заменены на пометку о паузе; архив коммитится в git (05.08.2026 01:44)

- 🟢 ROADMAP: тема анимаций сведена к одной записи в блоке `v*.* — Расширяем функционал`: «Система анимаций (вкл. микро-анимации иконок) — спецификация: `ANIMATIONS_SPEC.md`»; дубли удалены — из v0.5.1 (анимации — позже) и из бэклога («Анимации через egui lerp»); `ANIMATIONS_SPEC.md` добавлен в git с заголовком v1.0.0; ROADMAP.md v1.0.1→v1.0.2 (05.08.2026 01:33)

- 🟢 ROADMAP: версия v0.5 разделена — невыполненные пункты (15: бордюр, MenuBar, Slider, ComboBox, Tabs, ПКМ, Image, ProgressBar, Table, ScrollBar, ScrollArea, Custom Frame, шаблоны, IconBar, микро-анимации) перенесены в новую секцию `v0.5.1 *(текущая)*` (соответствует VERSION 0.5.1); в v0.5 остались только выполненные `[x]` — статус `*(завершена)*`; заголовок ROADMAP.md v1.0.0→v1.0.1 (05.08.2026 01:18)

- 🟢 CHANGELOG.md приведён к единому канон-стандарту (logs-md-files): убраны подзаголовки `### Изменено/Удалено/Исправлено/Добавлено` и «### Технический долг» — плоский список с префиксами `- Добавлено/Изменено/Исправлено/Удалено:`; секция «Инфраструктура» перенесена внутрь v0.5.1 (текущая версия — всё в ней, ничего сверху); заголовок v1.0.2→v1.0.3 (05.08.2026 00:51)

- 🟢 Унификация команд завершена: `.opencode/command/` проекта = только языковые команды (review, test) — hard links на канон; универсальные команды (10 шт) — глобальные линки `~/.config/opencode/command/` (install-global.ps1, двухсторонний sync с prune); канон `.opencode/command/` = мета-команды (learn/new/profile); `any_command/README.md` удалён (папка — только команды) (04.08.2026 23:29)
- 🟢 Канон-обновления (влияют на проект): `project-versioning` 1.2.0 — VERSION единый источник правды ВСЕГДА + обязательный чек-лист синхронизации (файл → VERSION → языковой файл → SESSIONS → CHANGELOG); команда `version` 1.1.0 (init создаёт VERSION); `new-session` 1.11.0 (версия проекта из VERSION); `apply-audit` 2.6.0 (в проект только языковые команды); `audit-project` 1.11.0 (версия из VERSION) (04.08.2026 23:06)
- 🟢 VERSION = 0.5.1 создан в корне проекта (единый источник правды); Cargo.toml исправлен 0.5.0 → 0.5.1 (журнал CHANGELOG говорил 0.5.1 — конфиг отставал, версия по журналам) (04.08.2026 23:06)
- 🟢 `/version init` — 110 текстовых git-файлов получили заголовок `v1.0.0 (2026-08-04)` по формату project-versioning (`.rs` → `//`, `.toml` → `#`, `.md` → `<!-- -->`, `.json` → `_version` первым полем, `.bat` → `rem`); 2 пропущено (бинарный phosphor.ttf, автосген. autopilot_state.json) (04.08.2026 23:06)
- 🟢 Тест project-versioning (пункт ROADMAP канона): шаги (1) `/version init` и (1а) создание VERSION — выполнены; шаги (2)–(4) (правка button.rs → PATCH-бамп + SESSIONS, заголовок, commit+push) — ожидают пользователя (04.08.2026)

---

## Сессия 22.07 — MenuBar доработки

- 2026-07-22 (время не указано) - начата
- 2026-08-04 (16:11) - завершена

---

## Сессия 16.07 — MenuBar: _children, state-атрибуты, border-fix, универсальный механизм _hover/_click/_focus + _children

- 2026-07-16 (18:00) - начата
- 2026-07-22 (время не указано) - завершена

---

- 🟢 `get_attr_ctx` объединяет `resolve_state_attr` — единая функция с state (hover/click/focus) + _parent theme fallback; `resolve_state_attr` удалён; `get_attr_ctx` принимает `Option<&egui::Response>` — `None` для базовых атрибутов, `Some(&resp)` для state-зависимых (20.07.2026)
- 🟢 `widget_paint_custom`/`widget_paint_egui` принимают `ctx: &RenderCtx` — вместо раздельных `(theme, inherited)`; сигнатура сокращена; `HashMap` импорт удалён из base.rs (20.07.2026)
- 🟢 Separator: не наследует `_children` — `std::mem::take(ctx.inherited)` перед рендером, восстановление после; Separator всегда рисуется с пустым inherited, не подхватывает padding/margin/цвет от parent (20.07.2026)
- 🟢 Измерение детей — после `inherit_children` — в menu.rs замер MenuItem перенесён после `inherit_children`, чтобы `get_padding` использовал тот же inherited, что и при рендере (20.07.2026)
- 🟢 Нейминг без сокращений — menu.rs, renderer.rs: все переменные переименованы в полные имена (icon_w→icon_width, cw→child_text_width, cp→child_padding, sfx→suffix, sk→state_key и т.д.) (20.07.2026)
- 🟢 `ctx.state` borrow conflict — slider.rs, checkbox.rs, color_picker.rs: клонирование state перед `widget_paint_egui`, запись обратно после (20.07.2026)
- 🟢 Нейминг-рефакторинг отрисовки — `widget_base` → `widget_paint_custom`, `widget_base_wrap` → `widget_paint_egui`, `BaseOut` → `PaintOut`; сигнатура сокращена с 12 до 8 параметров (убраны `default_bg`, `default_rounding`, `default_pad` — читаются внутри из `theme`); добавлена обёртка `reserve_exact_size` вместо `allocate_exact_size` (20.07.2026)
- 🟢 Конфликт padding_children и MenuItem padding — исправлен: `widget_paint_custom` перечитывал padding через `resolve_state_attr` с цепочкой node → inherited → theme → default; `padding_children: [10,40]` выигрывал у `MenuItem padding: [15,80]`; удалён `resolve_state_attr("padding")` из `widget_paint_custom`, padding вычисляется один раз в `menu_item.rs` через `get_padding` (20.07.2026)
- 🟢 Попап: измерение детей — ширина попапа считается по самому широкому MenuItem (text + icon + padding) до inherit_children; Separator и stretch используют `available_width()` от фиксированного alloc (20.07.2026)
- 🟢 MenuItem: `stretch` — растягивает MenuItem на всю ширину попапа; `stretch: true/false`, через node → inherited → theme (20.07.2026)
- 🟢 MenuItem: `align` — выравнивание контента `"left"/"center"/"right"`, через node → inherited → theme (20.07.2026)
- 🟢 MenuItem: `color_icon` — добавлен theme fallback (20.07.2026)
- 🟢 Separator: динамическая ширина — `available_width()` вместо хардкода 200px; дефолт min_width 50 (20.07.2026)
- 🟢 `popup_*` атрибуты — контекстное меню настраивается отдельно от кнопки Menu: `popup_background`, `popup_rounding`, `popup_padding`, `popup_gap`, `popup_min_width`, `popup_max_height`, `popup_border`, `popup_shadow`; все через `_children` наследование (20.07.2026)
- 🟢 popup open/close logic fix — три проблемы: (1) `clicked_elsewhere()` срабатывал на свой же клик → `!resp.clicked()` перед ним; (2) ховер-переключение не закрывало предыдущий попап → `ctx.state.set_bool(prev, false)` при ховере; (3) `open_popup_id` очищался для чужого ключа → проверка `ctx.open_popup_id.as_deref() == Some(&popup_key)` (20.07.2026)
- 🟢 `inherit_children`: theme fallback — `inherit_children(node, Some("MenuBar"))` читает `*_children` сначала из JSON-узла, потом из темы (entry.or_insert_with); `rounding_children`, `padding_children`, `popup_*_children` из `theme.json` работают (20.07.2026)
- 🟢 `always_on_top` — в `settings.json`; читается на старте → `ViewportBuilder.with_always_on_top()`; сохраняется; runtime-тогл через `ViewportCommand::WindowLevel(WindowLevel::AlwaysOnTop)` (20.07.2026)
- 🟢 `popup_padding` fix — читал ключ "padding" вместо "popup_padding" (через get_padding); исправлено: прямой `parse_padding(node.get("popup_padding"))` (20.07.2026)
- 🟢 RenderCtx: `inherited: HashMap<String, Value>` вместо 14 отдельных полей inherited_bg, inherited_bg_hover и т.д. — любой атрибут с `_children` суффиксом автоматом ложится в ctx.inherited (17.07.2026)
- 🟢 `inherit_children()` — drain всех текущих inherited → clear → apply только `_children` из узла; каждый уровень изолирован (17.07.2026)
- 🟢 `restore_children()` — clear + insert только из снапшота; предотвращает протекание ключей на уровень глубже (17.07.2026)
- 🟢 `resolve_state_attr()` — универсальная функция чтения атрибута с цепочкой: node[state] → inherited[state] → theme[state] → node → inherited → theme → default (17.07.2026)
- 🟢 `ctx.get_border()` — обогащает node из inherited для всех border-суб-атрибутов; border_position_children/width/color/type/gap/seg_len работают автоматом (17.07.2026)
- 🟢 `menu_bar.rs` — ручная обработка 30+ атрибутов заменена на inherit_children/restore_children (17.07.2026)
- 🟢 `menu_bar.rs` — rounding_children хранится как массив [nw,ne,sw,se], а не одно f64 (17.07.2026)
- 🟢 `menu.rs` — bg/bg_hover/bg_click/color/icon_position/icon_gap/border → resolve_state_attr (17.07.2026)
- 🟢 `menu.rs` — порядок: layout → resolve_state_attr → inherit_children → popup → restore_children; исправлено протекание и невидимость background_children (17.07.2026)
- 🟢 `menu_item.rs` — читает inherited через HashMap (17.07.2026)
- 🟢 `base.rs` — widget_paint_custom/widget_paint_egui принимают &HashMap вместо Option<Color32> (17.07.2026)
- 🟢 `border.rs` — Default для BorderStyle, BorderType, BorderPosition (17.07.2026)
- 🟢 Исправлено: `border_position_children` не работал — menu.rs захардкодил node.get("border_position"); исправлено через ctx.get_border() (17.07.2026)
- 🟢 Исправлено: `inherit_children` протекал глубже одного уровня — MenuBar's background_children доходил до Label/Button (17.07.2026)
- 🟢 Исправлено: Menu не видел `background_children` от MenuBar — inherit_children очищал HashMap до resolve_state_attr (17.07.2026)
- 🟢 Исправлено: `rounding_children` делал все 4 угла одинаковыми — хранилось f64, читалось как CornerRadius::same(); исправлено массивом [nw,ne,sw,se] (17.07.2026)
- 🟢 ROADMAP — добавлен пункт «Отключение сторон бордюра» в v0.5 (17.07.2026)
- 🟢 border: паника при rounding=0 — point_at_dist out of bounds (16.07.2026)
- 🟢 MenuBar: `_children` система — background/color/padding/margin/rounding + hover/click для детей (16.07.2026)
- 🟢 MenuBar: gap, padding, margin, rounding — все атрибуты корректно работают (16.07.2026)
- 🟢 MenuBar: border через draw_border — solid/dash/dot (16.07.2026)
- 🟢 Menu: state-aware фон и цвет — background_hover/click + color_hover/click через fg_stroke (16.07.2026)
- 🟢 Menu: margin top/bottom — вертикальные отступы (16.07.2026)
- 🟢 `widget_paint_custom` — единая функция отрисовки для custom-paint виджетов; Button -80 строк, IconButton -50 строк, MenuItem переведён на custom-paint (16.07.2026)
- 🟢 MenuItem: state-aware стили — background_hover/click/focus работают через widget_paint_custom (16.07.2026)
- 🟢 MenuBar: `{{syntax}}` резолвится — menu.rs подцепил resolve_text() (16.07.2026)
- 🟢 MenuBar: каскад наследования — MenuBar → Menu → MenuItem; background и color наследуются (16.07.2026)
- 🟢 MenuBar: weak_bg_fill — bg_fill → weak_bg_fill для всех состояний; попап наследует фон через window_fill (16.07.2026)
- 🟢 Числовое поле (mode=number) — дизайн, точность, степпер, тесты (15.07.2026)

---

## Сессия 14.07 — v0.4 Shadow Z-order + Button shadow

- 2026-07-14 (20:43) - начата
- 2026-07-15 (09:45) - завершена

---

- 🟢 Vars в theme.json — переменные `$var`, авторезолв внутри vars и во всех секциях темы + UI; работает с любыми JSON-типами (14.07.2026)
- 🟢 Settings persistence — save/load размера окна, позиции, вкладки, языка; debounce через сравнение дампа; watcher игнорит settings.json (14.07.2026)
- 🟢 Приоритет state — click > focus > hover > base (get_state_border, get_state_attr) (14.07.2026)
- 🟢 TextField: focus state — border_focus, background_focus, убрана синяя рамка egui (14.07.2026)
- 🟢 Каскад теней в Button — shadow_content (шорткат), shadow_icon + shadow_text (переопределения) (14.07.2026)
- 🟢 Button: state-aware — align_hover/click, padding_hover/click, margin_hover/click (14.07.2026)
- 🟢 IconButton — shadow_icon через parse_content_shadow с offset (1,1) (14.07.2026)
- 🟢 Button shadow — shadow_bg/border/icon для обычной Button (как в IconButton) (14.07.2026)
- 🟢 Shadow Z-order — параметр положения тени (под/над элементом) для shadow_border, shadow_content (14.07.2026)

---

## Сессия 09.07 — v0.3 иконки и документация

- 2026-07-09 (10:35) - начата
- 2026-07-14 (20:43) - завершена

---

- 🟢 Shadow система — Shadow struct, parse_shadow, draw_shadow_bg/border/icon, state-aware (09.07.2026)
- 🟢 border opacity — `[width, color, opacity, type, gap, seg_len]`, обратная совместимость (09.07.2026)
- 🟢 `color_icon` — отдельный цвет иконки на Button, раздельный рендер icon+text (09.07.2026)
- 🟢 suffix naming — `hover_color`→`color_hover`, `text_color`→`color_text` и т.д. (09.07.2026)
- 🟢 `parse_color_value` — поддержка `["#HEX", opacity]` (цвет + непрозрачность отдельно) (09.07.2026)
- 🟢 `fill` → `background` — переименование + `get_state_background()` универсальная (09.07.2026)
- 🟢 `Sense::click` → `click_and_drag` — Button/IconButton без таймаута удержания (09.07.2026)
- 🟢 `get_state_border` — условие `is_pointer_button_down_on` без `hovered` (чистый click) (09.07.2026)
- 🟢 `theme.json` — удалены Hover/Focus/Disabled — псевдо-виджеты, никем не читались (09.07.2026)
- 🟢 `border_hover` / `border_click` — get_state_border(), widget_border + resp, на всех виджетах с border (09.07.2026)
- 🟢 `galley` → `galley_with_override_text_color` — hover_color/click_color теперь перекрашивают иконку (09.07.2026)
- 🟢 `color`, `hover_color`, `click_color` — переименовано, `parse_hex_color` поддерживает #RGB/#RGBA (09.07.2026)
- 🟢 ZhukMax вычищен — git config, Cargo.toml, docs, вся история переписана (09.07.2026)
- 🟢 Универсальные марджины — IconButton + Button, per-widget, без Frame (не ломает wrap), `get_margin()` (09.07.2026)
- 🟢 Row: item_spacing = ZERO, gap_row — только явный gap, вертикальный отступ между wrapped-строками (09.07.2026)
- 🟢 Устранён дубликат `IconButton` в theme.json (слияние + icon_color) (09.07.2026)
- 🟢 `width` → `button_size` (переименование, точный размер кнопки) (09.07.2026)
- 🟢 `icon_size` вынесен из хардкода 14.0 в атрибут + fallback через тему (09.07.2026)
- 🟢 Высота кнопки считается от `icon_size`, а не от `maket.size().y` (убран line-height бонус) (09.07.2026)
- 🟢 Дефолтный padding `symmetric(16, 4)` → `symmetric(0, 0)` (09.07.2026)
- 🟢 IconRegistry — парсинг icons.json, резолв имени → codepoint (09.07.2026)
- 🟢 IconRegistry вкомпилирован в бинарь (include_str!) (09.07.2026)
- 🟢 RenderCtx.icons доступен всем виджетам (09.07.2026)
- 🟢 IconButton — иконки отображаются (глифы, не текст) (09.07.2026)
- 🟢 Button + icon — глиф перед текстом (09.07.2026)
- 🟢 Label + icon — глиф перед текстом (09.07.2026)
- 🟢 MenuItem + icon — глиф перед текстом (09.07.2026)
- 🟢 Demo: иконки на MenuBar (новый/открыть/экспорт/отмена) (09.07.2026)
- 🟢 Demo: иконки на кнопках (primary/danger/success) (09.07.2026)
- 🟢 Demo: иконки на Apply/Reset/Greeting (09.07.2026)
- 🟢 5 unit-тестов IconRegistry (09.07.2026)
- 🟢 Phosphor TTF заменён на официальный (Fonts/regular/Phosphor.ttf) (09.07.2026)
- 🟢 icons.json перегенерирован — 1512 реальных иконок Phosphor (38 KB) (09.07.2026)
- 🟢 icons/phosphor-icons/ добавлен в .gitignore (09.07.2026)
- 🟢 Demo — все иконки обновлены на реальные Phosphor-имена (09.07.2026)
- 🟢 IconRegistry — тест на все 1512, тест на 60+ common иконок (09.07.2026)
- 🟢 Отдельная вкладка «Иконки» со всеми 1512 иконками Phosphor (со скроллом) (09.07.2026)
- 🟢 text_field.rs: deprecated API обновлён (scope_builder, id_salt) (09.07.2026)
- 🟢 text_field.rs: убран лишний `mut`, `valign` → `_valign` (09.07.2026)
- 🟢 plural_key, render_context_menu, Notification — `#[allow(dead_code)]` (09.07.2026)

---

## Сессия 06.07 — multiline fixed

- 2026-07-06 (11:07) - начата
- 2026-07-09 (02:02) - завершена

---

- 🟡 Multiline TextField с фиксированной высотой (fixed=true) — долгая проблема (~2 дня итераций): поле расширялось вниз, нельзя было зафиксировать высоту и включить прокрутку; каждая попытка ScrollArea ломала визуал (фон и рамка «ехали» отдельно от текста, hover/focus ломались, ширина растягивалась). Как решили (6 шагов): (1) `allocate_exact_size(rect)` — резервирует ровно `field_w × field_h`, родитель не даёт больше; (2) `rect_filled(rect)` — фон рисуется ДО ScrollArea, строго внутри rect; (3) `allocate_ui_at_rect(rect, |ui| ScrollArea::vertical().max_height(field_h).show(...))` — ScrollArea привязан к тому же rect; (4) TextEdit внутри с `frame(false).desired_width(field_w)` — без своей рамки; (5) `draw_border(rect)` — кастомная рамка по внешнему rect, фон и бордюр едины; (6) фокус — синяя рамка вручную через `inner_resp.has_focus()`. Вывод: ключевое отличие от неудачных попыток — ScrollArea обёрнут в `allocate_ui_at_rect`, а не вызван напрямую после `allocate_exact_size`; это исключает разрыв между фоном и областью прокрутки (06.07.2026)
