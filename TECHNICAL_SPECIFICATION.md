# Rust4ui — Техническое задание

> **Rust4ui** (Rust for UI) — комбайн для быстрого прототипирования и сборки UI на базе `egui`. JSON → живой интерфейс. Без перекомпиляции. С возможностью «запечь» в Rust-код.

---

## 1. Концепт

### 1.1. Проблема

Разработка UI — это цикл: правишь код → ждёшь компиляцию → смотришь → снова правишь. На каждый чих — `cargo build`. Это медленно. А если UI сложный (настройки, панели, вкладки) — итерация занимает часы.

Готовые UI-фреймворки (Bootstrap, Qt Quick, Flutter) либо не на Rust, либо привязаны к конкретному рантайму.

### 1.2. Решение

Rust4ui разделяет **внешний вид**, **структуру** и **поведение** на три независимых слоя:

| Слой | Файл | Формат | Кто правит |
|------|------|--------|-----------|
| Внешний вид | `theme.json` | JSON | Дизайнер |
| Структура + данные | `ui.json` | JSON | Разработчик |
| Поведение (логика) | `actions` | Rust-код | Программист |

Разработчик UI собирает интерфейс как конструктор:
1. Описывает виджеты в `ui.json`
2. Назначает им цвета/отступы через `theme.json`
3. В Rust-коде регистрирует функции-обработчики (actions) и привязки к данным (state)

Никакой компиляции между изменениями JSON. Только перезапуск приложения — или live-reload через file watcher. При необходимости UI «запекается» в скомпилированный Rust-код через code generator.

### 1.3. Целевая аудитория

- Разработчики на Rust, которые хотят быстро прототипировать UI без `cargo build` на каждый чих
- Команды, где дизайнер правит `theme.json`, а разработчик — только логику
- Авторы тулов, админок, настроек — где UI не megasupercomplex, но должен выглядеть прилично

---

## 2. Архитектура

### 2.1. Схема работы

```
┌─────────────┐     ┌────────────────────┐     ┌─────────┐
│  ui.json     │────→│                    │     │         │
│  (структура) │     │    Rust4ui         │────→│  egui   │
├─────────────┤     │  ┌──────────────┐   │     │  (окно) │
│  theme.json  │────→│  │  Renderer    │   │     │         │
│  (стиль)     │     │  │  рекурсивный  │   │     └─────────┘
├─────────────┤     │  │  обход JSON   │   │
│  state       │◀───→│  └──────────────┘   │
│  (данные)    │     │  ┌──────────────┐   │
├─────────────┤     │  │  Реестр       │   │
│  actions     │◀───→│  │  действий    │   │
│  (функции)   │     │  └──────────────┘   │
└─────────────┘     └────────────────────┘
```

**Два пути рендеринга:**

| Путь | Описание | Когда использовать |
|------|----------|-------------------|
| **Runtime** | JSON → Renderer → egui | Прототипирование, частые правки UI |
| **Codegen** | JSON → raw Rust-код → компиляция → egui | Релиз, статичный UI, макс. скорость |

Каждый кадр egui вызывает `update()`. Renderer проходит по JSON-дереву и для каждого узла:

1. Определяет `type` узла (`"Button"`, `"Column"`, ...)
2. Берёт дефолтные атрибуты из темы
3. Накладывает атрибуты из `ui.json` (они выше приоритетом)
4. Рисует виджет через egui
5. Если виджет поддерживает state — читает/пишет данные из `StateRegistry`
6. Если виджет поддерживает actions — по событию (click, change) ищет функцию в `ActionRegistry` и вызывает её

### 2.2. Поток данных (один кадр)

```
egui:  update()
        │
        ▼
render(root_node)
  │
  ├── type="Column" → ui.vertical(|ui| {
  │     for child in children:
  │         render(child)
  │ })
  │
  ├── type="Label" → ui.label(text)
  │
  ├── type="Button" → let resp = ui.button(text)
  │     if resp.clicked():
  │         actions.get(action_name)(state)
  │
  ├── type="TextField" → let val = state.get(binding)
  │     ui.text_edit_singleline(&mut val)
  │     state.set(binding, val)
  │
  ├── type="Unknown" → ui.label("⚠ Неизвестный: <Unknown>")
  │
  └── ...
```

---

## 3. Формат ui.json

### 3.1. Базовый синтаксис

Каждый элемент — JSON-объект с обязательным полем `type`:

```json
{
  "type": "<имя_виджета>",
  "<атрибут>": "<значение>",
  "children": [ ... ]     // для контейнеров
}
```

### 3.2. Контейнеры

#### Column

Вертикальный контейнер. Генерирует `ui.vertical()`.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Column"` |
| `gap` | float | 0 | Отступ между дочерними элементами |
| `padding` | float | 0 | Внутренний отступ со всех сторон |
| `align` | string | `"left"` | Выравнивание: `"left"`, `"center"`, `"right"` |
| `children` | array | `[]` | Дочерние элементы |

```json
{
  "type": "Column",
  "gap": 8,
  "padding": 12,
  "children": [ ... ]
}
```

#### Row

Горизонтальный контейнер. Генерирует `ui.horizontal()`.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Row"` |
| `gap` | float | 0 | Отступ между элементами |
| `padding` | float | 0 | Внутренний отступ |
| `wrap` | bool | false | Переносить элементы на новую строку |
| `align` | string | `"top"` | Зарезервировано. Всегда прижат к верху по поперечной оси |
| `children` | array | `[]` | Дочерние элементы |

```json
{
  "type": "Row",
  "gap": 8,
  "wrap": true,
  "children": [ ... ]
}
```

#### ScrollArea

Прокручиваемая область.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"ScrollArea"` |
| `axis` | string | `"vertical"` | `"vertical"`, `"horizontal"`, `"both"` |
| `max_height` | float | ∞ | Максимальная высота |
| `max_width` | float | ∞ | Максимальная ширина |
| `children` | array | `[]` | Дочерние элементы |

#### Tabs

Контейнер с вкладками.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Tabs"` |
| `active` | string | — | ID активной вкладки (binding на строку) |
| `gap` | float | 4 | Отступ между заголовками вкладок |
| `children` | array | `[]` | Вкладки (Tab) |

Каждая вкладка:

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Tab"` |
| `id` | string | — | Уникальный ID вкладки |
| `title` | string | `id` | Текст заголовка |
| `enabled` | bool | true | Доступна ли вкладка |
| `children` | array | `[]` | Контент вкладки |

```json
{
  "type": "Tabs",
  "active": "main",
  "children": [
    { "type": "Tab", "id": "main",  "title": "Основные",   "children": [ ... ] },
    { "type": "Tab", "id": "audio", "title": "Запись",     "children": [ ... ] },
    { "type": "Tab", "id": "about", "title": "О программе","children": [ ... ] }
  ]
}
```

#### Panel

Группировка с рамкой/фоном.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Panel"` |
| `fill` | color | из темы | Цвет фона |
| `rounding` | float | из темы | Скругление |
| `stroke_width` | float | 0 | Толщина рамки |
| `stroke_color` | color | из темы | Цвет рамки |
| `padding` | float | из темы | Внутренний отступ |
| `children` | array | `[]` | Дочерние элементы |

#### Window

Всплывающее окно поверх основного. Поддерживает диалоговый режим, кастомную строку заголовка, сохранение позиции/размера, якорение к краям экрана.

**Базовые атрибуты**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Window"` |
| `title` | string | — | Заголовок окна (поддерживает `{{key}}`-ссылку на locale) |
| `id` | string | — | Уникальный ID для сохранения позиции/размера между сессиями |
| `open` | string | — | Binding на bool — открыто/закрыто |
| `modal` | bool | false | Диалоговый режим: блокирует взаимодействие с родительским UI |
| `children` | array | `[]` | Дочерние элементы |

**Заголовок и поведение**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `title_bar` | bool | true | Показать стандартную строку заголовка |
| `show_close` | bool | true | Показать кнопку закрытия (крестик) |
| `movable` | bool | true | Можно перетаскивать |
| `resizable` | bool | true | Можно менять размер |
| `collapsible` | bool | true | Можно свернуть в заголовок |
| `enabled` | bool | true | Контент активен (false = серый/заблокирован) |
| `constrain` | bool | true | Не давать уйти за границы вьюпорта |
| `auto_sized` | bool | false | Автоматический размер под контент при открытии |

**Размер**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `default_width` | float | 400 | Начальная ширина |
| `default_height` | float | 300 | Начальная высота |
| `min_width` | float | 100 | Минимальная ширина |
| `min_height` | float | 80 | Минимальная высота |
| `max_width` | float | ∞ | Максимальная ширина |
| `max_height` | float | ∞ | Максимальная высота |

**Позиция**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `default_x` | float | — | Начальная X-позиция |
| `default_y` | float | — | Начальная Y-позиция |
| `pos_x` | float | — | Фиксированная X-позиция (каждый кадр, перезаписывает default) |
| `pos_y` | float | — | Фиксированная Y-позиция (каждый кадр, перезаписывает default) |

**Якорение (привязка к краю экрана)**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `anchor_h` | string | — | Горизонтальный якорь: `"left"`, `"center"`, `"right"` |
| `anchor_v` | string | — | Вертикальный якорь: `"top"`, `"center"`, `"bottom"` |
| `anchor_x` | float | 0 | Отступ от якоря по X (например, `-12` — отступ слева) |
| `anchor_y` | float | 0 | Отступ от якоря по Y (например, `12` — отступ сверху) |

**Стиль**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `frame` | bool | true | Рамка окна |
| `fill` | color | из темы | Цвет фона |
| `stroke_width` | float | 1 | Толщина рамки |
| `stroke_color` | color | из темы | Цвет рамки |
| `padding` | float | 8 | Внутренний отступ контента |

**Примеры**

```json
// Диалоговое окно (модальное)
{
  "type": "Window",
  "id": "confirm_dialog",
  "title": "{{dialog.confirm}}",
  "open": "show_confirm",
  "modal": true,
  "title_bar": true,
  "show_close": true,
  "auto_sized": true,
  "movable": false,
  "resizable": false,
  "collapsible": false,
  "children": [
    { "type": "Column", "gap": 10, "children": [
      { "type": "Label", "text": "{{dialog.message}}" },
      { "type": "Row", "gap": 8, "children": [
        { "type": "Button", "text": "{{btn.ok}}", "action": "confirm_ok", "fill": "#00AA66" },
        { "type": "Button", "text": "{{btn.cancel}}", "action": "confirm_cancel" }
      ]}
    ]}
  ]
}

// Окно настроек без заголовка, с кастомным закрытием
{
  "type": "Window",
  "id": "custom_panel",
  "title": "",
  "open": "show_custom",
  "title_bar": false,
  "show_close": false,
  "default_width": 500,
  "default_height": 400,
  "anchor_h": "right",
  "anchor_v": "top",
  "anchor_x": -20,
  "anchor_y": 40,
  "fill": "#1A1D23",
  "stroke_color": "#00FF6644",
  "children": [
    { "type": "Column", "gap": 8, "children": [
      { "type": "Row", "gap": 4, "children": [
        { "type": "Label", "text": "{{panel.title}}", "bold": true, "size": 14 },
        { "type": "Button", "text": "✕", "action": "toggle_custom_panel", "fill": "#CC3333", "min_width": 28 }
      ]},
      { "type": "Separator" },
      { "type": "Label", "text": "{{panel.content}}" }
    ]}
  ]
}
```

