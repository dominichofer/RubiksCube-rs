use super::math::*;
use super::permutation::*;
use super::orientation::*;
use super::twist::*;
use std::ops::Mul;

/// Represents the corner pieces of a Rubik's cube.
///
/// Corner numbering scheme:
///      2---------3
///     /|        /|
///    / |       / |
///   0---------1  |
///   |  |      |  |
///   |  6------|--7
///   | /       | /
///   |/        |/
///   4---------5
///
/// Orientation scheme:
/// Each corner has an orientation value 0, 1, or 2 representing how much it is
/// twisted relative to its solved state. The sum of all 8 corner orientations
/// is always 0 mod 3 (parity constraint).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Corners {
    prm: Permutation<8>,
    ori: Orientation<8, 3>,
}

impl Corners {
    pub const PRM_SIZE: usize = factorial(8); // 40'320
    pub const ORI_SIZE: usize = 3_usize.pow(7); // 2'187
    pub const INDEX_SIZE: usize = Self::PRM_SIZE * Self::ORI_SIZE; // 88'179'840

    const fn new(prm: [usize; 8], ori: [usize; 8]) -> Self {
        Self { prm: Permutation::new(prm), ori: Orientation::new(ori) }
    }

    pub const fn solved() -> Self {
        Self { prm: Permutation::identity(), ori: Orientation::identity() }
    }

    pub fn twist(twist: Twist) -> Self {
        match twist {
            Twist::L1 => Self::new([2, 1, 6, 3, 0, 5, 4, 7], [1, 0, 2, 0, 2, 0, 1, 0]),
            Twist::R1 => Self::new([0, 5, 2, 1, 4, 7, 6, 3], [0, 2, 0, 1, 0, 1, 0, 2]),
            Twist::U1 => Self::new([1, 3, 0, 2, 4, 5, 6, 7], [0; 8]),
            Twist::D1 => Self::new([0, 1, 2, 3, 6, 4, 7, 5], [0; 8]),
            Twist::F1 => Self::new([4, 0, 2, 3, 5, 1, 6, 7], [2, 1, 0, 0, 1, 2, 0, 0]),
            Twist::B1 => Self::new([0, 1, 3, 7, 4, 5, 2, 6], [0, 0, 1, 2, 0, 0, 2, 1]),
            Twist::L2 => Self::twist(Twist::L1) * Self::twist(Twist::L1),
            Twist::L3 => Self::twist(Twist::L1) * Self::twist(Twist::L2),
            Twist::R2 => Self::twist(Twist::R1) * Self::twist(Twist::R1),
            Twist::R3 => Self::twist(Twist::R1) * Self::twist(Twist::R2),
            Twist::U2 => Self::twist(Twist::U1) * Self::twist(Twist::U1),
            Twist::U3 => Self::twist(Twist::U1) * Self::twist(Twist::U2),
            Twist::D2 => Self::twist(Twist::D1) * Self::twist(Twist::D1),
            Twist::D3 => Self::twist(Twist::D1) * Self::twist(Twist::D2),
            Twist::F2 => Self::twist(Twist::F1) * Self::twist(Twist::F1),
            Twist::F3 => Self::twist(Twist::F1) * Self::twist(Twist::F2),
            Twist::B2 => Self::twist(Twist::B1) * Self::twist(Twist::B1),
            Twist::B3 => Self::twist(Twist::B1) * Self::twist(Twist::B2),
        }
    }

    pub fn twists(twists: &[Twist]) -> Self {
        twists.iter().fold(Self::solved(), |acc, &twist| Self::twist(twist) * acc)
    }

    pub fn inverse(&self) -> Self {
        Self {
            prm: self.prm.inverse(),
            ori: self.prm.inverse() * self.ori.inverse(),
        }
    }

    pub fn conjugated_by(&self, rot: Axis) -> Self {
        let rot = match rot {
            Axis::X => Self::twist(Twist::L1) * Self::twist(Twist::R3),
            Axis::Y => Self::twist(Twist::F1) * Self::twist(Twist::B3),
            Axis::Z => Self::twist(Twist::D1) * Self::twist(Twist::U3),
        };
        rot * (*self) * rot.inverse()
    }

    pub fn prm(&self) -> [usize; 8] {
        self.prm.data()
    }

    pub fn ori(&self) -> [usize; 8] {
        self.ori.data()
    }

    pub fn from_indices(prm: usize, ori: usize) -> Self {
        let mut o = decode(ori.into(), 3, 7);
        o.push((7 * 3 - o.iter().sum::<usize>()) % 3); // Parity constraint
        Self {
            prm: Permutation::from_index(prm),
            ori: Orientation::new(o.try_into().unwrap()),
        }
    }

    pub fn prm_index(&self) -> usize {
        self.prm.index()
    }

    pub fn ori_index(&self) -> usize {
        encode(&self.ori.data()[..7], 3)
    }
}

impl Mul for Corners {
    type Output = Corners;

    fn mul(self, r: Corners) -> Corners {
        Corners {
            prm: self.prm * r.prm,
            ori: self.prm * r.ori + self.ori,
        }
    }
}

impl Mul<Corners> for Twist {
    type Output = Corners;

    fn mul(self, r: Corners) -> Corners {
        Corners::twist(self) * r
    }
}

impl Mul<Twist> for Corners {
    type Output = Corners;

    fn mul(self, twist: Twist) -> Corners {
        self * Corners::twist(twist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_generator::*;

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, &ALL_TWISTS);
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = rnd.gen_twist() * c;
            let prm = c.prm_index();
            let ori = c.ori_index();
            assert_eq!(c, Corners::from_indices(prm, ori));
        }
    }
}
