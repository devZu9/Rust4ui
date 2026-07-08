# Rust4ui — Summary

**Rust4ui** (Rust for UI) — комбайн для быстрого прототипирования и сборки UI на Rust/egui.

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

- **Border-система** — solid/dash/dot, gap, seg_len, `border_position`, `border_seg_cap`, shorthand-массивы `[width, color, type, gap, seg_len]`, поддержка тем
- **JSON-комментарии** — `//` и `/* */` во всех загрузчиках
- **valign для TextField** — top / center / bottom
- **Multiline fixed** — фиксированная высота с прокруткой (`fixed: true`/`false`)
- **Дизайн-словарь** — имена виджетов вдохновлены проектом [EFx](https://github.com/ZhukMax/efx), но технической зависимости нет
- **i18n с первого дня** — `{{key}}` из `locales/*.json`, плюрализация (CLDR), runtime-переключение языка
- **StateRegistry** — привязка данных к UI (`binding` в JSON → переменная в Rust)
- **ActionRegistry** — функции по имени из JSON (`action` → Rust-коллбэк)
- **Padding border-box** — `N`, `[N]`, `[V,H]`, `[T,R,B,L]` — раздвигает элемент, текст внутри
- **Hot-reload** — все файлы в `demo/` (theme.json + tabs + windows) через file watcher
- **text_align** — left/center/right для TextField и Button
- **Обработка ошибок** — никогда не паникует, все ошибки видны в UI (оранжевые/красные заглушки)

## Текущие ограничения

- Row `align` — зарезервирован, всегда прижат к верху
- `margin` — только верхний отступ через `add_space`
- Live-reload — только директория `demo/`, не весь проект

## Виджеты

Column, Row, Label, Button, TextField (text/password/number/multiline), Checkbox, RadioGroup, Slider, ComboBox, Separator, Spacer, FileDrop, Spinner, Shortcut, ColorPicker, Indicator, IconBar, IconButton, Caption, StatusBar, Tabs/Tab, Panel, ScrollArea, Window, Hyperlink, Image, Grid, MenuBar, Menu, MenuItem, SubMenu, Notifications, Tray, DataTable.

## Документация

| Файл | О чём |
|------|-------|
| `ROADMAP.md` | План развития (v0.1 → v0.2 → v0.*) |
| `CHANGELOG.md` | История изменений по версиям |
| `SESSIONS.md` | Логи сессий с описанием попыток и решений |
| `AGENTS.md` | Инструкции для ИИ-ассистентов |
| `TECHNICAL_SPECIFICATION.md` | Полная техническая спецификация |
| `.opencode/skills/session-log/SKILL.md` | Скилл ведения сессий |

## Лицензия

MIT.
