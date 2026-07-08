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

- [ ] **System Tray** (фича `tray`)
  - [ ] Иконка в трее + контекстное меню
  - [ ] Динамическая смена иконки + анимация
  - [ ] Windows (Shell), macOS (NSStatusBar), Linux (libappindicator)

- [ ] **Code Generator** (feature `gen`)
- [ ] Парсер JSON → raw egui Rust-код
- [ ] CLI-утилита `rust4ui-codegen`

### v0.2 — Расширение возможностей дизайна (текущая)

- [ ] `Image` — загрузка и отображение
- [ ] `ProgressBar` — индикатор прогресса
- [ ] **Table**
  - [ ] `<DataTable>` на базе egui_extras::TableBuilder
  - [ ] Сортировка колонок  

- [ ] `examples/simple` — минимальный: 3 виджета, 1 action, 1 binding

- [ ] **ScrollBar стилизация** — толщина, цвет, отступы (из SESSIONS.md)
- [ ] **ScrollArea: отступ текста от рамки** — сейчас текст обрезается строго по рамке, нужен внутренний padding
- [ ] **Кастомизированная рамка (Custom Frame)** — hover/focus с настройкой цвета, формы, толщины через тему; замена `frame(true)` под свою систему

- [ ] **Исправление ошибок MenuBar**
  - [ ] Отображение разделов (сейчас белые прямоугольники-кнопки с белым текстом)
  - [ ] Отображение названия пунктов (сейчас `{{синтаксис}}`)
  - [ ] Реакция на мышь — как у меню при наведении, а не только при нажатии
  - [ ] Внедрение кастомизирующих тегов для настройки отображения: цвет фона, цвет текста, hover-подсветка, отступы, скругление
- [ ] **Иконки Phosphor** — найти и решить вопрос по отображению иконок из `phosphor.ttf`
- [ ] **Система иконок** — сделать отдельные понятия: иконка, иконка-кнопка (hover, клик, реакции), панель иконок. Панель может располагаться внутри (окружая себя другими элементами), но прежде всего горизонтальные меню прижаты к верхнему/нижнему краю окна, вертикальные — к левому/правому
- [ ] **Иконки в элементах** — возможность добавлять иконки к заголовкам, текстам, внутрь кнопок — как кликабельными (hover), так и информационными
- [ ] **Числовое поле (mode=number)** — переделка дизайна и настроек
- [ ] **Slider** — доделка внешнего вида, взаимодействия, кастомизация настроек
- [ ] **ComboBox** — доделка дизайна и кастомизации
- [ ] **Tabs / Tab** — доделка дизайна и кастомизации

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

## Технический долг / Backlog

- [ ] Вложенные binding (`"settings.audio.mic_name"`)
- [ ] Анимации (через egui `lerp`)
- [ ] Accessibility (ARIA-подобные атрибуты)
- [ ] macOS/Linux порт (через egui — уже кроссплатформенно)
