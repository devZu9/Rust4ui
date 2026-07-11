# UI JSON — виджеты и атрибуты

Каждый виджет — объект JSON с обязательным полем `type` и набором атрибутов.

```json
{ "type": "Label", "text": "Hello", "color": "#66CCFF" }
```

В JSON можно использовать комментарии:
```json
{
  "type": "Column",
  "gap": 8,    // расстояние между элементами
  /* блочный комментарий */
  "children": []
}
```

---

## Label

Текстовая метка.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `text` | string | `""` | Текст (поддерживает `{{key}}`) |
| `icon` | string | — | Имя иконки Phosphor (ставится перед текстом) |
| `color` | string | `#CCCCCC` | Цвет текста в HEX |
| `size` | number | `13.0` | Размер шрифта |
| `bold` | bool | `false` | Жирное начертание |
| `italic` | bool | `false` | Курсив |
| `monospace` | bool | `false` | Моноширинный шрифт |
| `wrap` | bool | `false` | Перенос текста |
| `heading` | bool | `false` | Стиль заголовка (size=20) |
| `padding` | number/[2]/[4] | — | Внутренний отступ |

```json
{ "type": "Label", "text": "Привет, мир!" }
{ "type": "Label", "text": "Заголовок", "bold": true, "size": 18 }
{ "type": "Label", "text": "Код", "monospace": true, "color": "#66CCFF" }
{ "type": "Label", "icon": "star", "text": "Избранное", "size": 16 }
```

---

## Button

Кнопка.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `text` | string | `""` | Текст кнопки |
| `icon` | string | — | Имя иконки Phosphor (ставится перед текстом) |
| `action` | string | — | Имя экшена при клике |
| `target` | string | `""` | Строка цели для экшена |
| `enabled` | bool | `true` | Активна/отключена |
| `fill` | string | #303030 | Цвет фона |
| `text_color` | string | #E0E0E0 | Цвет текста |
| `min_width` | number | 100.0 | Минимальная ширина |
| `rounding` | number | 6.0 | Скругление углов |
| `align` | string | `"center"` | Выравнивание текста: left/center/right |
| `tooltip` | string | — | Подсказка при наведении |
| `padding` | number/[2]/[4] | [16,4] | Внутренний отступ |
| `hover_fill` | string | — | Цвет фона при наведении |
| `hover_text_color` | string | — | Цвет текста при наведении |
| `click_fill` | string | — | Цвет фона при нажатии |
| `click_text_color` | string | — | Цвет текста при нажатии |
| `border_width` | number | 0 | Толщина рамки |
| `border_color` | string | — | Цвет рамки |
| `border_type` | string | `"solid"` | Тип рамки: solid/dash/dot |
| `border` | array | — | Шорткат рамки: [width, "#color", "type"] |
| `shadow_offset_x` | number | 2.0 | Смещение тени по X |
| `shadow_offset_y` | number | 2.0 | Смещение тени по Y |
| `shadow_blur` | number | 4.0 | Размытие тени |
| `shadow_color` | string | rgba(0,0,0,40) | Цвет тени |

```json
{ "type": "Button", "text": "Сохранить", "action": "save", "icon": "floppy-disk" }
{ "type": "Button", "text": "Удалить", "fill": "#CC3333", "action": "delete" }
{ "type": "Button", "text": "Настройки", "enabled": false }
{ "type": "Button", "text": "Опасная кнопка",
  "hover_fill": "#FF4444", "click_fill": "#AA0000",
  "border": [2, "#FF4444", "dash"],
  "tooltip": "Осторожно!" }
```

---

## TextField

