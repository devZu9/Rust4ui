use crate::strip_json_comments;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct RefResolver {
    cache: HashMap<PathBuf, Value>,
    resolve_stack: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum ResolveError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Cycle(Vec<PathBuf>),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::Io(e) => write!(f, "Ошибка ввода-вывода: {e}"),
            ResolveError::Parse(e) => write!(f, "Ошибка парсинга JSON: {e}"),
            ResolveError::Cycle(chain) => {
                let names: Vec<_> = chain.iter().map(|p| p.display().to_string()).collect();
                write!(f, "Обнаружен цикл $ref: {}", names.join(" → "))
            }
        }
    }
}

impl RefResolver {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            resolve_stack: Vec::new(),
        }
    }

    pub fn resolve(&mut self, root: &Value, base_dir: &Path) -> Result<Value, ResolveError> {
        log::info!("RefResolver: начинаю резолвинг в {}", base_dir.display());
        self.resolve_value(root.clone(), base_dir)
    }

    fn resolve_value(&mut self, value: Value, base_dir: &Path) -> Result<Value, ResolveError> {
        match value {
            Value::Object(map) => {
                if let Some(ref_path) = map.get("$ref").and_then(|v| v.as_str()) {
                    let resolved = self.resolve_file(ref_path, base_dir)?;
                    let mut merged = resolved;
                    if let Value::Object(ref mut merged_map) = merged {
                        for (key, val) in &map {
                            if key != "$ref" {
                                merged_map.insert(key.clone(), val.clone());
                            }
                        }
                    }
                    Ok(merged)
                } else {
                    let mut new_map = serde_json::Map::new();
                    for (key, val) in map {
                        if key == "children" {
                            if let Value::Array(arr) = val {
                                let resolved: Result<Vec<_>, _> = arr
                                    .into_iter()
                                    .map(|v| self.resolve_value(v, base_dir))
                                    .collect();
                                new_map.insert(key, Value::Array(resolved?));
                            } else {
                                new_map.insert(key, val);
                            }
                        } else if key == "context_menu" {
                            let resolved = self.resolve_value(val, base_dir)?;
                            new_map.insert(key, resolved);
                        } else {
                            new_map.insert(key, val);
                        }
                    }
                    Ok(Value::Object(new_map))
                }
            }
            Value::Array(arr) => {
                let resolved: Result<Vec<_>, _> = arr
                    .into_iter()
                    .map(|v| self.resolve_value(v, base_dir))
                    .collect();
                Ok(Value::Array(resolved?))
            }
            other => Ok(other),
        }
    }

    fn resolve_file(&mut self, path: &str, base_dir: &Path) -> Result<Value, ResolveError> {
        let full_path = base_dir.join(path);

        if self.resolve_stack.contains(&full_path) {
            log::error!(
                "RefResolver: обнаружен цикл! Цепочка: {:?}",
                self.resolve_stack
            );
            return Err(ResolveError::Cycle(self.resolve_stack.clone()));
        }

        if let Some(cached) = self.cache.get(&full_path) {
            log::info!("RefResolver: кэш — {}", full_path.display());
            return Ok(cached.clone());
        }

        log::info!("RefResolver: загружаю {}", full_path.display());
        let content = fs::read_to_string(&full_path).map_err(|e| {
            log::error!(
                "RefResolver: не удалось прочитать {}: {e}",
                full_path.display()
            );
            ResolveError::Io(e)
        })?;
        let parsed: Value = serde_json::from_str(&strip_json_comments(&content)).map_err(|e| {
            log::error!(
                "RefResolver: невалидный JSON в {}: {e}",
                full_path.display()
            );
            ResolveError::Parse(e)
        })?;

        self.cache.insert(full_path.clone(), parsed.clone());
        self.resolve_stack.push(full_path.clone());

        let file_dir = full_path.parent().unwrap_or(base_dir).to_path_buf();
        let resolved = self.resolve_value(parsed, &file_dir);

        self.resolve_stack.pop();
        resolved
    }
}

impl Default for RefResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn setup_test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rust4ui_ref_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_ref_simple() {
        let dir = setup_test_dir("simple");
        fs::write(
            dir.join("label.json"),
            r#"{"type": "Label", "text": "Hello"}"#,
        )
        .unwrap();

        let json = serde_json::json!({"type": "Column", "children": [{"$ref": "label.json"}]});
        let mut resolver = RefResolver::new();
        let resolved = resolver.resolve(&json, &dir).unwrap();

        let children = resolved["children"].as_array().unwrap();
        assert_eq!(children[0]["type"], "Label");
        assert_eq!(children[0]["text"], "Hello");
    }

    #[test]
    fn test_ref_nested() {
        let dir = setup_test_dir("nested");
        fs::write(
            dir.join("inner.json"),
            r#"{"type": "Row", "children": [{"type": "Button", "text": "OK"}]}"#,
        )
        .unwrap();
        fs::write(
            dir.join("outer.json"),
            r#"{"type": "Column", "children": [{"$ref": "inner.json"}]}"#,
        )
        .unwrap();

        let json = serde_json::json!({"$ref": "outer.json"});
        let mut resolver = RefResolver::new();
        let resolved = resolver.resolve(&json, &dir).unwrap();

        assert_eq!(resolved["type"], "Column");
        let children = resolved["children"].as_array().unwrap();
        assert_eq!(children[0]["type"], "Row");
    }

    #[test]
    fn test_ref_cycle() {
        let dir = setup_test_dir("cycle");
        fs::write(dir.join("a.json"), r#"{"$ref": "b.json"}"#).unwrap();
        fs::write(dir.join("b.json"), r#"{"$ref": "a.json"}"#).unwrap();

        let json = serde_json::json!({"$ref": "a.json"});
        let mut resolver = RefResolver::new();
        let result = resolver.resolve(&json, &dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_ref_override() {
        let dir = setup_test_dir("override");
        fs::write(
            dir.join("btn.json"),
            r#"{"type": "Button", "text": "Default"}"#,
        )
        .unwrap();

        let json = serde_json::json!({"$ref": "btn.json", "text": "Overridden"});
        let mut resolver = RefResolver::new();
        let resolved = resolver.resolve(&json, &dir).unwrap();

        assert_eq!(resolved["type"], "Button");
        assert_eq!(resolved["text"], "Overridden");
    }
}
