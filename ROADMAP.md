# Roadmap — Rust4ui

> JSON → UI → Rust. Быстрое прототипирование интерфейсов на egui.

---

## Легенда

| 🟢 Реализовано | 🟡 В разработке | 🟠 Запланировано | ⚪ Будет рассмотрено |
|----------------|-----------------|------------------|----------------------|

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

- [ ] **Table**
  - [ ] `<DataTable>` на базе egui_extras::TableBuilder
  - [ ] Сортировка колонок
- [ ] **Валидация JSON при старте**
  - [ ] Проверка, что все binding есть в state
  - [ ] Проверка, что все action зарегистрированы
  - [ ] Визуальный отчёт об ошибках
- [ ] **10 встроенных тем**
- [ ] **System Tray** (фича `tray`)
  - [ ] Иконка в трее + контекстное меню
  - [ ] Динамическая смена иконки + анимация
  - [ ] Windows (Shell), macOS (NSStatusBar), Linux (libappindicator)
- [ ] Бенчмарки производительности

- [x] **Live-reload** 🟢 *(07.07.2026)*
  - [x] File watcher (notify crate) на всю `demo/` директорию
  - [x] Авто-перезагрузка UI-дерева + темы

</details>

<details>
<summary><strong>v0.3 — Продвинутые виджеты</strong></summary>

- [ ] `Image` — загрузка и отображение
- [ ] `ProgressBar` — индикатор прогресса
- [ ] **Темы**
  - [ ] `themes/nord.json`
  - [ ] `themes/gruvbox.json`
- [ ] **Локали**
  - [ ] `locales/fr.json` — французский
  - [ ] `locales/pl.json` — польский (с plural-правилами)
  - [ ] `locales/uk.json` — украинский (с plural-правилами)

- [x] **Виджеты** 🟢 *(06.07.2026)*
  - [x] `Hyperlink`
  - [x] `Grid`
  - [x] `Window`
- [x] **Layout** 🟢 *(07.07.2026)*
  - [x] `Padding` / `Margin` на любом контейнере
  - [x] `align` для Column (left/center/right)

</details>

<details open>
<summary><strong>v0.2 — Расширение возможностей (текущая)</strong></summary>

- [ ] **Code Generator** (feature `gen`)
  - [ ] Парсер JSON → raw egui Rust-код
  - [ ] CLI-утилита `rust4ui-codegen`
- [ ] **WidgetRegistry** — регистрация кастомных виджетов извне
  - [ ] `WidgetRegistry` + `register_widgets!` макрос
- [ ] **Локали**
  - [ ] `locales/de.json` — немецкий
- [ ] `examples/simple` — минимальный: 3 виджета, 1 action, 1 binding

- [x] **Виджеты** 🟢 *(06.07.2026)*
  - [x] `ComboBox`
  - [x] `Slider`
  - [x] `ScrollArea`
  - [x] `Tabs` / `Tab`
  - [x] `Panel`
  - [x] `Window`
- [x] **Контейнеры** 🟢 *(06.07.2026)*
  - [x] `Row` с `wrap: true`
- [x] **Меню и навигация** 🟢 *(06.07.2026)*
  - [x] Контекстное меню
  - [x] MenuBar + Menu
  - [x] Toast & Notifications
- [x] **Темы** 🟢 *(06.07.2026)*
  - [x] `themes/dj_green.json`
- [x] **Border-система** 🟢 *(07.07.2026)*
  - [x] `border.rs` — solid/dash/dot, gap, seg_len, shorthand
  - [x] `border_seg_cap`, `border_position`
- [x] **JSON-комментарии** 🟢 *(07.07.2026)*
- [x] **Multiline fixed** 🟢 *(09.07.2026)*
  - [x] `allocate_ui_at_rect(rect, ScrollArea.show(...))` — ScrollArea внутри rect
  - [x] Фон и бордюр едины, hover/focus работают

</details>

<details>
<summary><strong>v0.1 — MVP (Ядро)</strong> *(06.07.2026)*</summary>

- [x] **Ядро рендерера**
  - [x] Базовый тип `UiNode` + парсинг JSON
  - [x] Система `$ref` — модульность JSON
  - [x] Theme (загрузка + merge дефолтов + приоритет)
  - [x] StateRegistry
  - [x] ActionRegistry
  - [x] LocaleRegistry
  - [x] CLDR plural-правила (ru, en, de, fr, pl, uk, be, ja, zh, ko)
  - [x] Обработка ошибок
- [x] **Контейнеры**
  - [x] `Column`
  - [x] `Row`
- [x] **Виджеты**
  - [x] `Label`, `Button`, `TextField`, `Checkbox`, `RadioGroup`, `Separator`
  - [x] `FileDrop`, `Spinner`, `Shortcut`, `ColorPicker`
  - [x] Rich Tooltip
  - [x] Система иконок — Phosphor font
  - [x] `icons/` — phosphor.ttf, icons.json
  - [x] `IconBar`, `IconButton`, `Caption`, `Indicator`, `StatusBar`
  - [x] Универсальные визуальные атрибуты
  - [x] Тема Hover/Focus/Disabled
- [x] **Встроенные темы**
  - [x] `themes/dark.json`
  - [x] `themes/light.json`
- [x] **Встроенные локали**
  - [x] `locales/ru.json`
  - [x] `locales/en.json`
- [x] **Демо-прототип** (`demo/`)
  - [x] `demo/ui.json`, `demo/tabs/` (5 вкладок), `demo/windows/` (3 окна)
  - [x] `demo/theme.json`, `demo/themes/light.json`, `demo/themes/dj_green.json`
- [x] **Тестирование**
  - [x] Pre-flight валидатор JSON
  - [x] Unit-тесты (68), интеграционный, persistence, encoding, smoke
- [x] **OpenCode-скиллы**
- [x] Документация README.md + ТЗ

</details>

---

## Технический долг / Backlog

- [ ] Вложенные binding (`"settings.audio.mic_name"`)
- [ ] Анимации (через egui `lerp`)
- [ ] Accessibility (ARIA-подобные атрибуты)
- [ ] macOS/Linux порт (через egui — уже кроссплатформенно)