**Сохранение позиции и размера**

При заданном `id` (строка) окно сохраняет позицию и размер в `data/settings.json`. При следующей загрузке открывается в том же месте:

```
data/settings.json → windows.confirm_dialog.x → 240.0
data/settings.json → windows.confirm_dialog.y → 120.0
data/settings.json → windows.confirm_dialog.w → 360.0
data/settings.json → windows.confirm_dialog.h → 200.0
```

Если `id` не указан — позиция не сохраняется, используется позиция по умолчанию.

### 3.3. Базовые виджеты

#### Label

Текст.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Label"` |
| `text` | string | `""` | Текст (поддерживает `{expr}`-интерполяцию из state и `{{key}}`-ссылку на locale) |
| `size` | float | из темы | Размер шрифта |
| `color` | color | из темы | Цвет текста |
| `bold` | bool | false | Жирный |
| `italic` | bool | false | Курсив |
| `monospace` | bool | false | Моноширинный |
| `wrap` | bool | false | Перенос строк |

#### Button

Кнопка.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Button"` |
| `text` | string | `""` | Текст кнопки (поддерживает `{{key}}`-ссылку на locale) |
| `fill` | color | из темы | Цвет фона |
| `height` | float | из темы | Минимальная высота (padding расширяет сверху) |
| `min_width` | float | из темы | Минимальная ширина |
| `padding` | float/array | из темы | Внутренний отступ (border-box: добавляется к min_width/height) |
| `align` | string | `"left"` | Выравнивание текста: `"left"`, `"center"`, `"right"` |
| `rounding` | float | из темы | Скругление |
| `action` | string | — | Имя действия при клике |
| `target` | string | — | Доп. параметр для действия |
| `enabled` | bool | true | Доступна ли кнопка |
| `tooltip` | string | — | Подсказка при наведении (поддерживает `{{key}}`-ссылку на locale) |

```json
{
  "type": "Button",
  "text": "{{btn.save}}",
  "action": "save",
  "target": "mic"
}
```

#### TextField

Поле ввода. Поддерживает три режима: текст, пароль, число.

**Общие атрибуты:**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"TextField"` |
| `binding` | string | — | Ключ в StateRegistry |
| `hint` | string | `""` | Подсказка в пустом поле |
| `width` | float | из темы | Ширина |
| `height` | float | из темы | Минимальная высота (padding расширяет сверху) |
| `padding` | float/array | из темы | Внутренний отступ (border-box: field_h = max(height, font_h + padding)) |
| `text_align` | string | `"left"` | Выравнивание текста: `"left"`, `"center"`, `"right"` |
| `bg_fill` | color | из темы | Цвет заливки поля |
| `rounding` | float | из темы | Скругление углов |
| `multiline` | bool | false | Многострочный |

**Режимы (mode):**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `mode` | string | `"text"` | `"text"` — свободный текст, `"password"` — маскированный ввод, `"number"` — числовое поле со степером |
| `password` | bool | false | Алиас для `mode="password"`. Если `true` — режим пароля, независимо от `mode` |
| `min` | float | — | Минимум (только при `mode="number"`) |
| `max` | float | — | Максимум (только при `mode="number"`) |
| `step` | float | 1 | Шаг изменения (только при `mode="number"`) |
| `decimals` | int | из `step` | Количество знаков после запятой: `0` = целые, `2` = сотые. Авто: `step=0.5` → 1, `step=0.01` → 2. Явное значение переопределяет авто | |

**Поведение mode="number":**

- Поле отображается как обычное текстовое
- При наведении (hover) справа появляются две стрелки ▲ и ▼
- Стрелка ▲ — увеличить на `step`, ▼ — уменьшить на `step`
- Скролл мышью над полем — тоже меняет значение
- Можно ввести значение вручную с клавиатуры
- При уходе курсора стрелки плавно исчезают
- Значение форматируется с точностью `decimals` (авто из `step` или явно)
- Значение привязано к `f64` в StateRegistry. Если binding указывает на `String` — значение конвертируется в строку и обратно

**Примеры:**

```json
// Обычное текстовое поле (mode по умолчанию)
{
  "type": "TextField",
  "binding": "mic_name",
  "hint": "Название микрофона",
  "width": 250
}

// Пароль
{
  "type": "TextField",
  "binding": "password",
  "mode": "password",
  "width": 200
}

// Числовое поле со степером
{
  "type": "TextField",
  "binding": "font_size",
  "mode": "number",
  "min": 8,
  "max": 72,
  "step": 2,
  "decimals": 0,
  "width": 120
}

// Дробное число
{
  "type": "TextField",
  "binding": "gain",
  "mode": "number",
  "min": -60,
  "max": 12,
  "step": 0.5,
  "width": 120
}
```

#### ComboBox

Выпадающий список.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"ComboBox"` |
| `binding` | string | — | Ключ в StateRegistry (usize — индекс) |
| `items` | string | — | Ключ в StateRegistry (массив строк) |
| `width` | float | из темы | Ширина |

```json
{
  "type": "ComboBox",
  "binding": "mic_idx",
  "items": "mic_list",
  "width": 250
}
```

`items` указывает на элемент в `StateRegistry` типа `Vec<String>`. При каждом кадре рендерер читает список и отображает его.

#### Checkbox

Флажок.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Checkbox"` |
| `binding` | string | — | Ключ в StateRegistry (bool) |
| `text` | string | `""` | Текст рядом с флажком (поддерживает `{{key}}`-ссылку на locale) |

```json
{
  "type": "Checkbox",
  "binding": "use_gpu",
  "text": "Использовать GPU"
}
```

#### Slider

Ползунок.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Slider"` |
| `binding` | string | — | Ключ в StateRegistry (f64) |
| `min` | float | 0 | Минимум |
| `max` | float | 1 | Максимум |
| `step` | float | 0 | Шаг (0 = свободно) |
| `width` | float | из темы | Ширина |
| `text` | string | `""` | Подпись слева (поддерживает `{{key}}`-ссылку на locale) |

#### Separator

Разделитель.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Separator"` |
| `space` | float | из темы | Отступ сверху и снизу |

#### FileDrop

Зона приёма файлов из ОС (drag-and-drop из Проводника/Файндера). При перетаскивании файла подсвечивается, при отпускании вызывает action с путём файла.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"FileDrop"` |
| `action` | string | — | Action при drop (ctx.target() = путь к файлу или JSON-массив путей) |
| `accept` | array | `[]` | Расширения файлов (пустой = любые): `[".mid", ".json", ".txt"]` |
| `multi` | bool | false | Принимать несколько файлов за раз |
| `fill` | color | из темы | Цвет фона зоны |
| `rounding` | float | из темы | Скругление углов |
| `stroke_width` | float | 1 | Толщина рамки |
| `stroke_color` | color | из темы | Цвет рамки |
| `highlight_color` | color | `"#3366CC44"` | Цвет подсветки при наведении файла |
| `padding` | float | из темы | Внутренний отступ |
| `children` | array | `[]` | Контент внутри зоны (подсказки, иконка) |

**Как работает:**
- Каждый кадр рендерер проверяет `ctx.input(|i| i.raw.dropped_files)` из egui
- Если файлы упали в область FileDrop — вызывает `action` с путём в `ctx.target()`
- При `multi: true` — `ctx.target()` содержит JSON-массив: `["C:\\a.mid", "C:\\b.json"]`
- При `multi: false` — `ctx.target()` содержит одиночный путь: `"C:\\a.mid"`
- На время перетаскивания над зоной применяется `highlight_color`

**Примеры:**

```json
// Простая зона для JSON
{
  "type": "FileDrop",
  "action": "file_dropped",
  "accept": [".json"],
  "children": [
    { "type": "Column", "gap": 8, "align": "center", "children": [
      { "type": "Label", "text": "{{drop.hint}}", "size": 14, "color": "#888888" },
      { "type": "Label", "text": "JSON", "size": 11, "monospace": true, "color": "#666666" }
    ]}
  ]
}

// Мульти-файловая зона для аудио и изображений
{
  "type": "FileDrop",
  "action": "media_dropped",
  "accept": [".mid", ".wav", ".mp3", ".png", ".jpg"],
  "multi": true,
  "children": [
    { "type": "Label", "text": "{{drop.media_hint}}", "size": 14, "color": "#888888" }
  ]
}
```
  
#### RadioGroup

Группа радиопереключателей — выбор одного значения из нескольких.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"RadioGroup"` |
| `binding` | string | — | Ключ в StateRegistry (usize — индекс выбранного) |
| `options` | array | — | Массив объектов: `{ "value": int, "text": "метка" }` |
| `direction` | string | `"vertical"` | `"vertical"` или `"horizontal"` |

```json
{
  "type": "RadioGroup",
  "binding": "selected_theme",
  "direction": "vertical",
  "options": [
    { "value": 0, "text": "{{radio.dark}}" },
    { "value": 1, "text": "{{radio.light}}" },
    { "value": 2, "text": "{{radio.auto}}" }
  ]
}
```

#### Spinner

Индикатор загрузки / ожидания.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Spinner"` |
| `size` | float | 24 | Размер круга |
| `color` | color | из темы | Цвет индикатора |
| `text` | string | `""` | Текст под спиннером (поддерживает `{{key}}`) |

```json
{ "type": "Spinner", "size": 32, "color": "#66CCFF" }
{ "type": "Spinner", "size": 20, "text": "{{label.loading}}" }
```

#### Shortcut

Глобальная горячая клавиша без привязки к кнопке.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Shortcut"` |
| `key` | string | — | Комбинация: `"Ctrl+S"`, `"Ctrl+Shift+N"`, `"Alt+F4"` |
| `action` | string | — | Имя действия в ActionRegistry |
| `target` | string | — | Параметр действия |

```json
{ "type": "Shortcut", "key": "Ctrl+S", "action": "save" }
{ "type": "Shortcut", "key": "Ctrl+Z", "action": "undo" }
```

Атрибут `shortcut` также можно указать на **Button** — горячая клавиша привяжется к кнопке и отобразится в подсказке:

