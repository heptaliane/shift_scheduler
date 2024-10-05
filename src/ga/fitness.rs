pub trait Fitness {
    fn calculate(&self, indivisual: &Vec<bool>) -> f32;
}

pub struct CountFitness {}

impl Fitness for CountFitness {
    fn calculate(&self, indivisual: &Vec<bool>) -> f32 {
        indivisual.iter().filter(|&&v| v).count() as f32
    }
}

#[test]
fn test_count_fitness() {
    let arr1 = vec![true; 3];
    let arr2 = vec![false; 3];
    let arr3 = vec![true, false, true];

    let fitness = CountFitness {};

    assert_eq!(fitness.calculate(&arr1), 3.0);
    assert_eq!(fitness.calculate(&arr2), 0.0);
    assert_eq!(fitness.calculate(&arr3), 2.0);
}