Поле ввода текста.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `binding` | string | **обязательный** | Ключ в StateRegistry |
| `mode` | string | `"text"` | Режим: text/password/number/multiline |
| `password` | bool | `false` | Режим пароля |
| `multiline` | bool | `false` | Многострочный режим |
| `width` | number | 200.0 | Ширина поля |
| `hint` | string | — | Текст-подсказка внутри поля |
| `fixed` | bool | `true` | Фиксированная высота |
| `desired_rows` | number | 4.0 | Кол-во строк в multiline |
| `valign` | string | `"center"` | Вертикальное выравнивание: top/center/bottom |
| `min` | number | -∞ | Минимум (mode=number) |
| `max` | number | +∞ | Максимум (mode=number) |
| `step` | number | 1.0 | Шаг (mode=number) |
| `decimals` | number | авто | Кол-во знаков после запятой |

```json
{ "type": "TextField", "binding": "name", "hint": "Введите имя..." }
{ "type": "TextField", "binding": "password", "mode": "password", "width": 200 }
{ "type": "TextField", "binding": "description", "multiline": true, "desired_rows": 4 }
{ "type": "TextField", "binding": "volume", "mode": "number", "min": 0, "max": 100, "step": 1 }
```

---

## Checkbox

Флажок (чекбокс).

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `binding` | string | **обязательный** | Ключ bool в StateRegistry |
| `text` | string | `""` | Подпись |
| `padding` | number/[2]/[4] | — | Внутренний отступ |

```json
{ "type": "Checkbox", "binding": "use_gpu", "text": "Использовать GPU" }
{ "type": "Checkbox", "binding": "auto_start", "text": "Автозапуск" }
```

---

## RadioGroup

Группа радиокнопок.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `binding` | string | **обязательный** | Ключ usize в StateRegistry |
| `options` | array | **обязательный** | Массив `{value, text}` |
| `direction` | string | `"vertical"` | Направление: vertical/horizontal |
| `padding` | number/[2]/[4] | — | Внутренний отступ |

```json
{
  "type": "RadioGroup",
  "binding": "app_theme",
  "options": [
    { "value": 0, "text": "Тёмная" },
    { "value": 1, "text": "Светлая" },
    { "value": 2, "text": "Авто" }
  ]
}
```

---

## Slider

Ползунок.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `binding` | string | **обязательный** | Ключ f64 в StateRegistry |
| `min` | number | 0.0 | Минимум |
| `max` | number | 1.0 | Максимум |
| `step` | number | 0.01 | Шаг |
| `width` | number | 250.0 | Ширина |

```json
{ "type": "Slider", "binding": "volume", "min": 0, "max": 100, "step": 1, "width": 200 }
```

---

## ComboBox

Выпадающий список.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `binding` | string | **обязательный** | Ключ usize (индекс) в StateRegistry |
| `items` | string | =binding | Ключ VecString в StateRegistry со списком |

```json
{ "type": "ComboBox", "binding": "selected_device", "items": "device_list" }
```

---

## Column

Вертикальный контейнер.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `gap` | number | 0.0 | Расстояние между элементами |
| `padding` | number | 0.0 | Внутренний отступ со всех сторон |
| `align` | string | `"left"` | Выравнивание: left/center |

```json
{
  "type": "Column",
  "gap": 8,
  "padding": 12,
  "children": [
    { "type": "Label", "text": "Заголовок" },
    { "type": "Button", "text": "Нажми" }
  ]
}
```

---

## Row

Горизонтальный контейнер.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `gap` | number | 0.0 | Расстояние между элементами (по X) |
| `gap_row` | number | 0.0 | Расстояние между строками при wrap=true (по Y) |
| `wrap` | bool | `false` | Перенос строки |
| `columns` | number | — | Режим сетки: кол-во колонок |
| `padding` | number/[2]/[4] | — | Внутренний отступ |

```json
{
  "type": "Row",
  "gap": 8,
  "wrap": true,
  "children": [
    { "type": "Button", "text": "A" },
    { "type": "Button", "text": "B" }
  ]
}
```

---

## Tabs

Вкладки.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Массив Tab-узлов |
| `active` | string | `"__tab_active"` | Binding для активной вкладки |

