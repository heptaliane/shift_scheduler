use super::genotype::Genotype;
use super::index_picker::{
    IndexPicker, MultipleIndexPicker, RandomSizeIndexPicker, SingleIndexPicker,
};

trait CrossoverInternal {
    type Picker: IndexPicker;

    fn create_picker(&self, length: usize) -> Result<Self::Picker, ()>;
}

pub trait Crossover: CrossoverInternal {
    fn crossover<T: Clone>(
        &self,
        a: &Genotype<T>,
        b: &Genotype<T>,
    ) -> Result<(Genotype<T>, Genotype<T>), ()> {
        if a.len() != b.len() {
            return Err(());
        }

        let picker = self.create_picker(a.len())?;
        let indices = picker.pick();
        let &i0 = indices.first().unwrap_or(&a.len());

        let mut new_a: Genotype<T> = a[0..i0].to_vec();
        let mut new_b: Genotype<T> = b[0..i0].to_vec();

        let mut index_iter = indices.iter().enumerate().peekable();
        let last_peek = (indices.len(), &a.len());
        while let Some((i, &start)) = index_iter.next() {
            let &(_, &end) = index_iter.peek().unwrap_or(&last_peek);
            let (frag_a, frag_b) = match i % 2 {
                0 => (&b[start..end], &a[start..end]),
                _ => (&a[start..end], &b[start..end]),
            };
            new_a.extend_from_slice(frag_a);
            new_b.extend_from_slice(frag_b);
        }

        Ok((new_a, new_b))
    }
}

pub struct OnePointCrossover {}

impl CrossoverInternal for OnePointCrossover {
    type Picker = SingleIndexPicker;

    fn create_picker(&self, length: usize) -> Result<Self::Picker, ()> {
        Self::Picker::new(length)
    }
}

impl Crossover for OnePointCrossover {}

impl OnePointCrossover {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct MultiPointCrossover {
    n_cross: usize,
}

impl CrossoverInternal for MultiPointCrossover {
    type Picker = MultipleIndexPicker;

    fn create_picker(&self, length: usize) -> Result<Self::Picker, ()> {
        Self::Picker::new(length, self.n_cross)
    }
}

impl Crossover for MultiPointCrossover {}

impl MultiPointCrossover {
    pub fn new(n_cross: usize) -> Self {
        Self { n_cross }
    }
}

pub struct UniformCrossover {}

impl CrossoverInternal for UniformCrossover {
    type Picker = RandomSizeIndexPicker;

    fn create_picker(&self, length: usize) -> Result<Self::Picker, ()> {
        Self::Picker::new(length, 0.5)
    }
}

impl Crossover for UniformCrossover {}

impl UniformCrossover {
    pub fn new() -> Self {
        Self {}
    }
}

#[test]
fn test_one_point_crossover() {
    let geno_a = vec![0, 1, 2, 3, 4];
    let geno_b = vec![5, 6, 7, 8, 9];
    let crossover = OnePointCrossover::new();

    let crossovered = crossover.crossover(&geno_a, &geno_b);
    assert!(crossovered.is_ok());
    let (new_a, new_b) = crossovered.unwrap();
    assert_eq!(new_a.len(), 5);
    assert_eq!(new_b.len(), 5);
}

#[test]
fn test_multi_point_crossover() {
    let geno_a = vec![0, 1, 2, 3, 4];
    let geno_b = vec![5, 6, 7, 8, 9];
    let crossover = MultiPointCrossover::new(2);

    let crossovered = crossover.crossover(&geno_a, &geno_b);
    assert!(crossovered.is_ok());
    let (new_a, new_b) = crossovered.unwrap();
    assert_eq!(new_a.len(), 5);
    assert_eq!(new_b.len(), 5);
}

#[test]
fn test_uniform_crossover() {
    let geno_a = vec![0, 1, 2, 3, 4];
    let geno_b = vec![5, 6, 7, 8, 9];
    let crossover = UniformCrossover::new();

    let crossovered = crossover.crossover(&geno_a, &geno_b);
    assert!(crossovered.is_ok());
    let (new_a, new_b) = crossovered.unwrap();
    assert_eq!(new_a.len(), 5);
    assert_eq!(new_b.len(), 5);
}
