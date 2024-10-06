use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub trait Selection {
    fn get_selection_iter(&self, fitness: &Vec<f32>) -> Result<SelectionIter, ()>;
}

pub struct SelectionIter {
    weights: WeightedIndex<f32>,
    rng: ThreadRng,
}

impl SelectionIter {
    fn new(weights: WeightedIndex<f32>) -> Self {
        Self {
            weights,
            rng: thread_rng(),
        }
    }
}

impl Iterator for SelectionIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        Some((
            self.weights.sample(&mut self.rng),
            self.weights.sample(&mut self.rng),
        ))
    }
}

pub struct RouletteWheelSelection {}

impl Selection for RouletteWheelSelection {
    fn get_selection_iter(&self, fitness: &Vec<f32>) -> Result<SelectionIter, ()> {
        match WeightedIndex::new(fitness) {
            Ok(weights) => Ok(SelectionIter::new(weights)),
            _ => Err(()),
        }
    }
}

pub struct RankSelection<F>
where
    F: Fn(usize) -> f32,
{
    probability: F,
}

impl<F> Selection for RankSelection<F>
where
    F: Fn(usize) -> f32,
{
    fn get_selection_iter(&self, fitness: &Vec<f32>) -> Result<SelectionIter, ()> {
        let mut indexed_fitness: Vec<(usize, f32)> =
            fitness.iter().enumerate().map(|(i, &f)| (i, f)).collect();
        indexed_fitness.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());
        let mut indexed_rank: Vec<(usize, usize)> = indexed_fitness
            .iter()
            .enumerate()
            .map(|(rank, &(index, _))| (index, rank))
            .collect();
        indexed_rank.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        let weights = indexed_rank
            .iter()
            .map(|&(_, rank)| (self.probability)(rank));

        match WeightedIndex::new(weights) {
            Ok(weights) => Ok(SelectionIter::new(weights)),
            _ => Err(()),
        }
    }
}

#[test]
fn test_roulette_wheel_selection() {
    let selection = RouletteWheelSelection {};

    let fitness1 = vec![1.0, 2.0, 3.0];
    assert!(selection.get_selection_iter(&fitness1).is_ok());

    let fitness2 = vec![1.0];
    assert!(selection.get_selection_iter(&fitness2).is_ok());

    // non-positive fitness is not acceptable
    let fitness3 = vec![0.0; 3];
    assert!(selection.get_selection_iter(&fitness3).is_err());

    let fitness4: Vec<f32> = Vec::new();
    assert!(selection.get_selection_iter(&fitness4).is_err());
}

#[test]
fn test_rank_selection() {
    let probability = |rank: usize| 1.0 / ((rank + 1) as f32);
    let selection = RankSelection { probability };

    let fitness1 = vec![1.0, 2.0, 3.0];
    assert!(selection.get_selection_iter(&fitness1).is_ok());

    let fitness2 = vec![1.0];
    assert!(selection.get_selection_iter(&fitness2).is_ok());

    let fitness3 = vec![0.0; 3];
    assert!(selection.get_selection_iter(&fitness3).is_ok());

    let fitness4: Vec<f32> = Vec::new();
    assert!(selection.get_selection_iter(&fitness4).is_err());
}
