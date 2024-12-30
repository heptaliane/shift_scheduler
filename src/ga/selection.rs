use super::fitness::Fitness;
use super::genotype::Genotype;
use super::index_picker::{IndexPicker, WeightedIndexPicker};

pub trait Selection {
    fn select(&self) -> (usize, usize);
}

pub struct WeightedSelection {
    picker: WeightedIndexPicker,
}

impl WeightedSelection {
    fn new(weight: &Vec<f64>) -> Result<Self, ()> {
        let picker = WeightedIndexPicker::new(weight, 2)?;
        Ok(Self { picker })
    }
}

impl Selection for WeightedSelection {
    fn select(&self) -> (usize, usize) {
        let indices = self.picker.pick();
        (indices[0], indices[1])
    }
}

pub trait SelectionFactory {
    type Selection: Selection;
    type Chromosome: Clone;

    fn build(&self, genos: &Vec<Genotype<Self::Chromosome>>) -> Result<Self::Selection, ()>;
}

pub struct RouletteWheelSelection<F>
where
    F: Fitness,
{
    fitness: F,
}

impl<F> RouletteWheelSelection<F>
where
    F: Fitness,
{
    pub fn new(fitness: F) -> Self {
        Self { fitness: fitness }
    }
}

impl<F> SelectionFactory for RouletteWheelSelection<F>
where
    F: Fitness,
{
    type Selection = WeightedSelection;
    type Chromosome = F::Chromosome;

    fn build(&self, genos: &Vec<Genotype<Self::Chromosome>>) -> Result<Self::Selection, ()> {
        let fitness: Vec<f64> = genos
            .iter()
            .map(|geno| self.fitness.fitness(geno))
            .collect();
        Self::Selection::new(&fitness)
    }
}

pub struct RankSelection<F, P>
where
    F: Fitness,
    P: Fn(usize) -> f64,
{
    fitness: F,
    probability: P,
}

impl<F, P> RankSelection<F, P>
where
    F: Fitness,
    P: Fn(usize) -> f64,
{
    fn new(fitness: F, probability: P) -> Self {
        Self {
            fitness,
            probability,
        }
    }
}

impl<F, P> SelectionFactory for RankSelection<F, P>
where
    F: Fitness,
    P: Fn(usize) -> f64,
{
    type Selection = WeightedSelection;
    type Chromosome = F::Chromosome;

    fn build(&self, genos: &Vec<Genotype<Self::Chromosome>>) -> Result<Self::Selection, ()> {
        let mut indexed_fitness: Vec<(usize, f64)> = genos
            .iter()
            .enumerate()
            .map(|(i, geno)| (i, self.fitness.fitness(geno)))
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
        Self::Selection::new(&weights)
    }
}

#[test]
fn test_roulette_wheel_selection() {
    use super::fitness::MockConcreteFitness as MockFitness;

    let genos = vec![vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8]];
    let mut fitness = MockFitness::new();
    fitness.expect_fitness().return_const(1.0);

    let factory = RouletteWheelSelection::new(fitness);
    let result = factory.build(&genos);
    assert!(result.is_ok());
    let selection = result.unwrap();
    let (i1, i2) = selection.select();
    assert!(i1 < 3 && i2 < 3);
}

#[test]
fn test_rank_selection() {
    use super::fitness::MockConcreteFitness as MockFitness;

    let genos = vec![vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8]];
    let mut fitness = MockFitness::new();
    fitness.expect_fitness().return_const(1.0);
    let probability = |x: usize| x as f64;

    let factory = RankSelection::new(fitness, probability);
    let result = factory.build(&genos);
    assert!(result.is_ok());
    let selection = result.unwrap();
    let (i1, i2) = selection.select();
    assert!(i1 < 3 && i2 < 3);
}
