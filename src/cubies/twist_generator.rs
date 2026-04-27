use super::twist::*;
use rand::{rngs::StdRng, RngExt, SeedableRng};

pub struct RandomTwistGen {
    rng: StdRng,
    twists: Vec<Twist>,
}

impl RandomTwistGen {
    pub fn new(seed: u64, twists: &[Twist]) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            twists: twists.to_vec(),
        }
    }

    pub fn gen_twist(&mut self) -> Twist {
        let idx = self.rng.random_range(0..self.twists.len());
        self.twists[idx]
    }

    pub fn gen_twists(&mut self, count: usize) -> Vec<Twist> {
        (0..count).map(|_| self.gen_twist()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_twist() {
        let mut rng = RandomTwistGen::new(42, &H0_TWISTS);
        for _ in 0..100 {
            let twist = rng.gen_twist();
            assert!(H0_TWISTS.contains(&twist));
        }
    }

    #[test]
    fn test_gen_twists() {
        let mut rng = RandomTwistGen::new(42, &H0_TWISTS);
        let twists = rng.gen_twists(100);
        assert_eq!(twists.len(), 100);
        for twist in twists {
            assert!(H0_TWISTS.contains(&twist));
        }
    }
}
