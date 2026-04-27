use super::Twistable;
use super::Twister;
use crate::cubies::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CornerIndex {
    pub prm: usize, // 0..=40'319
    pub ori: usize, // 0..=2'186
}

impl CornerIndex {
    pub const INDEX_SIZE: usize = Corners::PRM_SIZE * Corners::ORI_SIZE; // 88'179'840

    pub fn solved() -> Self {
        let c = Corners::solved();
        Self {
            prm: c.prm_index(),
            ori: c.ori_index(),
        }
    }

    pub fn index(&self) -> usize {
        self.prm * Corners::ORI_SIZE + self.ori
    }

    pub fn from_index(index: usize) -> Self {
        let prm = index / Corners::ORI_SIZE;
        let ori = index % Corners::ORI_SIZE;
        CornerIndex { prm, ori }
    }

    pub fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        Self {
            prm: twister.twisted_c_prm(self.prm, twist),
            ori: twister.twisted_c_ori(self.ori, twist),
        }
    }

    pub fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |index, &twist| index.twisted(twister, twist))
    }
}

impl fmt::Display for CornerIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CornerIndex {{ prm: {}, ori: {} }}", self.prm, self.ori)
    }
}

impl Twistable for CornerIndex {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        self.twisted(twister, twist)
    }

    fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        self.twisted_by(twister, twists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, RngExt, SeedableRng};

    // Tests 'index' and 'from_index'
    #[test]
    fn test_corners_index() {
        let mut rnd = StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let rnd_index = rnd.random_range(0..CornerIndex::INDEX_SIZE);
            assert_eq!(CornerIndex::from_index(rnd_index).index(), rnd_index);
        }
    }
}
