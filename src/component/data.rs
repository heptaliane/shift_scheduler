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
