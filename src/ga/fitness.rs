#[cfg(test)]
use mockall::mock;

use super::genotype::Genotype;

pub trait Fitness: Clone {
    type Chromosome: Clone;

    fn fitness(&self, geno: &Genotype<Self::Chromosome>) -> f64;
}

#[cfg(test)]
mock! {
    pub ConcreteFitness {}

    impl Clone for ConcreteFitness {
        fn clone(&self) -> Self;
    }

    impl Fitness for ConcreteFitness {
        type Chromosome = usize;
        fn fitness(&self, geno: &Genotype<usize>) -> f64;
    }
}
