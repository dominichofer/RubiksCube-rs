use crate::math::*;
use crate::twist::*;
use std::fmt;

/// Represents the edge pieces of a Rubik's cube.
/// 
/// Edge numbering scheme:
///     +----1----+
///    /|        /|
///   4 11      5 10
///  +----0----+  |
///  |  |      |  |
///  |  +----2-|--+
///  8 /       9 /
///  |7        |6
///  +----3----+
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edges {
    prm: [usize; 12],
    ori: [usize; 12],
}

impl Edges {
    pub const SLICE_PRM_SIZE: usize = factorial(4); // 24
    pub const NON_SLICE_PRM_SIZE: usize = factorial(8); // 40'320
    pub const SLICE_LOC_SIZE: usize = binomial(12, 4); // 495
    pub const ORI_SIZE: usize = 2usize.pow(11); // 2'048

    pub const fn solved() -> Self {
        Self { prm: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], ori: [0; 12] }
    }

    pub fn prm(&self) -> [usize; 12] {
        self.prm
    }

    pub fn ori(&self) -> [usize; 12] {
        self.ori
    }

    pub fn is_even_permutation(&self) -> bool {
        is_even_permutation_array(&self.prm)
    }
    
    /// Constructs an `Edges` instance from the given indices.
    /// - `slice_prm`: Slice permutation index (0 to SLICE_PRM_SIZE - 1).
    /// - `non_slice_prm`: Non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    /// - `slice_loc_index`: Slice location index (0 to SLICE_LOC_SIZE - 1).
    /// - `ori_index`: Orientation index (0 to ORI_SIZE - 1).
    pub fn from_indices(slice_prm: usize, non_slice_prm: usize, slice_loc_index: usize, ori_index: usize) -> Self {
        let slice_loc = nth_combination(12, 4, slice_loc_index);
        let slice = nth_permutation(slice_prm, 4);
        let mut prm = nth_permutation(non_slice_prm, 8);

        for i in 0..4 {
            prm.insert(slice_loc[i], slice[i] + 8);
        }

        let mut ori = [0usize; 12];
        for i in 0..11 {
            ori[i] = (ori_index >> i) & 1;
        }
        ori[11] = (ori_index.count_ones() % 2) as usize; // Ensure orientation parity is even
        Self { prm: prm.try_into().unwrap(), ori }
    }

    /// Get the slice permutation index (0 to SLICE_PRM_SIZE - 1).
    pub fn slice_prm_index(&self) -> usize {
        let mut slice: [usize; 4] = [0; 4];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.prm[i] > 7 {
                slice[j] = self.prm[i] - 8;
                j += 1;
            }
            i += 1;
        }
        permutation_index(&slice)
    }

    /// Get the non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    pub fn non_slice_prm_index(&self) -> usize {
        let mut non_slice= [0; 8];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.prm[i] <= 7 {
                non_slice[j] = self.prm[i];
                j += 1;
            }
            i += 1;
        }
        permutation_index(&non_slice)
    }

    /// Get the slice location index (0 to SLICE_LOC_SIZE - 1).
    pub const fn slice_loc_index(&self) -> usize {
        let mut loc= [0; 4];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.prm[i] > 7 {
                loc[j] = i;
                j += 1;
            }
            i += 1;
        }
        combination_index(12, &loc)
    }

    /// Get the orientation index (0 to ORI_SIZE - 1).
    pub fn ori_index(&self) -> usize {
        let mut index = 0;
        for i in 0..11 {
            index |= (self.ori[i] as usize) << i;
        }
        index
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        let p = self.prm;
        let o = self.ori;
        match twist {
            Twist::L1 => Self { 
                prm: [p[0], p[1], p[2], p[3], p[11], p[5], p[6], p[8], p[4], p[9], p[10], p[7]],
                ori: [o[0], o[1], o[2], o[3], 1 - o[11], o[5], o[6], 1 - o[8], 1 - o[4], o[9], o[10], 1 - o[7]]
            },
            Twist::R1 => Self {
                prm: [p[0], p[1], p[2], p[3], p[4], p[9], p[10], p[7], p[8], p[6], p[5], p[11]],
                ori: [o[0], o[1], o[2], o[3], o[4], 1 - o[9], 1 - o[10], o[7], o[8], 1 - o[6], 1 - o[5], o[11]]
            },
            Twist::U1 => Self {
                prm: [p[5], p[4], p[2], p[3], p[0], p[1], p[6], p[7], p[8], p[9], p[10], p[11]],
                ori: [o[5], o[4], o[2], o[3], o[0], o[1], o[6], o[7], o[8], o[9], o[10], o[11]]
            },
            Twist::D1 => Self {
                prm: [p[0], p[1], p[6], p[7], p[4], p[5], p[3], p[2], p[8], p[9], p[10], p[11]],
                ori: [o[0], o[1], o[6], o[7], o[4], o[5], o[3], o[2], o[8], o[9], o[10], o[11]]
            },
            Twist::F1 => Self {
                prm: [p[8], p[1], p[2], p[9], p[4], p[5], p[6], p[7], p[3], p[0], p[10], p[11]],
                ori: [o[8], o[1], o[2], o[9], o[4], o[5], o[6], o[7], o[3], o[0], o[10], o[11]]
            },
            Twist::B1 => Self {
                prm: [p[0], p[10], p[11], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[2], p[1]],
                ori: [o[0], o[10], o[11], o[3], o[4], o[5], o[6], o[7], o[8], o[9], o[2], o[1]]
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
        twists.iter().fold(self.clone(), |cube, &twist| {
            cube.twisted(twist)
        })
    }

    // Return the counter-rotated (rotated in the opposite direction) version of the edges.
    pub fn counter_rotated(&self, rot: Rotation) -> Self {
        match rot {
            Rotation::L => {
                let newself = self.twisted_by(&[Twist::L3, Twist::R1]);
                let p = newself.prm;
                let o = newself.ori;

                // Rotate middle layer
                Self {
                    prm: [p[3], p[0], p[1], p[2], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11]],
                    ori: [1 - o[3], 1 - o[0], 1 - o[1], 1 - o[2], o[4], o[5], o[6], o[7], o[8], o[9], o[10], o[11]]
                }
            }
            Rotation::U => {
                let newself = self.twisted_by(&[Twist::U3, Twist::D1]);
                let p = newself.prm;
                let o = newself.ori;

                // Rotate middle layer
                Self {
                    prm: [p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[11], p[8], p[9], p[10]],
                    ori: [o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7], 1 - o[11], 1 - o[8], 1 - o[9], 1 - o[10]]
                }
            }
            Rotation::F => {
                let newself = self.twisted_by(&[Twist::F3, Twist::B1]);
                let p = newself.prm;
                let o = newself.ori;

                // Rotate middle layer
                Self {
                    prm: [p[0], p[1], p[2], p[3], p[5], p[6], p[7], p[4], p[8], p[9], p[10], p[11]],
                    ori: [o[0], o[1], o[2], o[3], 1 - o[5], 1 - o[6], 1 - o[7], 1 - o[4], o[8], o[9], o[10], o[11]]
                }
            }
        }
    }

    pub fn rotated_colours(&self, rot: Rotation) -> Self {
        let i = |o: usize| o; // Identity orientation change
        let f = |o: usize| 1 - o; // Flip orientation
        let l_prm = [1, 2, 3, 0, 11, 10, 9, 8, 4, 5, 6, 7];
        let l_ori = [f, f, f, f, f, f, f, f, f, f, f, f];
        let u_prm = [5, 4, 7, 6, 0, 1, 2, 3, 9, 10, 11, 8];
        let u_ori = [i, i, i, i, i, i, i, i, f, f, f, f];
        match rot {
            Rotation::L => {
                Self {
                    prm: self.prm.map(|x| l_prm[x]),
                    ori: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(|i| l_ori[self.prm[i]](self.ori[i]))
                }.counter_rotated(rot)
            }
            Rotation::U => {
                Self {
                    prm: self.prm.map(|x| u_prm[x]),
                    ori: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(|i| u_ori[self.prm[i]](self.ori[i]))
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

impl fmt::Display for Edges {
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
        let e = Edges::solved();
        assert_ne!(e.twisted(Twist::L1), Edges::solved());
        assert_eq!(e.to_string(), "0 1 2 3 4 5 6 7 8 9 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_twist_results() {
        let e = Edges::solved();
        assert_eq!(e.twisted(Twist::L1).to_string(), "0 1 2 3 11 5 6 8 4 9 10 7 | 0 0 0 0 1 0 0 1 1 0 0 1");
        assert_eq!(e.twisted(Twist::R1).to_string(), "0 1 2 3 4 9 10 7 8 6 5 11 | 0 0 0 0 0 1 1 0 0 1 1 0");
        assert_eq!(e.twisted(Twist::U1).to_string(), "5 4 2 3 0 1 6 7 8 9 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(e.twisted(Twist::D1).to_string(), "0 1 6 7 4 5 3 2 8 9 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(e.twisted(Twist::F1).to_string(), "8 1 2 9 4 5 6 7 3 0 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(e.twisted(Twist::B1).to_string(), "0 10 11 3 4 5 6 7 8 9 2 1 | 0 0 0 0 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, TwistSet::full());
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            e = e.twisted(rnd.gen_twist());
            let slice_prm = e.slice_prm_index();
            let non_slice_prm = e.non_slice_prm_index();
            let slice_loc = e.slice_loc_index();
            let ori = e.ori_index();
            assert!(slice_prm < Edges::SLICE_PRM_SIZE);
            assert!(non_slice_prm < Edges::NON_SLICE_PRM_SIZE);
            assert!(slice_loc < Edges::SLICE_LOC_SIZE);
            assert!(ori < Edges::ORI_SIZE);
            assert_eq!(e, Edges::from_indices(slice_prm, non_slice_prm, slice_loc, ori));
        }
    }
}