```json
{ "type": "Button", "text": "{{btn.save}}", "action": "save", "shortcut": "Ctrl+S" }
```

#### ColorPicker

Палитра выбора цвета.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"ColorPicker"` |
| `binding` | string | — | Ключ в StateRegistry (String — hex: `"#FF6633"`) |
| `alpha` | bool | false | Показывать альфа-канал |
| `width` | float | из темы | Ширина области превью |

```json
{ "type": "ColorPicker", "binding": "accent_color", "alpha": true }
{ "type": "ColorPicker", "binding": "bg_color", "alpha": false }
```
  
#### Indicator

Цветной индикатор (точка). Работает в любом контейнере — IconBar, Row, Panel, ContextMenu.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Indicator"` |
| `color` | color | `"#888888"` | Цвет индикатора |
| `size` | float | 8 | Диаметр в px |
| `pulse` | bool | false | Пульсирующая анимация |
| `tooltip` | string | — | Подсказка при наведении |

```json
{ "type": "Indicator", "color": "#00FF66", "size": 8, "tooltip": "{{tip.online}}" }
{ "type": "Indicator", "color": "#FF4444", "size": 10, "pulse": true }
```

#### IconBar

Панель иконок — вертикальный или горизонтальный контейнер для IconButton, Caption, Indicator и Separator.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"IconBar"` |
| `direction` | string | `"vertical"` | `"vertical"` или `"horizontal"` |
| `width` | float | 48 | Ширина (для vertical) |
| `height` | float | 36 | Высота (для horizontal) |
| `fill` | color | из темы | Цвет фона |
| `stroke_width` | float | 0 | Толщина рамки |
| `stroke_color` | color | из темы | Цвет рамки |
| `children` | array | `[]` | IconButton, IconBarButton, Caption, Indicator, Separator |

#### IconButton

Кнопка-иконка для размещения внутри IconBar. Только иконка, без текста.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"IconButton"` |
| `icon` | string | — | Имя иконки (см. 3.8.2) |
| `icon_size` | float | 1.2 | Множитель размера |
| `icon_color` | color | из темы | Цвет иконки |
| `action` | string | — | Действие |
| `target` | string | — | Параметр действия |
| `tooltip` | string | — | Подсказка |
| `anchor` | string | `"start"` | Позиция в панели |
| `enabled` | bool | true | Доступна ли |
| `indicator` | color | — | Цвет точки-индикатора на иконке (например, `"#FF4444"` для записи) |

#### Caption

Короткая надпись в панели иконок — версия, счётчик, статус.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Caption"` |
| `text` | string | — | Текст (поддерживает `{{key}}` и `{expr}`) |
| `color` | color | из темы | Цвет текста |
| `size` | float | 11 | Размер шрифта |
| `anchor` | string | `"start"` | Позиция в панели |
| `monospace` | bool | false | Моноширинный шрифт |

**Поведение `anchor` в IconBar:**

| Значение | Вертикальная панель | Горизонтальная панель |
|----------|--------------------|-----------------------|
| `"start"` ✦ | **Прижать к верху** (по умолчанию) | **Прижать к левому краю** (по умолчанию) |
| `"center"` | Отцентрировать по вертикали | Отцентрировать по горизонтали |
| `"end"` | Прижать к низу | Прижать к правому краю |
| `"fill"` | Растянуть равномерно по высоте | Растянуть равномерно по ширине |

**Примеры:**

```json
// Вертикальная панель (левая боковая)
{
  "type": "IconBar",
  "direction": "vertical",
  "width": 48,
  "children": [
    { "type": "IconButton", "icon": "home", "anchor": "start", "action": "go_home", "tooltip": "{{tip.home}}" },
    { "type": "IconButton", "icon": "search", "anchor": "start", "action": "search", "tooltip": "{{tip.search}}" },
    { "type": "Separator", "anchor": "start" },
    { "type": "IconButton", "icon": "mic", "anchor": "center", "action": "record", "tooltip": "{{tip.record}}" },
    { "type": "Separator", "anchor": "end" },
    { "type": "IconButton", "icon": "settings", "anchor": "end", "action": "settings", "tooltip": "{{tip.settings}}" },
    { "type": "Indicator", "color": "#00FF66", "size": 8, "anchor": "end", "tooltip": "{{tip.online}}" },
    { "type": "Caption", "text": "v1.0.2", "anchor": "end", "color": "#888888" }
  ]
}

// Горизонтальная панель (тулбар)
{
  "type": "IconBar",
  "direction": "horizontal",
  "height": 36,
  "children": [
    { "type": "IconButton", "icon": "save", "anchor": "start", "action": "save" },
    { "type": "IconButton", "icon": "undo", "anchor": "start", "action": "undo" },
    { "type": "Separator", "anchor": "start" },
    { "type": "IconButton", "icon": "play", "anchor": "fill", "action": "play" },
    { "type": "IconButton", "icon": "stop", "anchor": "fill", "action": "stop" },
    { "type": "IconButton", "icon": "forward", "anchor": "fill", "action": "forward" },
    { "type": "Separator", "anchor": "end" },
    { "type": "Caption", "text": "{word_count} слов", "anchor": "end" }
  ]
}
```

#### StatusBar

Строка состояния — горизонтальная полоса с тремя зонами (лево / центр / право). Обычно располагается внизу окна.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"StatusBar"` |
| `height` | float | 26 | Высота полосы |
| `fill` | color | из темы | Цвет фона |
| `stroke_width` | float | 1 | Толщина верхней линии-разделителя |
| `stroke_color` | color | из темы | Цвет разделителя |
| `padding` | float | 4 | Внутренний отступ |
| `children` | array | `[]` | Label, Indicator, Separator, Caption, IconButton |

Дети поддерживают атрибут `anchor`: `"start"` (слева, по умолчанию), `"center"`, `"end"` (справа).

```json
{
  "type": "StatusBar",
  "height": 26,
  "fill": "#1C1C22",
  "stroke_width": 1,
  "stroke_color": "#333333",
  "padding": 4,
  "children": [
    { "type": "Label", "text": "Слов: {word_count}", "anchor": "start", "size": 11, "color": "#888888" },
    { "type": "Indicator", "color": "#00FF66", "anchor": "center", "size": 6, "tooltip": "{{tip.online}}" },
    { "type": "Label", "text": "UTF-8", "anchor": "end", "size": 11, "monospace": true, "color": "#888888" },
    { "type": "Label", "text": "{active_locale}", "anchor": "end", "size": 11, "color": "#888888" }
  ]
}
```

### 3.4. Зарезервированные поля

- `type` — имя виджета (обязательное)
- `children` — массив дочерних узлов
- `binding` — привязка к StateRegistry
- `items` — привязка к StateRegistry для списка
- `action` — имя действия в ActionRegistry
- `target` — параметр действия

Все остальные поля — атрибуты стиля. Универсальные атрибуты применимы к любому виджету (если виджет поддерживает):

### 3.4.1. Универсальные визуальные атрибуты

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `opacity` | float | 1.0 | Прозрачность: 1.0 = непрозрачно, 0.5 = полупрозрачно, 0.0 = скрыто |
| `fill` | color | из темы | Заливка фона |
| `stroke_width` | float | из темы | Толщина обводки (бордюра) |
| `stroke_color` | color | из темы | Цвет обводки |
| `shadow` | string | `"none"` | Тень: `"none"`, `"small"`, `"medium"`, `"large"` |
| `rounding` | float / array | из темы | Скругление углов (см. ниже) |
| `padding` | float / array | из темы | Внутренний отступ (см. ниже) |
| `margin` | float / array | из темы | Внешний отступ (только верхний, через add_space) |

**Per-side атрибуты:**

| Атрибут | CSS-аналог |
|---------|-----------|
| `rounding_tl` | `border-top-left-radius` |
| `rounding_tr` | `border-top-right-radius` |
| `rounding_br` | `border-bottom-right-radius` |
| `rounding_bl` | `border-bottom-left-radius` |
| `padding_top` | `padding-top` |
| `padding_right` | `padding-right` |
| `padding_bottom` | `padding-bottom` |
| `padding_left` | `padding-left` |
| `margin_top` | `margin-top` |
| `margin_right` | `margin-right` |
| `margin_bottom` | `margin-bottom` |
| `margin_left` | `margin-left` |

**Shorthand:**

```
1 значение (number): N           → все стороны одинаково
1 значение (array): [N]          → все стороны одинаково
2 значения: [V, H]               → вертикаль, горизонталь
4 значения: [T, R, B, L]         → top, right, bottom, left
```

**Примечание:** `padding` во всех виджетах работает как border-box:
padding добавляется к указанному размеру, а не вычитается из него.
`field_h = max(specified_height, content_height + pad_top + pad_bottom)`.

**Примеры:**

```json
// Одинарное значение
{ "padding": 8, "rounding": 6 }

// Shorthand-массив (CSS-порядок)
{ "padding": [8, 12, 8, 12] }
{ "margin": [4, 0, 8, 0] }
{ "rounding": [0, 8, 8, 0] }

// Per-side (переопределяет shorthand)
{ "rounding_tr": 12, "rounding_bl": 4 }

