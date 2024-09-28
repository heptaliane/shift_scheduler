use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait IndexPicker {
    fn pick(&self, length: usize) -> Result<usize, ()>;

    fn pick_multiple(&self, length: usize, n: usize) -> Result<Vec<usize>, ()>;
}

pub struct SequentialIndexPicker {}

impl IndexPicker for SequentialIndexPicker {
    fn pick(&self, length: usize) -> Result<usize, ()> {
        match length {
            0 => Err(()),
            _ => Ok(0),
        }
    }

    fn pick_multiple(&self, length: usize, n: usize) -> Result<Vec<usize>, ()> {
        match length < n {
            true => Err(()),
            false => Ok((0..n).collect()),
        }
    }
}

pub struct RandomIndexPicker {}

impl IndexPicker for RandomIndexPicker {
    fn pick(&self, length: usize) -> Result<usize, ()> {
        let indices: Vec<usize> = (0..length).collect();
        let mut rng = thread_rng();
        match indices.choose(&mut rng) {
            Some(&index) => Ok(index),
            _ => Err(()),
        }
    }

    fn pick_multiple(&self, length: usize, n: usize) -> Result<Vec<usize>, ()> {
        match length < n {
            true => Err(()),
            false => {
                let indices: Vec<usize> = (0..length).collect();
                let mut rng = thread_rng();
                Ok(indices.choose_multiple(&mut rng, n).cloned().collect())
            }
        }
    }
}

#[test]
fn test_sequential_index_picker() {
    let picker = SequentialIndexPicker {};

    assert_eq!(picker.pick(3), Ok(0));
    assert_eq!(picker.pick(0), Err(()));
    assert_eq!(picker.pick_multiple(3, 3), Ok(vec![0, 1, 2]));
    assert_eq!(picker.pick_multiple(3, 10), Err(()));
}

#[test]
fn test_random_index_picker() {
    let picker = RandomIndexPicker {};

    assert!(picker.pick(3).is_ok());
    assert!(picker.pick(0).is_err());
    assert!(picker.pick_multiple(3, 3).is_ok());
    assert!(picker.pick_multiple(3, 10).is_err());
}
