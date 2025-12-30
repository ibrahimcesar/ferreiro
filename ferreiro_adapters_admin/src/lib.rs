/// Admin introspection and auto-generation
/// This will be expanded in future iterations

#[derive(Debug, Clone)]
pub struct AdminField {
    pub name: &'static str,
    pub display_name: String,
    pub field_type: AdminFieldType,
    pub required: bool,
    pub editable: bool,
}

#[derive(Debug, Clone)]
pub enum AdminFieldType {
    String { max_length: Option<usize> },
    Text,
    Integer,
    Boolean,
    DateTime,
    ForeignKey { model: &'static str },
    Enum { variants: Vec<String> },
}

pub trait AdminModel: Send + Sync {
    fn name(&self) -> &'static str;
    fn name_plural(&self) -> &'static str;
    fn fields(&self) -> Vec<AdminField>;
    fn primary_key(&self) -> &'static str;
    fn display(&self, instance: &dyn std::any::Any) -> String;
}

pub trait ModelAdmin: Send + Sync {
    fn list_display(&self) -> Vec<&'static str> {
        vec![]
    }
    fn list_filter(&self) -> Vec<&'static str> {
        vec![]
    }
    fn search_fields(&self) -> Vec<&'static str> {
        vec![]
    }
    fn readonly_fields(&self) -> Vec<&'static str> {
        vec![]
    }
    fn ordering(&self) -> Vec<&'static str> {
        vec!["-created_at"]
    }
}
