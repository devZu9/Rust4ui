use crate::strip_json_comments;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum StateValue {
    String(String),
    F64(f64),
    I64(i64),
    Usize(usize),
    Bool(bool),
    VecString(Vec<String>),
}

#[derive(Debug, Clone, Default)]
pub struct StateRegistry {
    data: HashMap<String, StateValue>,
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set_string(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), StateValue::String(value));
    }

    pub fn set_f64(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), StateValue::F64(value));
    }

    pub fn set_i64(&mut self, key: &str, value: i64) {
        self.data.insert(key.to_string(), StateValue::I64(value));
    }

    pub fn set_usize(&mut self, key: &str, value: usize) {
        self.data.insert(key.to_string(), StateValue::Usize(value));
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.data.insert(key.to_string(), StateValue::Bool(value));
    }

    pub fn set_vec_string(&mut self, key: &str, value: Vec<String>) {
        self.data
            .insert(key.to_string(), StateValue::VecString(value));
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        match self.data.get(key) {
            Some(StateValue::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        match self.data.get(key) {
            Some(StateValue::F64(v)) => Some(*v),
            Some(StateValue::I64(v)) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        match self.data.get(key) {
            Some(StateValue::I64(v)) => Some(*v),
            Some(StateValue::F64(v)) => Some(*v as i64),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.data.get(key) {
            Some(StateValue::Bool(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_usize(&self, key: &str) -> Option<usize> {
        match self.data.get(key) {
            Some(StateValue::Usize(v)) => Some(*v),
            Some(StateValue::I64(v)) => Some(*v as usize),
            _ => None,
        }
    }

    pub fn get_vec_string(&self, key: &str) -> Option<&Vec<String>> {
        match self.data.get(key) {
            Some(StateValue::VecString(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn to_json(&self) -> String {
        let mut map = serde_json::Map::new();
        for (key, val) in &self.data {
            let json_val = match val {
                StateValue::String(s) => serde_json::Value::String(s.clone()),
                StateValue::F64(v) => serde_json::json!(*v),
                StateValue::I64(v) => serde_json::json!(*v),
                StateValue::Usize(v) => serde_json::json!(*v),
                StateValue::Bool(v) => serde_json::Value::Bool(*v),
                StateValue::VecString(v) => serde_json::Value::Array(
                    v.iter()
                        .map(|s| serde_json::Value::String(s.clone()))
                        .collect(),
                ),
            };
            map.insert(key.clone(), json_val);
        }
        serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap_or_default()
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        std::fs::write(path, self.to_json()).map_err(|e| format!("Ошибка сохранения состояния: {e}"))
    }

    pub fn load(path: &std::path::Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(content) => Self::from_json(&crate::strip_json_comments(&content)).unwrap_or_default(),
            Err(_) => Self::new(),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let parsed: serde_json::Value = serde_json::from_str(&strip_json_comments(json))?;
        let mut state = Self::new();
        if let serde_json::Value::Object(map) = parsed {
            for (key, val) in map {
                match val {
                    serde_json::Value::String(s) => state.set_string(&key, s),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            state.set_i64(&key, i);
                        } else if let Some(f) = n.as_f64() {
                            state.set_f64(&key, f);
                        }
                    }
                    serde_json::Value::Bool(b) => state.set_bool(&key, b),
                    serde_json::Value::Array(arr) => {
                        let strings: Vec<String> = arr
                            .into_iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();
                        state.set_vec_string(&key, strings);
                    }
                    _ => {}
                }
            }
        }
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_bind_read_write() {
        let mut state = StateRegistry::new();
        state.set_string("name", "Test".into());
        state.set_f64("volume", 75.0);
        state.set_bool("gpu", true);

        assert_eq!(state.get_string("name"), Some("Test"));
        assert_eq!(state.get_f64("volume"), Some(75.0));
        assert_eq!(state.get_bool("gpu"), Some(true));
    }

    #[test]
    fn test_state_wrong_type() {
        let mut state = StateRegistry::new();
        state.set_string("name", "Test".into());
        assert_eq!(state.get_f64("name"), None);
        assert_eq!(state.get_bool("name"), None);
    }

    #[test]
    fn test_state_roundtrip_json() {
        let mut state = StateRegistry::new();
        state.set_string("name", "Test".into());
        state.set_f64("volume", 42.0);
        state.set_bool("gpu", true);

        let json = state.to_json();
        let restored = StateRegistry::from_json(&json).unwrap();

        assert_eq!(restored.get_string("name"), Some("Test"));
        assert_eq!(restored.get_f64("volume"), Some(42.0));
        assert_eq!(restored.get_bool("gpu"), Some(true));
    }

    #[test]
    fn test_state_vec_string() {
        let mut state = StateRegistry::new();
        state.set_vec_string("items", vec!["A".into(), "B".into()]);
        assert_eq!(state.get_vec_string("items").unwrap(), &vec!["A", "B"]);
    }

    #[test]
    fn test_state_save_load_roundtrip() {
        let dir = std::env::temp_dir().join("rust4ui_test_state");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.json");

        let mut state = StateRegistry::new();
        state.set_string("active_tab", "basic".into());
        state.set_f64("window_size_width", 1024.0);
        state.set_f64("window_size_height", 768.0);
        assert!(state.save(&path).is_ok());

        let loaded = StateRegistry::load(&path);
        assert_eq!(loaded.get_string("active_tab"), Some("basic"));
        assert_eq!(loaded.get_f64("window_size_width"), Some(1024.0));
        assert_eq!(loaded.get_f64("window_size_height"), Some(768.0));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
