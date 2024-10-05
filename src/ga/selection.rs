use super::fitness::Fitness;
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

        match WeightedIndex::new(fitness) {
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
