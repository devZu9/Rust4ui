# Roadmap — Rust4ui

> Правила оформления версий всегда перечитать перед работой с этим файлом в `.opencode/skills/session-log/SKILL.md`

#### JSON → UI → Rust. Быстрое прототипирование интерфейсов на egui.

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
- [ ] **SVG-текстуры** — `usvg` + `resvg`, duotone-иконки, кастомные стили

### v0.* — Продвинутые виджеты

- [ ] **Темы**
  - [ ] `themes/nord.json`
  - [ ] `themes/gruvbox.json`

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

### v0.5 — Числовое поле, оставшиеся виджеты и полировка *(текущая)*

- [ ] **Отключение сторон бордюра** — возможность отключить любую сторону (top/right/bottom/left) у виджетов с border, чтобы делать уголки или односторонние полоски

- [ ] **MenuBar** — исправление ошибок (белые кнопки, {{синтаксис}}, hover, state-aware фон, каскад _children) 🟢 *(16.07.2026)*
  - [ ] Состояние _focus на раскрытом меню можно настраивать через постфикс
  - [ ] Настройка авто-раскрытия после клика (т.е. первый клик раскрывает пункт меню, а дальше мы просто водим и меню раскрывается из-за hover)
  - [ ] **Кастомизация показа иконок в MenuItem** — настройка видимости иконок (всегда/при наведении/скрыть)
  - [ ] **Border _children** — border для children элементов со всеми состояниями (hover/click/focus)
  - [ ] **Shadow на MenuBar** — протестировать и донастроить shadow на MenuBar и его children
  - [x] **Кастомизация фона/цвета через _children** — background, background_hover/click, color, color_hover/click 🟢 *(16.07.2026)*
  - [x] padding_children / margin_children / rounding_children** 🟢 *(16.07.2026)

- [x] 

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
- [ ] **Микро-анимации иконок** — см. v0.* — Расширяем функционал
- [x] **Числовое поле (mode=number)** — дизайн, точность, степпер, тесты 🟢 *(15.07.2026)*

### v0.4 — Тени, Button shadow, плавное развитие *(завершена)*

- [x] **Vars в theme.json** 🟢 *(15.07.2026)* — переменные `$var`, авторезолв, работа со всеми JSON-типами
- [x] **Settings persistence** 🟢 *(15.07.2026)* — save/load размера/позиции окна, вкладки, языка
- [x] **Приоритет state click > focus > hover** 🟢 *(15.07.2026)* — get_state_border, get_state_attr
- [x] **TextField: focus state** 🟢 *(15.07.2026)* — border_focus, background_focus, убрана синяя рамка egui
- [x] **Каскад теней в Button** 🟢 *(15.07.2026)* — shadow_content (шорткат), shadow_icon + shadow_text (переопределения)
- [x] **Button state-aware** 🟢 *(15.07.2026)* — align_hover/click, padding_hover/click, margin_hover/click
- [x] **IconButton: offset (1,1)** 🟢 *(15.07.2026)* — shadow_icon через parse_content_shadow
- [x] **Button shadow** 🟢 *(14.07.2026)* — shadow_background/border/icon для Button (как в IconButton)
- [x] **Shadow Z-order** 🟢 *(14.07.2026)* — параметр положения тени (под/над элементом) для shadow_border, shadow_content

### v0.3 — Иконки и документация *(завершена)*

- [x] **Margins для всех виджетов** 🟢 *(13.07.2026)* — унифицировать (через `get_margin` + тема)
- [x] **Shadow система** 🟢 *(13.07.2026)* — `shadow_background`, `shadow_border`, `shadow_content`, state-aware
- [x] **border opacity** 🟢 *(13.07.2026)* — `[width, color, opacity, type, gap, seg_len]`
- [x] **`get_state_attr<T>`** 🟢 *(13.07.2026)* — универсальная функция для state-атрибутов любого типа
- [x] **`fill` → `background`** 🟢 *(13.07.2026)* — переименование, `background_hover/click/focus`
- [x] **Suffix naming** 🟢 *(13.07.2026)* — `hover_color`→`color_hover`, `text_color`→`color_text`
- [x] **`color_icon` на Button** 🟢 *(13.07.2026)* — отдельный цвет иконки, раздельный рендер
- [x] **`parse_color_value()`** 🟢 *(13.07.2026)* — `["#HEX", opacity]`
- [x] **`Sense::click`→`click_and_drag`** 🟢 *(12.07.2026)* — убрано таймаут удержания
- [x] **Margins для IconButton** 🟢 *(11.07.2026)* — марджины иконок
- [x] **Hover/Click реакции для IconButton** 🟢 *(11.07.2026)* — хитрый механизм
- [x] **gap_row** 🟢 *(11.07.2026)* — вертикальный отступ между wrapped-строками
- [x] **`parse_hex_color` #RGB/#RGBA** 🟢 *(11.07.2026)*

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
- [x] **Live-reload** 🟢 *(07.07.2026)*
  - [x] File watcher (notify crate) на всю `demo/` директорию
  - [x] Авто-перезагрузка UI-дерева + темы

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
