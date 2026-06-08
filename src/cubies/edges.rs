use super::math::*;
use super::permutation::*;
use super::orientation::*;
use super::twist::*;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocPrm {
    value: u16,
}

impl LocPrm {
    pub const LOC_SIZE: usize = binomial(12, 4); // 495
    pub const PRM_SIZE: usize = factorial(4); // 24
    pub const INDEX_SIZE: usize = Self::LOC_SIZE * Self::PRM_SIZE; // 11'880

    pub fn new(loc: usize, prm: usize) -> Self {
        assert!(loc < Self::LOC_SIZE);
        assert!(prm < Self::PRM_SIZE);
        Self { value: (loc * 32 + prm) as u16 }
    }

    pub fn index(&self) -> usize {
        self.loc() * Self::PRM_SIZE + self.prm()
    }

    pub fn from_index(index: usize) -> Self {
        let loc = index / Self::PRM_SIZE;
        let prm = index % Self::PRM_SIZE;
        Self::new(loc, prm)
    }

    pub fn loc(&self) -> usize {
        (self.value as usize) / 32
    }

    pub fn prm(&self) -> usize {
        (self.value as usize) % 32
    }
}

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

impl Edges {
    pub const LOC_PRM_SIZE: usize = LocPrm::INDEX_SIZE; // 11'880
    pub const ORI_SIZE: usize = 2_usize.pow(11); // 2'048

    const fn new(prm: [usize; 12], ori: [usize; 12]) -> Self {
        Self { prm: Permutation::new(prm), ori: Orientation::new(ori) }
    }

    pub const fn solved() -> Self {
        Self { prm: Permutation::identity(), ori: Orientation::identity() }
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

    pub fn conjugated_by(&self, rot: Axis) -> Self {
        let rot = match rot {
            Axis::X => Twist::L1 * Self::new([1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11], [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]) * Twist::R3,
            Axis::Y => Twist::F1 * Self::new([0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11], [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]) * Twist::B3,
            Axis::Z => Twist::D1 * Self::new([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10], [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1]) * Twist::U3,
        };
        rot * (*self) * rot.inverse()
    }

    pub fn prm(&self) -> [usize; 12] {
        self.prm.data()
    }

    pub fn ori(&self) -> [usize; 12] {
        self.ori.data()
    }

    pub fn from_indices(x: LocPrm, y: LocPrm, z: LocPrm, ori_index: usize) -> Self {
        assert!(ori_index < Self::ORI_SIZE);
        let x_loc = nth_combination(12, 4, x.loc());
        let y_loc = nth_combination(12, 4, y.loc());
        let z_loc = nth_combination(12, 4, z.loc());
        let x_prm = Permutation::<4>::from_index(x.prm());
        let y_prm = Permutation::<4>::from_index(y.prm());
        let z_prm = Permutation::<4>::from_index(z.prm());

        let mut prm = [0; 12];
        for i in 0..4 {
            prm[x_loc[i]] = x_prm[i];
        }
        for i in 0..4 {
            prm[y_loc[i]] = y_prm[i] + 4;
        }
        for i in 0..4 {
            prm[z_loc[i]] = z_prm[i] + 8;
        }

        let mut ori = decode(ori_index, 2, 11);
        ori.push((ori_index.count_ones() % 2) as usize); // Ensure orientation parity is even

        Self::new(prm, ori.try_into().unwrap())
    }

    pub fn from_subset_indices(xy_prm_index: usize, z_prm_index: usize) -> Self {
        let xy_prm = Permutation::<8>::from_index(xy_prm_index);
        let z_prm = Permutation::<4>::from_index(z_prm_index);
        let mut prm = [0; 12];
        for i in 0..8 {
            prm[i] = xy_prm[i];
        }
        for i in 0..4 {
            prm[i + 8] = z_prm[i] + 8;
        }
        Self::new(prm, [0; 12])
    }
    
    pub fn loc_prm(&self, slice: Axis) -> LocPrm {
        let min_val = match slice {
            Axis::X => 0,
            Axis::Y => 4,
            Axis::Z => 8,
        };
        let max_val = min_val + 4;
        let mut loc = [0; 4];
        let mut prm = [0; 4];
        let mut j = 0;
        for (i, &p) in self.prm.iter().enumerate() {
            if p >= min_val && p < max_val {
                loc[j] = i;
                prm[j] = p - min_val;
                j += 1;
            }
        }
        LocPrm::new(combination_index(12, &loc), permutation_index(&prm))
    }

    pub fn xy_prm_index(&self) -> usize {
        let mut prm = [0; 8];
        let mut j = 0;
        for &p in self.prm.iter() {
            if p < 8 {
                prm[j] = p;
                j += 1;
            }
        }
        permutation_index(&prm)
    }

    pub fn ori_index(&self) -> usize {
        encode(&self.ori.data()[..11], 2)
    }
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

impl Mul<Edges> for Twist {
    type Output = Edges;

    fn mul(self, r: Edges) -> Edges {
        Edges::twist(self) * r
    }
}

impl Mul<Twist> for Edges {
    type Output = Edges;

    fn mul(self, twist: Twist) -> Edges {
        self * Edges::twist(twist)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::twist_generator::*;

//     #[test]
//     fn test_indexing() {
//         let mut rnd = RandomTwistGen::new(181086, &ALL_TWISTS);
//         let mut e = Edges::solved();
//         for _ in 0..100_000 {
//             e = Edges::twist(rnd.gen_twist()) * e;
//             let x_loc = e.x_loc_index();
//             let y_loc = e.y_loc_index();
//             let z_loc = e.z_loc_index();
//             let x_prm = e.x_prm_index();
//             let y_prm = e.y_prm_index();
//             let z_prm = e.z_prm_index();
//             let ori = e.ori_index();
//             assert_eq!(e, Edges::from_indices(x_loc, y_loc, z_loc, x_prm, y_prm, z_prm, ori));
//         }
//     }

//     #[test]
//     fn test_subset_indexing() {
//         let mut rnd = RandomTwistGen::new(181086, &H0_TWISTS);
//         let mut e = Edges::solved();
//         for _ in 0..100_000 {
//             e = Edges::twist(rnd.gen_twist()) * e;
//             let xy_prm = e.xy_prm_index();
//             let z_prm = e.z_prm_index();
//             assert_eq!(e, Edges::from_subset_indices(xy_prm, z_prm));
//         }
//     }
// }
