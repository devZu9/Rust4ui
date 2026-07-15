# Rust4ui

> 🌐 [English version](README_EN.md)

> **Rust for UI** — собирай UI на Rust как конструктор: JSON → живой интерфейс на egui. Без перекомпиляции между правками. С возможностью «запечь» в Rust-код для релиза.

> 🛠 Создано в рамках освоения vibecoding в оболочке [OpenCode](https://opencode.ai/go?ref=DHSKBMGTK0)
> ☕ Если проект вам полезен — [поддержите автора](https://dalink.to/miska_moloka)

---

## Быстрый старт

```bash
# 1. Создать проект
cargo init my_app && cd my_app

# 2. Подключить Rust4ui
cargo add rust4ui --git https://github.com/devZu9/Rust4ui
cargo add eframe

# 3. Описать UI (ui.json)
$ @"
{ \"type\": \"Label\", \"text\": \"Привет, Rust4ui!\", \"size\": 24, \"bold\": true }
"@ | Set-Content ui.json -Encoding UTF8

# 4. Заменить main.rs (см. ниже)

# 5. Запустить
cargo run
```

**main.rs:**

```rust
use rust4ui::prelude::*;

fn main() -> eframe::Result<()> {
    let ui = UiNode::from_file("ui.json");
    let theme = Theme::from_file("theme.json").unwrap_or_default();
    let locales = LocaleRegistry::new("ru");
    let state = State::new();
    let actions = Actions::new();

    eframe::run_native("My App", Default::default(),
        Box::new(|_| Box::new(Rust4ui::new(ui, theme, locales, state, actions))))
}
```

Готово. Меняй `ui.json` — UI меняется. Не надо `cargo build` между правками.

---

## Что внутри

### Виджеты

`Column` `Row` `Label` `Button` `TextField` `Checkbox` `RadioGroup` `Slider` `ComboBox` `Separator` `FileDrop` `Spinner` `Shortcut` `ColorPicker` `Indicator` `IconBar` `IconButton` `Caption` `StatusBar` `Tabs` `Tab` `Panel` `ScrollArea` `Window` `Hyperlink` `Image` `Grid` `MenuBar` `Menu` `MenuItem` `SubMenu` `Notifications` `Tray` `DataTable`

### Фичи

| Фича | Описание |
|------|----------|
| **JSON → UI** | Пишешь `ui.json` — видишь окно. Без компиляции. |
| **i18n** | Многоязычность с нуля: `{{key}}`, CLDR plural, runtime-переключение |
| **Темы** | `theme.json` → цвета, отступы, скругления. Hover/Focus/Disabled из коробки |
| **Гибкость** | Любой виджет можно кастомизировать: `opacity`, `rounding`, `padding`, `margin`, `shadow`, `stroke` |
| **Расширяемость** | Через `WidgetRegistry` подключаются кастомные виджеты из любых крейтов |
| **$ref** | Большие UI разбиваются на модули → каждый файл < 200 строк |
| **Иконки** | Phosphor font встроен + Texture Registry для PNG |
| **Системный трей** | Иконка в трее, контекстное меню, анимация (опционально) |

---

## Как это выглядит

```json
{
  "type": "Column",
  "gap": 10,
  "padding": 16,
  "children": [
    {
      "type": "Label",
      "text": "{{app.title}}",
      "size": 20,
      "bold": true
    },
    {
      "type": "TextField",
      "binding": "name",
      "hint": "{{hint.name}}",
      "width": 300
    },
    {
      "type": "Button",
      "text": "{{btn.save}}",
      "action": "save",
      "fill": "#3366CC",
      "rounding": 6
    }
  ]
}
```

---

## Документация

| Файл | О чём |
|------|-------|
| **[TECHNICAL_SPECIFICATION.md](TECHNICAL_SPECIFICATION.md)** | Полная техническая спецификация: формат JSON, theme.json, state, actions, i18n, тесты |
| **[ROADMAP.md](ROADMAP.md)** | План развития (v0.1 → v1.0) |
| **[CHANGELOG.md](CHANGELOG.md)** | История изменений по версиям |
| **[SESSIONS.md](SESSIONS.md)** | Логи сессий разработки |
| **[AGENTS.md](AGENTS.md)** | Инструкции для ИИ-ассистентов |
| **[SUMMARY.md](SUMMARY.md)** | Краткое описание проекта и фич |
| **`docs/src/` (mdBook)** | Полная документация: введение, quickstart, архитектура, ui.json, theme.json, иконки, actions, state, locale, border, hover/click, тени, hot-reload, примеры |

---

## Демо

В папке `demo/` — прототип со всеми виджетами на 5 вкладках + 3 окна:

```
demo/ui.json → tabs/*.json → windows/*.json
```

- `ui.json` — 11 строк, 4 `$ref` на модули
- Каждая вкладка в отдельном файле (< 250 строк)
- 80+ locale-ключей (ru/en)
- 3 темы (dark, light, dj_green)

---

## Поддержка

Этот проект создан под собственные нужды в рамках изучения vibecoding на платформе
[OpenCode](https://opencode.ai/go?ref=DHSKBMGTK0). Публикую в открытом доступе —
пользуйтесь, форкайте, дорабатывайте.

К сожалению, в данный момент я нахожусь в крайне сложной жизненной ситуации,
и любая поддержка для меня жизненно необходима. Если проект оказался для вас
полезным — пожалуйста поддержите донатом или воспользуйтесь предложением по ссылке.
Это очень помогает не только продолжать разработку, но и время от времени
прерывать чувство голода.

- ☕ **Donation** — поддержать автора: https://dalink.to/miska_moloka
- 🚀 **OpenCode** — платформа для vibecoding: https://opencode.ai/go?ref=DHSKBMGTK0

Спасибо, что вы здесь. 🙏

---

## Благодарности

При проектировании словаря виджетов автор вдохновлялся сторонним проектом [EFx](https://github.com/ZhukMax/efx).

---

## Лицензия

MIT.
