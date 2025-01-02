use super::schedule::Schedule;
use crate::ga::genotype::Genotype;

pub struct PhenotypeTranslator {
    n_users: usize,
}

impl PhenotypeTranslator {
    pub fn new(n_users: usize) -> Self {
        Self { n_users }
    }

    pub fn to_schedule(&self, geno: &Genotype<usize>) -> Result<Schedule, ()> {
        if geno.len() % self.n_users != 0 {
            return Err(());
        }

        let mut schedule: Schedule = Schedule::new();
        for (i, &chromosome) in geno.iter().enumerate() {
            if i % self.n_users == 0 {
                schedule.push(Vec::new());
            }
            let shift = schedule.last_mut().unwrap();
            shift.push(chromosome);
        }
        Ok(schedule)
    }

    pub fn to_genotype(&self, schedule: &Schedule) -> Result<Genotype<usize>, ()> {
        let mut geno = Genotype::<usize>::new();
        for shift in schedule {
            if shift.len() != self.n_users {
                return Err(());
            }
            geno.extend(shift);
        }
        Ok(geno)
    }
}

#[test]
fn test_translate_to_schedule() {
    let translator = PhenotypeTranslator::new(3);

    let geno: Genotype<usize> = vec![0, 1, 2, 3, 4, 5];
    let result = translator.to_schedule(&geno);
    assert!(result.is_ok());

    let actual = result.unwrap();
    let expected = vec![vec![0, 1, 2], vec![3, 4, 5]];
    assert_eq!(actual, expected);

    assert!(translator.to_schedule(&vec![0, 1, 2, 3]).is_err());
}

#[test]
fn test_translate_to_genotype() {
    let translator = PhenotypeTranslator::new(3);

    let schedule: Schedule = vec![vec![0, 1, 2], vec![3, 4, 5]];
    let result = translator.to_genotype(&schedule);
    assert!(result.is_ok());

    let actual = result.unwrap();
    let expected: Genotype<usize> = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(actual, expected);

    let invalid_schedule: Schedule = vec![vec![0, 1, 2], vec![3, 4]];
    assert!(translator.to_genotype(&invalid_schedule).is_err());
}
