use eframe::egui;
use std::collections::HashMap;

fn relative_luminance(c: &egui::Color32) -> f32 {
    fn linearize(v: f32) -> f32 {
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    let r = linearize(c.r() as f32 / 255.0);
    let g = linearize(c.g() as f32 / 255.0);
    let b = linearize(c.b() as f32 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

pub fn wcag_contrast(a: &egui::Color32, b: &egui::Color32) -> f32 {
    let l1 = relative_luminance(a);
    let l2 = relative_luminance(b);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Проверяет только текст-на-фоне пары. Пропускает stroke/border/fill-цвета, которые не несут текста.
pub fn check_theme_contrasts(colors: &HashMap<String, String>, logger: &str) {
    let text_keys = [
        "text_primary",
        "text_dim",
        "label_color",
        "button_text_color",
        "link_color",
        "accent",
        "success",
        "danger",
        "warning",
    ];

    let default_bg = parse_color_or(colors.get("bg_fill").map(|s| s.as_str()), "#14161B");

    for key in &text_keys {
        if let Some(hex) = colors.get(*key) {
            let ratio = wcag_contrast(
                &crate::theme::parse_color_hex(hex).unwrap_or(default_bg),
                &default_bg,
            );
            if ratio < 3.0 {
                log::warn!(
                    "[{logger}] Контраст текста '{key}' ({hex}) на bg_fill = {ratio:.1}:1 — ниже 3:1"
                );
            }
        }
    }
}

pub fn check_contrast_pair(fg_hex: &str, bg_hex: &str, label: &str, logger: &str) {
    if let (Some(fg), Some(bg)) = (
        crate::theme::parse_color_hex(fg_hex),
        crate::theme::parse_color_hex(bg_hex),
    ) {
        if fg == bg {
            return;
        }
        let ratio = wcag_contrast(&fg, &bg);
        if ratio < 3.0 {
            log::warn!(
                "[{logger}] Контраст {label}: {fg_hex} на {bg_hex} = {ratio:.1}:1 — ниже 3:1"
            );
        }
    }
}

fn parse_color_or(hex: Option<&str>, default: &str) -> egui::Color32 {
    hex.and_then(crate::theme::parse_color_hex)
        .unwrap_or_else(|| crate::theme::parse_color_hex(default).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wcag_contrast() {
        let white = egui::Color32::WHITE;
        let black = egui::Color32::BLACK;
        let ratio = wcag_contrast(&white, &black);
        assert!(ratio > 20.0, "Белый/чёрный контраст должен быть > 20, получено {ratio}");
    }

    #[test]
    fn test_same_color_contrast() {
        let gray = egui::Color32::from_rgb(0x80, 0x80, 0x80);
        let ratio = wcag_contrast(&gray, &gray);
        assert!(ratio < 1.1, "Одинаковый цвет: контраст ~1, получено {ratio}");
    }

    #[test]
    fn test_default_theme_contrasts() {
        let theme = crate::theme::Theme::default();
        assert!(!theme.colors.is_empty());

        let bg = theme
            .color("background")
            .unwrap_or(egui::Color32::from_rgb(0x14, 0x16, 0x1B));
        let text = theme
            .color("text_primary")
            .unwrap_or(egui::Color32::from_rgb(0xE0, 0xE0, 0xE0));
        let ratio = wcag_contrast(&text, &bg);
        assert!(
            ratio >= 3.0,
            "Контраст текста на фоне должен быть >= 3:1, получено {ratio}:1"
        );
    }
}

