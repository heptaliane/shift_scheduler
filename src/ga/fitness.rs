use mockall::automock;

use super::genotype::Genotype;

#[automock(type Chromosome = usize;)]
pub trait Fitness {
    type Chromosome: Clone;

    fn fitness(&self, geno: &Genotype<Self::Chromosome>) -> f64;
}
