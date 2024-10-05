pub trait Fitness {
    fn calculate(&self, indivisual: &Vec<bool>) -> f32;
}