// Полный пример
{
  "type": "Button",
  "text": "OK",
  "fill": "#3366CC",
  "rounding": [8, 2, 8, 2],
  "padding": [6, 16, 6, 16],
  "margin": [0, 4, 0, 4],
  "opacity": 0.9,
  "shadow": "small"
}
```

### 3.5. Ссылки на переводы (`{{key}}`)

В любом текстовом атрибуте (`text`, `hint`, `title`, `tooltip` и т.д.) можно указать `{{key}}` — ссылку на ключ в текущем locale-файле. Разрешение происходит в два прохода:

1. `{{key}}` → LocaleRegistry → переведённая строка (из `locales/*.json`)
2. `{expr}` внутри переведённой строки → StateRegistry (как обычная интерполяция)

```json
// ui.json
{ "type": "Label", "text": "{{settings.title}}" }

// locales/ru.json  →  "settings.title": "Настройки"
// locales/en.json  →  "settings.title": "Settings"
```

Если ключ не найден — в UI отображается `{{key}}` как заглушка (оранжевый текст).

### 3.6. Дизайн-словарь тегов

Имена виджетов и базовые атрибуты следуют устоявшемуся дизайн-словарю, общему с проектами наподобие EFx (который послужил источником вдохновения при проектировании). Это означает, что `"type": "Column"` везде означает вертикальный контейнер с атрибутом `gap`, `"type": "Button"` — кнопка с `action`, и так далее. Никакой технической зависимости от EFx нет — все виджеты реализованы самостоятельно на egui.

Один и тот же набор виджетов: Column, Row, Label, Button, TextField, Checkbox, Slider, ComboBox, Separator, Tabs/Tab, Panel, Window, ScrollArea, Hyperlink, Image, Grid.

### 3.7. Модульность и `$ref` (разделение на файлы)

Большие JSON-файлы можно разбивать на модули через систему ссылок `$ref`. При загрузке ссылки рекурсивно резолвятся в единое дерево — рендерер видит уже плоскую структуру.

#### Синтаксис

В любом месте JSON — в корне, внутри `children`, в атрибутах — можно заменить узел ссылкой:

```json
// main ui.json — компактный корневой файл
{
  "type": "Column",
  "children": [
    { "$ref": "tabs/all.json" },
    { "$ref": "windows/info.json" }
  ]
}

// tabs/all.json — вынесенный модуль
{
  "type": "Tabs",
  "active": "basic",
  "children": [
    { "$ref": "basic.json" },
    { "$ref": "input.json" }
  ]
}
```

#### Формы `$ref`

| Форма | Пример | Описание |
|-------|--------|----------|
| Чистая замена | `{ "$ref": "tabs/basic.json" }` | Узел полностью заменяется содержимым файла |
| Override атрибутов | `{ "$ref": "btn.json", "text": "OK" }` | Загруженный узел + переопределение атрибутов |
| В массиве children | `"children": [{ "$ref": "a.json" }, { "$ref": "b.json" }]` | Естественная вложенность |

#### Правила разрешения

| Правило | Описание |
|---------|----------|
| **Относительные пути** | Разрешаются относительно файла, в котором написан `$ref` |
| **Рекурсия** | `$ref` внутри загруженного файла тоже резолвятся |
| **Циклы** | A→B→A → ошибка загрузки с указанием цепочки |
| **Кэш** | Каждый файл парсится один раз, повторные ссылки — из кэша |
| **Override** | Атрибуты в узле с `$ref` переопределяют атрибуты из загруженного файла |
| **Валидация** | Pre-flight валидатор проверяет резолвнутое дерево целиком |

#### API

```rust
// Автоматически при загрузке
let tree = UiNode::from_file("ui.json");  // рекурсивно резолвит все $ref

// Вручную (для тестов)
let node = UiNode::from_json_str(r#"{"$ref": "fragment.json"}"#);
let resolved = node.resolve_refs("base/dir/");
```

#### Что это даёт

- **Меньше контекста для ИИ** — правишь одну вкладку → отдаёшь 80 строк вместо 900
- **Переиспользование** — один фрагмент встраивается в несколько мест
- **Разделение труда** — дизайнер правит `windows/info.json`, разработчик — `tabs/input.json`
- **Code review** — diff на 30 строк вместо 900

### 3.8. Контекстные меню

Любой виджет может иметь атрибут `context_menu` — меню по правой кнопке. Внутри работают стандартные виджеты (MenuItem, SubMenu, Checkbox, RadioGroup, Separator).

```json
{
  "type": "Button",
  "text": "{{btn.settings}}",
  "context_menu": {
    "children": [
      { "type": "MenuItem", "text": "{{menu.undo}}", "action": "undo", "shortcut": "Ctrl+Z" },
      { "type": "MenuItem", "text": "{{menu.redo}}", "action": "redo", "shortcut": "Ctrl+Y", "enabled": false },
      { "type": "Separator" },
      { "type": "Checkbox", "binding": "autosave", "text": "{{menu.auto}}" },
      { "type": "RadioGroup", "binding": "theme_choice", "direction": "horizontal", "options": [
        { "value": 0, "text": "{{theme.dark}}" },
        { "value": 1, "text": "{{theme.light}}" }
      ]},
      { "type": "SubMenu", "text": "{{menu.export}}", "children": [
        { "type": "MenuItem", "text": "JSON", "action": "export_json" },
        { "type": "MenuItem", "text": "CSV",  "action": "export_csv" }
      ]}
    ]
  }
}
```

**MenuItem:**

| Атрибут | Тип | Описание |
|---------|-----|----------|
| `type` | string | `"MenuItem"` |
| `text` | string | Текст пункта (поддерживает `{{key}}`) |
| `action` | string | Действие |
| `shortcut` | string | Горячая клавиша |
| `enabled` | bool | Доступен ли пункт (по умолчанию true) |
| `icon` | string | Имя иконки (см. 3.8.2) |
| `checked` | bool | Галочка ✓ слева (опционально) |

**SubMenu:**

| Атрибут | Тип | Описание |
|---------|-----|----------|
| `type` | string | `"SubMenu"` |
| `text` | string | Текст |
| `icon` | string | Иконка |
| `children` | array | Вложенные пункты |

### 3.8.1. MenuBar и панель меню

Горизонтальная строка меню (Файл / Правка / Вид / Помощь):

```json
{
  "type": "MenuBar",
  "children": [
    { "type": "Menu", "text": "{{menu.file}}", "children": [
      { "type": "MenuItem", "text": "{{menu.new}}", "action": "new", "shortcut": "Ctrl+N" },
      { "type": "MenuItem", "text": "{{menu.open}}", "action": "open", "shortcut": "Ctrl+O" },
      { "type": "Separator" },
      { "type": "MenuItem", "text": "{{menu.exit}}", "action": "exit", "shortcut": "Alt+F4" }
    ]},
    { "type": "Menu", "text": "{{menu.edit}}", "children": [
      { "type": "MenuItem", "text": "{{menu.undo}}", "action": "undo", "shortcut": "Ctrl+Z" },
      { "type": "MenuItem", "text": "{{menu.redo}}", "action": "redo", "shortcut": "Ctrl+Y" }
    ]},
    { "type": "Menu", "text": "{{menu.view}}", "children": [
      { "type": "Checkbox", "binding": "show_sidebar", "text": "{{menu.sidebar}}" }
    ]},
    { "type": "Menu", "text": "{{menu.help}}", "children": [
      { "type": "MenuItem", "text": "{{menu.about}}", "action": "about" }
    ]}
  ]
}
```

| Тип | Назначение |
|-----|-----------|
| `MenuBar` | Горизонтальная строка меню |
| `Menu` | Выпадающий список (дети — любые виджеты: MenuItem, Checkbox, RadioGroup, Separator) |

### 3.8.2. Система иконок

Иконки работают через **шрифтовой рендеринг** (Phosphor — по умолчанию) или **текстуры** (PNG — через регистрацию). Шрифт `icons/phosphor.ttf` вкомпилирован в бинарь, `icons.json` маппит имена на codepoint'ы.

**`icons/icons.json` (фрагмент):**

```json
{ "save": "\uE00A", "delete": "\uE01B", "settings": "\uE02C",
  "search": "\uE03D", "folder": "\uE04E", "play": "\uE070",
  "close": "\uE222", "menu": "\uE233", "loading": "\uE255"
}
```

**Атрибуты иконок** (работают на Button, MenuItem, Tab, Window, Label):

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `icon` | string | — | Имя из icons.json, имя текстуры или прямой путь к PNG |
| `icon_size` | float | 1.0 | Для font: множитель размера шрифта |
| `icon_width` | float | — | Ширина в px (для texture) |
| `icon_height` | float | — | Высота в px (для texture) |
| `icon_position` | string | `"left"` | `"left"`, `"right"`, `"top"`, `"icon_only"` |
| `icon_color` | color | цвет текста | Цвет иконки (только для font) |

**Резолвинг:** `"icon": "save"` → icons.json → codepoint → рендер шрифтом. Не найдено в icons.json → Texture Registry → рендер как Image.

**Регистрация texture-иконок в Rust:**

```rust
icons.register_texture("logo", ctx.load_texture("logo", &logo_rgba, TextureOptions::LINEAR));
```

**Примеры:**

```json
// Font icon (шрифт Phosphor)
{ "type": "Button", "text": "{{btn.save}}", "icon": "save" }
{ "type": "Button", "text": "{{btn.danger}}", "icon": "delete", "icon_color": "#FF4444" }

// Texture icon (кастомный логотип)
{ "type": "Button", "icon": "logo", "icon_width": 24, "icon_height": 24 }

// MenuItem с иконкой
{ "type": "MenuItem", "text": "Открыть", "icon": "folder", "shortcut": "Ctrl+O" }

// Иконка на вкладке
{ "type": "Tab", "id": "audio", "title": "{{tab.audio}}", "icon": "mic" }
```

**Директория по умолчанию:**

```
icons/
├── phosphor.ttf          # шрифт (~50 КБ, включён в бинарь)
├── icons.json            # маппинг имя → codepoint (~40 записей)
└── README.md             # инструкция по добавлению своих иконок
```

### 3.8.3. Toast & Notifications

Зона уведомлений + вызов из кода:

```json
// Разместить зону уведомлений (обычно в корне UI)
{ "type": "Notifications", "position": "top-right", "max_count": 5, "width": 300 }
```

**Notifications:**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Notifications"` |
| `position` | string | `"top-right"` | `"top-left"`, `"top-right"`, `"bottom-left"`, `"bottom-right"` |
| `max_count` | int | 5 | Максимум одновременных уведомлений |
| `width` | float | 300 | Ширина уведомлений |

**Вызов из ActionCtx:**

```rust
ctx.notify("{{notify.saved}}", level: "success");        // зелёное, 3 секунды
ctx.notify("{{notify.error}}", level: "error", ttl: 5.0); // красное, 5 секунд
ctx.notify("Copy done", level: "info", ttl: 2.0);
```

**Уровни:** `info`, `success`, `warning`, `error` (цвет + иконка определяются автоматически).

**Rich Tooltip** — атрибут `tooltip` на любом виджете принимает строку ИЛИ объект:

```json
// Простой (строка)
{ "type": "Button", "text": "OK", "tooltip": "Нажми меня" }

// Богатый (объект с дочерними виджетами)
{ "type": "Button", "text": "{{btn.save}}", "tooltip": {
    "delay": 0.3,
    "children": [
      { "type": "Label", "text": "{{tip.save_title}}", "bold": true, "size": 13 },
      { "type": "Separator", "space": 4 },
      { "type": "Label", "text": "Ctrl+S", "monospace": true, "color": "#888888", "size": 11 }
    ]
  }
}
```

### 3.8.4. System Tray

Иконка в системном трее с контекстным меню:

```json
{
  "type": "Tray",
  "icon": "app_icon",
  "tooltip": "{{app.title}}",
  "context_menu": {
    "children": [
      { "type": "MenuItem", "text": "{{tray.show}}", "action": "show_window", "icon": "restore" },
      { "type": "Separator" },
      { "type": "Checkbox", "binding": "monitoring", "text": "{{tray.monitoring}}" },
      { "type": "SubMenu", "text": "{{tray.status}}", "children": [
        { "type": "RadioGroup", "binding": "tray_status", "options": [
          { "value": 0, "text": "{{tray.online}}" },
          { "value": 1, "text": "{{tray.away}}" }
        ]}
      ]},
      { "type": "Separator" },
      { "type": "MenuItem", "text": "{{tray.exit}}", "action": "exit", "icon": "close" }
    ]
  }
}
```

**Tray:**

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Tray"` |
| `icon` | string | — | Имя текстуры иконки (PNG) |
| `tooltip` | string | — | Всплывающая подсказка |
| `context_menu` | object | — | Контекстное меню трея |

**Динамическая смена иконки** — через binding в StateRegistry (меняет иконку в рантайме). Анимация — через смену текстур с таймером.

> **Платформенная поддержка:** Windows (Shell), macOS (NSStatusBar), Linux (libappindicator). Требуется крейт `tray-icon`. Функциональность доступна через фича-флаг `tray`.

---

## 4. Формат theme.json

### 4.1. Принцип

Тема описывает **внешний вид по умолчанию** для каждого типа виджета. Если в `ui.json` атрибут не указан — берётся из темы. Если и в теме нет — используется дефолт egui.

### 4.2. Структура

```json
{
  "Button": {
    "fill": "#303030",
    "rounding": 6,
    "min_width": 100,
    "text_color": "#FFFFFF"
  },
  "Label": {
    "color": "#CCCCCC",
    "size": 13
  },
  "TextField": {
    "width": 200,
    "height": 28
  },
  "ComboBox": {
    "width": 200
  },
  "Slider": {
    "width": 200
  },
  "Checkbox": {},
  "Separator": {
    "space": 6,
    "color": "#333333"
  },
  "Column": {
    "gap": 0,
    "padding": 0
  },
  "Row": {
    "gap": 0,
    "padding": 0
  },
  "Tabs": {
    "gap": 4
  },
  "Panel": {
    "fill": "#222222",
    "rounding": 8,
    "padding": 8
  },
  "Window": {
    "width": 400,
    "height": 300
  }
}
```

Поведение (on_click, on_hover) в тему **не выносится** — только внешний вид.

### 4.3. Кастомизация

Иерархия приоритетов (от высшего к низшему):

```
1. Атрибут в ui.json (конкретный элемент)
2. Атрибут в теме для этого типа (theme.json["Button"]["fill"])
3. Встроенный дефолт egui
```

Пример — три кнопки с разным цветом:

```json
// в ui.json
[
  { "type": "Button", "text": "ОК" },
  { "type": "Button", "text": "Опасная", "fill": "#CC3333" },
  { "type": "Button", "text": "Зелёная", "fill": "#00AA66" }
]

// в theme.json
{ "Button": { "fill": "#303030", "rounding": 6 } }
```

- «ОК» получит `fill: #303030` из темы
- «Опасная» получит `fill: #CC3333` (переопределил тему)
- «Зелёная» получит `fill: #00AA66` (переопределил тему)

### 4.4. Встроенные темы

Rust4ui поставляется с набором готовых тем в папке `themes/`:

| Файл | Описание |
|------|----------|
| `themes/dark.json` | Тёмная тема (нейтральные серые, белый текст) |
| `themes/light.json` | Светлая тема (белый фон, чёрный текст) |
| `themes/dj_green.json` | В стиле DJA (тёмный фон, зелёный акцент) |
| `themes/matrix.json` | Зелёный на чёрном, моноширинный |
| `themes/ocean.json` | Синие акценты на тёмном |

Разработчик может:
1. Использовать тему как есть
2. Загрузить тему из JSON и переопределить часть атрибутов
3. Написать свою тему с нуля

### 4.5. Hover / Focus / Disabled (автоматические реакции)

Тема определяет поведение виджетов в разных состояниях. Пользователь пишет только функциональный JSON — реакции даёт тема из коробки.

**`theme.json` — секции состояний:**

```json
{
  "Button": { "fill": "#303030", "rounding": 6 },

  "Hover": {
    "Button":    { "fill_brighten": 0.12, "stroke_brighten": 0.08 },
    "MenuItem":  { "fill_brighten": 0.08 },
    "IconButton": { "icon_brighten": 0.20 },
    "Hyperlink": { "underline": true },
    "ComboBox":  { "fill_brighten": 0.06 },
    "Checkbox":  { "fill_brighten": 0.04 }
  },
  "Focus": {
    "stroke_color": "#6699FF",
    "stroke_width": 2
  },
  "Disabled": {
    "opacity": 0.4,
    "fill_desaturate": true
  }
}
```

| Состояние | Что делает | Пример эффекта |
|-----------|-----------|---------------|
| **Hover** | Курсор над виджетом | Кнопка светлеет на 12%, рамка на 8% |
| **Focus** | Виджет в фокусе (Tab, клик) | Синяя обводка 2px |
| **Disabled** | `enabled: false` | Прозрачность 0.4 + обесцвечивание |

**Каждый виджет может иметь свои настройки в Hover.** Атрибуты состояний:

| Атрибут | Тип | Описание |
|---------|-----|----------|
| `fill_brighten` | float | Насколько осветлить фон (0.0 – 1.0) |
| `stroke_brighten` | float | Насколько осветлить обводку |
| `icon_brighten` | float | Для IconButton — осветление иконки |
| `underline` | bool | Подчеркнуть текст (Hyperlink) |
| `fill_desaturate` | bool | Обесцветить (Disabled) |

Отключить реакцию для конкретного виджета: `{ "type": "Button", "hover": false }`.

---

## 5. StateRegistry (привязка данных)

### 5.1. Принцип

StateRegistry — это `HashMap<String, Box<dyn Any>>`, где ключ = имя переменной (из `binding` в JSON), значение = данные.

Виджеты с `binding` читают значение по ключу и пишут обратно при изменении.

### 5.2. Как регистрировать

```rust
let mut state = rust4ui::State::new();

state.bind("mic_name", &mut app.mic_name);       // String
state.bind("volume", &mut app.volume);             // f64
state.bind("use_gpu", &mut app.use_gpu);           // bool
state.bind("mic_idx", &mut app.selected_mic);     // usize
state.bind("mic_list", &mut app.mic_list);         // Vec<String>
state.bind("cur_tab", &mut app.current_tab);       // String (для Tabs)
```

### 5.3. Как работает внутри

```rust
// Пример для TextField:
// В JSON: { "type": "TextField", "binding": "mic_name" }

fn render_textfield(ui, node, state) {
    let key = node["binding"].as_str().unwrap();
    let mut value = state.get_str(key).unwrap_or_default();
    
    if ui.text_edit_singleline(&mut value).changed() {
        state.set_str(key, &value);
    }
}
```

Каждый кадр: прочитал из state → показал в UI → если изменилось → записал обратно. Immediate mode.

---

## 6. ActionRegistry (привязка поведения)

### 6.1. Принцип

ActionRegistry — это `HashMap<&str, Box<dyn Fn(&mut ActionCtx)>>`, где ключ = имя действия из `action` в JSON.

### 6.2. Как регистрировать

```rust
let mut actions = rust4ui::Actions::new();

actions.insert("save", |ctx| {
    let app = ctx.app::<App>();
    app.config.save();
    ctx.show_feedback("Сохранено!");
});

actions.insert("cancel", |ctx| {
    let app = ctx.app::<App>();
    app.config.load();
    app.close_window();
});

actions.insert("open_dialog", |ctx| {
    let app = ctx.app::<App>();
    app.open_file_dialog();
});
```

### 6.3. ActionCtx

`ActionCtx` — контекст, который получает обработчик:

```rust
pub struct ActionCtx<'a> {
    pub state: &'a mut State,
    pub app: &'a mut dyn Any,  // ссылка на приложение
    pub target: Option<&'a str>,  // из поля "target" в JSON
    pub ui: &'a mut egui::Ui,
}
```

Методы:

| Метод | Описание |
|-------|----------|
| `ctx.app::<T>()` | Получить ссылку на приложение (downcast) |
| `ctx.state()` | Доступ к StateRegistry |
| `ctx.target()` | Доп. параметр из JSON |
| `ctx.show_toast(text)` | Показать всплывающее уведомление |

---

## 7. WidgetRegistry (расширение виджетов)

Позволяет регистрировать кастомные виджеты из сторонних крейтов или самого приложения. Построен по тому же принципу, что и ActionRegistry — строковый ключ → функция-обработчик.

### 7.1. Принцип

Рендерер диспатчит по полю `type` в JSON. Встроенные типы (`Button`, `Label`, ...) зарегистрированы в ядре. Сторонний крейт может добавить свои:

```json
// audio-plugin регистрирует "Spectrogram" и "Fader"
// Теперь в ui.json можно писать:
{ "type": "Spectrogram", "binding": "audio_buffer", "width": 600, "height": 200 }
{ "type": "Fader", "binding": "volume", "min": -60, "max": 12 }
```

Если `type` не найден ни во встроенных, ни в зарегистрированных — оранжевый блок `⚠ Неизвестный: <имя>` (без паники).

### 7.2. API

```rust
pub struct WidgetRegistry {
    handlers: HashMap<String, WidgetHandler>,
}

pub type WidgetHandler = Box<
    dyn Fn(&mut egui::Ui, &UiNode, &State, &Actions, &Theme) -> WidgetResponse
>;

pub struct WidgetResponse {
    pub changed: bool,
    pub clicked: bool,
}

impl WidgetRegistry {
    /// Зарегистрировать новый тип виджета
    pub fn register(&mut self, type_name: &str, handler: WidgetHandler);

    /// Отрендерить виджет по имени типа
    pub fn render(
        &self,
        type_name: &str,
        ui: &mut egui::Ui,
        node: &UiNode,
        state: &State,
        actions: &Actions,
        theme: &Theme,
    ) -> Result<WidgetResponse, UnknownWidgetError>;
}
```

### 7.3. Как регистрировать

**Макрос для удобства:**

```rust
use rust4ui::register_widgets;

register_widgets! {
    "Spectrogram" => |ui, node, state, actions, theme| {
        let data = state.get_vec_f64(node.binding());
        let width = node.float("width").unwrap_or(600.0);
        let height = node.float("height").unwrap_or(200.0);
        // рисуем спектрограмму через egui::Painter
        draw_spectrogram(ui, &data, width, height);
        WidgetResponse::none()
    },
    "Fader" => |ui, node, state, actions, theme| {
        let mut val = state.get_f64(node.binding()).unwrap_or(0.0);
        let min = node.float("min").unwrap_or(0.0);
        let max = node.float("max").unwrap_or(100.0);
        // рисуем вертикальный фейдер
        let resp = draw_fader(ui, &mut val, min, max);
        if resp.changed {
            state.set_f64(node.binding(), val);
        }
        WidgetResponse { changed: resp.changed, clicked: false }
    },
}
```

**Ручная регистрация:**

```rust
let mut widgets = WidgetRegistry::new();
widgets.register("Spectrogram", Box::new(|ui, node, state, actions, theme| {
    // ...
    WidgetResponse::none()
}));
```

### 7.4. Доступ к данным виджета

Обработчик получает полный контекст:

| Параметр | Что даёт |
|----------|---------|
| `ui: &mut egui::Ui` | Низкоуровневое рисование (Painter, allocate_rect, shapes) |
| `node: &UiNode` | Атрибуты из JSON (`node.float("width")`, `node.str("text")`, `node.children()`) |
| `state: &State` | Чтение/запись через binding (`state.get_f64("volume")`, `state.set_bool(...)`) |
| `actions: &Actions` | Вызов действий (`actions.invoke("save", &mut ctx)`) |
| `theme: &Theme` | Стили по умолчанию (`theme.get("Fader", "fill")`) |

### 7.5. Примеры кастомных виджетов

**Аудио-плагин:**
```json
// Spectrogram: спектрограмма аудио-буфера
{ "type": "Spectrogram", "binding": "audio_fft", "width": 600, "height": 200, "color": "#00FF66" }

// Fader: вертикальный фейдер громкости
{ "type": "Fader", "binding": "track_volume", "min": -60, "max": 12, "step": 0.5 }

// VUMeter: индикатор уровня
{ "type": "VUMeter", "binding": "peak_level", "channels": 2 }
```

**Математика / графика:**
```json
// Chart: график (line, bar, scatter)
{ "type": "Chart", "binding": "chart_data", "chart_type": "line", "width": 400, "height": 250 }

// Vector: отображение вектора
{ "type": "Vector", "binding": "vector_data", "scale": 1.0 }

// Gauge: круговой индикатор
{ "type": "Gauge", "binding": "progress", "min": 0, "max": 100, "radius": 80 }
```

**Видео / графика:**
```json
// Viewport: вьюпорт для 3D/2D рендера
{ "type": "Viewport", "binding": "render_target", "width": 800, "height": 600 }

// Image: изображение (встроенный, но можно заменить кастомным с zoom/pan)
{ "type": "ImageViewer", "src": "binding_image_path", "zoomable": true }
```

### 7.6. WidgetResponse

Каждый кастомный виджет возвращает структуру:

```rust
pub struct WidgetResponse {
    pub changed: bool,   // виджет изменил данные (нужно сохранить state)
    pub clicked: bool,   // был клик (если виджет кликабельный)
}
```

Если виджету нужно вернуть сложный ответ — он пишет данные в `StateRegistry` (через binding) или вызывает `action`.

---

## 8. Обработка ошибок

Rust4ui **никогда не паникует** при рендеринге. Любая ошибка в JSON отображается визуально:

| Проблема | Что рисуется |
|----------|-------------|
| Неизвестный `type` | Оранжевый блок с текстом `⚠ Неизвестный: <Padding>` |
| `binding` указан, но нет в StateRegistry | Красный блок `⚠ binding 'xyz' не найден` |
| `items` указан, но нет в StateRegistry | Красный блок `⚠ items 'xyz' не найден` |
| Поле неверного типа | Жёлтый блок `⚠ 'width' должно быть числом` |
| Ошибка парсинга JSON | Текст ошибки компилятора serde |

Всё это видно сразу при запуске. Никаких runtime-паник.

```rust
fn render_error(ui: &mut egui::Ui, msg: &str) {
    let rect = ui.allocate_exact_size(Vec2::new(200.0, 24.0), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect.0, Rounding::same(4.0), Color32::from_rgb(60, 30, 0));
    painter.text(
        rect.0.center(),
        Align2::CENTER_CENTER,
        format!("⚠ {msg}"),
        FontId::monospace(11.0),
        Color32::from_rgb(255, 180, 0),
    );
}
```

---

## 9. Code Generator (JSON → Rust)

Модуль `gen`. Проходит по тому же JSON, что и Renderer, но вместо вызова egui пишет файл с Rust-кодом (прямые egui-вызовы).

```bash
rust4ui-codegen ui.json --output ui.rs
```

На выходе:

```rust
// ui.rs — сгенерировано rust4ui-codegen
fn render_ui(ui: &mut egui::Ui, state: &mut State, actions: &Actions) {
    ui.vertical(|ui| {
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Настройки").size(16.0).bold());
        ui.separator();
        let mut val = state.get_str("mic_name").unwrap_or_default();
        ui.text_edit_singleline(&mut val);
        state.set_str("mic_name", &val);
    });
}
```

Генератор доступен с v0.1. Зависимости: `proc-macro2` + `quote` для генерации кода.

---

## 10. API — как это выглядит в коде

### 10.1. Полный пример

```rust
use rust4ui::prelude::*;

struct SettingsApp {
    mic_name: String,
    volume: f64,
    use_gpu: bool,
    current_lang: String,   // ← язык
}

impl SettingsApp {
    fn save(&mut self) {
        println!("сохранено: mic={}, vol={}", self.mic_name, self.volume);
    }

    fn switch_lang(&mut self, lang: &str) {
        self.current_lang = lang.to_string();
    }
}

fn main() -> eframe::Result<()> {
    let ui_json = serde_json::from_str(include_str!("../ui.json")).unwrap();
    let theme = Theme::from_file("theme.json").unwrap_or_default();

    // Загружаем локали
    let mut locales = LocaleRegistry::new("ru");
    locales.load(include_str!("../locales/ru.json"));
    locales.load(include_str!("../locales/en.json"));

    let mut app = SettingsApp {
        mic_name: "USB Mic".into(),
        volume: 0.8,
        use_gpu: true,
        current_lang: "ru".into(),
    };

    let mut state = State::new();
    state.bind("mic_name",       &mut app.mic_name);
    state.bind("volume",         &mut app.volume);
    state.bind("use_gpu",        &mut app.use_gpu);
    state.bind("active_locale",  &mut app.current_lang);  // ← для переключения из UI

    let mut actions = Actions::new();
    actions.insert("save", |ctx| ctx.app::<SettingsApp>().save());
    actions.insert("switch_lang", |ctx| {
        let lang = ctx.target().unwrap_or("en");
        ctx.app::<SettingsApp>().switch_lang(lang);
    });

    eframe::run_native(
        "Rust4ui Demo",
        Default::default(),
        Box::new(|_cc| {
            Box::new(Rust4ui::new(ui_json, theme, locales, state, actions))
        }),
    )
}
```

### 10.2. Rust4ui struct

```rust
pub struct Rust4ui {
    ui_json: UiNode,
    theme: Theme,
    locales: LocaleRegistry,
    state: State,
    actions: Actions,
}

impl Rust4ui {
    /// Создать из JSON (runtime-режим)
    pub fn new(
        ui_json: UiNode,
        theme: Theme,
        locales: LocaleRegistry,
        state: State,
        actions: Actions,
    ) -> Self;
}

impl eframe::App for Rust4ui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
}
```

См. полный пример в секции 10.1.

---

## 11. Зависимости

```toml
[dependencies]
egui = "0.32"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Опционально:
# gen = ["proc-macro2", "quote", "syn"]  — для code generator
```

---

## 12. Лицензия

MIT.

---

## 13. Интернационализация (i18n)

Rust4ui поддерживает многоязычные интерфейсы с первого дня. Система построена на трёх принципах:

1. **Data-driven переводы** — все строки хранятся в JSON, не в коде
2. **Runtime-переключение** — язык меняется без перекомпиляции
3. **Единая интерполяция** — `{param}` внутри строк перевода резолвятся из StateRegistry

### 13.1. Формат locale-файлов

Файлы лежат в директории `locales/`, по одному на язык:

```json
{
  "locale": "ru",
  "name": "Русский",
  "translations": {
    "app.title":            "Rust4ui",
    "settings.title":       "Настройки",
    "settings.save":        "Сохранить",
    "settings.cancel":      "Отмена",
    "settings.language":    "Язык",
    "btn.start":            "Старт",
    "btn.stop":             "Стоп",
    "btn.save":             "Сохранить",
    "label.status":         "Статус: {status_text}",
    "label.volume":         "Громкость: {volume}%",
    "files.count": {
      "one":  "{count} файл",
      "few":  "{count} файла",
      "many": "{count} файлов"
    },
    "tab.main":             "Основные",
    "tab.audio":            "Аудио",
    "tab.about":            "О программе"
  }
}
```

| Поле | Тип | Описание |
|------|-----|----------|
| `locale` | string | Код языка (`ru`, `en`, `de`...) |
| `name` | string | Отображаемое название (для UI переключателя) |
| `translations` | object | Плоский объект с переводами (ключ → строка) |

### 13.2. Ключи перевода

Ключи используют dot-нотацию для группировки:

```
app.title          — заголовок приложения
settings.title     — заголовок вкладки настроек
settings.save      — кнопка сохранения
label.status       — строка статуса (с интерполяцией)
files.count        — счётчик файлов (плюрализация)
tab.main           — название вкладки
```

Максимальная вложенность — 2 уровня (группа + имя). Глубже не рекомендуется.

### 13.3. Ссылки в ui.json (`{{key}}`)

ui.json ссылается на переводы через синтаксис `{{key}}`:

```json
{
  "type": "Tabs",
  "active": "main",
  "children": [
    { "type": "Tab", "id": "main",  "title": "{{tab.main}}",  "children": [...] },
    { "type": "Tab", "id": "audio", "title": "{{tab.audio}}", "children": [...] },
    { "type": "Tab", "id": "about", "title": "{{tab.about}}", "children": [...] }
  ]
}
```

Поддерживается во всех текстовых атрибутах: `text`, `title`, `hint`, `tooltip`, `placeholder`.

### 13.4. Pipeline разрешения

Каждый текстовый атрибут проходит два прохода рендеринга:

```
"{{label.status}}"
  │
  ├── Проход 1: LocaleRegistry
  │     └── ищет "label.status" в активной локали
  │     └── получает "Статус: {status_text}"
  │
  ├── Проход 2: StateRegistry
  │     └── резолвит {status_text} → "Работаю"
  │     └── получает "Статус: Работаю"
  │
  └── Результат: ui.label("Статус: Работаю")
```

```
"{{btn.save}}"
  │
  ├── Проход 1: "btn.save" → LocaleRegistry → "Сохранить"
  │     (нет {expr} — второй проход пропускается)
  │
  └── Результат: ui.button("Сохранить")
```

Если значение не является `{{key}}` — используется как есть (без i18n). Если ключ не найден — отображается `{{key}}` оранжевым цветом.

### 13.5. Плюрализация

Когда перевод — объект (не строка), он считается plural-картой:

```json
{
  "files.count": {
    "one":  "{count} файл",
    "few":  "{count} файла",
    "many": "{count} файлов"
  }
}
```

**Как работает:**
1. Рендерер встречает `"text": "{{files.count}}"` и получает plural-объект
2. Система находит все `{param}` внутри форм — в данном случае `{count}`
3. Читает `count` из StateRegistry
4. Определяет plural-категорию для текущей локали (по CLDR-правилам)
5. Выбирает соответствующую форму и подставляет число

**Встроенные plural-правила (CLDR):**

| Локаль | Категории | Пример (count=1,2,5) |
|--------|-----------|---------------------|
| `ru`, `uk`, `be` | one / few / many / other | 1 файл / 2 файла / 5 файлов |
| `en`, `de`, `fr` | one / other | 1 file / 2 files |
| `pl` | one / few / many / other | 1 plik / 2 pliki / 5 plików |
| `ja`, `zh`, `ko` | other | 1 ファイル / 2 ファイル |

Если для нужной формы нет ключа — используется `other` (или первая доступная форма).

### 13.6. LocaleRegistry API

```rust
pub struct LocaleRegistry { /* .. */ }

