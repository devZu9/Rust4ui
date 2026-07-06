# Rust4ui — Summary

**Rust4ui** (Rust for UI) — комбайн для быстрого прототипирования и сборки UI на Rust/egui.

## Как работает

UI собирается как конструктор из трёх слоёв:

| Слой | Формат | Описание |
|------|--------|----------|
| `ui.json` | JSON | Структура и виджеты (Column, Row, Button, Tabs, ...) |
| `theme.json` | JSON | Цвета, отступы, скругления |
| actions | Rust-код | Логика: обработчики кликов, связь с данными |

## Три режима рендеринга

1. **Runtime** — JSON → живой UI, без `cargo build` на каждый чих
2. **Codegen** — JSON → `efx!()` Rust-код через `rust4ui-codegen`
3. **from_efx()** — EFx-шаблон строкой в рантайме

## Ключевые фичи

- **Совместимость с EFx** — единый словарь тегов, любой UI можно и в JSON, и в `efx!()`
- **i18n с первого дня** — `{{key}}` из `locales/*.json`, плюрализация (CLDR), runtime-переключение языка
- **StateRegistry** — привязка данных к UI (`binding` в JSON → переменная в Rust)
- **ActionRegistry** — функции по имени из JSON (`action` → Rust-коллбэк)
- **Обработка ошибок** — никогда не паникует, все ошибки видны в UI (оранжевые/красные заглушки)

## Виджеты

Column, Row, Label, Button, TextField, Checkbox, Slider, ComboBox, Separator, Tabs/Tab, Panel, ScrollArea, Window, Hyperlink, Image, Grid, DataTable.

## Лицензия

MIT.
