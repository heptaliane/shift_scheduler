use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;

use super::genotype::Genotype;
use super::index_picker::IndexPicker;
use super::index_picker::{MultipleIndexPicker, RandomSizeIndexPicker};

pub trait Mutation<T>
where
    T: Clone,
{
    fn mutate(&self, genos: &Vec<Genotype<T>>) -> Result<Vec<Genotype<T>>, ()>;
}

pub struct BitFlipMutation {
    genotype_mutate_rate: f64,
    chromotype_mutate_rate: f64,
}

impl BitFlipMutation {
    pub fn new(genotype_mutate_rate: f64, chromotype_mutate_rate: f64) -> Self {
        Self {
            genotype_mutate_rate,
            chromotype_mutate_rate,
        }
    }
}

impl Mutation<bool> for BitFlipMutation {
    fn mutate(&self, genos: &Vec<Genotype<bool>>) -> Result<Vec<Genotype<bool>>, ()> {
        let geno_picker = RandomSizeIndexPicker::new(genos.len(), self.genotype_mutate_rate)?;
        let chromo_size = genos.first().unwrap_or(&Vec::new()).len();
        let chromosome_picker =
            RandomSizeIndexPicker::new(chromo_size, self.chromotype_mutate_rate)?;

        let mut mutated: Vec<Genotype<bool>> = Vec::new();
        for i in geno_picker.pick() {
            let mutate_pos = chromosome_picker.pick();
            let geno = genos.get(i).unwrap();
            mutated.push(
                geno.iter()
                    .enumerate()
                    .map(|(j, &v)| if mutate_pos.contains(&j) { !v } else { v })
                    .collect(),
            );
        }
        Ok(mutated)
    }
}

pub struct NormalPertubativeMutation {
    genotype_mutate_rate: f64,
    chromotype_mutate_rate: f64,
}

impl NormalPertubativeMutation {
    pub fn new(genotype_mutate_rate: f64, chromotype_mutate_rate: f64) -> Self {
        Self {
            genotype_mutate_rate,
            chromotype_mutate_rate,
        }
    }
}

impl Mutation<f64> for NormalPertubativeMutation {
    fn mutate(&self, genos: &Vec<Genotype<f64>>) -> Result<Vec<Genotype<f64>>, ()> {
        let geno_picker = RandomSizeIndexPicker::new(genos.len(), self.genotype_mutate_rate)?;
        let chromo_size = genos.first().unwrap_or(&Vec::new()).len();
        let chromosome_picker =
            RandomSizeIndexPicker::new(chromo_size, self.chromotype_mutate_rate)?;
        let geno_max = genos
            .iter()
            .flatten()
            .fold(f64::NAN, |a, &b| f64::max(a, b));
        let geno_min = genos
            .iter()
            .flatten()
            .fold(f64::NAN, |a, &b| f64::min(a, b));
        let dist = Normal::new(0.0, (geno_max - geno_min) / 6.0).unwrap(); // 3 sigma
        let mut rng = thread_rng();

        let mut mutated: Vec<Genotype<f64>> = Vec::new();
        for i in geno_picker.pick() {
            let mutate_pos = chromosome_picker.pick();
            let geno = genos.get(i).unwrap();
            mutated.push(
                geno.iter()
                    .enumerate()
                    .map(|(j, &v)| {
                        if mutate_pos.contains(&j) {
                            v + dist.sample(&mut rng)
                        } else {
                            v
                        }
                    })
                    .collect(),
            );
        }
        Ok(mutated)
    }
}

pub struct RotateMutation {
    genotype_mutate_rate: f64,
}

impl RotateMutation {
    pub fn new(genotype_mutate_rate: f64) -> Self {
        Self {
            genotype_mutate_rate,
        }
    }
}

impl<T> Mutation<T> for RotateMutation
where
    T: Clone,
{
    fn mutate(&self, genos: &Vec<Genotype<T>>) -> Result<Vec<Genotype<T>>, ()> {
        let geno_picker = RandomSizeIndexPicker::new(genos.len(), self.genotype_mutate_rate)?;

        let mut mutated: Vec<Genotype<T>> = Vec::new();
        for i in geno_picker.pick() {
            let mut geno = genos.get(i).unwrap().clone();
            geno.rotate_left(1);
            mutated.push(geno);
        }
        Ok(mutated)
    }
}

pub struct InvertMutation {
    genotype_mutate_rate: f64,
}

impl InvertMutation {
    pub fn new(genotype_mutate_rate: f64) -> Self {
        Self {
            genotype_mutate_rate,
        }
    }
}

impl<T> Mutation<T> for InvertMutation
where
    T: Clone,
{
    fn mutate(&self, genos: &Vec<Genotype<T>>) -> Result<Vec<Genotype<T>>, ()> {
        let geno_picker = RandomSizeIndexPicker::new(genos.len(), self.genotype_mutate_rate)?;
        let chromo_size = genos.first().unwrap_or(&Vec::new()).len();
        let chromosome_picker = MultipleIndexPicker::new(chromo_size, 2)?;

        let mut mutated: Vec<Genotype<T>> = Vec::new();
        for i in geno_picker.pick() {
            let invert_indices = chromosome_picker.pick();
            let invert_min = invert_indices.iter().min().unwrap().clone();
            let invert_max = invert_indices.iter().max().unwrap().clone();
            let mut geno = genos.get(i).unwrap().clone();
            geno[invert_min..=invert_max].reverse();
            mutated.push(geno);
        }
        Ok(mutated)
    }
}

#[test]
fn test_bit_flip_mutation() {
    let genos: Vec<Genotype<bool>> = vec![vec![true, true], vec![false, false]];
    let mutation = BitFlipMutation::new(1.0, 1.0);

    let result = mutation.mutate(&genos);
    assert!(result.is_ok());
    let expected = vec![vec![false, false], vec![true, true]];
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_normal_pertubative_mutation() {
    let genos: Vec<Genotype<f64>> = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
    let mutation = NormalPertubativeMutation::new(1.0, 1.0);

    let result = mutation.mutate(&genos);
    assert!(result.is_ok());
}

#[test]
fn test_rotate_mutation() {
    let genos: Vec<Genotype<f64>> = vec![vec![1.0, 2.0, 3.0, 4.0, 5.0]];
    let mutation = RotateMutation::new(1.0);

    let result = mutation.mutate(&genos);
    assert!(result.is_ok());
    let expected = vec![vec![2.0, 3.0, 4.0, 5.0, 1.0]];
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_invert_mutation() {
    let genos: Vec<Genotype<f64>> = vec![vec![1.0, 2.0, 3.0, 4.0, 5.0]];
    let mutation = InvertMutation::new(1.0);

    let result = mutation.mutate(&genos);
    assert!(result.is_ok());
    assert_ne!(result.unwrap(), genos);
}
