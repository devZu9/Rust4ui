use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LocaleRegistry {
    pub current_lang: String,
    pub lang_name: String,
    locales: HashMap<String, HashMap<String, String>>,
}

impl LocaleRegistry {
    pub fn new(default_lang: &str) -> Self {
        Self {
            current_lang: default_lang.to_string(),
            lang_name: String::new(),
            locales: HashMap::new(),
        }
    }

    pub fn load_file(&mut self, lang: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        log::debug!(
            "LocaleRegistry: загружаю локализацию '{lang}' из {}",
            path.display()
        );
        self.load_str(lang, &content)
    }

    pub fn load_str(&mut self, lang: &str, json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: serde_json::Value = serde_json::from_str(json)?;

        let translations = if let Some(tr) = parsed.get("translations") {
            tr
        } else {
            &parsed
        };

        if let Some(name) = parsed.get("name").and_then(|v| v.as_str()) {
            self.lang_name = name.to_string();
        }

        let flat = flatten_locale(translations, String::new());
        let count = flat.len();
        self.locales
            .entry(lang.to_string())
            .or_default()
            .extend(flat);
        log::info!("LocaleRegistry: загружено {count} ключей для '{lang}'");
        Ok(())
    }

    pub fn switch(&mut self, lang: &str) {
        log::info!("LocaleRegistry: переключение на '{lang}'");
        self.current_lang = lang.to_string();
    }

    fn resolve_lang(&self, key: &str, lang: &str) -> Option<String> {
        self.locales.get(lang).and_then(|map| map.get(key)).cloned()
    }

    fn has_key(&self, key: &str) -> bool {
        self.locales.values().any(|map| map.contains_key(key))
    }

    fn resolve_plural(&self, key: &str, state: &crate::state::StateRegistry) -> Option<String> {
        if !self.has_key(&format!("{key}.one")) {
            return None;
        }
        let count_key = key.rsplit('.').next().unwrap_or("count");
        let n = state.get_f64(count_key).unwrap_or(0.0);
        let form = crate::plural::plural_form(&self.current_lang, n);
        let plural_key = format!("{key}.{form}");
        self.resolve_lang(&plural_key, &self.current_lang)
            .or_else(|| {
                let en_form = crate::plural::plural_form("en", n);
                self.resolve_lang(&format!("{key}.{en_form}"), "en")
            })
    }

    pub fn resolve(&self, key: &str) -> String {
        self.resolve_lang(key, &self.current_lang)
            .or_else(|| self.resolve_lang(key, "en"))
            .unwrap_or_else(|| {
                log::debug!(
                    "LocaleRegistry: ключ '{}' не найден ни в '{}', ни в 'en'",
                    key,
                    self.current_lang
                );
                format!("{{{{{key}}}}}")
            })
    }

    pub fn resolve_raw(&self, key: &str) -> Option<&str> {
        self.locales
            .get(&self.current_lang)
            .and_then(|map| map.get(key))
            .map(|s| s.as_str())
    }

    pub fn i18n_text(&self, text: &str, state: &crate::state::StateRegistry) -> String {
        let re = regex_lite::Regex::new(r"\{\{(.+?)\}\}").unwrap();
        if re.is_match(text) {
            let result = re
                .replace_all(text, |caps: &regex_lite::Captures| {
                    let key = caps.get(1).unwrap().as_str();
                    self.resolve_lang(key, &self.current_lang)
                        .or_else(|| self.resolve_lang(key, "en"))
                        .or_else(|| self.resolve_plural(key, state))
                        .unwrap_or_else(|| {
                            log::warn!(
                                "LocaleRegistry: ключ '{}' не найден ни в '{}', ни в 'en', нет plural-форм",
                                key,
                                self.current_lang
                            );
                            format!("{{{{{key}}}}}")
                        })
                })
                .to_string();
            self.interpolate_state(&result, state)
        } else {
            self.interpolate_state(text, state)
        }
    }

