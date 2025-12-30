use crate::{Context, TemplateEngine, TemplateError};
use std::sync::RwLock;
use tera::Tera;

pub struct TeraEngine {
    tera: RwLock<Tera>,
}

impl TeraEngine {
    pub fn new(template_dir: &str) -> Result<Self, TemplateError> {
        let glob = format!("{}/**/*", template_dir);
        let tera = Tera::new(&glob).map_err(|e| TemplateError::Parse(e.to_string()))?;
        Ok(Self {
            tera: RwLock::new(tera),
        })
    }

    pub fn from_tera(tera: Tera) -> Self {
        Self {
            tera: RwLock::new(tera),
        }
    }
}

impl TemplateEngine for TeraEngine {
    fn render(&self, name: &str, context: &Context) -> Result<String, TemplateError> {
        let tera = self.tera.read().unwrap();
        let tera_context = tera::Context::from_serialize(&context.data)
            .map_err(|e| TemplateError::Render(e.to_string()))?;
        tera.render(name, &tera_context)
            .map_err(|e| TemplateError::Render(e.to_string()))
    }

    fn render_string(&self, template: &str, context: &Context) -> Result<String, TemplateError> {
        let mut tera = self.tera.write().unwrap();
        let tera_context = tera::Context::from_serialize(&context.data)
            .map_err(|e| TemplateError::Render(e.to_string()))?;
        tera.render_str(template, &tera_context)
            .map_err(|e| TemplateError::Render(e.to_string()))
    }
}
