use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Удаляет комментарии из JSON-строки:
/// - `//` однострочные (удаляет от `//` до конца строки)
/// - `/* */` блочные (удаляет от `/*` до `*/`).
/// Не трогает `//` и `/*` внутри строковых литералов.
pub fn strip_json_comments(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let chars: Vec<char> = raw.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Строковый литерал — копируем как есть
        if c == '"' {
            out.push('"');
            i += 1;
            while i < len {
                let sc = chars[i];
                out.push(sc);
                i += 1;
                if sc == '\\' && i < len {
                    out.push(chars[i]);
                    i += 1;
                } else if sc == '"' {
                    break;
                }
            }
            continue;
        }

        // Однострочный комментарий //
        if c == '/' && i + 1 < len && chars[i + 1] == '/' {
            i += 2;
            while i < len && chars[i] != '\n' && chars[i] != '\r' {
                i += 1;
            }
            continue;
        }

        // Блочный комментарий /* */
        if c == '/' && i + 1 < len && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2; // пропускаем */
            continue;
        }

        out.push(c);
        i += 1;
    }

    out
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiNode {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub children: Vec<Value>,
    #[serde(flatten)]
    pub attrs: Value,
}

impl UiNode {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&strip_json_comments(json))
    }

    pub fn from_value(value: &Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value.clone())
    }

    pub fn node_type(&self) -> &str {
        &self.node_type
    }

    pub fn attr_str(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).and_then(|v| v.as_str())
    }

    pub fn attr_f64(&self, key: &str) -> Option<f64> {
        self.attrs.get(key).and_then(|v| v.as_f64())
    }

    pub fn attr_i64(&self, key: &str) -> Option<i64> {
        self.attrs.get(key).and_then(|v| v.as_i64())
    }

    pub fn attr_bool(&self, key: &str) -> Option<bool> {
        self.attrs.get(key).and_then(|v| v.as_bool())
    }

    pub fn attr_array(&self, key: &str) -> Option<&Vec<Value>> {
        self.attrs.get(key).and_then(|v| v.as_array())
    }

    pub fn has_attr(&self, key: &str) -> bool {
        self.attrs.get(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_node() {
        let json = r#"{"type": "Label", "text": "Привет"}"#;
        let node = UiNode::from_json(json).unwrap();
        assert_eq!(node.node_type(), "Label");
        assert_eq!(node.attr_str("text"), Some("Привет"));
    }

    #[test]
    fn test_node_with_children() {
        let json = r#"{"type": "Column", "gap": 8, "children": [{"type": "Label", "text": "OK"}]}"#;
        let node = UiNode::from_json(json).unwrap();
        assert_eq!(node.node_type(), "Column");
        assert_eq!(node.attr_f64("gap"), Some(8.0));
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_attr_bool() {
        let json = r#"{"type": "Label", "bold": true, "italic": false}"#;
        let node = UiNode::from_json(json).unwrap();
        assert_eq!(node.attr_bool("bold"), Some(true));
        assert_eq!(node.attr_bool("italic"), Some(false));
    }

    #[test]
    fn test_missing_attr() {
        let json = r#"{"type": "Label"}"#;
        let node = UiNode::from_json(json).unwrap();
        assert!(node.attr_str("text").is_none());
        assert!(node.attr_f64("gap").is_none());
    }
}
