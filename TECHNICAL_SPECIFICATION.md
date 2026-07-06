# Rust4ui — Техническое задание

> **Rust4ui** (Rust for UI) — комбайн для быстрого прототипирования и сборки UI на базе `egui`. JSON / EFx → живой интерфейс. Без перекомпиляции. С возможностью «запечь» в Rust-код через `efx!()`.

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

Rust4ui использует **единый словарь виджетов с проектом [EFx](https://github.com/ZhukMax/efx)**. Каждый `type` в JSON (`"Button"`, `"Column"`, `"Tabs"`, ...) соответствует EFx-тегу (`<Button>`, `<Column>`, `<Tabs>`), атрибуты — EFx-атрибутам. Один и тот же UI можно:

- **Запустить как JSON** (runtime) — без компиляции
- **Скомпилировать через `efx!()`** (compile-time) — максимальная производительность
- **Миксовать** — сложные статичные части на `efx!()`, динамические — через JSON

Разработчик UI собирает интерфейс как конструктор:
1. Описывает виджеты в `ui.json` (или в `efx!()`-шаблоне)
2. Назначает им цвета/отступы через `theme.json`
3. В Rust-коде регистрирует функции-обработчики (actions) и привязки к данным (state)

Никакой компиляции между изменениями JSON. Только перезапуск приложения — или live-reload через file watcher. При необходимости UI «запекается» в `efx!()`-код без смены API.

### 1.3. Целевая аудитория

- Разработчики на Rust, которые хотят быстро прототипировать UI без `cargo build` на каждый чих
- Команды, где дизайнер правит `theme.json`, а разработчик — только логику
- Авторы тулов, админок, настроек — где UI не megasupercomplex, но должен выглядеть прилично

---

## 2. Архитектура

### 2.1. Схема работы

```
                        ┌─────────────────────┐
                        │     ui.json /        │
                        │  efx!()-шаблон       │
                        │  (EFx-семантика)     │
                        └──────────┬───────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    ▼              ▼              ▼
          ┌──────────────┐ ┌────────────┐ ┌──────────────┐
          │  Runtime     │ │  Codegen   │ │  from_efx()  │
          │  Renderer    │ │  JSON→efx  │ │  efx!(...)   │
          │  (обход JSON)│ │  .rs файл  │ │  compile-time│
          └──────┬───────┘ └─────┬──────┘ └──────┬───────┘
                 │               │               │
                 └───────────────┼───────────────┘
                                 ▼
                    ┌───────────────────────┐
                    │  EFx-семантика        │
                    │  (Column, Row, Label,  │
                    │   Button, Tabs, ...)   │
                    └───────────┬───────────┘
                                ▼
                    ┌───────────────────────┐
                    │  egui (окно)          │
                    │  + Theme + State      │
                    │  + Actions + Locales  │
                    └───────────────────────┘
```

**Три пути рендеринга:**

| Путь | Описание | Когда использовать |
|------|----------|-------------------|
| **Runtime** | JSON → Renderer → egui | Прототипирование, частые правки UI |
| **Codegen** | JSON → `efx!()` → egui | Релиз, статичный UI, макс. скорость |
| **from_efx()** | `efx!()`-шаблон → runtime | Гибрид: часть UI статична, часть динамична |

Каждый кадр egui вызывает `update()`. Renderer проходит по JSON-дереву и для каждого узла:

1. Определяет `type` узла (`"Button"`, `"Column"`, ...) — **соответствует EFx-тегу**
2. Берёт дефолтные атрибуты из темы
3. Накладывает атрибуты из `ui.json` (они выше приоритетом)
4. Рисует виджет через egui (по тем же правилам, что и EFx)
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
| `align` | string | `"center"` | Выравнивание по вертикали: `"top"`, `"center"`, `"bottom"` |
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

Всплывающее окно поверх основного.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"Window"` |
| `title` | string | — | Заголовок окна |
| `open` | string | — | Binding на bool — открыто/закрыто |
| `width` | float | 400 | Ширина |
| `height` | float | 300 | Высота |
| `resizable` | bool | true | Можно ли менять размер |
| `children` | array | `[]` | Дочерние элементы |

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
| `rounding` | float | из темы | Скругление |
| `min_width` | float | из темы | Минимальная ширина |
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

Поле ввода текста.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | — | `"TextField"` |
| `binding` | string | — | Ключ в StateRegistry (строка) |
| `hint` | string | `""` | Подсказка в пустом поле |
| `width` | float | из темы | Ширина |
| `multiline` | bool | false | Многострочный |
| `password` | bool | false | Маскировать ввод |

```json
{
  "type": "TextField",
  "binding": "mic_name",
  "hint": "Название микрофона",
  "width": 250
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

### 3.4. Зарезервированные поля

- `type` — имя виджета (обязательное)
- `children` — массив дочерних узлов
- `binding` — привязка к StateRegistry
- `items` — привязка к StateRegistry для списка
- `action` — имя действия в ActionRegistry
- `target` — параметр действия

Все остальные поля — атрибуты стиля (цвет, размер, ширина и т.д.)

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

### 3.6. Совместимость с EFx

Формат `ui.json` полностью совместим с проектом [EFx](https://github.com/ZhukMax/efx). Каждый `type` в JSON соответствует EFx-тегу, атрибуты — EFx-атрибутам:

```json
// ui.json (Rust4ui)
{ "type": "Column", "gap": 8, "padding": 6, "children": [
    { "type": "Label", "text": "{{settings.title}}", "size": 16, "bold": true }
]}

// EFx (тот же UI)
// <Column gap="8" padding="6">
//     <Label size="16" bold="true">{{settings.title}}</Label>
// </Column>
```

**Что это даёт:**
- Можно писать UI хоть в JSON, хоть в `efx!()` — результат одинаковый
- Code Generator конвертит JSON в `efx!()`-код без потери семантики
- Разработчик может начать с JSON (быстрое прототипирование), а на релизе «запечь» в EFx (максимальная скорость)
- Один и тот же набор виджетов: Column, Row, Label, Button, TextField, Checkbox, Slider, ComboBox, Separator, Tabs/Tab, Panel, Window, ScrollArea, Hyperlink, Image, Grid

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

## 7. Обработка ошибок

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

## 8. Code Generator (JSON → EFx / Rust)

Модуль `gen`. Проходит по тому же JSON, что и Renderer, но вместо вызова egui пишет Rust-файл с `efx!()`-вызовами (режим по умолчанию) или raw egui-код (опционально).

```bash
# По умолчанию — генерация efx!()-кода
rust4ui-codegen ui.json --output ui.rs

# Raw egui (без зависимости от efx)
rust4ui-codegen ui.json --output ui.rs --format raw
```

**Режим по умолчанию (EFx):**

```rust
// ui.rs — сгенерировано rust4ui-codegen
use efx::efx;

fn render_ui(ui: &mut egui::Ui, state: &mut State, actions: &Actions) {
    efx!(ui, r#"
        <Column gap="8" padding="6">
            <Label size="16" bold="true">Настройки микрофона</Label>
            <Separator/>
            <TextField value="{state.get_str("mic_name")}" hint="Название микрофона" width="250"/>
            <Slider binding="volume" min="0" max="100" text="Громкость"/>
        </Column>
    "#);
}
```

**Режим raw (без EFx):**

```rust
// ui.rs — сгенерировано rust4ui-codegen (raw)
fn render_ui(ui: &mut egui::Ui, state: &mut State, actions: &Actions) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Настройки микрофона").size(16.0).bold());
        ui.separator();
        let mut val = state.get_str("mic_name").unwrap_or_default();
        ui.text_edit_singleline(&mut val);
        state.set_str("mic_name", &val);
    });
}
```

Генератор доступен с v0.1. Зависимости: `efx` (core, без макроса) для разбора EFx-семантики, `proc-macro2` + `quote` для генерации кода.

---

## 9. API — как это выглядит в коде

### 9.1. Полный пример

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

### 9.2. Rust4ui struct

```rust
pub struct Rust4ui {
    ui_json: Option<UiNode>,        // JSON-дерево (runtime)
    efx_template: Option<String>,   // EFx-шаблон (from_efx)
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

    /// Создать из EFx-шаблона (compile-time режим)
    /// Шаблон разбирается через efx-core в рантайме
    pub fn from_efx(
        template: &str,
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

---

## 10. Зависимости

```toml
[dependencies]
egui = "0.32"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Опционально:
# gen = ["proc-macro2", "quote", "syn"]  — для code generator
```

---

## 11. Лицензия

MIT.

---

## 12. Интернационализация (i18n)

Rust4ui поддерживает многоязычные интерфейсы с первого дня. Система построена на трёх принципах:

1. **Data-driven переводы** — все строки хранятся в JSON, не в коде
2. **Runtime-переключение** — язык меняется без перекомпиляции
3. **Единая интерполяция** — `{param}` внутри строк перевода резолвятся из StateRegistry

### 12.1. Формат locale-файлов

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

### 12.2. Ключи перевода

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

### 12.3. Ссылки в ui.json (`{{key}}`)

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

### 12.4. Pipeline разрешения

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

### 12.5. Плюрализация

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

### 12.6. LocaleRegistry API

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

### 12.7. Переключение языка в рантайме

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

### 12.8. Fallback-цепочка

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

### 12.9. Пример locale-файлов

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

## 13. Ограничения (известные)

1. **Только egui** — не абстрагирован под другие бэкенды. Если появится спрос — можно вынести ядро в `rust4ui_core`.
2. **Binding только по имени** — без вложенных путей (`"settings.mic.name"`). В будущем можно добавить.
3. **Нет анимаций** — egui сам не поддерживает, а транслировать из JSON бессмысленно.
4. **ComboBox требует Vec<String> в state** — кастомные типы через downcast в будущем.
5. **Code Generator — только для статичных UI** — если UI динамически меняется в рантайме, генерация бессмысленна.

---

## 14. Совместимость с EFx

Rust4ui и [EFx](https://github.com/ZhukMax/efx) — два проекта с общей семантикой виджетов, но разными сценариями использования:

| | EFx | Rust4ui |
|--|-----|---------|
| **Тип** | Proc-macro (compile-time) | Библиотека (runtime) |
| **Вход** | XML-подобные теги `efx!()` | JSON / EFx-шаблон |
| **Рендеринг** | Компилируется в egui-вызовы | Интерпретирует JSON → egui |
| **Производительность** | Нулевой оверхед | Оверхед парсинга JSON |
| **Прототипирование** | Правка → компиляция → запуск | Правка → запуск (без компиляции) |

### 14.1. Dual-mode архитектура

Любой UI, описанный в JSON для Rust4ui, может быть «запечён» в EFx без изменений:

```rust
// === Режим 1: Runtime (Rust4ui) ===
// ui.json:
// { "type": "Column", "gap": 8, "children": [
//   { "type": "Button", "text": "Start", "action": "start" }
// ]}

let app = Rust4ui::new(ui_json, theme, locales, state, actions);

// === Режим 2: Compile-time (EFx) ===
// Тот же UI, сгенерированный rust4ui-codegen:
fn render_static(ui: &mut egui::Ui, actions: &Actions) {
    efx!(ui, r#"
        <Column gap="8">
            <Button action="start">Start</Button>
        </Column>
    "#);
}

// === Режим 3: Гибрид (Rust4ui + EFx-шаблон) ===
let app = Rust4ui::from_efx(r#"
    <Column gap="8">
        <Button action="start">Start</Button>
    </Column>
"#, theme, locales, state, actions);
```

### 14.2. Единый словарь тегов

Все виджеты Rust4ui и EFx используют одинаковые имена тегов и атрибуты:

| Тег | Назначение | Совпадает? |
|-----|-----------|-----------|
| `Column` | Вертикальный контейнер | ✅ |
| `Row` | Горизонтальный контейнер | ✅ |
| `Label` | Текст | ✅ |
| `Button` | Кнопка | ✅ |
| `TextField` | Поле ввода | ✅ |
| `Checkbox` | Флажок | ✅ |
| `Slider` | Ползунок | ✅ |
| `ComboBox` | Выпадающий список | ✅ |
| `Separator` | Разделитель | ✅ |
| `Tabs` / `Tab` | Вкладки | ✅ |
| `Panel` | Группа с рамкой | ✅ |
| `ScrollArea` | Скроллируемая область | ✅ |
| `Window` | Всплывающее окно | ✅ |
| `Hyperlink` | Ссылка | ✅ |
| `Image` | Изображение | ✅ |
| `Grid` | Табличная сетка | ✅ |
| `DataTable` | Таблица (egui_extras) | ✅ (v0.4) |

### 14.3. Семантическая карта

При рендеринге Rust4ui выполняет те же действия, что сгенерированный `efx!()`-код:

```rust
// Rust4ui runtime:
// { "type": "Button", "text": "OK", "fill": "#303030", "rounding": 6 }
//   ↓
// Renderer находит type="Button", применяет тему, рисует через egui

// EFx compile-time:
// <Button fill="#303030" rounding="6">OK</Button>
//   ↓
// Макрос разворачивается в те же egui-вызовы

// Результат одинаковый
```

### 14.4. Зависимости

При использовании Rust4ui с EFx:
- **Без codegen** — зависимость только `efx-core` (парсинг атрибутов, разделение семантики)
- **С codegen** — зависимость `efx` (макрос) для скомпилированных UI
- **Runtime + from_efx()** — `efx-core` для разбора EFx-шаблона в рантайме
