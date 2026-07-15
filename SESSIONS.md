# Sessions — логи сессий

> **Правила ведения:**
> 1. `## Сессия ДД.ММ — название`
> 2. `- 2026-ММ-ДД (ЧЧ:ММ) - начата`
> 3. `- 2026-ММ-ДД (ЧЧ:ММ) - завершена` — только после согласования с пользователем
> 4. `---` — разделитель между датами и списком пунктов
> 5. `[ ]` — не сделано, вверху списка
> 6. `[x]` — сделано, внизу списка; новые сверху, старые снизу

## Сессия 14.07 — v0.4 Shadow Z-order + Button shadow
- 2026-07-14 (20:43) - начата

---

- [ ] **MenuBar** — исправление ошибок (белые кнопки, {{синтаксис}}, hover, иконки при наведении)
- [ ] **IconBar anchor** — start/center/end
- [ ] **Separator в IconBar** — разделитель между иконками
- [ ] **SVG-текстуры** — SVGs/, SVGs Flat/ (отложено)
- [ ] **ScrollBar стилизация** — толщина, цвет, отступы
- [ ] **ScrollArea: отступ текста от рамки**
- [ ] **Slider, ComboBox, Tabs** — доделка дизайна
- [ ] **Числовое поле, Image, ProgressBar, DataTable** — новые виджеты
- [x] **Vars в theme.json** — переменные `$var`, авторезолв внутри vars и во всех секциях темы + UI. Работает с любыми JSON-типами
- [x] **Settings persistence** — save/load размера окна, позиции, вкладки, языка. debounce через сравнение дампа. Watcher игнорит settings.json
- [x] **Приоритет state** — click > focus > hover > base (get_state_border, get_state_attr)
- [x] **TextField: focus state** — border_focus, background_focus, убрана синяя рамка egui
- [x] **Каскад теней в Button** — shadow_content (шорткат), shadow_icon + shadow_text (переопределения)
- [x] **Button: state-aware** — align_hover/click, padding_hover/click, margin_hover/click
- [x] **IconButton** — shadow_icon через parse_content_shadow с offset (1,1)
- [x] **Button shadow** — shadow_bg/border/icon для обычной Button (как в IconButton)
- [x] **Shadow Z-order** — параметр положения тени (под/над элементом) для shadow_border, shadow_content

---

## Сессия 09.07 — v0.3 иконки и документация
- 2026-07-09 (10:35) - начата
- 2026-07-14 (20:43) - завершена

---

- [x] **Shadow система** — Shadow struct, parse_shadow, draw_shadow_bg/border/icon, state-aware
- [x] **border opacity** — `[width, color, opacity, type, gap, seg_len]`, обратная совместимость
- [x] **`color_icon`** — отдельный цвет иконки на Button, раздельный рендер icon+text
- [x] **suffix naming** — `hover_color`→`color_hover`, `text_color`→`color_text` и т.д.
- [x] **`parse_color_value`** — поддержка `["#HEX", opacity]` (цвет + непрозрачность отдельно)
- [x] **`fill` → `background`** — переименование + `get_state_background()` универсальная
- [x] **`Sense::click` → `click_and_drag`** — Button/IconButton без таймаута удержания
- [x] **`get_state_border`** — условие `is_pointer_button_down_on` без `hovered` (чистый click)
- [x] **`theme.json` — удалены Hover/Focus/Disabled** — псевдо-виджеты, никем не читались
- [x] **`border_hover` / `border_click`** — get_state_border(), widget_border + resp, на всех виджетах с border
- [x] **`galley` → `galley_with_override_text_color`** — hover_color/click_color теперь перекрашивают иконку
- [x] **`color`, `hover_color`, `click_color`** — переименовано, `parse_hex_color` поддерживает #RGB/#RGBA
- [x] **ZhukMax вычищен** — git config, Cargo.toml, docs, вся история переписана
- [x] **Универсальные марджины** — IconButton + Button, per-widget, без Frame (не ломает wrap), `get_margin()`
- [x] **Row: item_spacing = ZERO, gap_row** — только явный gap, вертикальный отступ между wrapped-строками
- [x] **Устранён дубликат `IconButton` в theme.json** (слияние + icon_color)
- [x] **`width` → `button_size`** (переименование, точный размер кнопки)
- [x] `icon_size` вынесен из хардкода 14.0 в атрибут + fallback через тему
- [x] Высота кнопки считается от `icon_size`, а не от `maket.size().y` (убран line-height бонус)
- [x] Дефолтный padding `symmetric(16, 4)` → `symmetric(0, 0)`
- [x] IconRegistry — парсинг icons.json, резолв имени → codepoint
- [x] IconRegistry вкомпилирован в бинарь (include_str!)
- [x] RenderCtx.icons доступен всем виджетам
- [x] IconButton — иконки отображаются (глифы, не текст)
- [x] Button + icon — глиф перед текстом
- [x] Label + icon — глиф перед текстом
- [x] MenuItem + icon — глиф перед текстом
- [x] Demo: иконки на MenuBar (новый/открыть/экспорт/отмена)
- [x] Demo: иконки на кнопках (primary/danger/success)
- [x] Demo: иконки на Apply/Reset/Greeting
- [x] 5 unit-тестов IconRegistry
- [x] Phosphor TTF заменён на официальный (Fonts/regular/Phosphor.ttf)
- [x] icons.json перегенерирован — 1512 реальных иконок Phosphor (38 KB)
- [x] icons/phosphor-icons/ добавлен в .gitignore
- [x] Demo — все иконки обновлены на реальные Phosphor-имена
- [x] IconRegistry — тест на все 1512, тест на 60+ common иконок
- [x] Отдельная вкладка «Иконки» со всеми 1512 иконками Phosphor (со скроллом)
- [x] text_field.rs: deprecated API обновлён (scope_builder, id_salt)
- [x] text_field.rs: убран лишний `mut`, `valign` → `_valign`
- [x] plural_key, render_context_menu, Notification — `#[allow(dead_code)]`

---

## Сессия 06.07 — multiline fixed
- 2026-07-06 (11:07) - начата
- 2026-07-09 (02:02) - завершена

---

- [x] ScrollArea + allocate_ui_at_rect работает
- [x] Фон и рамка едины
- [x] Hover/focus работают
