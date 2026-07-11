# Rust4ui

> 🌐 [Русская версия](README.md)

> **Rust for UI** — build UI in Rust like Lego: JSON → live interface powered by egui. No recompilation between edits. Bake into Rust code for release.

> 🛠 Created as part of learning vibecoding on [OpenCode](https://opencode.ai/go?ref=DHSKBMGTK0)
> ☕ If you find this project useful — [support the author](https://dalink.to/miska_moloka)

---

## Quick start

```bash
# 1. Create a new project
cargo init my_app && cd my_app

# 2. Add Rust4ui and eframe
cargo add rust4ui --git https://github.com/devZu9/Rust4ui
cargo add eframe

# 3. Write your UI (ui.json)
$ @"
{ \"type\": \"Label\", \"text\": \"Hello, Rust4ui!\", \"size\": 24, \"bold\": true }
"@ | Set-Content ui.json -Encoding UTF8

# 4. Replace main.rs (see below)

# 5. Run
cargo run
```

**main.rs:**

```rust
use rust4ui::prelude::*;

fn main() -> eframe::Result<()> {
    let ui = UiNode::from_file("ui.json");
    let theme = Theme::from_file("theme.json").unwrap_or_default();
    let locales = LocaleRegistry::new("en");
    let state = State::new();
    let actions = Actions::new();

    eframe::run_native("My App", Default::default(),
        Box::new(|_| Box::new(Rust4ui::new(ui, theme, locales, state, actions))))
}
```

Done. Edit `ui.json` — UI updates instantly. No `cargo build` between changes.

---

## What's inside

### Widgets

`Column` `Row` `Label` `Button` `TextField` `Checkbox` `RadioGroup` `Slider` `ComboBox` `Separator` `FileDrop` `Spinner` `Shortcut` `ColorPicker` `Indicator` `IconBar` `IconButton` `Caption` `StatusBar` `Tabs` `Tab` `Panel` `ScrollArea` `Window` `Hyperlink` `Image` `Grid` `MenuBar` `Menu` `MenuItem` `SubMenu` `Notifications` `Tray` `DataTable`

### Features

| Feature | Description |
|---------|-------------|
| **JSON → UI** | Write `ui.json` → see the window. No compilation step. |
| **i18n** | Multi-language from day one: `{{key}}`, CLDR plural, runtime switching |
| **Themes** | `theme.json` → colors, spacing, rounding. Hover/Focus/Disabled out of the box |
| **Flexibility** | Any widget can be customized: `opacity`, `rounding`, `padding`, `margin`, `shadow`, `stroke` |
| **Extensible** | Custom widgets from third-party crates via `WidgetRegistry` |
| **$ref** | Large UIs split into modules → each file < 200 lines |
| **Icons** | Phosphor font built-in + Texture Registry for PNG |
| **System tray** | Tray icon, context menu, icon animation (optional) |

---

## What it looks like

```json
{
  "type": "Column",
  "gap": 10,
  "padding": 16,
  "children": [
    {
      "type": "Label",
      "text": "{{app.title}}",
      "size": 20,
      "bold": true
    },
    {
      "type": "TextField",
      "binding": "name",
      "hint": "{{hint.name}}",
      "width": 300
    },
    {
      "type": "Button",
      "text": "{{btn.save}}",
      "action": "save",
      "fill": "#3366CC",
      "rounding": 6
    }
  ]
}
```

---

## Documentation

| File | About |
|------|-------|
| **[TECHNICAL_SPECIFICATION.md](TECHNICAL_SPECIFICATION.md)** | Full technical specification: JSON format, theme.json, state, actions, i18n, tests |
| **[ROADMAP.md](ROADMAP.md)** | Development plan (v0.1 → v1.0) |

---

## Demo

The `demo/` folder contains a prototype with all widgets across 5 tabs + 3 windows:

```
demo/ui.json → tabs/*.json → windows/*.json
```

- `ui.json` — 11 lines, 4 `$ref` to modules
- Each tab is a separate file (< 250 lines)
- 80+ locale keys (ru/en)
- 3 themes (dark, light, dj_green)

---

## Support

This project was created for personal needs as part of learning vibecoding on
[OpenCode](https://opencode.ai/go?ref=DHSKBMGTK0). I'm publishing it openly —
use it, fork it, improve it.

Unfortunately, I'm currently in a very difficult life situation, and any support
is critically important to me. If you find this project useful — please support
with a donation or follow the link. This helps not only to continue development,
but also to stave off hunger from time to time.

- ☕ **Donation** — support the author: https://dalink.to/miska_moloka
- 🚀 **OpenCode** — platform for vibecoding: https://opencode.ai/go?ref=DHSKBMGTK0

Thank you for being here. 🙏

---

## Acknowledgements

The author was inspired by the third-party [EFx](https://github.com/ZhukMax/efx) project. There is no technical dependency on EFx — the EFx project is not related to Rust4ui.

---

## License

MIT.