impl LocaleRegistry {
    /// Создать реестр с указанным языком по умолчанию
    pub fn new(default_locale: &str) -> Self;

    /// Загрузить locale из JSON-строки
    pub fn load(&mut self, json: &str);

    /// Переключить активный язык
    pub fn switch(&mut self, code: &str);

    /// Получить активный код языка
    pub fn current(&self) -> &str;

    /// Резолв ключа с интерполяцией из state
    pub fn resolve(&self, key: &str, state: &State) -> Option<String>;

    /// Резолв ключа без интерполяции (только перевод)
    pub fn resolve_raw(&self, key: &str) -> Option<&str>;
}
```

**Пример использования:**

```rust
let mut locales = LocaleRegistry::new("ru");
locales.load(include_str!("../locales/ru.json"));
locales.load(include_str!("../locales/en.json"));

// В рантайме:
locales.switch("en");
assert_eq!(locales.resolve_raw("btn.save"), Some("Save"));

// С интерполяцией:
// state.get_str("status_text") == "Running"
assert_eq!(
    locales.resolve("label.status", &state),
    Some("Status: Running")
);
```

### 13.7. Переключение языка в рантайме

Язык переключается через binding в StateRegistry:

```json
{
  "type": "ComboBox",
  "binding": "active_locale",
  "items": "locale_list"
}
```

```rust
// В Rust:
state.bind("active_locale", &mut app.current_lang);
state.bind("locale_list", &mut app.locale_options);  // ["Русский", "English"]

