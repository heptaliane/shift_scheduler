use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait IndexPicker {
    fn pick<T, const N: usize>(&self, arr: &[T; N]) -> Result<usize, ()>;

    fn pick_multiple<T, const N: usize>(&self, arr: &[T; N], n: usize) -> Result<Vec<usize>, ()>;
}

pub struct SequentialIndexPicker {}

impl IndexPicker for SequentialIndexPicker {
    fn pick<T, const N: usize>(&self, _arr: &[T; N]) -> Result<usize, ()> {
        match N {
            0 => Err(()),
            _ => Ok(0),
        }
    }

    fn pick_multiple<T, const N: usize>(&self, _arr: &[T; N], n: usize) -> Result<Vec<usize>, ()> {
        match N < n {
            true => Err(()),
            false => Ok((0..n).collect()),
        }
    }
}

pub struct RandomIndexPicker {}

impl IndexPicker for RandomIndexPicker {
    fn pick<T, const N: usize>(&self, _arr: &[T; N]) -> Result<usize, ()> {
        let indices: Vec<usize> = (0..N).collect();
        let mut rng = thread_rng();
        match indices.choose(&mut rng) {
            Some(&index) => Ok(index),
            _ => Err(()),
        }
    }

    fn pick_multiple<T, const N: usize>(&self, _arr: &[T; N], n: usize) -> Result<Vec<usize>, ()> {
        match N < n {
            true => Err(()),
            false => {
                let indices: Vec<usize> = (0..N).collect();
                let mut rng = thread_rng();
                Ok(indices.choose_multiple(&mut rng, n).cloned().collect())
            }
        }
    }
}

#[test]
fn test_sequential_index_picker() {
    let picker = SequentialIndexPicker {};
    let arr1: [bool; 5] = [true; 5];
    let arr2: [bool; 0] = [];

    assert_eq!(picker.pick(&arr1), Ok(0));
    assert_eq!(picker.pick(&arr2), Err(()));
    assert_eq!(picker.pick_multiple(&arr1, 3), Ok(vec![0, 1, 2]));
    assert_eq!(picker.pick_multiple(&arr1, 10), Err(()));
}

#[test]
fn test_random_index_picker() {
    let picker = RandomIndexPicker {};
    let arr1: [bool; 5] = [true; 5];
    let arr2: [bool; 0] = [];

    assert!(picker.pick(&arr1).is_ok());
    assert!(picker.pick(&arr2).is_err());
    assert!(picker.pick_multiple(&arr1, 3).is_ok());
    assert!(picker.pick_multiple(&arr1, 10).is_err());
}
