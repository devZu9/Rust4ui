# Локализация (i18n)

## LocaleRegistry

`LocaleRegistry` управляет переводами. Загружает JSON-файлы локалей с флат-ключей и поддерживает fallback на английский.

```rust
let mut locale = LocaleRegistry::new("ru");
locale.load_file("ru", &base.join("locales/ru.json"))?;
locale.load_file("en", &base.join("locales/en.json"))?;
```

## Синтаксис `{{key}}`

В любом текстовом атрибуте виджета можно использовать `{{key}}`:

```json
{ "type": "Label", "text": "{{label.volume}}" }
{ "type": "Button", "text": "{{btn.save}}" }
{ "type": "TextField", "hint": "{{hint.name}}" }
```

При рендеринге `{{key}}` заменяется на перевод из текущей локали.

## Файлы локалей

### locales/ru.json

```json
{
  "locale": "ru",
  "name": "Русский",
  "translations": {
    "btn.save": "Сохранить",
    "btn.cancel": "Отмена",
    "label.volume": "Громкость",
    "tab.basic": "Основные",
    "hint.name": "Введите ваше имя...",
    "files.count": {
      "one": "{count} файл",
      "few": "{count} файла",
      "many": "{count} файлов"
    }
  }
}
```

### locales/en.json

```json
{
  "locale": "en",
  "name": "English",
  "translations": {
    "btn.save": "Save",
    "btn.cancel": "Cancel",
    "label.volume": "Volume",
    "tab.basic": "Basic",
    "hint.name": "Enter your name...",
    "files.count": {
      "one": "{count} file",
      "other": "{count} files"
    }
  }
}
```

## Ключи: плоские и вложенные

Ключи могут быть плоскими (`"btn.save": "Сохранить"`) или вложенными:

```json
{
  "files.count": {
    "one": "{count} файл",
    "few": "{count} файла",
    "many": "{count} файлов"
  }
}
```

Вложенные объекты автоматически разворачиваются во flat-ключи: `"files.count.one"`, `"files.count.few"`, `"files.count.many"`.

## Fallback на английский

Если ключ не найден в текущем языке, система ищет его в `"en"`. Если и там нет — возвращается `{{key}}`:

```rust
locale.switch("ru");
locale.resolve("btn.save"); // "Сохранить"
locale.resolve("btn.undo"); // "Undo" (fallback на en)
locale.resolve("btn.missing"); // "{{btn.missing}}" (не найден)
```

## Переключение языка в runtime

```rust
locale.switch("en");
// Следующий рендер покажет текст на английском
```

В демо-приложении переключение привязано к `active_locale` (usize):

```rust
if let Some(idx) = state.get_usize("active_locale") {
    let lang = match idx { 0 => "ru", 1 => "en", _ => "ru" };
    locale.switch(lang);
}
```

## Интерполяция переменных

В тексте перевода можно использовать `{variable}` — они заменяются значениями из состояния:

```rust
state.set_string("name", "Анна".into());
let text = locale.i18n_text("{{greeting}}", &state);
// "Привет, Анна!" (из "greeting": "Привет, {name}!")
```

## Плюрализация (CLDR)

Поддерживаются CLDR plural rules для 10 языков:

| Язык | Код | Формы |
|------|-----|-------|
| Русский | ru | one, few, many, other |
| Украинский | uk | one, few, many, other |
| Белорусский | be | one, few, many, other |
| Английский | en | one, other |
| Немецкий | de | one, other |
| Французский | fr | one, other |
| Польский | pl | one, few, many, other |
| Японский | ja | other |
| Китайский | zh | other |
| Корейский | ko | other |

Использование в JSON:

```json
{ "type": "Label", "text": "{{files.count}}" }
```

В состоянии должен быть ключ `count` (f64):

```rust
state.set_f64("count", 1.0); // "1 файл"
state.set_f64("count", 2.0); // "2 файла"
state.set_f64("count", 5.0); // "5 файлов"
```

## i18n_text()

Основной метод для обработки текста:

```rust
pub fn i18n_text(&self, text: &str, state: &StateRegistry) -> String
```

1. Ищет `{{key}}` в тексте
2. Заменяет на перевод из текущего языка (или fallback en)
3. Если есть plural-формы — вычисляет по CLDR правилам
4. Интерполирует `{variable}` из состояния
