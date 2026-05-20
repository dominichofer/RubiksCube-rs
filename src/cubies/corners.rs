use super::math::*;
use super::rotation::*;
use super::twist::*;
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
        Self {
            prm: [0, 1, 2, 3, 4, 5, 6, 7],
            ori: [0; 8],
        }
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
        let mut o: [usize; 8] = std::array::from_fn(|i| (ori / 3usize.pow(i as u32)) % 3);
        o[7] = (7 * 3 - o.iter().take(7).sum::<usize>()) % 3; // Parity constraint
        Self {
            prm: nth_permutation(prm, 8).try_into().unwrap(),
            ori: o,
        }
    }

    /// Get the permutation index (0 to PRM_SIZE-1)
    pub fn prm_index(&self) -> usize {
        permutation_index(&self.prm)
    }

    /// Get the orientation index (0 to ORI_SIZE-1)
    pub fn ori_index(&self) -> usize {
        self.ori[..7].iter().rev().fold(0, |acc, &o| acc * 3 + o)
    }

    /// Return the permuted cubies and orientations according to their location.
    /// `from`'s value at index i indicates the location of the cubie/orientation that will be moved to index i.
    /// For example, if `from` is `[2, ...]`, it means the cubie/orientation currently at location 2 will be moved to location 0.
    fn permuted_locations(&self, from: [usize; 8]) -> Self {
        Self {
            prm: std::array::from_fn(|i| self.prm[from[i]]),
            ori: std::array::from_fn(|i| self.ori[from[i]]),
        }
    }

    /// Return the reoriented cubies according to their location.
    /// `ori`'s value at index i indicates how much the cubie at location i will be twisted (0 = identity, 1 = clockwise, 2 = counter-clockwise).
    /// For example, if `ori` is `[2, ...]`, it means the cubie at location 0 will be twisted counter-clockwise.
    fn reoriented_locations(&self, ori: [usize; 8]) -> Self {
        Self {
            prm: self.prm,
            ori: std::array::from_fn(|i| (self.ori[i] + ori[i]) % 3),
        }
    }

    /// Return the permuted cubies according to their cubie number.
    /// `from`'s value at index i indicates the cubie number of the cubie that will be moved to the location of cubie i.
    /// For example, if `from` is `[2, ...]`, it means cubie 2 will be moved to the location of cubie 0.
    fn permuted_cubies(&self, prm: [usize; 8]) -> Self {
        Self {
            prm: std::array::from_fn(|i| prm[self.prm[i]]),
            ori: self.ori
        }
    }

    /// Return the reoriented cubies according to their cubie number.
    /// `ori`'s value at index i indicates how much the cubie with cubie number i will be twisted (0 = identity, 1 = clockwise, 2 = counter-clockwise).
    /// For example, if `ori` is `[2, ...]`, it means the cubie 0 will be twisted counter-clockwise.
    fn reoriented_cubies(&self, ori: [usize; 8]) -> Self {
        Self {
            prm: self.prm,
            ori: std::array::from_fn(|i| (self.ori[i] + ori[self.prm[i]]) % 3),
        }
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        // Orientation twist offsets: 0 = identity, 1 = clockwise, 2 = counter-clockwise
        match twist {
            Twist::L1 => self.permuted_locations([2, 1, 6, 3, 0, 5, 4, 7]).reoriented_locations([1, 0, 2, 0, 2, 0, 1, 0]),
            Twist::R1 => self.permuted_locations([0, 5, 2, 1, 4, 7, 6, 3]).reoriented_locations([0, 2, 0, 1, 0, 1, 0, 2]),
            Twist::U1 => self.permuted_locations([1, 3, 0, 2, 4, 5, 6, 7]),
            Twist::D1 => self.permuted_locations([0, 1, 2, 3, 6, 4, 7, 5]),
            Twist::F1 => self.permuted_locations([4, 0, 2, 3, 5, 1, 6, 7]).reoriented_locations([2, 1, 0, 0, 1, 2, 0, 0]),
            Twist::B1 => self.permuted_locations([0, 1, 3, 7, 4, 5, 2, 6]).reoriented_locations([0, 0, 1, 2, 0, 0, 2, 1]),
            Twist::L2 => self.twisted(Twist::L1).twisted(Twist::L1),
            Twist::L3 => self.twisted(Twist::L1).twisted(Twist::L2),
            Twist::R2 => self.twisted(Twist::R1).twisted(Twist::R1),
            Twist::R3 => self.twisted(Twist::R1).twisted(Twist::R2),
            Twist::U2 => self.twisted(Twist::U1).twisted(Twist::U1),
            Twist::U3 => self.twisted(Twist::U1).twisted(Twist::U2),
            Twist::D2 => self.twisted(Twist::D1).twisted(Twist::D1),
            Twist::D3 => self.twisted(Twist::D1).twisted(Twist::D2),
            Twist::F2 => self.twisted(Twist::F1).twisted(Twist::F1),
            Twist::F3 => self.twisted(Twist::F1).twisted(Twist::F2),
            Twist::B2 => self.twisted(Twist::B1).twisted(Twist::B1),
            Twist::B3 => self.twisted(Twist::B1).twisted(Twist::B2),
        }
    }

    pub fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists.iter().fold(*self, |state, &twist| state.twisted(twist))
    }

    pub fn rotated_colours(&self, rot: Rotation) -> Self {
        match rot {
            Rotation::L => self
                .permuted_cubies([2, 3, 6, 7, 0, 1, 4, 5])
                .reoriented_cubies([2, 1, 1, 2, 1, 2, 2, 1])
                .twisted(Twist::L3).twisted(Twist::R1), // Twist back to match the original colour scheme.
            Rotation::U => self
                .permuted_cubies([1, 3, 0, 2, 5, 7, 4, 6])
                .twisted(Twist::U3).twisted(Twist::D1), // Twist back to match the original colour scheme.
            Rotation::F => {
                const L: Rotation = Rotation::L;
                const U: Rotation = Rotation::U;
                self.rotated_colours_by(&[L, U, L, L, L])
            }
        }
    }

    pub fn rotated_colours_by(&self, rots: &[Rotation]) -> Self {
        rots.iter().fold(*self, |cube, &rot| cube.rotated_colours(rot))
    }

    pub fn inverted(&self) -> Self {
        let mut inv = Self::solved();
        for i in 0..8 {
            inv.prm[self.prm[i]] = i;
            inv.ori[self.prm[i]] = (3 - self.ori[i]) % 3;
        }
        inv
    }
}

