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

- [ ] **Ядро рендерера**
  - [ ] Базовый тип `UiNode` + парсинг JSON
  - [ ] Theme (загрузка + merge дефолтов + приоритет ui.json > theme.json)
  - [ ] StateRegistry (HashMap binding)
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
  - [ ] `TextField` — singleline, binding → String
  - [ ] `Checkbox` — binding → bool
  - [ ] `Separator` — разделитель
- [ ] **Встроенные темы**
  - [ ] `themes/dark.json`
  - [ ] `themes/light.json`
- [ ] **Встроенные локали**
  - [ ] `locales/ru.json` — русский язык (дефолтный)
  - [ ] `locales/en.json` — английский язык (fallback)
- [ ] **Пример**
  - [ ] Минимальный `examples/simple` — 3 виджета, 1 action, 1 binding
- [ ] Документация README.md + базовое ТЗ

---

## v0.2 — Расширение возможностей

- [ ] **Виджеты**
  - [ ] `ComboBox` — binding → usize, items → Vec<String>
  - [ ] `Slider` — binding → f64, min/max/step
  - [ ] `ScrollArea` — скроллируемый контейнер
  - [ ] `Tabs` / `Tab` — вкладки с переключением
  - [ ] `Panel` — группа с рамкой/фоном
- [ ] **Контейнеры**
  - [ ] `Row` с `wrap: true` — перенос элементов
- [ ] **Code Generator** (feature `gen`)
  - [ ] Парсер JSON → `efx!()`-код (режим по умолчанию)
  - [ ] Парсер JSON → raw egui-код (опционально, `--format raw`)
  - [ ] CLI-утилита `rust4ui-codegen`
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
  - [ ] Регистрация кастомных виджетов извне
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
