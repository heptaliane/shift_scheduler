use super::fitness::Fitness;
use rand::distributions::uniform::SampleBorrow;
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::thread_rng;

pub trait Selection {
    fn select(
        &self,
        individuals: &Vec<Vec<bool>>,
        samples: usize,
    ) -> Result<Vec<(usize, usize)>, ()>;
}

fn weighted_select<I>(weights: I, samples: usize) -> Result<Vec<(usize, usize)>, ()>
where
    I: IntoIterator,
    I::Item: SampleBorrow<f32>,
{
    match WeightedIndex::new(weights) {
        Ok(distributions) => {
            let mut rng = thread_rng();
            Ok((0..samples)
                .map(|_| {
                    (
                        distributions.sample(&mut rng),
                        distributions.sample(&mut rng),
                    )
                })
                .collect())
        }
        _ => Err(()),
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
    fn select(
        &self,
        individuals: &Vec<Vec<bool>>,
        samples: usize,
    ) -> Result<Vec<(usize, usize)>, ()> {
        let fitness = individuals.iter().map(|i| self.fitness.calculate(i));
        weighted_select(fitness, samples)
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
    fn select(
        &self,
        individuals: &Vec<Vec<bool>>,
        samples: usize,
    ) -> Result<Vec<(usize, usize)>, ()> {
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
        weighted_select(weights, samples)
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
    assert!(selection.select(&individuals1, 2).is_ok());

    let individuals2 = vec![vec![true; 3]];
    assert!(selection.select(&individuals2, 2).is_ok());

    // non-positive fitness is not acceptable
    let individuals3 = vec![vec![false; 3]];
    assert!(selection.select(&individuals3, 2).is_err());

    let individuals4: Vec<Vec<bool>> = Vec::new();
    assert!(selection.select(&individuals4, 2).is_err());
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
    assert!(selection.select(&individuals1, 2).is_ok());

    let individuals2 = vec![vec![true; 3]];
    assert!(selection.select(&individuals2, 2).is_ok());

    let individuals3 = vec![vec![false; 3]];
    assert!(selection.select(&individuals3, 2).is_ok());

    let individuals4: Vec<Vec<bool>> = Vec::new();
    assert!(selection.select(&individuals4, 2).is_err());
}