impl fmt::Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {}",
            self.prm.map(|x| x.to_string()).join(" "),
            self.ori.map(|x| x.to_string()).join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_generator::*;

    #[test]
    fn test_solved() {
        assert_eq!(Corners::solved().to_string(), "0 1 2 3 4 5 6 7 | 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_twist_results() {
        assert_eq!(Corners::solved().twisted(Twist::L1).to_string(), "2 1 6 3 0 5 4 7 | 1 0 2 0 2 0 1 0");
        assert_eq!(Corners::solved().twisted(Twist::R1).to_string(), "0 5 2 1 4 7 6 3 | 0 2 0 1 0 1 0 2");
        assert_eq!(Corners::solved().twisted(Twist::U1).to_string(), "1 3 0 2 4 5 6 7 | 0 0 0 0 0 0 0 0");
        assert_eq!(Corners::solved().twisted(Twist::D1).to_string(), "0 1 2 3 6 4 7 5 | 0 0 0 0 0 0 0 0");
        assert_eq!(Corners::solved().twisted(Twist::F1).to_string(), "4 0 2 3 5 1 6 7 | 2 1 0 0 1 2 0 0");
        assert_eq!(Corners::solved().twisted(Twist::B1).to_string(), "0 1 3 7 4 5 2 6 | 0 0 1 2 0 0 2 1");
    }

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, &ALL_TWISTS);
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
