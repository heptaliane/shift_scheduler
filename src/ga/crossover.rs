use super::picker::IndexPicker;

pub trait CrossOver {
    fn crossover(&self, a: &Vec<bool>, b: &Vec<bool>) -> Result<(Vec<bool>, Vec<bool>), ()>;
}

pub struct OnePointCrossOver<P>
where
    P: IndexPicker,
{
    picker: P,
}

impl<P> CrossOver for OnePointCrossOver<P>
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

#[test]
fn test_one_point_crossover() {
    use super::picker::SequentialIndexPicker;
    let picker = SequentialIndexPicker {};
    let crossover = OnePointCrossOver { picker };

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