Каждый **Tab**:
| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `type` | string | `"Tab"` | Тип (Tab) |
| `id` | string | `""` | Идентификатор вкладки |
| `title` | string | =id | Заголовок вкладки |
| `enabled` | bool | `true` | Доступна ли вкладка |
| `children` | array | `[]` | Содержимое вкладки |

```json
{
  "type": "Tabs",
  "active": "basic",
  "children": [
    {
      "type": "Tab",
      "id": "basic",
      "title": "Основные",
      "children": [
        { "type": "Label", "text": "Содержимое вкладки" }
      ]
    },
    {
      "type": "Tab",
      "id": "settings",
      "title": "Настройки",
      "children": []
    }
  ]
}
```

---

## Panel

Панель с фоном, рамкой и скруглением.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `fill` | string | #1A1D23 | Цвет фона |
| `rounding` | number | 8.0 | Скругление углов |
| `padding` | number | 12.0 | Внутренний отступ |
| `border_width` | number | 0 | Толщина рамки |
| `border_color` | string | — | Цвет рамки |

```json
{
  "type": "Panel",
  "fill": "#1A1D23",
  "rounding": 8,
  "padding": 16,
  "border": [1, "#333333"],
  "children": [
    { "type": "Label", "text": "Содержимое панели" }
  ]
}
```

---

## ScrollArea

Область прокрутки.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `axis` | string | `"vertical"` | Направление: vertical/horizontal/both |
| `max_height` | number | — | Максимальная высота |
| `max_width` | number | — | Максимальная ширина |

```json
{
  "type": "ScrollArea",
  "axis": "vertical",
  "max_height": 300,
  "children": [
    { "type": "Label", "text": "Длинный текст..." }
  ]
}
```

---

## Window

Всплывающее окно.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `open` | string | — | **Обязательный** Binding bool для видимости |
| `title` | string | `""` | Заголовок окна |
| `id` | string | `"window"` | Идентификатор окна |
| `title_bar` | bool | `true` | Показывать заголовок |
| `default_width` | number | 400.0 | Ширина по умолчанию |
| `default_height` | number | 300.0 | Высота по умолчанию |
| `min_width` | number | 100.0 | Минимальная ширина |
| `min_height` | number | 80.0 | Минимальная высота |
| `movable` | bool | `true` | Можно перемещать |
| `resizable` | bool | `true` | Можно изменять размер |
| `collapsible` | bool | `true` | Можно свернуть |
| `constrain` | bool | `true` | Ограничивать родителем |
| `auto_sized` | bool | `false` | Авторазмер по содержимому |
| `fill` | string | #1E1E24 | Цвет фона окна |
| `padding` | number | 8.0 | Внутренний отступ |
| `anchor_h` | string | — | Привязка: left/center/right |
| `anchor_x` | number | 0.0 | Смещение по X при anchor_h |
| `anchor_y` | number | 0.0 | Смещение по Y |

```json
{
  "type": "Window",
  "id": "info_win",
  "open": "show_info_window",
  "title": "Информация",
  "default_width": 350,
  "default_height": 200,
  "children": [
    { "type": "Label", "text": "Содержимое окна" }
  ]
}
```

---

## Spinner

Индикатор загрузки.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `size` | number | 24.0 | Размер спиннера |
| `color` | string | #66CCFF | Цвет |
| `text` | string | — | Текст рядом со спиннером |

```json
{ "type": "Spinner", "size": 32, "color": "#66CCFF", "text": "Загрузка..." }
```

---

## Shortcut

Горячая клавиша (невидимый виджет).

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `key` | string | — | Комбинация: `"Ctrl+S"`, `"Ctrl+Shift+Z"` |
| `action` | string | — | Имя экшена |
| `target` | string | `""` | Цель для экшена |

Поддерживаемые модификаторы: `Ctrl`, `Shift`, `Alt`.
Поддерживаемые клавиши: `S`, `Z`, `Y`, `N`, `O`, `F`, `Q`, `W`, `E`, `R`, `T`, `A`, `D`, `G`, `H`, `F4`.