    pub fn interpolate_state(&self, text: &str, state: &crate::state::StateRegistry) -> String {
        let re = regex_lite::Regex::new(r"\{(\w+)\}").unwrap();
        re.replace_all(text, |caps: &regex_lite::Captures| {
            let key = caps.get(1).unwrap().as_str();
            if let Some(s) = state.get_string(key) {
                s.to_string()
            } else if let Some(v) = state.get_f64(key) {
                format!("{v}")
            } else if let Some(v) = state.get_i64(key) {
                format!("{v}")
            } else if let Some(v) = state.get_bool(key) {
                format!("{v}")
            } else {
                format!("{{{key}}}")
            }
        })
        .to_string()
    }
}

fn flatten_locale(value: &serde_json::Value, prefix: String) -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Some(obj) = value.as_object() {
        for (key, val) in obj {
            let full_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{prefix}.{key}")
            };
            match val {
                serde_json::Value::String(s) => {
                    result.insert(full_key, s.clone());
                }
                serde_json::Value::Object(_) => {
                    let nested = flatten_locale(val, full_key);
                    result.extend(nested);
                }
                _ => {}
            }
        }
    }
    result
}

impl Default for LocaleRegistry {
    fn default() -> Self {
        Self::new("ru")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::StateRegistry;

    #[test]
    fn test_locale_resolve() {
        let mut loc = LocaleRegistry::new("ru");
        loc.load_str("ru", r#"{"btn.save": "Сохранить", "btn.cancel": "Отмена"}"#)
            .unwrap();
        loc.load_str("en", r#"{"btn.save": "Save", "btn.cancel": "Cancel"}"#)
            .unwrap();

        assert_eq!(loc.resolve("btn.save"), "Сохранить");
        loc.switch("en");
        assert_eq!(loc.resolve("btn.save"), "Save");
    }

    #[test]
    fn test_locale_fallback_en() {
        let mut loc = LocaleRegistry::new("ru");
        loc.load_str("ru", r#"{"btn.save": "Сохранить"}"#).unwrap();
        loc.load_str("en", r#"{"btn.save": "Save", "btn.undo": "Undo"}"#)
            .unwrap();

        assert_eq!(loc.resolve("btn.undo"), "Undo");
        assert_eq!(loc.resolve("btn.save"), "Сохранить");
    }

    #[test]
    fn test_missing_key_returns_bracketed() {
        let mut loc = LocaleRegistry::new("ru");
        loc.load_str("ru", r#"{"btn.ok": "OK"}"#).unwrap();
        let result = loc.resolve("btn.missing");
        assert_eq!(result, "{{btn.missing}}");
    }

    #[test]
    fn test_plural_key_inline() {
        let mut loc = LocaleRegistry::new("ru");
        loc.load_str(
            "ru",
            r#"{"files": {"count": {"one": "{count} файл", "few": "{count} файла", "many": "{count} файлов"}}}"#,
        )
        .unwrap();

        let mut state = StateRegistry::new();
        state.set_f64("count", 1.0);
        let r = loc.i18n_text("{{files.count}}", &state);
        assert_eq!(r, "1 файл");

        state.set_f64("count", 2.0);
        let r = loc.i18n_text("{{files.count}}", &state);
        assert_eq!(r, "2 файла");

        state.set_f64("count", 5.0);
        let r = loc.i18n_text("{{files.count}}", &state);
        assert_eq!(r, "5 файлов");
    }

    #[test]
    fn test_locale_nested_keys() {
        let mut loc = LocaleRegistry::new("ru");
        let json =
            r#"{"translations": {"files": {"count": {"one": "1 файл", "few": "{count} файла"}}}}"#;
        loc.load_str("ru", json).unwrap();

        assert_eq!(loc.resolve("files.count.one"), "1 файл");
        assert_eq!(loc.resolve("files.count.few"), "{count} файла");
    }

    #[test]
    fn test_i18n_text() {
        let mut loc = LocaleRegistry::new("ru");
        loc.load_str("ru", r#"{"greeting": "Привет, {name}!"}"#)
            .unwrap();
        loc.load_str("en", r#"{"greeting": "Hello, {name}!"}"#)
            .unwrap();

        let mut state = StateRegistry::new();
        state.set_string("name", "Мир".into());

        let result = loc.i18n_text("{{greeting}}", &state);
        assert_eq!(result, "Привет, Мир!");
    }
}
