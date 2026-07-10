# Roadmap — Rust4ui

> JSON → UI → Rust. Быстрое прототипирование интерфейсов на egui.

---

## Легенда

| Метка  | 🟢 Ready    | 🟡 In_Progress | 🟠 Planned    | ⚪ Backlog          |
| ------ | ----------- | -------------- | ------------- | ------------------ |
| Статус | Реализовано | В__разработке  | Запланировано | Будет__рассмотрено |

---

### v0.* — Инструменты и локали

- [ ] **GUI-редактор (ранняя версия)**
  - [ ] Просмотр ui.json с живым превью
  - [ ] Drag & drop из палитры на холст
  - [ ] Inspector свойств выбранного элемента
- [ ] **Экспорт**
  - [ ] JSON → standalone .rs файл с raw egui
- [ ] **Плагины (Rust-крейты)**
  - [ ] Регистрация кастомных тем
- [ ] **WidgetRegistry** — регистрация кастомных виджетов извне
  - [ ] `WidgetRegistry` + `register_widgets!` макрос
- [ ] **Локали**
  - [ ] `locales/de.json` — немецкий
  - [ ] `locales/fr.json` — французский
  - [ ] `locales/pl.json` — польский (с plural-правилами)
  - [ ] `locales/uk.json` — украинский (с plural-правилами)

### v0.* — Полировка

- [ ] **Валидация JSON при старте**  
  - [ ] Проверка, что все binding есть в state
  - [ ] Проверка, что все action зарегистрированы
  - [ ] Визуальный отчёт об ошибках
- [ ] **Margins для всех виджетов** — унифицировать
- [ ] **10 встроенных тем**
- [ ] Бенчмарки производительности
- [x] **Live-reload** 🟢 *(07.07.2026)*
  - [x] File watcher (notify crate) на всю `demo/` директорию
  - [x] Авто-перезагрузка UI-дерева + темы

### v0.* — Продвинутые виджеты

- [ ] **Темы**
 - [ ] `themes/nord.json`
 - [ ] `themes/gruvbox.json`
- [x] **Виджеты** 🟢 *(06.07.2026)*
  - [x] `Hyperlink`
  - [x] `Grid`
  - [x] `Window`
- [x] **Layout** 🟢 *(07.07.2026)*
  - [x] `Padding` / `Margin` на любом контейнере
  - [x] `align` для Column (left/center/right)

### v0.* - Расширяем функционал

- [ ] **Микро-анимации иконок** — последовательная смена иконок по таймеру
  - [ ] `animation` — массив имён иконок для анимации
  - [ ] `animation_interval_ms`, `animation_loop`
  - [ ] Примеры: spinner, battery-level, wifi-signal, temperature
- [ ] **System Tray** (фича `tray`)
  - [ ] Иконка в трее + контекстное меню
  - [ ] Динамическая смена иконки + анимация
  - [ ] Windows (Shell), macOS (NSStatusBar), Linux (libappindicator)
- [ ] **Code Generator** (feature `gen`)
  - [ ] Парсер JSON → raw egui Rust-код
  - [ ] CLI-утилита `rust4ui-codegen`

---

### v0.3 — Иконки и документация *(текущая)*

- [ ] **Margins для IconButton** — марджины иконок
- [ ] **Hover/Click реакции для IconButton** — хитрый механизм
- [ ] **MenuBar** — исправление ошибок (белые кнопки, {{синтаксис}}, hover)
- [ ] **Числовое поле (mode=number)** — дизайн и настройки
- [ ] **Slider** — доделка внешнего вида
- [ ] **ComboBox** — доделка дизайна
- [ ] **Tabs / Tab** — доделка дизайна
- [ ] **Контекстное меню (ПКМ)** — кастомизация (цвета, hover, иконки)
- [ ] `Image` — загрузка и отображение
- [ ] `ProgressBar` — индикатор прогресса
- [ ] **Table (DataTable)** — на базе egui_extras::TableBuilder
- [ ] **ScrollBar стилизация** — толщина, цвет, отступы
- [ ] **ScrollArea: отступ текста от рамки**
- [ ] **Кастомизированная рамка (Custom Frame)**
- [ ] **Шаблоны / `examples/simple`**
- [ ] **Система иконок** — IconBar anchor (start/center/end), Separator в IconBar
- [ ] **SVG-текстуры** — отложено (помечено в коде)
- [ ] **Микро-анимации иконок** — см. v0.* — Расширяем функционал

- [x] **Иконки Phosphor** 🟢 *(09.07.2026)*
  - [x] IconRegistry: 1512 иконок, `resolve(name)` → codepoint, fallback ⬡
  - [x] `icons/phosphor-icons/` в .gitignore, замена TTF на официальный
  - [x] Иконки в IconButton, Button, Label, MenuItem
  - [x] icon_size на IconButton, size на MenuItem (+ поддержка темы)
- [x] **Hover/Click-стейты** 🟢 *(09.07.2026)*
  - [x] `hover_fill`, `click_fill`, `hover_text_color`, `click_text_color` на Button
  - [x] `hover_fill`, `click_fill` на IconButton
  - [x] `w_color_opt()` в theme.rs
- [x] **Тени** 🟢 *(09.07.2026)* — `shadow_offset_x/y`, `shadow_blur`, `shadow_color` на Button
- [x] **Галерея 1512 иконок** 🟢 *(09.07.2026)* — отдельная вкладка, 95 Label-строк (без тормозов)
- [x] **Документация mdBook** 🟢 *(09.07.2026)* — 14 глав, `___docs.bat`

### v0.2 — Расширение возможностей дизайна *(завершена)*

- [x] **Виджеты** 🟢 *(06.07.2026)*
  - [x] `ComboBox`, `Slider`, `ScrollArea`, `Tabs` / `Tab`, `Panel`, `Window`
- [x] **Контейнеры** 🟢 *(06.07.2026)*
  - [x] `Row` с `wrap: true`
- [x] **Меню и навигация** 🟢 *(06.07.2026)*
  - [x] Контекстное меню, MenuBar + Menu, Toast & Notifications
- [x] **Темы** 🟢 *(06.07.2026)* — `themes/dj_green.json`
- [x] **Border-система** 🟢 *(07.07.2026)* — solid/dash/dot, gap, seg_len, shorthand, border_position
- [x] **JSON-комментарии** 🟢 *(07.07.2026)*
- [x] **Multiline fixed** 🟢 *(09.07.2026)* — ScrollArea + allocate_ui_at_rect

### v0.1 — MVP (Ядро) *(06.07.2026)*

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

---

## Идеи и мысли

- [ ] **Rust4ui как UI-сервер** *(09.07.2026)* — единый сервер, к которому по WebSocket/TCP подключаются разные приложения (Rust, Python, JS, Go). Сервер строит их интерфейсы (JSON → egui) и отправляет действия обратно. Один демон — любое количество клиентов, любой язык. Протокол: `{"cmd":"show","ui":{...}}` / `{"event":"action","name":"..."}`.
- [ ] Анимации (через egui `lerp`)
- [ ] Accessibility (ARIA-подобные атрибуты)
- [ ] macOS/Linux порт (через egui — уже кроссплатформенно)
