# Примеры JSON

## 1. Форма с отправкой

```json
{
  "type": "Column",
  "gap": 8,
  "padding": 16,
  "children": [
    {
      "type": "Label",
      "text": "Регистрация",
      "bold": true,
      "size": 20,
      "color": "#E0EDFF"
    },
    { "type": "Separator", "space": 4 },
    {
      "type": "TextField",
      "binding": "reg_name",
      "hint": "Имя пользователя"
    },
    {
      "type": "TextField",
      "binding": "reg_email",
      "hint": "Email"
    },
    {
      "type": "TextField",
      "binding": "reg_password",
      "mode": "password",
      "hint": "Пароль"
    },
    {
      "type": "Checkbox",
      "binding": "reg_agree",
      "text": "Я согласен с условиями"
    },
    {
      "type": "Row",
      "gap": 8,
      "children": [
        {
          "type": "Button",
          "text": "Зарегистрироваться",
          "action": "register",
          "fill": "#3366CC"
        },
        {
          "type": "Button",
          "text": "Отмена",
          "action": "cancel"
        }
      ]
    }
  ]
}
```

## 2. Меню-бар

```json
{
  "type": "MenuBar",
  "children": [
    {
      "type": "Menu",
      "text": "Файл",
      "children": [
        {
          "type": "MenuItem",
          "text": "Новый",
          "action": "new_file",
          "icon": "file",
          "shortcut": "Ctrl+N"
        },
        {
          "type": "MenuItem",
          "text": "Открыть",
          "action": "open",
          "icon": "folder-simple",
          "shortcut": "Ctrl+O"
        },
        { "type": "Separator" },
        {
          "type": "MenuItem",
          "text": "Сохранить",
          "action": "save",
          "icon": "floppy-disk",
          "shortcut": "Ctrl+S"
        },
        { "type": "Separator" },
        {
          "type": "MenuItem",
          "text": "Выход",
          "action": "exit",
          "icon": "x"
        }
      ]
    },
    {
      "type": "Menu",
      "text": "Правка",
      "children": [
        {
          "type": "MenuItem",
          "text": "Отменить",
          "action": "undo",
          "shortcut": "Ctrl+Z"
        },
        {
          "type": "MenuItem",
          "text": "Повторить",
          "action": "redo",
          "shortcut": "Ctrl+Shift+Z"
        },
        { "type": "Separator" },
        {
          "type": "SubMenu",
          "text": "Экспорт",
          "children": [
            { "type": "MenuItem", "text": "JSON", "action": "export_json" },
            { "type": "MenuItem", "text": "CSV", "action": "export_csv" }
          ]
        }
      ]
    },
    {
      "type": "Menu",
      "text": "Вид",
      "children": [
        {
          "type": "MenuItem",
          "text": "Боковая панель",
          "action": "toggle_sidebar",
          "icon": "sidebar-simple"
        }
      ]
    }
  ]
}
```

## 3. Панель настроек

```json
{
  "type": "Panel",
  "fill": "#1A1D23",
  "rounding": 8,
  "padding": 16,
  "border": [1, "#33333A"],
  "children": [
    {
      "type": "Label",
      "text": "Настройки приложения",
      "bold": true,
      "size": 18,
      "color": "#E0EDFF"
    },
    { "type": "Separator", "space": 6 },
    {
      "type": "Label",
      "text": "Язык интерфейса"
    },
    {
      "type": "ComboBox",
      "binding": "selected_lang",
      "items": "lang_list"
    },
    { "type": "Spacer" },
    {
      "type": "Label",
      "text": "Громкость"
    },
    {
      "type": "Slider",
      "binding": "volume",
      "min": 0,
      "max": 100,
      "step": 1,
      "width": 300
    },
    { "type": "Spacer" },
    {
      "type": "Label",
      "text": "Размер шрифта"
    },
    {
      "type": "Slider",
      "binding": "font_size",
      "min": 8,
      "max": 32,
      "step": 1,
      "width": 300
    },
    { "type": "Spacer" },
    {
      "type": "Checkbox",
      "binding": "use_gpu",
      "text": "Использовать GPU"
    },
    {
      "type": "Checkbox",
      "binding": "auto_start",
      "text": "Автозапуск"
    },
    { "type": "Spacer" },
    {
      "type": "Label",
      "text": "Тема оформления"
    },
    {
      "type": "RadioGroup",
      "binding": "app_theme",
      "options": [
        { "value": 0, "text": "Тёмная" },
        { "value": 1, "text": "Светлая" },
        { "value": 2, "text": "Авто" }
      ]
    },
    { "type": "Separator" },
    {
      "type": "Row",
      "gap": 8,
      "children": [
        {
          "type": "Button",
          "text": "Применить",
          "action": "apply",
          "fill": "#3366CC"
        },
        {
          "type": "Button",
          "text": "Сбросить",
          "action": "reset"
        }
      ]
    }
  ]
}
```

## 4. Галерея иконок (IconBar + IconButton)

