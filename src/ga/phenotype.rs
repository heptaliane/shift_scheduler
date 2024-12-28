use super::genotype::Genotype;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock(type GenoItem = bool;))]
pub trait Phenotype {
    type GenoItem;

    fn new(geno: &Genotype<Self::GenoItem>) -> Self;

    fn fitness(&self) -> f64;
}
