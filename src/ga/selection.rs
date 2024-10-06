use super::fitness::Fitness;
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub trait Selection {
    fn get_selection_iter(&self, individuals: &Vec<Vec<bool>>) -> Result<SelectionIter, ()>;
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

pub struct RouletteWheelSelection<F>
where
    F: Fitness,
{
    fitness: F,
}

impl<F> Selection for RouletteWheelSelection<F>
where
    F: Fitness,
{
    fn get_selection_iter(&self, individuals: &Vec<Vec<bool>>) -> Result<SelectionIter, ()> {
        let fitness = individuals.iter().map(|i| self.fitness.calculate(i));
        match WeightedIndex::new(fitness) {
            Ok(weights) => Ok(SelectionIter::new(weights)),
            _ => Err(()),
        }
    }
}

pub struct RankSelection<F, G>
where
    F: Fitness,
    G: Fn(usize) -> f32,
{
    fitness: F,
    probability: G,
}

impl<F, G> Selection for RankSelection<F, G>
where
    F: Fitness,
    G: Fn(usize) -> f32,
{
    fn get_selection_iter(&self, individuals: &Vec<Vec<bool>>) -> Result<SelectionIter, ()> {
        let mut indexed_fitness: Vec<(usize, f32)> = individuals
            .iter()
            .map(|v| self.fitness.calculate(v))
            .enumerate()
            .collect();
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
    use super::fitness::CountFitness;

    let fitness = CountFitness {};
    let selection = RouletteWheelSelection { fitness };

    let individuals1 = vec![
        vec![true, true, true],
        vec![true, true, false],
        vec![true, false, false],
    ];
    assert!(selection.get_selection_iter(&individuals1).is_ok());

    let individuals2 = vec![vec![true; 3]];
    assert!(selection.get_selection_iter(&individuals2).is_ok());

    // non-positive fitness is not acceptable
    let individuals3 = vec![vec![false; 3]];
    assert!(selection.get_selection_iter(&individuals3).is_err());

    let individuals4: Vec<Vec<bool>> = Vec::new();
    assert!(selection.get_selection_iter(&individuals4).is_err());
}

#[test]
fn test_rank_selection() {
    use super::fitness::CountFitness;

    let fitness = CountFitness {};
    let probability = |rank: usize| 1.0 / ((rank + 1) as f32);
    let selection = RankSelection {
        fitness,
        probability,
    };

    let individuals1 = vec![
        vec![true, true, true],
        vec![true, true, false],
        vec![true, false, false],
    ];
    assert!(selection.get_selection_iter(&individuals1).is_ok());

    let individuals2 = vec![vec![true; 3]];
    assert!(selection.get_selection_iter(&individuals2).is_ok());

    let individuals3 = vec![vec![false; 3]];
    assert!(selection.get_selection_iter(&individuals3).is_ok());

    let individuals4: Vec<Vec<bool>> = Vec::new();
    assert!(selection.get_selection_iter(&individuals4).is_err());
}
