use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub name: String,
    pub color: Option<String>,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name, color: None }
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl From<String> for Tag {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for Tag {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}
