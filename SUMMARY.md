<!-- v1.0.1 (2026-08-05) SUMMARY.md -->
# Rust4ui — Summary

**Rust4ui** (Rust for UI) — комбайн для быстрого прототипирования и сборки UI на Rust/egui.

## Состояние

- **Активная сессия:** нет (последняя закрытая — №6 (04.08) «Внедрение версионности (project-versioning)», закрыта 05.08 02:55; следующая — №7 при открытии)
- **Текущая версия:** v0.5.1 (VERSION — единый источник правды)
- **Последняя закрытая сессия:** №6 (04.08) — версионность файлов, VERSION, унификация команд, INBOX-тест
- **Насущные задачи (ROADMAP):** бордюр (стороны), MenuBar (иконки/_focus/авто-раскрытие/shadow/width/icon_gap/align_children), Slider, ComboBox, Tabs, ПКМ, Image, ProgressBar, Table, ScrollBar, ScrollArea, Custom Frame, шаблоны, IconBar

## Правила чтения журналов (экономия контекста)

- **SESSIONS.md** — читать только заголовки сессий + тело текущей (единственной) сессии (Select-String → Read с offset). В файле только активная сессия (или последняя закрытая, если новая не открыта); история — в `logs_archive/sessions/` (по запросу).
- **ROADMAP.md** — только насущные `[ ]` текущей версии (v0.5.1). `[x]` в ROADMAP нет — выполненные в `logs_archive/roadmap/`.
- **ROADMAP-IDEAS.md** — ТОЛЬКО по явному запросу пользователя.
- **CHANGELOG.md** — НЕ читать (для пользователя).
- **logs_archive/** — по запросу: `rg "<текст>" logs_archive/` или имена файлов (даты + заголовок в имени).

## Как работает

UI собирается как конструктор из трёх слоёв:

| Слой | Формат | Описание |
|------|--------|----------|
| `ui.json` | JSON | Структура и виджеты (Column, Row, Button, Tabs, ...) |
| `theme.json` | JSON | Цвета, отступы, скругления, обводки |
| actions | Rust-код | Логика: обработчики кликов, связь с данными |

## Два режима рендеринга

1. **Runtime** — JSON → живой UI, без `cargo build` на каждый чих
2. **Codegen** — JSON → raw Rust-код через `rust4ui-codegen`

## Ключевые фичи

- **Border-система** — solid/dash/dot, gap, seg_len, `border_position`, `border_seg_cap`, shorthand-массивы `[width, color, type, gap, seg_len]`, `border_hover`/`border_click`, поддержка тем
- **JSON-комментарии** — `//` и `/* */` во всех загрузчиках
- **valign для TextField** — top / center / bottom
- **Multiline fixed** — фиксированная высота с прокруткой (`fixed: true`/`false`)
- **Дизайн-словарь** — имена виджетов вдохновлены сторонним проектом [EFx](https://github.com/ZhukMax/efx), но технической зависимости нет
- **i18n с первого дня** — `{{key}}` из `locales/*.json`, плюрализация (CLDR), runtime-переключение языка
- **StateRegistry** — привязка данных к UI (`binding` в JSON → переменная в Rust)
- **ActionRegistry** — функции по имени из JSON (`action` → Rust-коллбэк)
- **widget_paint_custom** — единый слой отрисовки для custom-paint виджетов: alloc, фон, обводка, тени, padding/margin, state-атрибуты. Принимает `ctx: &RenderCtx`. **widget_paint_egui** — то же + child_ui для egui-виджетов.
- **`get_attr_ctx`** — универсальная функция чтения атрибута с state (hover/click/focus) + _parent theme fallback. Принимает `Option<&egui::Response>` — `None` для базовых атрибутов, `Some(&resp)` для state-зависимых. Заменяет `resolve_state_attr`.
- **Separator не наследует `_children`** — разделитель всегда рисуется с пустым inherited, не подхватывает padding/margin/цвет от родителя.
- **Универсальное наследование `_children`** — любой атрибут с суффиксом `_children` автоматом наследуется на 1 уровень вниз. `background_children`, `icon_position_hover_children`, `border_focus_children` — все без per-виджетного кода. Поддержка `_children` из `theme.json` как глобальных defaults. Документация: `docs/src/15-menu-children.md`.
- **popup_* атрибуты Menu** — раздельная настройка кнопки на MenuBar и контекстного меню (попап): `popup_background`, `popup_border`, `popup_padding`, `popup_gap`, `popup_min_width`, `popup_max_height`, `popup_shadow`. Все через `_children` наследование.
- **Padding border-box** — `N`, `[N]`, `[V,H]`, `[T,R,B,L]` — раздвигает элемент, текст внутри
- **Hot-reload** — все файлы в `demo/` (theme.json + tabs + windows) через file watcher
- **text_align** — left/center/right для TextField и Button
- **Settings persistence** — `demo/settings.json` с автосохранением размера/позиции окна, вкладки, языка
- **Обработка ошибок** — никогда не паникует, все ошибки видны в UI (оранжевые/красные заглушки)

## Текущие ограничения

- Row `align` — зарезервирован, всегда прижат к верху
- `margin` — внешний отступ со всех сторон через `get_margin()` (число, [V,H], [T,R,B,L])
- Live-reload — только директория `demo/`, не весь проект

## Виджеты

Column, Row, Label, Button, TextField (text/password/number/multiline), Checkbox, RadioGroup, Slider, ComboBox, Separator, Spacer, FileDrop, Spinner, Shortcut, ColorPicker, Indicator, IconBar, IconButton, Caption, StatusBar, Tabs/Tab, Panel, ScrollArea, Window, Hyperlink, Image, Grid, MenuBar, Menu, MenuItem, SubMenu, Notifications, Tray, DataTable.

## Документация

| Файл | О чём |
|------|-------|
| `ROADMAP.md` | План развития (насущные задачи текущей версии) |
| `ROADMAP-IDEAS.md` | Идеи и отложенное («долгий ящик», по запросу) |
| `CHANGELOG.md` | История изменений по версиям (для пользователя) |
| `SESSIONS.md` | Логи сессий с описанием попыток и решений (3 последние) |
| `logs_archive/` | История журналов: закрытые сессии и версии ROADMAP |
| `AGENTS.md` | Инструкции для ИИ-ассистентов |
| `TECH_SPECIFICATION.md` | Полная техническая спецификация |

## Лицензия

MIT.
