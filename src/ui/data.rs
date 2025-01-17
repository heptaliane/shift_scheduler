use yew::{Properties, AttrValue};

#[derive(Properties, PartialEq, Clone)]
pub struct WorkType {
    pub id: usize,
    pub name: AttrValue,
    pub color: AttrValue,
}
