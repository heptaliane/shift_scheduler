use super::picker::IndexPicker;

pub trait Crossover {
    fn crossover(&self, a: &Vec<bool>, b: &Vec<bool>) -> Result<(Vec<bool>, Vec<bool>), ()>;
}

pub struct OnePointCrossover<P>
where
    P: IndexPicker,
{
    picker: P,
}

impl<P> Crossover for OnePointCrossover<P>
where
    P: IndexPicker,
{
    fn crossover(&self, a: &Vec<bool>, b: &Vec<bool>) -> Result<(Vec<bool>, Vec<bool>), ()> {
        match a.len() {
            n if n == b.len() => {
                let index = self.picker.pick(n)?;
                Ok((
                    [&a[..index], &b[index..]].concat(),
                    [&b[..index], &a[index..]].concat(),
                ))
            }
            _ => Err(()),
        }
    }
}

pub struct KPointCrossover<P>
where
    P: IndexPicker,
{
    k: usize,
    picker: P,
}

impl<P> Crossover for KPointCrossover<P>
where
    P: IndexPicker,
{
    fn crossover(&self, a: &Vec<bool>, b: &Vec<bool>) -> Result<(Vec<bool>, Vec<bool>), ()> {
        match a.len() {
            n if n == b.len() => {
                let indices = self.picker.pick_n(n, self.k)?;
                let mut index_iter = indices.iter().enumerate().peekable();

                let mut new_a: Vec<bool> = Vec::new();
                let mut new_b: Vec<bool> = Vec::new();
                while let Some((i, &start)) = index_iter.next() {
                    let &(_, &end) = index_iter.peek().unwrap_or(&(self.k, &n));
                    let (target_a, target_b) = match i % 2 {
                        0 => (&b, &a),
                        _ => (&a, &b),
                    };
                    new_a.extend_from_slice(&target_a[start..end]);
                    new_b.extend_from_slice(&target_b[start..end]);
                }

                Ok((new_a, new_b))
            }
            _ => Err(()),
        }
    }
}

pub struct UniformCrossover<P>
where
    P: IndexPicker,
{
    picker: P,
}
impl<P> Crossover for UniformCrossover<P>
where
    P: IndexPicker,
{
    fn crossover(&self, a: &Vec<bool>, b: &Vec<bool>) -> Result<(Vec<bool>, Vec<bool>), ()> {
        match a.len() {
            n if n == b.len() => {
                let indices = self.picker.pick_some(n)?;
                let n_crossover = indices.len();
                let mut index_iter = indices.iter().enumerate().peekable();

                let mut new_a: Vec<bool> = Vec::new();
                let mut new_b: Vec<bool> = Vec::new();
                while let Some((i, &start)) = index_iter.next() {
                    let &(_, &end) = index_iter.peek().unwrap_or(&(n_crossover, &n));
                    let (target_a, target_b) = match i % 2 {
                        0 => (&b, &a),
                        _ => (&a, &b),
                    };
                    new_a.extend_from_slice(&target_a[start..end]);
                    new_b.extend_from_slice(&target_b[start..end]);
                }

                Ok((new_a, new_b))
            }
            _ => Err(()),
        }
    }
}

#[test]
fn test_one_point_crossover() {
    use super::picker::SequentialIndexPicker;
    let picker = SequentialIndexPicker {};
    let crossover = OnePointCrossover { picker };

    let mut arr1 = vec![true; 3];
    let arr2 = vec![false; 3];
    let arr3 = vec![true; 1];

    let expected1 = Ok((vec![false, false, false], vec![true, true, true]));
    let actual1 = crossover.crossover(&arr1, &arr2);
    assert_eq!(actual1, expected1);

    assert!(crossover.crossover(&arr1, &arr3).is_err());

    // Original indivisual values change will not affect crossovered one
    arr1[0] = false;
    assert_eq!(actual1, expected1);
}

#[test]
fn test_k_point_crossover() {
    use super::picker::SequentialIndexPicker;

    let arr1 = vec![true; 3];
    let arr2 = vec![false; 3];
    let arr3 = vec![true; 1];

    let picker = SequentialIndexPicker {};
    let crossover1 = KPointCrossover { picker, k: 2 };

    let expected1 = Ok((vec![false, true, true], vec![true, false, false]));
    let actual1 = crossover1.crossover(&arr1, &arr2);
    assert_eq!(actual1, expected1);

    assert!(crossover1.crossover(&arr1, &arr3).is_err());

    let picker = SequentialIndexPicker {};
    let crossover2 = KPointCrossover { picker, k: 5 };

    assert!(crossover2.crossover(&arr1, &arr2).is_err());
}

#[test]
fn test_uniform_crossover() {
    use super::picker::SequentialIndexPicker;

    let arr1 = vec![true; 4];
    let arr2 = vec![false; 4];
    let arr3 = vec![true; 1];

    let picker = SequentialIndexPicker {};
    let crossover = UniformCrossover { picker };

    let expected = Ok((
        vec![false, true, true, true],
        vec![true, false, false, false],
    ));
    let actual = crossover.crossover(&arr1, &arr2);
    assert_eq!(actual, expected);

    assert!(crossover.crossover(&arr1, &arr3).is_err());
}
