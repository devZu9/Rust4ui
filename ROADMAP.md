# Roadmap — Rust4ui

> JSON → UI → Rust. Быстрое прототипирование интерфейсов на egui.

---

## Легенда

| Метка | Статус |
|-------|--------|
| 🟢 Ready | Реализовано |
| 🟡 In Progress | В разработке |
| 🟠 Planned | Запланировано |
| ⚪ Backlog | Будет рассмотрено |

---

<details open>
<summary><strong>v0.5 — Инструменты</strong></summary>

- [ ] **GUI-редактор (ранняя версия)**
  - [ ] Просмотр ui.json с живым превью
  - [ ] Drag & drop из палитры на холст
  - [ ] Inspector свойств выбранного элемента
- [ ] **Экспорт**
  - [ ] JSON → standalone .rs файл с raw egui
- [ ] **Плагины (Rust-крейты)**
  - [ ] Регистрация кастомных тем

</details>

<details>
<summary><strong>v0.4 — Полировка</strong></summary>

- [x] **Live-reload** 🟢
  - [x] File watcher (notify crate) на всю `demo/` директорию 🟢
  - [x] Авто-перезагрузка UI-дерева + темы 🟢
- [ ] **Table**
  - [ ] `<DataTable>` на базе egui_extras::TableBuilder
  - [ ] Сортировка колонок
- [ ] **Валидация JSON при старте**
  - [ ] Проверка, что все binding есть в state
  - [ ] Проверка, что все action зарегистрированы
  - [ ] Визуальный отчёт об ошибках
- [ ] **10 встроенных тем**
- [ ] **System Tray** (фича `tray`)
  - [ ] Иконка в трее + контекстное меню (MenuItem, Checkbox, RadioGroup, SubMenu)
  - [ ] Динамическая смена иконки + анимация
  - [ ] Windows (Shell), macOS (NSStatusBar), Linux (libappindicator)
- [ ] Бенчмарки производительности

</details>

<details>
<summary><strong>v0.3 — Продвинутые виджеты</strong></summary>

- [x] **Виджеты** 🟢
  - [x] `Hyperlink` 🟢
  - [x] `Grid` 🟢
  - [x] `Window` 🟢
  - [ ] `Image` — загрузка и отображение
  - [ ] `ProgressBar` — индикатор прогресса
- [x] **Layout** 🟢
  - [x] `Padding` / `Margin` на любом контейнере 🟢
  - [x] `align` для Column (left/center/right) 🟢
- [ ] **Темы**
  - [ ] `themes/nord.json`
  - [ ] `themes/gruvbox.json`
- [ ] **Локали**
  - [ ] `locales/fr.json` — французский
  - [ ] `locales/pl.json` — польский (с plural-правилами)
  - [ ] `locales/uk.json` — украинский (с plural-правилами)

</details>

<details open>
<summary><strong>v0.2 — Расширение возможностей (текущая)</strong></summary>

- [x] **Виджеты** 🟢
  - [x] `ComboBox` 🟢
  - [x] `Slider` 🟢
  - [x] `ScrollArea` 🟢
  - [x] `Tabs` / `Tab` 🟢
  - [x] `Panel` 🟢
  - [x] `Window` 🟢
- [x] **Контейнеры** 🟢
  - [x] `Row` с `wrap: true` 🟢
- [x] **Border-система** 🟢
  - [x] `border.rs` — solid/dash/dot, gap, seg_len, shorthand 🟢
  - [x] `border_seg_cap`, `border_position` 🟢
- [x] **Меню и навигация** 🟢
  - [x] Контекстное меню 🟢
  - [x] MenuBar + Menu 🟢
  - [x] Toast & Notifications 🟢
- [x] **Темы** 🟢
  - [x] `themes/dj_green.json` 🟢
- [x] **JSON-комментарии** 🟢
- [x] **Multiline fixed** 🟢
- [ ] **Code Generator** (feature `gen`)
  - [ ] Парсер JSON → raw egui Rust-код
  - [ ] CLI-утилита `rust4ui-codegen`
- [ ] **WidgetRegistry** — регистрация кастомных виджетов извне
  - [ ] `WidgetRegistry` + `register_widgets!` макрос
- [ ] **Локали**
  - [ ] `locales/de.json` — немецкий

</details>

<details>
<summary><strong>v0.1 — MVP (Ядро)</strong></summary>

- [x] **Ядро рендерера** 🟢
  - [x] Базовый тип `UiNode` + парсинг JSON 🟢
  - [x] Система `$ref` — модульность JSON 🟢
  - [x] Theme (загрузка + merge дефолтов + приоритет) 🟢
  - [x] StateRegistry 🟢
  - [x] ActionRegistry 🟢
  - [x] LocaleRegistry 🟢
  - [x] CLDR plural-правила (ru, en, de, fr, pl, uk, be, ja, zh, ko) 🟢
  - [x] Обработка ошибок 🟢
- [x] **Контейнеры** 🟢
  - [x] `Column` 🟢
  - [x] `Row` 🟢
- [x] **Виджеты** 🟢
  - [x] `Label` 🟢
  - [x] `Button` 🟢
  - [x] `TextField` 🟢
  - [x] `Checkbox` 🟢
  - [x] `RadioGroup` 🟢
  - [x] `Separator` 🟢
  - [x] `FileDrop` 🟢
  - [x] `Spinner` 🟢
  - [x] `Shortcut` 🟢
  - [x] `ColorPicker` 🟢
  - [x] Rich Tooltip 🟢
  - [x] Система иконок — Phosphor font 🟢
  - [x] `icons/` — phosphor.ttf, icons.json 🟢
  - [x] `IconBar` 🟢
  - [x] `IconButton` 🟢
  - [x] `Caption` 🟢
  - [x] `Indicator` 🟢
  - [x] `StatusBar` 🟢
  - [x] Универсальные визуальные атрибуты 🟢
  - [x] Тема Hover/Focus/Disabled 🟢
- [x] **Встроенные темы** 🟢
  - [x] `themes/dark.json`
  - [x] `themes/light.json`
- [x] **Встроенные локали** 🟢
  - [x] `locales/ru.json`
  - [x] `locales/en.json`
- [ ] **Пример**
  - [ ] `examples/simple` — минимальный: 3 виджета, 1 action, 1 binding
- [x] **Демо-прототип** (`demo/`) 🟢
  - [x] `demo/ui.json` — корневой файл 🟢
  - [x] `demo/tabs/` — 5 вкладок 🟢
  - [x] `demo/windows/` — 3 окна 🟢
  - [x] `demo/theme.json` 🟢
  - [x] `demo/themes/light.json` 🟢
  - [x] `demo/themes/dj_green.json` 🟢
  - [x] `locales/ru.json` 🟢
  - [x] `locales/en.json` 🟢
- [x] **Тестирование** 🟢
  - [x] Pre-flight валидатор JSON 🟢
  - [x] Unit-тесты подсистем (68 тестов) 🟢
  - [x] Интеграционный тест 🟢
  - [x] Persistence-тесты 🟢
  - [x] Encoding-тесты 🟢
  - [x] Smoke-тесты (каждый виджет) 🟢
- [x] **OpenCode-скиллы** 🟢
- [x] Документация README.md + ТЗ 🟢

</details>

---

## Технический долг / Backlog

- [ ] Вложенные binding (`"settings.audio.mic_name"`)
- [ ] Анимации (через egui `lerp`)
- [ ] Accessibility (ARIA-подобные атрибуты)
- [ ] macOS/Linux порт (через egui — уже кроссплатформенно)
