use super::math::*;
use super::permutation::*;
use super::orientation::*;
use super::twist::*;
use std::fmt;
use std::ops::Mul;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edges {
    prm: Permutation<12>,
    ori: Orientation<12, 2>,
}

impl Mul for Edges {
    type Output = Edges;

    fn mul(self, r: Edges) -> Edges {
        Edges {
            prm: self.prm * r.prm,
            ori: self.prm * r.ori + self.ori,
        }
    }
}

impl Edges {
    pub const SLICE_PRM_SIZE: usize = factorial(4); // 24
    pub const NON_SLICE_PRM_SIZE: usize = factorial(8); // 40'320
    pub const SLICE_LOC_SIZE: usize = binomial(12, 4); // 495
    pub const ORI_SIZE: usize = 2usize.pow(11); // 2'048

    const fn new(prm: [usize; 12], ori: [usize; 12]) -> Self {
        Self { prm: Permutation::new(prm), ori: Orientation::new(ori) }
    }

    pub const fn solved() -> Self {
        Self { prm: Permutation::identity(), ori: Orientation::zero() }
    }

    pub fn twist(twist: Twist) -> Self {
        match twist {
            Twist::L1 => Self::new([0, 1, 2, 3, 11, 5, 6, 8, 4, 9, 10, 7], [0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1]),
            Twist::R1 => Self::new([0, 1, 2, 3, 4, 9, 10, 7, 8, 6, 5, 11], [0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0]),
            Twist::U1 => Self::new([5, 4, 2, 3, 0, 1, 6, 7, 8, 9, 10, 11], [0; 12]),
            Twist::D1 => Self::new([0, 1, 6, 7, 4, 5, 3, 2, 8, 9, 10, 11], [0; 12]),
            Twist::F1 => Self::new([8, 1, 2, 9, 4, 5, 6, 7, 3, 0, 10, 11], [0; 12]),
            Twist::B1 => Self::new([0, 10, 11, 3, 4, 5, 6, 7, 8, 9, 2, 1], [0; 12]),
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

    pub fn conjugated_by(&self, rot: Rotation) -> Self {
        let x_middle_layer = Self::new([1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11], [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
        let y_middle_layer = Self::new([0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11], [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]);
        let z_middle_layer = Self::new([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10], [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1]);
        let rot = match rot {
            Rotation::X => x_middle_layer * Self::twist(Twist::L1) * Self::twist(Twist::R3),
            Rotation::Y => y_middle_layer * Self::twist(Twist::F1) * Self::twist(Twist::B3),
            Rotation::Z => z_middle_layer * Self::twist(Twist::D1) * Self::twist(Twist::U3),
        };
        rot * (*self) * rot.inverse()
    }

    pub fn prm(&self) -> [usize; 12] {
        self.prm.data()
    }

    pub fn ori(&self) -> [usize; 12] {
        self.ori.data()
    }
    pub fn from_indices(slice_prm: usize, non_slice_prm: usize, slice_loc_index: usize, ori_index: usize) -> Self {
        let slice_loc = nth_combination_size_4(12, slice_loc_index);
        let slice = nth_permutation_size_4(slice_prm);
        let mut prm = nth_permutation(non_slice_prm, 8);

        for i in 0..4 {
            prm.insert(slice_loc[i], slice[i] + 8);
        }

        let mut ori = decode(ori_index, 2, 11);
        ori.push((ori_index.count_ones() % 2) as usize); // Ensure orientation parity is even
        Self::new(prm.try_into().unwrap(), ori.try_into().unwrap())
    }

    pub fn x_loc_index(&self) -> usize {
        let loc: Vec<usize> = self.prm.data().iter().enumerate().filter_map(|(i, &p)| if p < 4 { Some(i) } else { None }).collect();
        combination_index(12, &loc)
    }

    pub fn y_loc_index(&self) -> usize {
        let loc: Vec<usize> = self.prm.data().iter().enumerate().filter_map(|(i, &p)| if p >= 4 && p < 8 { Some(i) } else { None }).collect();
        combination_index(12, &loc)
    }

    pub fn z_loc_index(&self) -> usize {
        let loc: Vec<usize> = self.prm.data().iter().enumerate().filter_map(|(i, &p)| if p >= 8 { Some(i) } else { None }).collect();
        combination_index(12, &loc)
    }

    pub fn x_prm_index(&self) -> usize {
        let prm: Vec<usize> = self.prm.data().iter().filter_map(|&p| if p < 4 { Some(p) } else { None }).collect();
        permutation_index(&prm)
    }

    pub fn y_prm_index(&self) -> usize {
        let prm: Vec<usize> = self.prm.data().iter().filter_map(|&p| if p >= 4 && p < 8 { Some(p - 4) } else { None }).collect();
        permutation_index(&prm)
    }

    pub fn z_prm_index(&self) -> usize {
        let prm: Vec<usize> = self.prm.data().iter().filter_map(|&p| if p >= 8 { Some(p - 8) } else { None }).collect();
        permutation_index(&prm)
    }

    /// Get the slice permutation index (0 to SLICE_PRM_SIZE - 1).
    pub fn slice_prm_index(&self) -> usize {
        let mut slice: [usize; 4] = [0; 4];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.prm.data()[i] > 7 {
                slice[j] = self.prm.data()[i] - 8;
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
            if self.prm.data()[i] <= 7 {
                non_slice[j] = self.prm.data()[i];
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
            if self.prm.data()[i] > 7 {
                loc[j] = i;
                j += 1;
            }
            i += 1;
        }
        combination_index(12, &loc)
    }

    pub fn ori_index(&self) -> usize {
        encode(&self.ori.data()[..11], 2)
    }
}

impl fmt::Display for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} | {}", self.prm, self.ori)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_generator::*;

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, &ALL_TWISTS);
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            e = Edges::twist(rnd.gen_twist()) * e;
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
