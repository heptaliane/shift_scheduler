use std::collections::HashMap;

use super::schedule::Schedule;
use crate::{ga::genotype::Genotype, scheduler::translator};

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
            let user_id = i % self.n_users;
            if user_id == 0 {
                schedule.push(HashMap::new());
            }
            let shift = schedule.last_mut().unwrap();
            shift.insert(user_id, chromosome);
        }
        Ok(schedule)
    }

    pub fn to_genotype(&self, schedule: &Schedule) -> Result<Genotype<usize>, ()> {
        let mut geno = Genotype::<usize>::new();
        for shift in schedule {
            for user_id in 0..self.n_users {
                if let Some(&worktype) = shift.get(&user_id) {
                    geno.push(worktype);
                } else {
                    return Err(());
                }
            }
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
    let expected = vec![
        HashMap::from([(0, 0), (1, 1), (2, 2)]),
        HashMap::from([(0, 3), (1, 4), (2, 5)]),
    ];
    assert_eq!(actual, expected);

    assert!(translator.to_schedule(&vec![0, 1, 2, 3]).is_err());
}

#[test]
fn test_translate_to_genotype() {
    let translator = PhenotypeTranslator::new(3);

    let schedule: Schedule = vec![
        HashMap::from([(0, 0), (1, 1), (2, 2)]),
        HashMap::from([(0, 3), (1, 4), (2, 5)]),
    ];
    let result = translator.to_genotype(&schedule);
    assert!(result.is_ok());

    let actual = result.unwrap();
    let expected: Genotype<usize> = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(actual, expected);

    let invalid_schedule: Schedule = vec![
        HashMap::from([(0, 0), (1, 1), (2, 2)]),
        HashMap::from([(0, 3), (1, 4)]),
    ];
    assert!(translator.to_genotype(&invalid_schedule).is_err());
}
