use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub path: String,
}

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, root: &Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        self.validate_node(root, "", &mut errors);
        errors
    }

    fn validate_node(&self, node: &Value, path: &str, errors: &mut Vec<ValidationError>) {
        let obj = match node {
            Value::Object(o) => o,
            _ => return,
        };

        let node_type = obj
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        if obj.get("binding").is_some() && obj.get("binding").and_then(|v| v.as_str()).is_none() {
            errors.push(ValidationError {
                message: format!("Атрибут 'binding' должен быть строкой (узел '{node_type}')"),
                path: path.to_string(),
            });
        }

        if let Some(action) = obj.get("action") {
            if action.as_str().is_none() {
                errors.push(ValidationError {
                    message: format!("Атрибут 'action' должен быть строкой (узел '{node_type}')"),
                    path: path.to_string(),
                });
            }
        }

        if let Some(items) = obj.get("items") {
            if items.as_str().is_none() {
                errors.push(ValidationError {
                    message: format!("Атрибут 'items' должен быть строкой — ключ в StateRegistry (узел '{node_type}')"),
                    path: path.to_string(),
                });
            }
        }

        if let Some(children) = obj.get("children") {
            if let Value::Array(arr) = children {
                for (i, child) in arr.iter().enumerate() {
                    let child_path = if path.is_empty() {
                        format!("children[{i}]")
                    } else {
                        format!("{path}.children[{i}]")
                    };
                    self.validate_node(child, &child_path, errors);
                }
            }
        }

        if let Some(ctx_menu) = obj.get("context_menu") {
            self.validate_node(ctx_menu, &format!("{path}.context_menu"), errors);
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_json() {
        let json: Value = serde_json::json!({
            "type": "Column",
            "children": [
                { "type": "Label", "text": "OK" },
                { "type": "Button", "text": "Save", "action": "save" }
            ]
        });
        let validator = Validator::new();
        let errors = validator.validate(&json);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_missing_binding() {
        let json: Value = serde_json::json!({
            "type": "TextField",
            "binding": 42
        });
        let validator = Validator::new();
        let errors = validator.validate(&json);
        assert!(!errors.is_empty());
    }
}
