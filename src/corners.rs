use crate::math::*;
use crate::twist::*;
use std::fmt;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Corners {
    prm: [usize; 8],
    ori: [usize; 8],
}

impl Corners {
    pub const PRM_SIZE: usize = factorial(8); // 40'320
    pub const ORI_SIZE: usize = 3usize.pow(7); // 2'187
    pub const INDEX_SIZE: usize = Self::PRM_SIZE * Self::ORI_SIZE; // 88'179'840

    pub const fn solved() -> Self {
        Self { prm: [0, 1, 2, 3, 4, 5, 6, 7], ori: [0; 8] }
    }

    pub fn prm(&self) -> [usize; 8] {
        self.prm
    }

    pub fn ori(&self) -> [usize; 8] {
        self.ori
    }

    /// Create Corners from permutation and orientation indices
    /// - `prm`: permutation index (0 to PRM_SIZE-1)
    /// - `ori`: orientation index (0 to ORI_SIZE-1)
    pub fn from_indices(prm: usize, ori: usize) -> Self {
        // Decode orientations from base-3 representation
        let o0 = (ori / 3usize.pow(0)) % 3;
        let o1 = (ori / 3usize.pow(1)) % 3;
        let o2 = (ori / 3usize.pow(2)) % 3;
        let o3 = (ori / 3usize.pow(3)) % 3;
        let o4 = (ori / 3usize.pow(4)) % 3;
        let o5 = (ori / 3usize.pow(5)) % 3;
        let o6 = (ori / 3usize.pow(6)) % 3;
        let o7 = (7 * 3 - o0 - o1 - o2 - o3 - o4 - o5 - o6) % 3; // Parity constraint
        Self {
            prm: nth_permutation(prm, 8).try_into().unwrap(),
            ori: [o0, o1, o2, o3, o4, o5, o6, o7],
        }
    }

    /// Get the permutation index (0 to PRM_SIZE-1)
    pub fn prm_index(&self) -> usize {
        permutation_index(&self.prm)
    }

    /// Get the orientation index (0 to ORI_SIZE-1)
    pub fn ori_index(&self) -> usize {
        self.ori[0] * 3usize.pow(0)
        + self.ori[1] * 3usize.pow(1)
        + self.ori[2] * 3usize.pow(2)
        + self.ori[3] * 3usize.pow(3)
        + self.ori[4] * 3usize.pow(4)
        + self.ori[5] * 3usize.pow(5)
        + self.ori[6] * 3usize.pow(6)
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        let p = self.prm;
        let o = self.ori;
        let r = |i: usize| (i + 1) % 3; // turn right (clockwise)
        let l = |i: usize| (i + 2) % 3; // turn left (counter-clockwise)
        match twist {
            Twist::L1 => Self {
                prm: [p[2], p[1], p[6], p[3], p[0], p[5], p[4], p[7]],
                ori: [r(o[2]), o[1], l(o[6]), o[3], l(o[0]), o[5], r(o[4]), o[7]]
            },
            Twist::R1 => Self {
                prm: [p[0], p[5], p[2], p[1], p[4], p[7], p[6], p[3]],
                ori: [o[0], l(o[5]), o[2], r(o[1]), o[4], r(o[7]), o[6], l(o[3])]
            },
            Twist::U1 => Self {
                prm: [p[1], p[3], p[0], p[2], p[4], p[5], p[6], p[7]],
                ori: [o[1], o[3], o[0], o[2], o[4], o[5], o[6], o[7]]
            },
            Twist::D1 => Self {
                prm: [p[0], p[1], p[2], p[3], p[6], p[4], p[7], p[5]],
                ori: [o[0], o[1], o[2], o[3], o[6], o[4], o[7], o[5]]
            },
            Twist::F1 => Self {
                prm: [p[4], p[0], p[2], p[3], p[5], p[1], p[6], p[7]],
                ori: [l(o[4]), r(o[0]), o[2], o[3], r(o[5]), l(o[1]), o[6], o[7]]
            },
            Twist::B1 => Self {
                prm: [p[0], p[1], p[3], p[7], p[4], p[5], p[2], p[6]],
                ori: [o[0], o[1], r(o[3]), l(o[7]), o[4], o[5], l(o[2]), r(o[6])]
            },
            Twist::L2 => self.twisted_by(&[Twist::L1, Twist::L1]),
            Twist::L3 => self.twisted_by(&[Twist::L1, Twist::L2]),
            Twist::R2 => self.twisted_by(&[Twist::R1, Twist::R1]),
            Twist::R3 => self.twisted_by(&[Twist::R1, Twist::R2]),
            Twist::U2 => self.twisted_by(&[Twist::U1, Twist::U1]),
            Twist::U3 => self.twisted_by(&[Twist::U1, Twist::U2]),
            Twist::D2 => self.twisted_by(&[Twist::D1, Twist::D1]),
            Twist::D3 => self.twisted_by(&[Twist::D1, Twist::D2]),
            Twist::F2 => self.twisted_by(&[Twist::F1, Twist::F1]),
            Twist::F3 => self.twisted_by(&[Twist::F1, Twist::F2]),
            Twist::B2 => self.twisted_by(&[Twist::B1, Twist::B1]),
            Twist::B3 => self.twisted_by(&[Twist::B1, Twist::B2]),
            Twist::None => *self,
        }
    }

