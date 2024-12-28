use super::genotype::Genotype;
use super::index_picker::IndexPicker;

pub struct Crossover<P>
where
    P: IndexPicker,
{
    picker: P,
}

impl<P> Crossover<P>
where
    P: IndexPicker,
{
    pub fn crossover<T>(
        &self,
        a: &Genotype<T>,
        b: &Genotype<T>,
    ) -> Result<(Genotype<T>, Genotype<T>), ()>
    where
        T: Clone,
    {
        match a.len() == b.len() {
            true => {
                let indices = self.picker.pick();
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
            false => Err(()),
        }
    }
}

#[test]
fn test_crossover() {
    use super::index_picker::MockIndexPicker;

    let mut picker = MockIndexPicker::new();
    picker.expect_pick().return_const(vec![1, 3]);

    let crossover = Crossover { picker };

    let geno_a = vec![0, 2, 4, 6, 8];
    let geno_b = vec![1, 3, 5, 7, 9];
    let crossovered = crossover.crossover(&geno_a, &geno_b);
    assert!(crossovered.is_ok());

    let (geno_c, geno_d) = crossovered.unwrap();
    assert_eq!(geno_c, vec![0, 3, 5, 6, 8]);
    assert_eq!(geno_d, vec![1, 2, 4, 7, 9]);

    let geno_e = vec![0];
    let failed_crossover = crossover.crossover(&geno_a, &geno_e);
    assert!(failed_crossover.is_err());
}
