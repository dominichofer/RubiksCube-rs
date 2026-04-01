use crate::twist::*;
use crate::twist_set::*;
use rand::{RngExt, SeedableRng, rngs::StdRng};

pub struct RandomTwistGen {
    rng: StdRng,
    twists: TwistSet,
}

impl RandomTwistGen {
    pub fn new(seed: u64, twists: TwistSet) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            twists,
        }
    }

    pub fn gen_twist(&mut self) -> Twist {
        let idx = self.rng.random_range(0..self.twists.count());
        self.twists.nth(idx)
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
        let mut rng = RandomTwistGen::new(42, TwistSet::h0());
        for _ in 0..100 {
            let twist = rng.gen_twist();
            assert!(TwistSet::h0().contains(twist));
        }
    }

    #[test]
    fn test_gen_twists() {
        let mut rng = RandomTwistGen::new(42, TwistSet::h0());
        let twists = rng.gen_twists(100);
        assert_eq!(twists.len(), 100);
        for twist in twists {
            assert!(TwistSet::h0().contains(twist));
        }
    }
}