// LocaleRegistry следит за active_locale каждый кадр
```

Когда пользователь выбирает язык в ComboBox:
1. `app.current_lang` меняется на `"en"`
2. LocaleRegistry замечает изменение и вызывает `self.switch("en")`
3. На следующем кадре все `{{key}}` резолвятся из английской локали
4. Никакого перезапуска приложения

### 13.8. Fallback-цепочка

```
Ищем "settings.save" в текущей локали (ru)
  → есть → возвращаем "Сохранить"

Ищем "settings.save" в текущей локали (de)
  → нет → ищем в en (дефолтная)
    → есть → возвращаем "Save"

Ищем "new.feature" в en
  → нет → возвращаем "{{new.feature}}" (заглушка)
```

Заглушка отображается оранжевым блоком, чтобы разработчик сразу видел пропущенные ключи.

### 13.9. Пример locale-файлов

**`locales/ru.json` (дефолтная):**
```json
{
  "locale": "ru",
  "name": "Русский",
  "translations": {
    "app.title":          "Rust4ui",
    "settings.title":     "Настройки",
    "settings.save":      "Сохранить",
    "settings.cancel":    "Отмена",
    "settings.language":  "Язык",
    "btn.start":          "Старт",
    "btn.stop":           "Стоп",
    "btn.save":           "Сохранить",
    "tab.main":           "Основные",
    "tab.audio":          "Аудио",
    "tab.about":          "О программе",
    "label.status":       "Статус: {status_text}",
    "label.volume":       "Громкость: {volume}%",
    "files.count": {
      "one":  "{count} файл",
      "few":  "{count} файла",
      "many": "{count} файлов"
    }
  }
}
```

**`locales/en.json` (fallback):**
```json
{
  "locale": "en",
  "name": "English",
  "translations": {
    "app.title":          "Rust4ui",
    "settings.title":     "Settings",
    "settings.save":      "Save",
    "settings.cancel":    "Cancel",
    "settings.language":  "Language",
    "btn.start":          "Start",
    "btn.stop":           "Stop",
    "btn.save":           "Save",
    "tab.main":           "Main",
    "tab.audio":          "Audio",
    "tab.about":          "About",
    "label.status":       "Status: {status_text}",
    "label.volume":       "Volume: {volume}%",
    "files.count": {
      "one":  "{count} file",
      "other": "{count} files"
    }
  }
}
```

---

## 14. Ограничения (известные)

1. **Только egui** — не абстрагирован под другие бэкенды. Если появится спрос — можно вынести ядро в `rust4ui_core`.
2. **Binding только по имени** — без вложенных путей (`"settings.mic.name"`). В будущем можно добавить.
3. **Нет анимаций** — egui сам не поддерживает, а транслировать из JSON бессмысленно.
4. **ComboBox требует Vec<String> в state** — кастомные типы через downcast в будущем.
5. **Code Generator — только для статичных UI** — если UI динамически меняется в рантайме, генерация бессмысленна.

---

## 15. Дизайн-словарь и история

При проектировании Rust4ui использовался устоявшийся дизайн-словарь имён виджетов, общий с проектами на egui. Проект [EFx](https://github.com/ZhukMax/efx) послужил сторонним источником вдохновения для этого словаря. Технической зависимости от EFx нет — EFx не имеет отношения к Rust4ui.

### 15.1. Где пересечения

Имена тегов (`Column`, `Row`, `Label`, `Button`, `Tabs`, `Window` и др.) и базовых атрибутов (`gap`, `padding`, `fill`, `rounding`) сознательно совпадают с EFx — это делает переход между проектами интуитивно понятным. Но Rust4ui идёт значительно дальше:

- **36+ виджетов** против ~20 в EFx (RadioGroup, ColorPicker, FileDrop, IconBar, StatusBar, MenuBar, System Tray, Spinner, Shortcut, Notifications, Indicator, Caption — всё уникально для Rust4ui)
- **Темы с Hover/Focus/Disabled** — у EFx темы отсутствуют
- **i18n с CLDR plural** — нет в EFx
- **StateRegistry + ActionRegistry** — собственный механизм, не связанный с EFx
- **WidgetRegistry для кастомных виджетов** — уникальная фича
- **$ref для модульных JSON** — уникальная фича
- **Per-corner rounding, CSS-shorthand отступы** — расширенная кастомизация

### 15.2. Общий словарь тегов

| Тег | Rust4ui | EFx | Примечание |
|-----|---------|-----|-----------|
| `Column` | ✅ | ✅ | |
| `Row` | ✅ | ✅ | |
| `Label` | ✅ | ✅ | |
| `Button` | ✅ | ✅ | |
| `TextField` | ✅ | ✅ | Rust4ui: + mode=number |
| `Checkbox` | ✅ | ✅ | |
| `Slider` | ✅ | ✅ | |
| `ComboBox` | ✅ | ✅ | |
| `Separator` | ✅ | ✅ | |
| `Tabs` / `Tab` | ✅ | ✅ | |
| `Panel` | ✅ | ✅ | |
| `ScrollArea` | ✅ | ✅ | |
| `Window` | ✅ | ✅ | Rust4ui: + modal, anchor, id-persist |
| `Hyperlink` | ✅ | ✅ | |
| `Image` | ✅ | ✅ | |
| `Grid` | ✅ | ✅ | |
| `RadioGroup` | ✅ | ❌ | Уникально для Rust4ui |
| `Spinner` | ✅ | ❌ | |
| `Shortcut` | ✅ | ❌ | |
| `ColorPicker` | ✅ | ❌ | |
| `MenuItem` / `SubMenu` | ✅ | ❌ | |
| `MenuBar` / `Menu` | ✅ | ❌ | |
| `Notifications` | ✅ | ❌ | |
| `Tray` | ✅ | ❌ | |
| `FileDrop` | ✅ | ❌ | |
| `IconBar` | ✅ | ❌ | |
| `IconButton` | ✅ | ❌ | |
| `Caption` | ✅ | ❌ | |
| `Indicator` | ✅ | ❌ | |
| `StatusBar` | ✅ | ❌ | |

---

## 16. Демо-файлы (демонстрационный прототип)

В директории `demo/` лежит эталонный прототип, включающий **все виджеты**. Используется как тестовый полигон при разработке ядра и как референс для изучения формата `ui.json`.

### 16.1. Структура

```
demo/
├── ui.json                     # 11 строк — Column с 3 $ref на tabs и windows
├── theme.json                  # дефолтная тёмная тема
├── tabs/
│   ├── all.json                # 11 строк — Tabs с 5 $ref на вкладки
│   ├── basic.json              # 176 строк — Tab «Базовые»
│   ├── input.json              # 107 строк — Tab «Ввод»
│   ├── containers.json         # 238 строк — Tab «Контейнеры»
│   ├── windows.json            # 144 строки — Tab «Окна»
│   └── i18n.json               # 84 строки — Tab «Язык»
├── windows/
│   ├── info_window.json        # 66 строк — информационное Window
│   ├── confirm_dialog.json     # 50 строк — модальный диалог
│   └── custom_panel.json       # 68 строк — кастомное окно
└── themes/
    ├── light.json              # светлая тема
    └── dj_green.json           # DJA-стиль