    pub fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists.iter().fold(*self, |s, &twist| { s.twisted(twist) })
    }

    // Return the counter-rotated (rotated in the opposite direction) version of the corners.
    pub fn counter_rotated(&self, rot: Rotation) -> Self {
        match rot {
            Rotation::L => self.twisted_by(&[Twist::L3, Twist::R1]),
            Rotation::U => self.twisted_by(&[Twist::U3, Twist::D1]),
            Rotation::F => self.twisted_by(&[Twist::F3, Twist::B1]),
        }
    }

    pub fn rotated_colours(&self, rot: Rotation) -> Self {
        match rot {
            Rotation::L => {
                let r = |i: usize| (i + 1) % 3; // turn right (clockwise)
                let l = |i: usize| (i + 2) % 3; // turn left (counter-clockwise)

                // Independent of a cubie's location,
                // cubie 0 becomes cubie 2,
                // cubie 1 becomes cubie 3,
                // etc.
                let l_prm = [2, 3, 6, 7, 0, 1, 4, 5];

                // Independent of a cubie's location,
                // cubie 0 is twisted right (clockwise) so the red face becomes white,
                // etc.
                let l_ori = [r, l, l, r, l, r, r, l];

                Self {
                    prm: self.prm.map(|x| l_prm[x]),
                    ori: [0, 1, 2, 3, 4, 5, 6, 7].map(|i| l_ori[self.prm[i]](self.ori[i]))
                }.counter_rotated(rot)
            }
            Rotation::U => {
                // Independent of a cubie's location, cubie 0 becomes cubie 1, cubie 1 becomes cubie 3, etc.
                let u_prm = [1, 3, 0, 2, 5, 7, 4, 6];

                Self {
                    prm: self.prm.map(|x| u_prm[x]),
                    ori: self.ori
                }.counter_rotated(rot)
            }
            Rotation::F => {
                const L: Rotation = Rotation::L;
                const U: Rotation = Rotation::U;
                self.rotated_colours_by(&[L, U, L, L, L])
            }
        }
    }

    pub fn rotated_colours_by(&self, rots: &[Rotation]) -> Self {
        rots.iter().fold(self.clone(), |cube, &rot| {
            cube.rotated_colours(rot)
        })
    }
}

impl fmt::Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}",
            self.prm.map(|x| x.to_string()).join(" "),
            self.ori.map(|x| x.to_string()).join(" ")
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_set::*;
    use crate::twist_generator::*;

    #[test]
    fn test_solved() {
        let c = Corners::solved();
        assert_eq!(c.to_string(), "0 1 2 3 4 5 6 7 | 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_twist_results() {
        let c = Corners::solved();
        assert_eq!(c.twisted(Twist::L1).to_string(), "2 1 6 3 0 5 4 7 | 1 0 2 0 2 0 1 0");
        assert_eq!(c.twisted(Twist::R1).to_string(), "0 5 2 1 4 7 6 3 | 0 2 0 1 0 1 0 2");
        assert_eq!(c.twisted(Twist::U1).to_string(), "1 3 0 2 4 5 6 7 | 0 0 0 0 0 0 0 0");
        assert_eq!(c.twisted(Twist::D1).to_string(), "0 1 2 3 6 4 7 5 | 0 0 0 0 0 0 0 0");
        assert_eq!(c.twisted(Twist::F1).to_string(), "4 0 2 3 5 1 6 7 | 2 1 0 0 1 2 0 0");
        assert_eq!(c.twisted(Twist::B1).to_string(), "0 1 3 7 4 5 2 6 | 0 0 1 2 0 0 2 1");
    }

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, TwistSet::full());
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            let prm = c.prm_index();
            let ori = c.ori_index();
            assert!(prm < Corners::PRM_SIZE);
            assert!(ori < Corners::ORI_SIZE);
            assert_eq!(c, Corners::from_indices(prm, ori));
        }
    }
}
