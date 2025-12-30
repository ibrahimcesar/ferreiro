use crate::{Context, TemplateEngine, TemplateError};
use minijinja::Environment;
use std::sync::RwLock;

pub struct MiniJinjaEngine {
    env: RwLock<Environment<'static>>,
}

impl MiniJinjaEngine {
    pub fn new(template_dir: &str) -> Result<Self, TemplateError> {
        let mut env = Environment::new();
        env.set_loader(minijinja::path_loader(template_dir));
        Ok(Self {
            env: RwLock::new(env),
        })
    }
}

impl TemplateEngine for MiniJinjaEngine {
    fn render(&self, name: &str, context: &Context) -> Result<String, TemplateError> {
        let env = self.env.read().unwrap();
        let template = env
            .get_template(name)
            .map_err(|e| TemplateError::NotFound(e.to_string()))?;
        template
            .render(&context.data)
            .map_err(|e| TemplateError::Render(e.to_string()))
    }

    fn render_string(&self, template: &str, context: &Context) -> Result<String, TemplateError> {
        let env = self.env.read().unwrap();
        env.render_str(template, &context.data)
            .map_err(|e| TemplateError::Render(e.to_string()))
    }
}
