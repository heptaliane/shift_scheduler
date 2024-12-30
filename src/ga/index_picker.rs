use rand::distributions::{Bernoulli, Uniform, WeightedIndex};
use rand::prelude::Distribution;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait IndexPicker {
    fn pick(&self) -> Vec<usize>;
}

pub struct SingleIndexPicker {
    length: usize,
}

impl SingleIndexPicker {
    pub fn new(length: usize) -> Result<Self, ()> {
        match length {
            0 => Err(()),
            _ => Ok(Self { length }),
        }
    }
}

impl IndexPicker for SingleIndexPicker {
    fn pick(&self) -> Vec<usize> {
        let mut rng = thread_rng();
        let dist = Uniform::new(0, self.length);
        vec![dist.sample(&mut rng)]
    }
}

pub struct MultipleIndexPicker {
    indices: Vec<usize>,
    n_pick: usize,
}

impl MultipleIndexPicker {
    pub fn new(length: usize, n_pick: usize) -> Result<Self, ()> {
        match 0 < n_pick && n_pick <= length {
            true => Ok(MultipleIndexPicker {
                indices: (0..length).collect(),
                n_pick,
            }),
            false => Err(()),
        }
    }
}

impl IndexPicker for MultipleIndexPicker {
    fn pick(&self) -> Vec<usize> {
        let mut rng = thread_rng();
        self.indices
            .choose_multiple(&mut rng, self.n_pick)
            .cloned()
            .collect()
    }
}

pub struct WeightedIndexPicker {
    dist: WeightedIndex<f64>,
    n_pick: usize,
}

impl WeightedIndexPicker {
    pub fn new(weights: &Vec<f64>, n_pick: usize) -> Result<Self, ()> {
        match WeightedIndex::new(weights) {
            Ok(dist) => Ok(Self { dist, n_pick }),
            _ => Err(()),
        }
    }
}

impl IndexPicker for WeightedIndexPicker {
    fn pick(&self) -> Vec<usize> {
        let mut rng = thread_rng();
        (0..self.n_pick)
            .map(|_| self.dist.sample(&mut rng))
            .collect()
    }
}

pub struct RandomSizeIndexPicker {
    indices: Vec<usize>,
    dist: Bernoulli,
}

impl RandomSizeIndexPicker {
    pub fn new(length: usize, pick_rate: f64) -> Result<Self, ()> {
        match 0 < length && 0.0 <= pick_rate && pick_rate <= 1.0 {
            true => Ok(Self {
                indices: (0..length).collect(),
                dist: Bernoulli::new(pick_rate).unwrap(),
            }),
            _ => Err(()),
        }
    }
}

impl IndexPicker for RandomSizeIndexPicker {
    fn pick(&self) -> Vec<usize> {
        let mut rng = thread_rng();
        self.indices
            .iter()
            .filter_map(|&i| match self.dist.sample(&mut rng) {
                true => Some(i),
                _ => None,
            })
            .collect()
    }
}

#[test]
fn test_single_index_picker() {
    let invalid_picker = SingleIndexPicker::new(0);
    assert!(invalid_picker.is_err());

    let valid_picker = SingleIndexPicker::new(10);
    assert!(valid_picker.is_ok());

    let picker = valid_picker.unwrap();
    let values = picker.pick();
    assert_eq!(values.len(), 1);

    let samples: Vec<usize> = (0..100)
        .map(|_| {
            let values = picker.pick();
            values[0]
        })
        .collect();
    let max_val = samples.iter().max().unwrap().clone();
    assert!(max_val < 10);
}

#[test]
fn test_multiple_index_picker() {
    let invalid_picker1 = MultipleIndexPicker::new(0, 10);
    assert!(invalid_picker1.is_err());
    let invalid_picker2 = MultipleIndexPicker::new(10, 20);
    assert!(invalid_picker2.is_err());

    let valid_picker1 = MultipleIndexPicker::new(10, 10);
    assert!(valid_picker1.is_ok());
    let valid_picker2 = MultipleIndexPicker::new(10, 5);
    assert!(valid_picker2.is_ok());

    let picker = valid_picker2.unwrap();
    let values = picker.pick();
    assert_eq!(values.len(), 5);
    let max_val = values.iter().max().unwrap().clone();
    assert!(max_val < 10);

    use std::collections::HashSet;
    let uniq_values: HashSet<usize> = HashSet::from_iter(values);
    assert_eq!(uniq_values.len(), 5);
}

#[test]
fn test_weighted_index_picker() {
    let invalid_picker1 = WeightedIndexPicker::new(&vec![0.0, 0.0], 10);
    assert!(invalid_picker1.is_err());
    let invalid_picker2 = WeightedIndexPicker::new(&vec![-1.0, 0.0], 10);
    assert!(invalid_picker2.is_err());

    let valid_picker = WeightedIndexPicker::new(&vec![0.0, 1.0, 2.0], 10);
    assert!(valid_picker.is_ok());
    let picker = valid_picker.unwrap();
    let values = picker.pick();
    assert_eq!(values.len(), 10);
    let max_val = values.iter().max().unwrap().clone();
    assert!(max_val < 10);
}

#[test]
fn test_random_size_index_picker() {
    let invalid_picker1 = RandomSizeIndexPicker::new(0, 0.1);
    assert!(invalid_picker1.is_err());
    let invalid_picker2 = RandomSizeIndexPicker::new(10, -1.0);
    assert!(invalid_picker2.is_err());
    let invalid_picker3 = RandomSizeIndexPicker::new(10, 2.0);
    assert!(invalid_picker3.is_err());

    let valid_picker = RandomSizeIndexPicker::new(10, 0.1);
    assert!(valid_picker.is_ok());
}
