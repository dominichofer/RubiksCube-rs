use super::math::*;
use super::rotation::*;
use super::twist::*;
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
        Self {
            prm: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            ori: [0; 12],
        }
    }

    pub fn prm(&self) -> [usize; 12] {
        self.prm
    }

    pub fn ori(&self) -> [usize; 12] {
        self.ori
    }

    /// Constructs an `Edges` instance from the given indices.
    /// - `slice_prm`: Slice permutation index (0 to SLICE_PRM_SIZE - 1).
    /// - `non_slice_prm`: Non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    /// - `slice_loc_index`: Slice location index (0 to SLICE_LOC_SIZE - 1).
    /// - `ori_index`: Orientation index (0 to ORI_SIZE - 1).
    pub fn from_indices(slice_prm: usize, non_slice_prm: usize, slice_loc_index: usize, ori_index: usize) -> Self {
        let slice_loc = nth_combination_size_4(12, slice_loc_index);
        let slice = nth_permutation_size_4(slice_prm);
        let mut prm = nth_permutation(non_slice_prm, 8);

        for i in 0..4 {
            prm.insert(slice_loc[i], slice[i] + 8);
        }

        let mut ori: [usize; 12] = std::array::from_fn(|i| (ori_index >> i) & 1);
        ori[11] = (ori_index.count_ones() % 2) as usize; // Ensure orientation parity is even
        Self {
            prm: prm.try_into().unwrap(),
            ori,
        }
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
        let mut non_slice = [0; 8];
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
        let mut loc = [0; 4];
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
            index |= self.ori[i] << i;
        }
        index
    }

    /// Return the permuted cubies and orientations according to their location.
    /// `from`'s value at index i indicates the location of the cubie/orientation that will be moved to index i.
    /// For example, if `from` is `[2, ...]`, it means the cubie/orientation currently at location 2 will be moved to location 0.
    fn permuted_locations(&self, from: [usize; 12]) -> Self {
        Self {
            prm: std::array::from_fn(|i| self.prm[from[i]]),
            ori: std::array::from_fn(|i| self.ori[from[i]]),
        }
    }

    /// Return the reoriented cubies according to their location.
    /// `ori`'s value at index i indicates how much the cubie at location i will be twisted (0 = identity, 1 = flipped).
    /// For example, if `ori` is `[1, ...]`, it means the cubie at location 0 will be flipped.
    fn reoriented_locations(&self, ori: [usize; 12]) -> Self {
        Self {
            prm: self.prm,
            ori: std::array::from_fn(|i| (self.ori[i] + ori[i]) % 2),
        }
    }

    /// Return the permuted cubies according to their cubie number.
    /// `from`'s value at index i indicates the cubie number of the cubie that will be moved to the location of cubie i.
    /// For example, if `from` is `[2, ...]`, it means cubie 2 will be moved to the location of cubie 0.
    fn permuted_cubies(&self, prm: [usize; 12]) -> Self {
        Self {
            prm: std::array::from_fn(|i| prm[self.prm[i]]),
            ori: self.ori
        }
    }

    /// Return the reoriented cubies according to their cubie number.
    /// `ori`'s value at index i indicates how much the cubie with cubie number i will be twisted (0 = identity, 1 = flipped).
    /// For example, if `ori` is `[1, ...]`, it means the cubie 0 will be flipped.
    fn reoriented_cubies(&self, ori: [usize; 12]) -> Self {
        Self {
            prm: self.prm,
            ori: std::array::from_fn(|i| (self.ori[i] + ori[self.prm[i]]) % 2),
        }
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        match twist {
            Twist::L1 => self.permuted_locations([0, 1, 2, 3, 11, 5, 6, 8, 4, 9, 10, 7]).reoriented_locations([0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1]),
            Twist::R1 => self.permuted_locations([0, 1, 2, 3, 4, 9, 10, 7, 8, 6, 5, 11]).reoriented_locations([0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0]),
            Twist::U1 => self.permuted_locations([5, 4, 2, 3, 0, 1, 6, 7, 8, 9, 10, 11]),
            Twist::D1 => self.permuted_locations([0, 1, 6, 7, 4, 5, 3, 2, 8, 9, 10, 11]),
            Twist::F1 => self.permuted_locations([8, 1, 2, 9, 4, 5, 6, 7, 3, 0, 10, 11]),
            Twist::B1 => self.permuted_locations([0, 10, 11, 3, 4, 5, 6, 7, 8, 9, 2, 1]),
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

    pub fn twisted_middle_layer(&self, twist: Twist) -> Self {
        match twist {
            Twist::L3 => self.permuted_locations([3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11]).reoriented_locations([1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            Twist::U3 => self.permuted_locations([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10]).reoriented_locations([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1]),
            Twist::F3 => self.permuted_locations([0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11]).reoriented_locations([0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]),
            _ => panic!("Invalid middle layer twist"),
        }
    }

    pub fn rotated_colours(&self, rot: Rotation) -> Self {
        match rot {
            Rotation::L => self
                .permuted_cubies([1, 2, 3, 0, 11, 10, 9, 8, 4, 5, 6, 7])
                .reoriented_locations([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
                .twisted_middle_layer(Twist::L3).twisted(Twist::L3).twisted(Twist::R1), // Twist back to match the original colour scheme.
            Rotation::U => self
                .permuted_cubies([5, 4, 7, 6, 0, 1, 2, 3, 9, 10, 11, 8])
                .reoriented_cubies([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1])
                .twisted_middle_layer(Twist::U3).twisted(Twist::U3).twisted(Twist::D1), // Twist back to match the original colour scheme.
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
        for i in 0..12 {
            inv.prm[self.prm[i]] = i;
            inv.ori[self.prm[i]] = self.ori[i];
        }
        inv
    }
}

impl fmt::Display for Edges {
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
        assert_ne!(Edges::solved().twisted(Twist::L1), Edges::solved());
        assert_eq!(Edges::solved().to_string(), "0 1 2 3 4 5 6 7 8 9 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_twist_results() {
        assert_eq!(Edges::solved().twisted(Twist::L1).to_string(), "0 1 2 3 11 5 6 8 4 9 10 7 | 0 0 0 0 1 0 0 1 1 0 0 1");
        assert_eq!(Edges::solved().twisted(Twist::R1).to_string(), "0 1 2 3 4 9 10 7 8 6 5 11 | 0 0 0 0 0 1 1 0 0 1 1 0");
        assert_eq!(Edges::solved().twisted(Twist::U1).to_string(), "5 4 2 3 0 1 6 7 8 9 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(Edges::solved().twisted(Twist::D1).to_string(), "0 1 6 7 4 5 3 2 8 9 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(Edges::solved().twisted(Twist::F1).to_string(), "8 1 2 9 4 5 6 7 3 0 10 11 | 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(Edges::solved().twisted(Twist::B1).to_string(), "0 10 11 3 4 5 6 7 8 9 2 1 | 0 0 0 0 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, &ALL_TWISTS);
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
