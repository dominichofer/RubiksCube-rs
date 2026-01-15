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
    /// Internal state: each byte encodes an edge where:
    /// - bits 0 to 3: cubie position (values 0 to 11)
    /// - bit 4: orientation (values 0 or 1)
    s: [u8; 12],
}

impl Edges {
    /// Size constants for indexing
    pub const SLICE_PRM_SIZE: usize = factorial(4); // 24
    pub const NON_SLICE_PRM_SIZE: usize = factorial(8); // 40'320
    pub const SLICE_LOC_SIZE: usize = binomial(12, 4); // 495
    pub const ORI_SIZE: usize = 2usize.pow(11); // 2'048

    pub const fn solved() -> Self {
        Self { s: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] }
    }

    fn cubie(&self, index: usize) -> u8 {
        self.s[index] & 0x0F
    }

    fn orientation(&self, index: usize) -> u8 {
        self.s[index] >> 4
    }

    // pub fn is_even_permutation(&self) -> bool {
    //     is_even_permutation_array(&self.cubies())
    // }
    
    /// Constructs an `Edges` instance from the given indices.
    /// - `slice_prm`: Slice permutation index (0 to SLICE_PRM_SIZE - 1).
    /// - `non_slice_prm`: Non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    /// - `slice_loc_index`: Slice location index (0 to SLICE_LOC_SIZE - 1).
    /// - `ori`: Orientation index (0 to ORI_SIZE - 1).
    pub fn from_indices(slice_prm: usize, non_slice_prm: usize, slice_loc_index: usize, ori: usize) -> Self {
        let slice_loc = nth_combination(12, 4, slice_loc_index);
        let non_slice = nth_permutation(non_slice_prm, 8);
        let mut slice = nth_permutation(slice_prm, 4);
        for i in 0..4 {
            slice[i] += 8;
        }

        let mut s = non_slice;
        for i in 0..4 {
            s.insert(slice_loc[i] as usize, slice[i]);
        }

        for i in 0..11 {
            s[i] |= (((ori >> i) & 1) << 4) as u8;
        }
        s[11] |= ((ori.count_ones() % 2) << 4) as u8;

        Self { s: s.try_into().unwrap() }
    }

    /// Get the slice permutation index (0 to SLICE_PRM_SIZE - 1).
    pub fn slice_prm_index(&self) -> usize {
        let mut slice: [u8; 4] = [0; 4];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.cubie(i) > 7 {
                slice[j] = self.cubie(i) - 8;
                j += 1;
            }
            i += 1;
        }
        permutation_index(&slice)
    }

    /// Get the non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    pub fn non_slice_prm_index(&self) -> usize {
        let mut non_slice: [u8; 8] = [0; 8];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.cubie(i) <= 7 {
                non_slice[j] = self.cubie(i);
                j += 1;
            }
            i += 1;
        }
        permutation_index(&non_slice)
    }

    /// Get the slice location index (0 to SLICE_LOC_SIZE - 1).
    pub fn slice_loc_index(&self) -> usize {
        let mut loc: [u8; 4] = [0; 4];
        let mut i = 0;
        let mut j = 0;
        while i < 12 {
            if self.cubie(i) > 7 {
                loc[j] = i as u8;
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
            index |= (self.orientation(i) as usize) << i;
        }
        index
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        match twist {
            Twist::L1 => self.shuffled(0, 1, 2, 3, 11, 5, 6, 8, 4, 9, 10, 7).ori_swap([4, 7, 8, 11]),
            Twist::L2 => self.shuffled(0, 1, 2, 3, 7, 5, 6, 4, 11, 9, 10, 8),
            Twist::L3 => self.shuffled(0, 1, 2, 3, 8, 5, 6, 11, 7, 9, 10, 4).ori_swap([4, 7, 8, 11]),
            Twist::R1 => self.shuffled(0, 1, 2, 3, 4, 9, 10, 7, 8, 6, 5, 11).ori_swap([5, 6, 9, 10]),
            Twist::R2 => self.shuffled(0, 1, 2, 3, 4, 6, 5, 7, 8, 10, 9, 11),
            Twist::R3 => self.shuffled(0, 1, 2, 3, 4, 10, 9, 7, 8, 5, 6, 11).ori_swap([5, 6, 9, 10]),
            Twist::U1 => self.shuffled(5, 4, 2, 3, 0, 1, 6, 7, 8, 9, 10, 11),
            Twist::U2 => self.shuffled(1, 0, 2, 3, 5, 4, 6, 7, 8, 9, 10, 11),
            Twist::U3 => self.shuffled(4, 5, 2, 3, 1, 0, 6, 7, 8, 9, 10, 11),
            Twist::D1 => self.shuffled(0, 1, 6, 7, 4, 5, 3, 2, 8, 9, 10, 11),
            Twist::D2 => self.shuffled(0, 1, 3, 2, 4, 5, 7, 6, 8, 9, 10, 11),
            Twist::D3 => self.shuffled(0, 1, 7, 6, 4, 5, 2, 3, 8, 9, 10, 11),
            Twist::F1 => self.shuffled(8, 1, 2, 9, 4, 5, 6, 7, 3, 0, 10, 11),
            Twist::F2 => self.shuffled(3, 1, 2, 0, 4, 5, 6, 7, 9, 8, 10, 11),
            Twist::F3 => self.shuffled(9, 1, 2, 8, 4, 5, 6, 7, 0, 3, 10, 11),
            Twist::B1 => self.shuffled(0, 10, 11, 3, 4, 5, 6, 7, 8, 9, 2, 1),
            Twist::B2 => self.shuffled(0, 2, 1, 3, 4, 5, 6, 7, 8, 9, 11, 10),
            Twist::B3 => self.shuffled(0, 11, 10, 3, 4, 5, 6, 7, 8, 9, 1, 2),
            Twist::None => *self,
        }
    }

    pub fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists.iter().fold(self.clone(), |cube, &twist| {
            cube.twisted(twist)
        })
    }

    fn shuffled(&self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, g: usize, h: usize, i: usize, j: usize, k: usize, l: usize) -> Self {
        Self { s: [self.s[a], self.s[b], self.s[c], self.s[d], self.s[e], self.s[f], self.s[g], self.s[h], self.s[i], self.s[j], self.s[k], self.s[l]] }
    }
    
    fn ori_swap(mut self, indices: [usize; 4]) -> Self {
        for i in indices {
            self.s[i] ^= 0x10;
        }
        self
    }
}

