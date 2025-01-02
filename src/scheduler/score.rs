pub trait Scorer {
    fn score(&self, x: f64) -> f64;
}

pub struct GaussianScorer {
    mu: f64,
    sigma: f64,
    max_score: f64,
}

impl Scorer for GaussianScorer {
    fn score(&self, x: f64) -> f64 {
        let exponent = -(x - self.mu).powi(2) / (2.0 * self.sigma.powi(2));
        exponent.exp() * self.max_score
    }
}

impl GaussianScorer {
    pub fn new(target: f64, hwhm: f64, max_score: f64) -> Self {
        Self {
            mu: target,
            sigma: hwhm / (2.0 * 2.0f64.ln()).sqrt(),
            max_score,
        }
    }
}

#[test]
fn test_gaussian_scorer() {
    let scorer = GaussianScorer::new(1.0, 2.0, 10.0);

    let actual1 = scorer.score(1.0);
    let expected1 = 10.0;
    assert!((actual1 - expected1).abs() < 1.0E-12);

    let actual2 = scorer.score(3.0);
    let expected2 = 5.0;
    assert!((actual2 - expected2).abs() < 1.0E-12);
}
