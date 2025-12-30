use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Context {
    pub data: HashMap<String, serde_json::Value>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(v) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), v);
        }
    }
}

#[macro_export]
macro_rules! context {
    ($($key:ident : $value:expr),* $(,)?) => {{
        let mut ctx = $crate::Context::new();
        $(ctx.insert(stringify!($key), $value);)*
        ctx
    }};
}

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    NotFound(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Render error: {0}")]
    Render(String),
}

pub trait TemplateEngine: Send + Sync {
    fn render(&self, name: &str, context: &Context) -> Result<String, TemplateError>;
    fn render_string(&self, template: &str, context: &Context) -> Result<String, TemplateError>;
}

#[cfg(feature = "tera-engine")]
pub mod tera_adapter;

#[cfg(feature = "minijinja-engine")]
pub mod minijinja_adapter;
