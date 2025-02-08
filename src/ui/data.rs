use yew::{AttrValue, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct WorkType {
    pub id: usize,
    pub name: AttrValue,
    pub color: AttrValue,
}

#[derive(Properties, PartialEq, Clone)]
pub struct UserConfig {
    pub id: usize,
    pub name: AttrValue,
}

#[derive(Properties, PartialEq, Clone)]
pub struct UserTag {
    pub id: usize,
    pub label: AttrValue,
}
