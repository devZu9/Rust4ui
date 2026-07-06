# Иконки Rust4ui

По умолчанию иконки отрисовываются шрифтом **Phosphor** через файл `phosphor.ttf`.

## Использование

```json
{ "type": "Button", "text": "Сохранить", "icon": "save" }
{ "type": "MenuItem", "text": "Открыть", "icon": "folder", "shortcut": "Ctrl+O" }
```

## Как добавить свою иконку

### Шрифтовую (Phosphor)

1. Найти codepoint в [Phosphor Icons](https://phosphoricons.com)
2. Добавить запись в `icons.json`: `"my_icon": "\uXXXX"`
3. Иконка сразу доступна в JSON: `"icon": "my_icon"`

### Текстурную (PNG)

Зарегистрировать текстуру в Rust:

```rust
rust4ui::icons::register_texture(
    "my_logo",
    ctx.load_texture("logo", &my_rgba_image, egui::TextureOptions::LINEAR)
);
```

Использовать в JSON:

```json
{ "type": "Button", "icon": "my_logo", "icon_width": 24, "icon_height": 24 }
```
