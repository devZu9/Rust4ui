# Roadmap — Rust4ui

> JSON / EFx → UI → Rust. Быстрое прототипирование интерфейсов на egui.

---

## Легенда

| Метка | Статус |
|-------|--------|
| 🟢 Ready | Реализовано |
| 🟡 In Progress | В разработке |
| 🟠 Planned | Запланировано |
| ⚪ Backlog | Будет рассмотрено |

---

## v0.1 — MVP (Ядро)

Цель: JSON → живой UI в egui-окне.

**🔴 Ближайшие задачи (прямо сейчас):** базовая реализация UiNode + парсер JSON, Theme, StateRegistry, ActionRegistry, Renderer, LocaleRegistry, публикация на [crates.io](https://crates.io).

- [ ] **Ядро рендерера**
  - [ ] Базовый тип `UiNode` + парсинг JSON
  - [ ] Система `$ref` — модульность JSON (вложенные файлы, override атрибутов, кэш, циклы)
  - [ ] Theme (загрузка + merge дефолтов + приоритет ui.json > theme.json)
  - [ ] StateRegistry (HashMap binding + persist-атрибут для автосохранения)
  - [ ] ActionRegistry (функции по имени)
  - [ ] LocaleRegistry (загрузка locale-файлов, резолв `{{key}}`, `{expr}`-интерполяция из state)
  - [ ] Встроенные CLDR plural-правила (ru, en, de, fr, pl, uk, be, ja, zh, ko)
  - [ ] Совместимость с EFx: единый словарь тегов и атрибутов
  - [ ] `Rust4ui::from_efx()` — конструктор из EFx-шаблона
  - [ ] Обработка ошибок (оранжевые/красные заглушки вместо паники)
- [ ] **Контейнеры**
  - [ ] `Column` — `ui.vertical()`
  - [ ] `Row` — `ui.horizontal()`
- [ ] **Виджеты**
  - [ ] `Label` — текст, стили (size, color, bold, italic, monospace)
  - [ ] `Button` — клик → action
  - [ ] `TextField` — singleline/multiline/password, binding → String, mode=number (min/max/step, hover-stepper, scroll)
  - [ ] `Checkbox` — binding → bool
  - [ ] `RadioGroup` — выбор одного значения из списка
  - [ ] `Separator` — разделитель
  - [ ] `FileDrop` — drag-and-drop файлов из ОС (accept, multi, highlight)
  - [ ] `Spinner` — индикатор загрузки
  - [ ] `Shortcut` — глобальные горячие клавиши + shortcut на Button
  - [ ] `ColorPicker` — палитра выбора цвета (hex, alpha)
  - [ ] Rich Tooltip — атрибут tooltip: строка или объект с children
  - [ ] Система иконок — Phosphor font (дефолт) + Texture Registry (PNG)
  - [ ] `icons/` — phosphor.ttf, icons.json (~40 иконок)
  - [ ] `IconBar` — панель иконок (vertical/horizontal, anchor: start/center/end/fill)
  - [ ] `IconButton` — кнопка-иконка (+ indicator-точка)
  - [ ] `Caption` — короткая надпись в панелях (версия, счётчик)
  - [ ] `Indicator` — цветная точка (в IconBar, Row, ContextMenu)
  - [ ] `StatusBar` — строка состояния (start/center/end, Label, Indicator, Separator)
  - [ ] Универсальные визуальные атрибуты: `opacity`, `rounding` (per-corner), `padding`/`margin` (CSS-shorthand), `shadow`, `stroke`
  - [ ] Тема Hover/Focus/Disabled — автоматические реакции из коробки
- [ ] **Встроенные темы**
  - [ ] `themes/dark.json`
  - [ ] `themes/light.json`
- [ ] **Встроенные локали**
  - [ ] `locales/ru.json` — русский язык (дефолтный)
  - [ ] `locales/en.json` — английский язык (fallback)
- [ ] **Пример**
  - [ ] `examples/simple` — минимальный: 3 виджета, 1 action, 1 binding
- [ ] **Демо-прототип** (`demo/`)
  - [ ] `demo/ui.json` — корневой файл (11 строк, 4 `$ref`)
  - [ ] `demo/tabs/` — 6 файлов (all.json + 5 вкладок)
  - [ ] `demo/windows/` — 3 файла (инфо, диалог, кастом)
  - [ ] `demo/theme.json` — дефолтная тёмная тема
  - [ ] `demo/themes/light.json` — светлая тема
  - [ ] `demo/themes/dj_green.json` — DJA-стиль
  - [ ] `locales/ru.json` — переводы для демки (80+ ключей)
  - [ ] `locales/en.json` — переводы для демки (80+ ключей)
- [ ] **Тестирование** (6 уровней)
  - [ ] Pre-flight валидатор JSON (type/binding/action/items/{{key}}/attr-types)
  - [ ] Unit-тесты подсистем (Theme, State, Actions, Locale, Plural)
  - [ ] Интеграционный тест на `demo/ui.json` (все виджеты + state + i18n)
  - [ ] Persistence-тесты (автосохранение: save → reload → restore)
  - [ ] Encoding-тесты (все файлы UTF-8 без BOM)
  - [ ] Smoke-тесты (каждый виджет изолированно, без паники)
- [ ] **OpenCode-скиллы** (`.opencode/skills/`)
  - [ ] `rust-single-source` — единая точка обращения к данным
  - [ ] `rust-localization` — многоязычность, локали
  - [ ] `rust-autosave` — автосохранение настроек
  - [ ] `rust-encoding` — UTF-8 без BOM
  - [ ] `rust-file-sizes` — лимит 250 строк на файл
  - [ ] `rust-quick-launch` — run.bat быстрого запуска
  - [ ] `rust-versioning` — версия только из Cargo.toml
  - [ ] `rust-api-first` — строго следовать спеке
  - [ ] `rust-github` — работа с GitHub
  - [ ] `rust-project-structure` — порядок описания структуры
  - [ ] `rust-testing` — автотесты для каждой фичи
- [ ] Документация README.md + базовое ТЗ

---

## v0.2 — Расширение возможностей

- [ ] **Виджеты**
  - [ ] `ComboBox` — binding → usize, items → Vec<String>
  - [ ] `Slider` — binding → f64, min/max/step
  - [ ] `ScrollArea` — скроллируемый контейнер
  - [ ] `Tabs` / `Tab` — вкладки с переключением
  - [ ] `Panel` — группа с рамкой/фоном
  - [ ] `Window` — расширенное управление (modal, title_bar, anchor, id-persist, show_close)
- [ ] **Контейнеры**
  - [ ] `Row` с `wrap: true` — перенос элементов
- [ ] **Code Generator** (feature `gen`)
  - [ ] Парсер JSON → `efx!()`-код (режим по умолчанию)
  - [ ] Парсер JSON → raw egui-код (опционально, `--format raw`)
  - [ ] CLI-утилита `rust4ui-codegen`
- [ ] **WidgetRegistry** — регистрация кастомных виджетов извне
  - [ ] `WidgetRegistry` + `register_widgets!` макрос
  - [ ] WidgetResponse (changed, clicked)
  - [ ] Доступ к egui::Ui, State, Actions, Theme из обработчика
- [ ] **Меню и навигация**
  - [ ] Контекстное меню (`context_menu` на любом виджете, MenuItem, SubMenu)
  - [ ] MenuBar + Menu (Файл / Правка / Вид / Помощь)
  - [ ] Toast & Notifications (зона + ctx.notify уровни info/success/warning/error)
- [ ] **Темы**
  - [ ] `themes/dj_green.json` (в стиле DJA)
  - [ ] `themes/matrix.json`
  - [ ] `themes/ocean.json`
- [ ] **Локали**
  - [ ] `locales/de.json` — немецкий

---

## v0.3 — Продвинутые виджеты

- [ ] **Виджеты**
  - [ ] `Image` — загрузка и отображение
  - [ ] `Hyperlink` — кликабельная ссылка
  - [ ] `Window` — всплывающее окно
  - [ ] `Grid` — табличная сетка (через egui::Grid)
  - [ ] `ProgressBar` — индикатор прогресса
- [ ] **Layout**
  - [ ] `Padding` / `Margin` на любом контейнере
  - [ ] `align` для Column (left/center/right)
- [ ] **Темы**
  - [ ] `themes/nord.json`
  - [ ] `themes/gruvbox.json`
- [ ] **Локали**
  - [ ] `locales/fr.json` — французский
  - [ ] `locales/pl.json` — польский (с plural-правилами)
  - [ ] `locales/uk.json` — украинский (с plural-правилами)

---

## v0.4 — Полировка

- [ ] **Live-reload**
  - [ ] File watcher (notify crate) → авто-перезагрузка ui.json
  - [ ] Без перезапуска программы
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

---

## v0.5 — Инструменты

- [ ] **GUI-редактор (ранняя версия)**
  - [ ] Просмотр ui.json с живым превью
  - [ ] Drag & drop из палитры на холст
  - [ ] Inspector свойств выбранного элемента
- [ ] **Экспорт**
  - [ ] JSON → standalone .rs файл с `efx!()` (по умолчанию)
  - [ ] JSON → standalone .rs файл с raw egui (опция)
- [ ] **Плагины (Rust-крейты)**
  - [ ] Регистрация кастомных тем

---

## v1.0 — Стабильный релиз

- [ ] Стабильное API (semver 1.0)
- [ ] Полное покрытие тестами
- [ ] CI/CD (GitHub Actions)
- [ ] 20+ встроенных тем
- [ ] Документация на docs.rs
- [ ] Примеры для eframe, bevy_egui, winit+wgpu
- [ ] WASM-поддержка

---

## Технический долг / Backlog

- [ ] Вложенные binding (`"settings.audio.mic_name"`)
- [ ] Анимации (через egui `lerp`)
- [ ] Accessibility (ARIA-подобные атрибуты)
- [ ] macOS/Linux порт (через egui — уже кроссплатформенно)
