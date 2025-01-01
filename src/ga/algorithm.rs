use super::crossover::Crossover;
use super::fitness::Fitness;
use super::genotype::Genotype;
use super::index_picker::{IndexPicker, RandomSizeIndexPicker};
use super::mutation::Mutation;
use super::selection::{Selection, SelectionFactory};

pub trait Algorithm {
    type Chromosome: Clone;

    fn next_generation(
        &self,
        genos: &Vec<Genotype<Self::Chromosome>>,
    ) -> Result<Vec<Genotype<Self::Chromosome>>, ()>;
}

pub struct StandardAlgorithm<S, C, M>
where
    S: SelectionFactory,
    C: Crossover,
    M: Mutation<Chromosome = S::Chromosome>,
{
    selection: S,
    crossover: C,
    mutation: M,
    mutation_rate: f64,
}

impl<S, C, M> Algorithm for StandardAlgorithm<S, C, M>
where
    S: SelectionFactory,
    C: Crossover,
    M: Mutation<Chromosome = S::Chromosome>,
{
    type Chromosome = S::Chromosome;

    fn next_generation(
        &self,
        genos: &Vec<Genotype<Self::Chromosome>>,
    ) -> Result<Vec<Genotype<Self::Chromosome>>, ()> {
        let geno_size = genos.len();
        let mutation_picker = RandomSizeIndexPicker::new(geno_size, self.mutation_rate)?;
        let selector = self.selection.build(genos)?;

        let mut next_genos: Vec<Genotype<Self::Chromosome>> = Vec::with_capacity(geno_size);
        loop {
            let (i0, i1) = selector.select();
            let (ng0, ng1) = self.crossover.crossover(&genos[i0], &genos[i1])?;
            next_genos.push(ng0);
            next_genos.push(ng1);

            for i in mutation_picker.pick() {
                next_genos.push(self.mutation.mutate(&genos[i])?);
            }

            if next_genos.len() >= geno_size {
                break;
            }
        }
        Ok(next_genos[0..geno_size].to_vec())
    }
}

impl<S, C, M> StandardAlgorithm<S, C, M>
where
    S: SelectionFactory,
    C: Crossover,
    M: Mutation<Chromosome = S::Chromosome>,
{
    pub fn new(selection: S, crossover: C, mutation: M, mutation_rate: f64) -> Self {
        Self {
            selection,
            crossover,
            mutation,
            mutation_rate,
        }
    }
}

struct ElitismAlgorithm<S, C, M, F>
where
    S: SelectionFactory,
    C: Crossover,
    M: Mutation<Chromosome = S::Chromosome>,
    F: Fitness<Chromosome = S::Chromosome>,
{
    algorithm: StandardAlgorithm<S, C, M>,
    fitness: F,
}

impl<S, C, M, F> Algorithm for ElitismAlgorithm<S, C, M, F>
where
    S: SelectionFactory,
    C: Crossover,
    M: Mutation<Chromosome = S::Chromosome>,
    F: Fitness<Chromosome = S::Chromosome>,
{
    type Chromosome = S::Chromosome;

    fn next_generation(
        &self,
        genos: &Vec<Genotype<Self::Chromosome>>,
    ) -> Result<Vec<Genotype<Self::Chromosome>>, ()> {
        let (idx, _) = genos
            .iter()
            .map(|geno| self.fitness.fitness(geno))
            .enumerate()
            .max_by(|(_, f1), (_, f2)| f1.partial_cmp(f2).unwrap())
            .unwrap();
        let mut next_genos = vec![genos[idx].clone()];
        next_genos.extend(self.algorithm.next_generation(genos)?);
        Ok(next_genos[0..genos.len()].to_vec())
    }
}

impl<S, C, M, F> ElitismAlgorithm<S, C, M, F>
where
    S: SelectionFactory,
    C: Crossover,
    M: Mutation<Chromosome = S::Chromosome>,
    F: Fitness<Chromosome = S::Chromosome>,
{
    pub fn new(selection: S, crossover: C, mutation: M, mutation_rate: f64, fitness: F) -> Self {
        Self {
            algorithm: StandardAlgorithm::new(selection, crossover, mutation, mutation_rate),
            fitness,
        }
    }
}

#[test]
fn test_standard_algorithm() {
    use super::crossover::OnePointCrossover;
    use super::fitness::MockConcreteFitness as MockFitness;
    use super::mutation::MockMutation;
    use super::selection::{MockSelection, MockSelectionFactory};

    let mut fitness = MockFitness::new();
    fitness.expect_fitness().return_const(1.0);

    let mut selection = MockSelectionFactory::new();
    selection.expect_build().returning(|_| {
        let mut selector = MockSelection::new();
        selector.expect_select().return_const((0, 1));
        Ok(selector)
    });

    let crossover = OnePointCrossover::new();

    let mut mutation = MockMutation::new();
    mutation.expect_mutate().returning(|geno| Ok(geno.clone()));

    let algorithm = StandardAlgorithm::new(selection, crossover, mutation, 0.1);
    let genos = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let result = algorithm.next_generation(&genos);
    assert!(result.is_ok());
    let next_genos = result.unwrap();
    assert_eq!(next_genos.len(), 3);
}

#[test]
fn test_elitism_algorithm() {
    use super::crossover::OnePointCrossover;
    use super::fitness::MockConcreteFitness as MockFitness;
    use super::mutation::MockMutation;
    use super::selection::{MockSelection, MockSelectionFactory};

    let mut fitness = MockFitness::new();
    fitness
        .expect_fitness()
        .returning(|v| v.iter().map(|&x| x as f64).sum());
    let mut fitness2 = MockFitness::new();
    fitness2
        .expect_fitness()
        .returning(|v| v.iter().map(|&x| x as f64).sum());

    let mut selection = MockSelectionFactory::new();
    selection.expect_build().returning(|_| {
        let mut selector = MockSelection::new();
        selector.expect_select().return_const((0, 1));
        Ok(selector)
    });

    let crossover = OnePointCrossover::new();

    let mut mutation = MockMutation::new();
    mutation.expect_mutate().returning(|geno| Ok(geno.clone()));
    let algorithm = ElitismAlgorithm::new(selection, crossover, mutation, 0.1, fitness2);
    let genos = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let result = algorithm.next_generation(&genos);
    assert!(result.is_ok());
    let next_genos = result.unwrap();
    assert_eq!(next_genos.len(), 3);
    assert_eq!(next_genos[0], vec![6, 7, 8]);
}
