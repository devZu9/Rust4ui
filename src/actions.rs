use crate::state::StateRegistry;
use std::collections::HashMap;

pub type ActionFn = Box<dyn Fn(&mut ActionCtx) + Send + Sync>;

pub struct ActionCtx {
    pub target: String,
    pub state: StateRegistry,
}

impl ActionCtx {
    pub fn new() -> Self {
        Self {
            target: String::new(),
            state: StateRegistry::new(),
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    pub fn with_state(mut self, state: &StateRegistry) -> Self {
        self.state = state.clone();
        self
    }
}

impl Default for ActionCtx {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ActionRegistry {
    actions: HashMap<String, ActionFn>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, action: F)
    where
        F: Fn(&mut ActionCtx) + Send + Sync + 'static,
    {
        self.actions.insert(name.to_string(), Box::new(action));
    }

    pub fn invoke(&self, name: &str, ctx: &mut ActionCtx) -> bool {
        if let Some(action) = self.actions.get(name) {
            action(ctx);
            true
        } else {
            log::warn!("ActionRegistry: экшен '{}' не зарегистрирован", name);
            false
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.actions.contains_key(name)
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_invoke() {
        let mut registry = ActionRegistry::new();
        registry.register("test", |ctx| {
            ctx.target = "invoked".into();
        });

        let mut ctx = ActionCtx::new();
        assert!(registry.invoke("test", &mut ctx));
        assert_eq!(ctx.target, "invoked");
    }

    #[test]
    fn test_action_target() {
        let mut registry = ActionRegistry::new();
        registry.register("save", |ctx| {
            assert!(!ctx.target.is_empty());
        });

        let mut ctx = ActionCtx::new().with_target("mic");
        assert!(registry.invoke("save", &mut ctx));
    }

    #[test]
    fn test_action_not_found() {
        let registry = ActionRegistry::new();
        let mut ctx = ActionCtx::new();
        assert!(!registry.invoke("missing", &mut ctx));
    }
}
