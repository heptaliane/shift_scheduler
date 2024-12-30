use super::genotype::Genotype;

pub trait Fitness {
    type Chromosome: Clone;

    fn fitness(&self, geno: &Genotype<Self::Chromosome>) -> f64;
}