impl fmt::Display for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", 
            (0..12).map(|i| self.cubie(i).to_string()).collect::<Vec<String>>().join(" "),
            (0..12).map(|i| self.orientation(i).to_string()).collect::<Vec<String>>().join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_composed_twists() {
        let mut rnd = RandomTwistGen::new(4729365, TwistSet::full());
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            e = e.twisted(rnd.gen_twist());
            assert_eq!(e.twisted(Twist::L2), e.twisted_by(&[Twist::L1, Twist::L1]));
            assert_eq!(e.twisted(Twist::L3), e.twisted_by(&[Twist::L1, Twist::L1, Twist::L1]));
            assert_eq!(e.twisted(Twist::R2), e.twisted_by(&[Twist::R1, Twist::R1]));
            assert_eq!(e.twisted(Twist::R3), e.twisted_by(&[Twist::R1, Twist::R1, Twist::R1]));
            assert_eq!(e.twisted(Twist::U2), e.twisted_by(&[Twist::U1, Twist::U1]));
            assert_eq!(e.twisted(Twist::U3), e.twisted_by(&[Twist::U1, Twist::U1, Twist::U1]));
            assert_eq!(e.twisted(Twist::D2), e.twisted_by(&[Twist::D1, Twist::D1]));
            assert_eq!(e.twisted(Twist::D3), e.twisted_by(&[Twist::D1, Twist::D1, Twist::D1]));
            assert_eq!(e.twisted(Twist::F2), e.twisted_by(&[Twist::F1, Twist::F1]));
            assert_eq!(e.twisted(Twist::F3), e.twisted_by(&[Twist::F1, Twist::F1, Twist::F1]));
            assert_eq!(e.twisted(Twist::B2), e.twisted_by(&[Twist::B1, Twist::B1]));
            assert_eq!(e.twisted(Twist::B3), e.twisted_by(&[Twist::B1, Twist::B1, Twist::B1]));
        }
    }

    #[test] 
    fn test_inverse_twists() {
        let mut rnd = RandomTwistGen::new(4729365, TwistSet::full());
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            e = e.twisted(rnd.gen_twist());
            for twist in TwistSet::full().iter() {
                let t1 = twist;
                let t2 = inversed(twist);
                assert_eq!(e.twisted_by(&[t1, t2]), e, "Inverse twist failed for {:?}", twist);
            }
        }
    }

    #[test]
    fn test_twists_cycle() {
        let mut rnd = RandomTwistGen::new(4729365, TwistSet::full());
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            e = e.twisted(rnd.gen_twist());
            for t in TwistSet::full().iter() {
                assert_eq!(e.twisted_by(&[t, t, t, t]), e, "4x twist failed for {:?}", t);
            }
        }
    }

    fn assert_twists_commute(a: Twist, b: Twist) {
        let mut rnd = RandomTwistGen::new(32468723, TwistSet::full());
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            e = e.twisted(rnd.gen_twist());
            assert_eq!(
                e.twisted_by(&[a, b]),
                e.twisted_by(&[b, a]),
                "Twists {:?} and {:?} did not commute", a, b
            );
        }
    }

    #[test]
    fn test_twist_commutation() {
        assert_twists_commute(Twist::L1, Twist::R1);
        assert_twists_commute(Twist::U1, Twist::D1);
        assert_twists_commute(Twist::F1, Twist::B1);
    }

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, TwistSet::full());

        // Test from_indices and prm_index/ori_index
        let mut e = Edges::solved();
        println!("Solved Edges: {}", e);
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            print!("Applying twist {:?} -> ", twist);
            e = e.twisted(twist);
            let slice_prm = e.slice_prm_index();
            let non_slice_prm = e.non_slice_prm_index();
            let slice_loc = e.slice_loc_index();
            let ori = e.ori_index();
            println!("Indices: {}, {}, {}, {}", slice_prm, non_slice_prm, slice_loc, ori);
            assert!(slice_prm < Edges::SLICE_PRM_SIZE);
            assert!(non_slice_prm < Edges::NON_SLICE_PRM_SIZE);
            assert!(slice_loc < Edges::SLICE_LOC_SIZE);
            assert!(ori < Edges::ORI_SIZE);
            assert_eq!(e, Edges::from_indices(slice_prm, non_slice_prm, slice_loc, ori));
        }
    }
}
