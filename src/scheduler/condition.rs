use super::schedule::Schedule;
use super::score::{GaussianScorer, Scorer};
use std::collections::HashSet;

pub trait Condition {
    fn score(&self, schedule: &Schedule) -> f64;
}

pub struct WorkTypeBalancingCondition {
    worktype: HashSet<usize>,
    target: HashSet<usize>,
    scorer: GaussianScorer,
}

impl Condition for WorkTypeBalancingCondition {
    fn score(&self, schedule: &Schedule) -> f64 {
        let workdays: Vec<usize> =
            schedule
                .iter()
                .fold(vec![0; schedule[0].len()], |acc, shift| {
                    acc.iter()
                        .enumerate()
                        .filter(|(i, _)| self.target.contains(i))
                        .map(|(i, &n)| {
                            if self.worktype.contains(&shift[i]) {
                                n + 1
                            } else {
                                n
                            }
                        })
                        .collect()
                });
        let n_samples = self.target.len() as f64;
        let avg_workdays = workdays.iter().sum::<usize>() as f64 / n_samples;
        let dispersion: f64 = workdays
            .iter()
            .map(|&n| (n as f64 - avg_workdays).powi(2))
            .sum::<f64>()
            / n_samples;
        self.scorer.score(dispersion.sqrt())
    }
}

impl WorkTypeBalancingCondition {
    fn new(worktype: &Vec<usize>, target: &Vec<usize>, gain: f64, sensitivity: f64) -> Self {
        Self {
            worktype: HashSet::from_iter(worktype.clone().into_iter()),
            target: HashSet::from_iter(target.clone().into_iter()),
            scorer: GaussianScorer::new(0.0, sensitivity, gain),
        }
    }
}