```json
{ "type": "Shortcut", "key": "Ctrl+S", "action": "save" }
{ "type": "Shortcut", "key": "Ctrl+Z", "action": "undo" }
```

---

## ColorPicker

Пипетка выбора цвета.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `binding` | string | **обязательный** | Ключ string (HEX) в StateRegistry |
| `alpha` | bool | `false` | Показывать альфа-канал |

```json
{ "type": "ColorPicker", "binding": "accent_color", "alpha": true }
```

---

## FileDrop

Область приёма файлов drag-and-drop.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты внутри области |
| `action` | string | — | Экшен при броске файла |
| `multi` | bool | `false` | Множественные файлы |
| `accept` | array | — | Массив расширений для фильтрации |
| `fill` | string | #1A1D23 | Цвет фона |
| `rounding` | number | 8.0 | Скругление |
| `padding` | number | 16.0 | Внутренний отступ |
| `highlight_color` | string | rgba(51,102,204,0.27) | Цвет подсветки при наведении |

```json
{
  "type": "FileDrop",
  "action": "file_dropped",
  "accept": [".json", ".wav"],
  "children": [
    { "type": "Label", "text": "Перетащите файлы сюда" }
  ]
}
```

---

## Indicator

Индикатор-точка.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `color` | string | #888888 | Цвет точки |
| `size` | string | `"8"` | Диаметр в пикселях |
| `pulse` | string | `"false"` | Анимация пульсации |
| `tooltip` | string | — | Подсказка при наведении |

```json
{ "type": "Indicator", "color": "#00FF66", "size": "10", "tooltip": "В сети" }
{ "type": "Indicator", "color": "#FF3333", "size": "6", "tooltip": "Ошибка" }
```

---

## IconButton

Кнопка-иконка.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `icon` | string | — | Имя иконки Phosphor |
| `icon_size` | number | 14.0 | Размер иконки (через тему) |
| `icon_color` | string | #CCCCCC | Цвет иконки |
| `button_size` | number | 24.0 | Размер кнопки (ширина/высота) |
| `margin` | number/[2]/[4] | 0 | Внешний отступ со всех сторон |
| `action` | string | — | Имя экшена при клике |
| `target` | string | `""` | Цель для экшена |
| `enabled` | bool | `true` | Активна/отключена |
| `tooltip` | string | — | Подсказка |
| `hover_fill` | string | rgba(68,68,85,0.25) | Цвет фона при наведении |
| `click_fill` | string | — | Цвет фона при нажатии |

```json
{ "type": "IconButton", "icon": "gear-six", "action": "open_settings", "tooltip": "Настройки" }
{ "type": "IconButton", "icon": "trash-simple", "action": "delete", "icon_color": "#FF4444" }
{ "type": "IconButton", "icon": "floppy-disk", "icon_size": 22, "action": "save" }
```

---

## IconBar

Панель инструментов с иконками.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты (обычно IconButton) |
| `direction` | string | `"vertical"` | Направление: vertical/horizontal |
| `fill` | string | #1C1C22 | Цвет фона |
| `width` | number | 48.0 | Ширина (для vertical) |
| `height` | number | 36.0 | Высота (для horizontal) |

```json
{
  "type": "IconBar",
  "direction": "vertical",
  "fill": "#1C1C22",
  "children": [
    { "type": "IconButton", "icon": "folder-simple", "action": "open" },
    { "type": "IconButton", "icon": "floppy-disk", "action": "save" },
    { "type": "IconButton", "icon": "gear-six", "action": "settings" }
  ]
}
```

---

## Caption

Мелкий текст-подпись.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `text` | string | `""` | Текст |
| `color` | string | #888888 | Цвет |
| `size` | number | 11.0 | Размер шрифта |
| `monospace` | bool | `false` | Моноширинный |

```json
{ "type": "Caption", "text": "v1.0.0", "color": "#888888" }
```

---

## StatusBar