```json
{
  "type": "Row",
  "gap": 0,
  "children": [
    {
      "type": "IconBar",
      "direction": "vertical",
      "width": 48,
      "fill": "#1C1C22",
      "children": [
        {
          "type": "IconButton",
          "icon": "folder-simple",
          "action": "open",
          "tooltip": "Открыть"
        },
        {
          "type": "IconButton",
          "icon": "floppy-disk",
          "action": "save",
          "tooltip": "Сохранить"
        },
        {
          "type": "IconButton",
          "icon": "magnifying-glass",
          "action": "search",
          "tooltip": "Поиск"
        },
        { "type": "Separator" },
        {
          "type": "IconButton",
          "icon": "trash-simple",
          "action": "delete",
          "icon_color": "#FF4444",
          "tooltip": "Удалить"
        },
        { "type": "Separator" },
        {
          "type": "IconButton",
          "icon": "gear-six",
          "action": "settings",
          "icon_size": 22,
          "tooltip": "Настройки"
        }
      ]
    },
    {
      "type": "Column",
      "gap": 4,
      "padding": 8,
      "children": [
        {
          "type": "Label",
          "text": "Галерея иконок",
          "bold": true,
          "size": 16
        },
        {
          "type": "Label",
          "text": "Наведите на иконку слева для подсказки"
        },
        {
          "type": "Label",
          "text": "Иконки Phosphor в действии:"
        },
        {
          "type": "Row",
          "gap": 8,
          "children": [
            {
              "type": "Button",
              "text": "Звезда",
              "icon": "star",
              "action": "star"
            },
            {
              "type": "Button",
              "text": "Сердце",
              "icon": "heart",
              "action": "like"
            },
            {
              "type": "Button",
              "text": "Колокольчик",
              "icon": "bell",
              "action": "notify"
            }
          ]
        }
      ]
    }
  ]
}
```

## 5. Окно с кастомным заголовком

```json
{
  "type": "Window",
  "id": "custom_win",
  "open": "show_custom_window",
  "title_bar": false,
  "default_width": 350,
  "default_height": 200,
  "anchor_h": "right",
  "anchor_x": -10,
  "anchor_y": 10,
  "children": [
    {
      "type": "Column",
      "gap": 4,
      "children": [
        {
          "type": "Row",
          "gap": 4,
          "children": [
            {
              "type": "Label",
              "text": "Кастомная панель",
              "bold": true,
              "size": 14
            },
            {
              "type": "Spacer"
            },
            {
              "type": "IconButton",
              "icon": "x",
              "action": "toggle_custom_window",
              "tooltip": "Закрыть"
            }
          ]
        },
        { "type": "Separator" },
        {
          "type": "Label",
          "text": "Это окно без стандартной строки заголовка."
        },
        {
          "type": "Label",
          "text": "Кнопка закрытия — кастомный виджет внутри окна."
        }
      ]
    }
  ]
}
```

## 6. Страница логина с диалогом

```json
{
  "type": "Column",
  "gap": 8,
  "padding": 16,
  "align": "center",
  "children": [
    {
      "type": "Label",
      "text": "Вход в систему",
      "bold": true,
      "size": 24,
      "color": "#E0EDFF"
    },
    { "type": "Spacer" },
    {
      "type": "Panel",
      "fill": "#1E1E24",
      "rounding": 8,
      "padding": 16,
      "border": [1, "#33333A"],
      "children": [
        {
          "type": "Column",
          "gap": 8,
          "children": [
            {
              "type": "Label",
              "text": "Логин"
            },
            {
              "type": "TextField",
              "binding": "login_name",
              "hint": "Введите логин",
              "width": 250
            },
            {
              "type": "Label",
              "text": "Пароль"
            },
            {
              "type": "TextField",
              "binding": "login_password",
              "mode": "password",
              "hint": "Введите пароль",
              "width": 250
            },
            {
              "type": "Checkbox",
              "binding": "remember_me",
              "text": "Запомнить меня"
            },
            { "type": "Spacer" },
            {
              "type": "Row",
              "gap": 8,
              "children": [
                {
                  "type": "Button",
                  "text": "Войти",
                  "action": "login",
                  "fill": "#3366CC",
                  "min_width": 120
                },
                {
                  "type": "Button",
                  "text": "Отмена",
                  "action": "cancel",
                  "min_width": 120
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "type": "Window",
      "id": "confirm_dialog",
      "open": "show_dialog",
      "title": "Подтверждение",
      "default_width": 300,
      "default_height": 150,
      "resizable": false,
      "children": [
        {
          "type": "Column",
          "gap": 8,
          "padding": 8,
          "children": [
            {
              "type": "Label",
              "text": "Вы уверены?"
            },
            {
              "type": "Spacer"
            },
            {
              "type": "Row",
              "gap": 8,
              "children": [
                {
                  "type": "Button",
                  "text": "OK",
                  "action": "confirm_ok",
                  "fill": "#3366CC"
                },
                {
                  "type": "Button",
                  "text": "Отмена",
                  "action": "confirm_cancel"
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "type": "StatusBar",
      "height": 26,
      "children": [
        {
          "type": "Label",
          "text": "{{label.status}}: {{status_text}}",
          "size": 11,
          "anchor": "start"
        },
        {
          "type": "Label",
          "text": "v{{version}}",
          "size": 11,
          "anchor": "end"
        }
      ]
    }
  ]
}
```
