use super::index_picker::{IndexPicker, WeightedIndexPicker};
use super::phenotype::Phenotype;

pub trait Selection<P>
where
    P: Phenotype,
{
    fn select(&self, phenos: &Vec<&P>) -> Result<Vec<(usize, usize)>, ()>;
}

pub struct RouletteWheelSelection {}

impl<P> Selection<P> for RouletteWheelSelection
where
    P: Phenotype,
{
    fn select(&self, phenos: &Vec<&P>) -> Result<Vec<(usize, usize)>, ()> {
        let fitness = phenos.iter().map(|p| p.fitness()).collect();
        let size = phenos.len();
        match WeightedIndexPicker::new(fitness, size * 2) {
            Ok(picker) => {
                let indices = picker.pick();
                Ok(indices[0..size]
                    .iter()
                    .zip(indices[size..size * 2].iter())
                    .map(|(&a, &b)| (a, b))
                    .collect())
            }
            _ => Err(()),
        }
    }
}

pub struct RankSelection<Q>
where
    Q: Fn(usize) -> f64,
{
    probability: Q,
}

impl<P, Q> Selection<P> for RankSelection<Q>
where
    P: Phenotype,
    Q: Fn(usize) -> f64,
{
    fn select(&self, phenos: &Vec<&P>) -> Result<Vec<(usize, usize)>, ()> {
        let size = phenos.len();

        let mut indexed_fitness: Vec<(usize, f64)> = phenos
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.fitness()))
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
            .map(|&(_, rank)| (self.probability)(rank))
            .collect();

        match WeightedIndexPicker::new(weights, size * 2) {
            Ok(picker) => {
                let indices = picker.pick();
                Ok(indices[0..size]
                    .iter()
                    .zip(indices[size..size * 2].iter())
                    .map(|(&a, &b)| (a, b))
                    .collect())
            }
            _ => Err(()),
        }
    }
}

#[test]
fn test_roulette_wheel_selection() {
    use super::phenotype::MockPhenotype;

    let mut pheno1 = MockPhenotype::default();
    pheno1.expect_fitness().return_const(0.0);
    let mut pheno2 = MockPhenotype::default();
    pheno2.expect_fitness().return_const(1.0);
    let mut pheno3 = MockPhenotype::default();
    pheno3.expect_fitness().return_const(2.0);

    let selection = RouletteWheelSelection {};
    let invalid_result = selection.select(&vec![&pheno1, &pheno1]);
    assert!(invalid_result.is_err());

    let valid_result = selection.select(&vec![&pheno1, &pheno2, &pheno3]);
    assert!(valid_result.is_ok());
    let result = valid_result.unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_rank_selection() {
    use super::phenotype::MockPhenotype;

    let mut pheno1 = MockPhenotype::default();
    pheno1.expect_fitness().return_const(0.0);
    let mut pheno2 = MockPhenotype::default();
    pheno2.expect_fitness().return_const(1.0);
    let mut pheno3 = MockPhenotype::default();
    pheno3.expect_fitness().return_const(2.0);

    let selection = RankSelection {
        probability: |rank: usize| 1.0 / (rank + 1) as f64,
    };
    let valid_result1 = selection.select(&vec![&pheno1, &pheno1]);
    assert!(valid_result1.is_ok());

    let valid_result2 = selection.select(&vec![&pheno1, &pheno2, &pheno3]);
    assert!(valid_result2.is_ok());
    let result = valid_result2.unwrap();
    assert_eq!(result.len(), 3);
}
