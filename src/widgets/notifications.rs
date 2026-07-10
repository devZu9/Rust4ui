use crate::renderer::{attr_f64, attr_str, RenderCtx};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Notification {
    pub text: String,
    pub level: String,
    pub ttl: f32,
    pub elapsed: f32,
}

pub fn render(ui: &mut egui::Ui, node: &serde_json::Value, _ctx: &mut RenderCtx) {
    let position = attr_str(node, "position").unwrap_or("top-right");
    let _max_count = attr_f64(node, "max_count").unwrap_or(5.0) as usize;
    let _width = attr_f64(node, "width").unwrap_or(300.0);

    let _ = position;
    // Notifications are pushed from action handlers; zone is a placeholder
    ui.add_space(4.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_notifications() {
        let json = serde_json::json!({
            "type": "Notifications",
            "position": "top-right",
            "max_count": 5
        });
        assert_eq!(attr_str(&json, "position"), Some("top-right"));
    }
}
