# Иконки Phosphor

## IconRegistry

`IconRegistry` загружает 1512 иконок из официальной коллекции Phosphor. Иконки вкомпилированы в бинарник через `include_str!("../icons/icons.json")`, поэтому внешних файлов во время выполнения не требуется.

## Загрузка

```rust
let icons = IconRegistry::new();
// Автоматически загружает icons/icons.json
log::info!("Загружено {} иконок", icons.count()); // 1512
```

## Использование в JSON

Иконки указываются по **реальному имени Phosphor** (kebab-case):

```json
{ "type": "Button", "text": "Сохранить", "icon": "floppy-disk" }
{ "type": "Label", "icon": "star", "text": "Избранное" }
{ "type": "MenuItem", "text": "Удалить", "icon": "trash-simple" }
{ "type": "IconButton", "icon": "gear-six", "action": "settings" }
```

## Виджеты с поддержкой иконок

| Виджет | Атрибут | Назначение |
|--------|---------|-----------|
| `Button` | `icon` | Иконка перед текстом кнопки |
| `Label` | `icon` | Иконка перед текстом метки |
| `MenuItem` | `icon` | Иконка слева от пункта меню |
| `IconButton` | `icon` | Кнопка-иконка (без текста) |

## IconButton — дополнительные атрибуты

| Атрибут | Тип | По умолчанию | Описание |
|---------|-----|-------------|----------|
| `icon` | string | — | Имя иконки |
| `icon_size` | number | 18.0 | Размер иконки в px |
| `icon_color` | string | #CCCCCC | Цвет иконки |

```json
{ "type": "IconButton", "icon": "magnifying-glass", "icon_size": 22, "icon_color": "#66CCFF", "action": "search" }
```

## Примеры имён иконок

```
floppy-disk       — сохранить
trash-simple      — удалить
gear-six          — настройки
magnifying-glass  — поиск
folder-simple     — папка
star              — избранное
house-simple      — домой
bell              — уведомления
user              — пользователь
check             — галочка
x                 — крестик
plus              — плюс
minus             — минус
arrow-right       — стрелка вправо
arrow-left        — стрелка влево
arrow-up          — стрелка вверх
arrow-down        — стрелка вниз
copy-simple       — копировать
download-simple   — скачать
upload-simple     — загрузить
lock-simple       — замок (закрыто)
lock-simple-open  — замок (открыто)
pencil-simple     — редактировать
eye               — глаз (показать)
play              — воспроизвести
pause             — пауза
stop              — стоп
microphone        — микрофон
music-note        — нота
speaker-high      — динамик (громко)
speaker-none      — динамик (без звука)
link-simple       — ссылка
calendar          — календарь
clock             — часы
question          — вопрос
info              — информация
warning           — предупреждение
check-circle      — галочка в круге
x-circle          — крестик в круге
share-network     — поделиться
shopping-cart     — корзина
image             — изображение
video-camera      — видеокамера
headphones        — наушники
wrench            — гаечный ключ
notification      — колокольчик
book              — книга
cloud             — облако
download          — загрузка
file              — файл
heart             — сердце
key               — ключ
lightbulb         — лампочка
map-pin           — метка на карте
moon              — луна
note              — заметка
phone             — телефон
power             — питание
shield            — щит
sun               — солнце
tag               — тег
wifi-high         — Wi-Fi
clipboard         — буфер обмена
alarm             — будильник
camera            — камера
chat              — чат
user-circle       — пользователь в круге
```

Полный список — 1512 имён — находится в `icons/icons.json`.

## Нейминг

Иконки следуют официальной номенклатуре Phosphor:
- Простые: `"star"`, `"bell"`, `"house"`
- Составные: `"folder-simple"`, `"trash-simple"`, `"gear-six"`
- С модификаторами: `"check-circle"`, `"x-circle"`, `"lock-simple-open"`
- Технические: `"magnifying-glass"`, `"share-network"`, `"speaker-high"`

Никаких вымышленных имён — только реальные Phosphor-идентификаторы.

## Шрифт Phosphor

Для отображения иконок используется `icons/phosphor.ttf`. Шрифт загружается при старте приложения:

```rust
load_phosphor_font(&egui_ctx);
```

Если шрифт не найден, иконки отображаться не будут (будет показан символ "⬡").

## Будущее: SVG-текстуры

Планируется поддержка SVG-иконок из `icons/phosphor-icons/SVGs/` с рендерингом через `usvg` + `resvg`. Формат в JSON:

```json
{ "type": "IconButton", "icon": "SVGs/regular/acorn" }
{ "type": "IconButton", "icon": "SVGs Flat/duotone/acorn-duotone" }
```

Это позволит использовать многоцветные duotone-иконки и кастомные стили.