Строка состояния.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `height` | number | 26.0 | Высота строки |
| `padding` | number | 4.0 | Внутренний отступ |
| `fill` | string | #18181D | Цвет фона |

Дочерние виджеты могут иметь атрибут `anchor`:
- `"start"` — слева (по умолчанию)
- `"center"` — по центру
- `"end"` — справа

```json
{
  "type": "StatusBar",
  "height": 26,
  "children": [
    { "type": "Label", "text": "Готов к работе", "size": 11, "anchor": "start" },
    { "type": "Label", "text": "v1.0", "size": 11, "anchor": "end" }
  ]
}
```

---

## Hyperlink

Гиперссылка.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `url` | string | `""` | URL |
| `text` | string | =url | Отображаемый текст |
| `tooltip` | string | — | Подсказка |

```json
{ "type": "Hyperlink", "url": "https://github.com", "text": "GitHub" }
```

---

## Grid

Таблица (сетка).

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты |
| `columns` | number | 2 | Количество колонок |
| `spacing_x` | number | 8.0 | Расстояние по X |
| `spacing_y` | number | 4.0 | Расстояние по Y |
| `striped` | bool | `false` | Чередование строк |

```json
{
  "type": "Grid",
  "columns": 3,
  "striped": true,
  "children": [
    { "type": "Label", "text": "Проект", "bold": true },
    { "type": "Label", "text": "Статус", "bold": true },
    { "type": "Label", "text": "Версия", "bold": true },
    { "type": "Label", "text": "Rust4ui" },
    { "type": "Label", "text": "В разработке" },
    { "type": "Label", "text": "0.2.1" }
  ]
}
```

---

## MenuBar

Строка меню.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `children` | array | `[]` | Дочерние виджеты (Menu) |

```json
{
  "type": "MenuBar",
  "children": [
    { "type": "Menu", "text": "Файл", "children": [
      { "type": "MenuItem", "text": "Открыть", "action": "open" },
      { "type": "MenuItem", "text": "Выход", "action": "exit" }
    ]}
  ]
}
```

---

## Menu

Выпадающее меню.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `text` | string | `"Menu"` | Заголовок меню |
| `children` | array | `[]` | Дочерние (MenuItem, SubMenu, Separator) |

```json
{ "type": "Menu", "text": "Файл", "children": [...] }
```

---

## MenuItem

Пункт меню.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `text` | string | `""` | Текст пункта |
| `action` | string | — | Имя экшена |
| `target` | string | — | Цель для экшена |
| `enabled` | bool | `true` | Доступен ли пункт |
| `icon` | string | — | Имя иконки Phosphor |
| `shortcut` | string | — | Текст шортката (отображается, но не обрабатывается) |
| `size` | number | 12.0 | Размер шрифта |

```json
{ "type": "MenuItem", "text": "Сохранить", "action": "save", "icon": "floppy-disk", "shortcut": "Ctrl+S" }
```

---

## SubMenu

Вложенное подменю.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `text` | string | `""` | Заголовок |
| `children` | array | `[]` | Дочерние (MenuItem, SubMenu) |

```json
{
  "type": "SubMenu",
  "text": "Экспорт",
  "children": [
    { "type": "MenuItem", "text": "JSON" },
    { "type": "MenuItem", "text": "CSV" }
  ]
}
```

---

## Notifications

Область уведомлений (заглушка).

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `position` | string | `"top-right"` | Позиция |
| `max_count` | number | 5 | Макс. количество |
| `width` | number | 300.0 | Ширина области |

```json
{ "type": "Notifications", "position": "top-right", "max_count": 5 }
```

---

## Separator

Горизонтальный разделитель.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `space` | number | 6.0 | Отступ сверху и снизу |

```json
{ "type": "Separator", "space": 8 }
```

---

## Spacer

Заполнитель пространства.

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `width` | number | `0` (auto) | Ширина (0 = всё доступное место) |

```json
{ "type": "Spacer" }
{ "type": "Spacer", "width": 16 }
```