```

Файлы связаны через `$ref` (см. секцию 3.7). Каждый файл самодостаточен — открыл, отредактировал, запустил. Рендерер при загрузке рекурсивно резолвит все ссылки в единое дерево.

### 16.2. ui.json — модульная структура

UI разбит на файлы через систему `$ref` (секция 3.7). Корневой `ui.json` — всего 11 строк, ссылается на вынесенные модули:

```
ui.json (11 строк) ──$ref──→ tabs/all.json ──$ref──→ basic.json, input.json, ...
                    ──$ref──→ windows/info_window.json
                    ──$ref──→ windows/confirm_dialog.json
                    ──$ref──→ windows/custom_panel.json
```

Контент по вкладкам:

| Вкладка | Ключ | Содержание |
|---------|------|-----------|
| **1. Базовые компоненты** | `tab.basic` | Label (обычный, цветной, bold, italic, monospace), Button (primary, danger, success, disabled), Checkbox (×2), TextField (singleline + hint), Separator |
| **2. Элементы ввода** | `tab.input` | ComboBox (выбор устройства), Slider (громкость 0–100), TextField (multiline), статусная строка с интерполяцией `{volume}` |
| **3. Контейнеры** | `tab.containers` | ScrollArea, Panel (карточка с рамкой/фоном), Row (wrap=true — 10 кнопок-тегов в строку), Grid (таблица 3×4) |
| **4. Окна и навигация** | `tab.windows` | Hyperlink (github), FileDrop (drag-and-drop файлов из ОС), три кнопки: «Открыть окно» (информационное Window с id), «Открыть диалог» (модальный Window), «Без заголовка» (Window с title_bar=false и кастомной кнопкой закрытия) |
| **5. i18n / Язык** | `tab.i18n` | ComboBox переключения языка (ru/en), Label с `{{key}}`-ссылками, plural-демонстрация (`{count} файл/файла/файлов`) |

### 16.3. State bindings

Для работы демо-файла приложение должно зарегистрировать следующие bindings:

| Binding | Тип | Начальное | Назначение |
|---------|-----|-----------|-----------|
| `use_gpu` | `bool` | `true` | Checkbox GPU |
| `auto_start` | `bool` | `false` | Checkbox автозапуска |
| `name` | `String` | `"Rust4ui"` | TextField имя |
| `selected_device` | `usize` | `0` | ComboBox устройство |
| `device_list` | `Vec<String>` | `["USB Mic", "Line In", "Stereo Mix"]` | Список для ComboBox |
| `volume` | `f64` | `75.0` | Slider громкости |
| `font_size` | `f64` | `16.0` | TextField mode=number, размер шрифта |
| `description` | `String` | `""` | Multiline TextField |
| `active_locale` | `String` | `"ru"` | Текущий язык |
| `locale_list` | `Vec<String>` | `["Русский", "English"]` | Список языков |
| `count` | `f64` | `5.0` | Счётчик для plural |
| `show_info_window` | `bool` | `false` | Видимость Window (информационное) |
| `show_dialog` | `bool` | `false` | Видимость модального диалога |
| `show_custom_window` | `bool` | `false` | Видимость кастомного окна (без заголовка) |
| `greeting_text` | `String` | `"Мир"` | Текст приветствия |
| `status_text` | `String` | `"Готов"` | Статусная строка |

### 16.4. Actions

| Action | Что делает |
|--------|-----------|
| `click` | `ctx.show_toast("Нажата кнопка: " + ctx.target())` |
| `greeting` | `ctx.show_toast("Привет, " + state.get("name"))` |
| `apply` | `ctx.show_toast("Применено!")` |
| `reset` | Сброс `volume=50`, `use_gpu=true`, `description=""` в state |
| `toggle_window` | `show_info_window = !show_info_window` |
| `toggle_dialog` | `show_dialog = !show_dialog` |
| `toggle_custom_window` | `show_custom_window = !show_custom_window` |
| `confirm_ok` | `ctx.show_toast("Подтверждено!"); show_dialog = false` |
| `confirm_cancel` | `show_dialog = false` |
| `file_dropped` | `ctx.show_toast("Получены файлы: " + ctx.target())` |

### 16.5. Тема по умолчанию (`demo/theme.json`)

Тёмная минималистичная тема:

| Виджет | Атрибут | Значение |
|--------|---------|----------|
| Button | fill | `#303030` |
| Button | rounding | `6` |
| Button | min_width | `100` |
| Label | color | `#CCCCCC` |
| Label | size | `13` |
| TextField | width | `200` |
| Panel | fill | `#1A1D23` |
| Panel | rounding | `8` |
| Panel | padding | `12` |
| Panel | stroke_width | `1` |
| Panel | stroke_color | `#333333` |

