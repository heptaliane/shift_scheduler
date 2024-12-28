use super::genotype::Genotype;

trait Phenotype<T>
where
    T: Clone,
{
    fn new(geno: &Genotype<T>) -> Self;

    fn fitness(&self) -> f64;
}
