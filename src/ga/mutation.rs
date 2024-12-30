use std::marker::PhantomData;

use super::genotype::Genotype;
use super::index_picker::{IndexPicker, MultipleIndexPicker, RandomSizeIndexPicker};

pub trait Mutation {
    type Chromosome: Clone;

    fn mutate(&self, geno: &Genotype<Self::Chromosome>) -> Result<Genotype<Self::Chromosome>, ()>;
}

pub struct BitFlipMutation {
    rate: f64,
}

impl Mutation for BitFlipMutation {
    type Chromosome = bool;

    fn mutate(&self, geno: &Genotype<Self::Chromosome>) -> Result<Genotype<Self::Chromosome>, ()> {
        let picker = RandomSizeIndexPicker::new(geno.len(), self.rate)?;
        let mutate_pos = picker.pick();
        let mutated: Genotype<Self::Chromosome> = geno
            .iter()
            .enumerate()
            .map(|(i, &c)| if mutate_pos.contains(&i) { !c } else { c })
            .collect();
        Ok(mutated)
    }
}

impl BitFlipMutation {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

pub struct RotateMutation<T>
where
    T: Clone,
{
    _phantom: PhantomData<T>,
}

impl<T> Mutation for RotateMutation<T>
where
    T: Clone,
{
    type Chromosome = T;

    fn mutate(&self, geno: &Genotype<Self::Chromosome>) -> Result<Genotype<Self::Chromosome>, ()> {
        let mut mutated = geno.clone();
        mutated.rotate_left(1);
        Ok(mutated)
    }
}

impl<T> RotateMutation<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self {
            _phantom: PhantomData::<T> {},
        }
    }
}

pub struct InvertMutation<T>
where
    T: Clone,
{
    _phantom: PhantomData<T>,
}

impl<T> Mutation for InvertMutation<T>
where
    T: Clone,
{
    type Chromosome = T;

    fn mutate(&self, geno: &Genotype<Self::Chromosome>) -> Result<Genotype<Self::Chromosome>, ()> {
        let picker = MultipleIndexPicker::new(geno.len(), 2)?;
        let mut indices = picker.pick();
        indices.sort();

        let mut mutated = geno.clone();
        mutated[indices[0]..=indices[1]].reverse();
        Ok(mutated)
    }
}

impl<T> InvertMutation<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self {
            _phantom: PhantomData::<T> {},
        }
    }
}

#[test]
fn test_bit_flip_mutation() {
    let geno = vec![true, true, true];
    let mutation = BitFlipMutation::new(1.0);
    let result = mutation.mutate(&geno);
    assert!(result.is_ok());
    let actual = result.unwrap();
    let expected = vec![false, false, false];
    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_mutation() {
    let geno = vec![0, 1, 2, 3, 4];
    let mutation = RotateMutation::<usize>::new();
    let result = mutation.mutate(&geno);
    assert!(result.is_ok());
    let actual = result.unwrap();
    let expected = vec![1, 2, 3, 4, 0];
    assert_eq!(actual, expected);
}

#[test]
fn test_invert_mutation() {
    let geno = vec![0, 1, 2, 3, 4];
    let mutation = InvertMutation::<usize>::new();
    let result = mutation.mutate(&geno);
    assert!(result.is_ok());
    let mutated = result.unwrap();
    assert_eq!(mutated.len(), 5);
}