### 16.6. Дополнительные темы

| Тема | Фон | Акцент | Текст |
|------|-----|--------|-------|
| `light.json` | `#FAFAFA` | `#3366CC` | `#222222` |
| `dj_green.json` | `#0A0A0A` | `#00FF66` | `#E0E0E0` |

### 16.7. Переводы для демки

Переводы входят в основные locale-файлы (`locales/ru.json`, `locales/en.json`). Список ключей:

```
tab.basic, tab.input, tab.containers, tab.windows, tab.i18n
label.basic, label.input, label.containers, label.windows, label.i18n
label.simple_text, label.colored_text, label.mono, label.italic, label.status
label.combo_title, label.volume, label.multiline, label.number_field, label.font_size
label.panel_title, label.panel_desc, label.row_wrap, label.grid
label.open_win, label.select_lang, label.greeting, label.files_count
label.file_drop
btn.primary, btn.danger, btn.success, btn.disabled, btn.greeting
btn.item, btn.apply, btn.reset, btn.open_window, btn.open_dialog, btn.open_custom, btn.close, btn.ok
chk.gpu, chk.auto_start
hint.name, hint.description
window.info, custom.title, custom.desc, custom.note
dialog.confirm, dialog.message
drop.hint
radio.dark, radio.light, radio.auto
menu.file, menu.edit, menu.view, menu.help
menu.new, menu.open, menu.exit, menu.undo, menu.redo
menu.auto, menu.export, menu.sidebar, menu.about
label.loading, tip.save_title, tip.save_desc
notify.saved, notify.error
tray.show, tray.monitoring, tray.status, tray.online, tray.away, tray.exit
grid.val_1, grid.val_2, grid.val_3, grid.val_4
```

---

## 17. Тестирование и контроль качества

Тестирование выстроено в шесть уровней — от статической валидации данных до интеграционной проверки всего UI. Каждый уровень покрывает конкретное правило из `AGENTS.md`.

### 17.1. Структура тестов

```
tests/
├── validation.rs          # 1. Pre-flight: валидация JSON при загрузке
├── subsystems/
│   ├── state.rs           # 2a. StateRegistry: read/write всех типов
│   ├── actions.rs         # 2b. ActionRegistry: вызов, ctx.target()
│   ├── theme.rs           # 2c. Theme: приоритет ui.json > theme > дефолт
│   ├── locale.rs          # 2d. LocaleRegistry: резолв, fallback en, missing→{{key}}
│   └── plural.rs          # 2e. Plural-правила: ru/en/pl/uk, проверка чисел 0–199
├── integration.rs         # 3. Интеграция: demo/ui.json (все 5 вкладок + Window)
├── persistence.rs         # 4. Автосохранение: save → reload → restore
├── encoding.rs            # 5. UTF-8 без BOM: проверка всех файлов проекта
└── smoke.rs               # 6. Smoke: каждый виджет изолированно, без паники
```

### 17.2. Уровень 1 — Pre-flight валидация

Запускается при старте приложения, до рендеринга. Проверяет загруженный `ui.json`:

| Проверка | Что ищет | При ошибке |
|----------|---------|------------|
| `type` известен | Все узлы ссылаются на зарегистрированные виджеты | `⚠ Неизвестный тип: <Foo>` |
| `binding` в state | Каждый binding зарегистрирован в StateRegistry | `⚠ binding 'xyz' не найден` |
| `action` зарегистрирован | Каждый action есть в ActionRegistry | `⚠ action 'save' не зарегистрирован` |
| `items` в state | ComboBox-списки зарегистрированы | `⚠ items 'xyz' не найден` |
| `{{key}}` в locale | Каждый i18n-ключ есть хотя бы в fallback-локали (en) | `⚠ ключ 'foo' отсутствует в локалях` |
| Типы атрибутов | `width`, `gap` — числа, `enabled` — bool | `⚠ 'width' должно быть числом` |

Валидатор возвращает `Vec<ValidationError>` с уровнем (Error / Warning). При критических ошибках UI не рендерится, показывает отчёт.

### 17.3. Уровень 2 — Unit-тесты подсистем

| Подсистема | Покрытие AGENTS.md | Пример теста |
|-----------|-------------------|-------------|
| **Theme** | единый источник стилей | `assert_eq!(resolve("fill", ui_node, theme), "#CC3333")` |
| **StateRegistry** | автосохранение данных | `state.set_f64("v", 42.0); assert_eq!(val, 42.0)` |
| **ActionRegistry** | обработка событий | `action("save", target="file") → ctx.target() == "file"` |
| **LocaleRegistry** | многоязычность | `resolve("missing.key", state) == Some("{{missing.key}}")` |
| **Plural rules** | склонения (яйцо/яйца/яиц) | `plural_form("ru", 2) == "few"`, `plural_form("en", 2) == "other"` |

Все тесты изолированы, не требуют egui-контекста. Запускаются через `cargo test --lib`.

### 17.4. Уровень 3 — Интеграционный тест (demo/ui.json)

Главный предохранитель. Один тест, который:

- Загружает `demo/ui.json`
- Регистрирует все 13 state-bindings и 5 actions
- Загружает обе локали
- Рендерит весь UI (все 5 вкладок, Window) через `egui::Ui`
- Проверяет, что ни один виджет не упал в error-заглушку
- Меняет state → проверяет обновление UI
- Переключает язык → проверяет смену переводов

**Критерий:** тест зелёный = проект работает как задумано. Красный = что-то сломалось.

### 17.5. Уровень 4 — Persistence (автосохранение)

Проверяет правило **«Все настройки автосохраняются при изменении и восстанавливаются после перезагрузки»**:

- Создаёт `StateRegistry` с несколькими bindings
- Меняет значения через виджеты
- «Сохраняет» (сериализует state в JSON)
- «Перезагружает» (создаёт новый StateRegistry, десериализует)
- Сравнивает: все значения восстановились

### 17.6. Уровень 5 — Encoding (UTF-8 без BOM)

Проверяет правило **«Весь обмен данными — UTF-8 без BOM»**:

- Проходит по всем файлам проекта (`.rs`, `.json`, `.toml`, `.md`, `.bat`, `.py`)
- Читает первые 3 байта — проверяет отсутствие BOM (`EF BB BF`)
- Валидирует содержимое как UTF-8
- При ошибке — имя файла + смещение проблемного байта

### 17.7. Уровень 6 — Smoke (каждый виджет изолированно)

Для каждого типа виджета рендерится минимально возможный JSON-узел:

```json
{ "type": "Button", "text": "Test" }
{ "type": "Slider", "binding": "test_val", "min": 0, "max": 10 }
// ... для каждого из 13+ типов
```

Критерий: ни один вызов не вызывает panic/unwrap-падение. Ошибки атрибутов — ожидаемы (оранжевые заглушки), паника — нет.

### 17.8. Cargo-алиасы

```toml
# .cargo/config.toml
[alias]
test-all = "test --lib --test validation --test integration --test persistence --test encoding --test smoke"
lint-files = "run --bin lint-file-sizes"   # проверка лимита 250 строк
lint-utf8  = "run --bin lint-utf8"         # проверка UTF-8 без BOM
```
