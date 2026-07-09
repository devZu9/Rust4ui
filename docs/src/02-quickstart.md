# Быстрый старт

## Установка и запуск

```bash
git clone https://github.com/ZhukMax/rust4ui.git
cd rust4ui
cargo run
```

После сборки откроется окно демо-приложения с примерами всех виджетов.

## Структура проекта

```
rust4ui/
├── Cargo.toml
├── src/
│   ├── main.rs          # точка входа
│   ├── lib.rs           # публичное API
│   ├── renderer.rs      # рендеринг дерева
│   ├── node.rs          # UiNode, парсинг JSON
│   ├── theme.rs         # темы
│   ├── actions.rs       # ActionRegistry
│   ├── state.rs         # StateRegistry
│   ├── locale.rs        # локализация
│   ├── icons.rs         # иконки Phosphor
│   ├── border.rs        # рамки и обводки
│   ├── plural.rs        # CLDR plural rules
│   ├── ref_resolver.rs  # $ref резолвер
│   ├── contrast.rs      # проверка контрастности
│   ├── validator.rs     # валидация JSON
│   └── widgets/         # виджеты
├── demo/
│   ├── ui.json          # корневой UI
│   ├── theme.json       # тема (опционально)
│   └── tabs/            # вкладки через $ref
├── locales/
│   ├── ru.json          # русский
│   └── en.json          # английский
└── icons/
    ├── icons.json       # 1512 иконок Phosphor
    └── phosphor.ttf     # шрифт Phosphor
```

## Редактирование JSON

Демо-приложение загружает `demo/ui.json`. Вы можете редактировать его в любом текстовом редакторе.

```json
{
  "type": "Column",
  "gap": 8,
  "children": [
    { "type": "Label", "text": "Привет, мир!" },
    { "type": "Button", "text": "Нажми меня", "action": "click" }
  ]
}
```

## Hot-reload цикл

1. Отредактируйте `demo/ui.json` или `demo/theme.json`
2. Файловый watcher (крейт `notify`) обнаруживает изменение
3. Приложение перезагружает дерево и тему без перезапуска
4. Изменения отображаются мгновенно

> Hot-reload срабатывает на любые изменения `.json` файлов в папке `demo/`.

## Быстрый запуск

В корне проекта доступен `___run.bat`:

```batch
@echo off
chcp 65001 >nul 2>&1
cd /d "%~dp0"
echo [1/2] Сборка...
cargo build
if %errorlevel% neq 0 (
    echo [ОШИБКА] Сборка не удалась!
    pause
    exit /b 1
)
echo Запуск...
target\debug\rust4ui.exe
pause
```
