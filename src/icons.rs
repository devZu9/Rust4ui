use std::collections::HashMap;

/// Реестр иконок Phosphor.
///
/// Загружает `icons.json` (1512 иконок из официальной коллекции Phosphor).
/// Иконки вкомпилированы в бинарь через `include_str!`.
///
/// ## Будущее: SVG-текстуры
///
/// Планируется поддержка SVG-иконок из `icons/phosphor-icons/SVGs/` и
/// `icons/phosphor-icons/SVGs Flat/` с рендерингом через `usvg` + `resvg`.
/// Формат в JSON: `"icon": "SVGs/regular/acorn"` или `"icon": "SVGs Flat/duotone/acorn-duotone"`.
/// Это позволит использовать многоцветные duotone-иконки и кастомные стили.
#[derive(Debug, Clone)]
pub struct IconRegistry {
    map: HashMap<String, String>,
}

impl IconRegistry {
    pub fn new() -> Self {
        let raw = include_str!("../icons/icons.json");
        let parsed: HashMap<String, String> =
            serde_json::from_str(raw).unwrap_or_default();
        log::info!("IconRegistry: загружено {} иконок", parsed.len());
        Self { map: parsed }
    }

    pub fn resolve(&self, name: &str) -> Option<&str> {
        self.map.get(name).map(|s| s.as_str())
    }

    pub fn resolve_glyph(&self, name: &str) -> String {
        self.resolve(name).unwrap_or_else(|| {
            log::warn!("IconRegistry: иконка '{name}' не найдена");
            "⬡"
        }).to_string()
    }

    pub fn count(&self) -> usize {
        self.map.len()
    }

    pub fn has(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
}

impl Default for IconRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1512_icons_loaded() {
        let reg = IconRegistry::new();
        assert_eq!(reg.count(), 1512);
    }

    #[test]
    fn test_common_icons_exist() {
        let reg = IconRegistry::new();
        // Реальные имена из официальной коллекции Phosphor
        let names = [
            "acorn", "alarm", "arrow-down", "arrow-left", "arrow-right", "arrow-up",
            "bell", "book", "calendar", "camera", "chat", "check",
            "check-circle", "clipboard", "clock", "cloud", "copy-simple",
            "download", "download-simple", "eye", "file", "folder-simple",
            "gear-six", "headphones", "heart", "house", "house-simple",
            "image", "info", "key", "lightbulb", "link-simple",
            "lock-simple", "lock-simple-open", "magnifying-glass",
            "map-pin", "microphone", "minus", "moon", "music-note",
            "note", "notification", "pencil-simple", "phone",
            "play", "plus", "power", "question", "share-network", "shield",
            "shopping-cart", "speaker-high", "speaker-none", "star",
            "sun", "tag", "trash-simple", "upload-simple",
            "user", "user-circle", "video-camera", "warning",
            "wifi-high", "wrench", "x", "x-circle",
        ];
        for name in &names {
            assert!(reg.has(name), "иконка '{name}' не найдена в коллекции");
        }
    }

    #[test]
    fn test_resolve_unknown() {
        let reg = IconRegistry::new();
        assert_eq!(reg.resolve("nonexistent"), None);
    }

    #[test]
    fn test_resolve_glyph_fallback() {
        let reg = IconRegistry::new();
        assert_eq!(reg.resolve_glyph("nonexistent"), "⬡");
    }

    #[test]
    fn test_resolve_known_glyph() {
        let reg = IconRegistry::new();
        let glyph = reg.resolve("acorn").unwrap();
        assert!(!glyph.is_empty());
    }
}
