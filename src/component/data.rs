use yew::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct WorkType {
    pub id: usize,
    pub name: String,
    pub color: String,
}

impl WorkType {
    pub fn new(id: usize, name: &str, color: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            color: color.to_string(),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct UserTag {
    pub id: usize,
    pub label: String,
}

impl UserTag {
    pub fn new(id: usize, label: &str) -> Self {
        Self { id, label: label.to_string() }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct UserConfig {
    pub id: usize,
    pub name: String,
    pub tags: Vec<usize>,
}

impl UserConfig {
    pub fn new(id: usize, name: &str, tags: Vec<usize>) -> Self {
        Self {
            id,
            name: name.to_string(),
            tags,
        }
    }
}
