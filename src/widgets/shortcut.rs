use crate::renderer::{attr_str, RenderCtx};

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, ctx: &mut RenderCtx) {
    let key = attr_str(node, "key");
    let action = attr_str(node, "action");
    let target = attr_str(node, "target").unwrap_or("");

    if let (Some(key_combo), Some(action_name)) = (key, action) {
        let shortcut = parse_shortcut(key_combo);
        if ui.input_mut(|i| i.consume_shortcut(&shortcut)) {
            let mut action_ctx = crate::actions::ActionCtx::new()
                .with_target(target)
                .with_state(&ctx.state);
            ctx.actions.invoke(action_name, &mut action_ctx);
            ctx.state = action_ctx.state;
        }
    }
}

fn parse_shortcut(s: &str) -> egui::KeyboardShortcut {
    let mut modifiers = egui::Modifiers::NONE;
    let mut key = egui::Key::Escape;

    let parts: Vec<&str> = s.split('+').collect();
    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers.ctrl = true,
            "shift" => modifiers.shift = true,
            "alt" => modifiers.alt = true,
            "s" => key = egui::Key::S,
            "z" => key = egui::Key::Z,
            "y" => key = egui::Key::Y,
            "n" => key = egui::Key::N,
            "o" => key = egui::Key::O,
            "f" => key = egui::Key::F,
            "q" => key = egui::Key::Q,
            "w" => key = egui::Key::W,
            "e" => key = egui::Key::E,
            "r" => key = egui::Key::R,
            "t" => key = egui::Key::T,
            "a" => key = egui::Key::A,
            "d" => key = egui::Key::D,
            "g" => key = egui::Key::G,
            "h" => key = egui::Key::H,
            "f4" => key = egui::Key::F4,
            _ => {}
        }
    }

    egui::KeyboardShortcut::new(modifiers, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_shortcut() {
        let json = serde_json::json!({"type": "Shortcut", "key": "Ctrl+S", "action": "save"});
        assert_eq!(attr_str(&json, "key"), Some("Ctrl+S"));
    }
}
