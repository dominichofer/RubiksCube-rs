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
    /// - bits 0-3: cubie position (0-11)
    /// - bits 4-7: orientation (0-1, stored << 4)
    s: [u8; 12],
}

impl Edges {
    /// Size constants for indexing
    pub const SLICE_PRM_SIZE: u8 = factorial(4) as u8; // 24
    pub const NON_SLICE_PRM_SIZE: u16 = factorial(8) as u16; // 40'320
    pub const SLICE_LOC_SIZE: u16 = binomial(12, 4) as u16; // 495
    pub const ORI_SIZE: u16 = 2u16.pow(11); // 2'048

    fn new(corners: [u8; 12], orientations: [u8; 12]) -> Self {
        Self {
            s: [
                corners[0] | (orientations[0] << 4),
                corners[1] | (orientations[1] << 4),
                corners[2] | (orientations[2] << 4),
                corners[3] | (orientations[3] << 4),
                corners[4] | (orientations[4] << 4),
                corners[5] | (orientations[5] << 4),
                corners[6] | (orientations[6] << 4),
                corners[7] | (orientations[7] << 4),
                corners[8] | (orientations[8] << 4),
                corners[9] | (orientations[9] << 4),
                corners[10] | (orientations[10] << 4),
                corners[11] | (orientations[11] << 4),
            ]
        }
    }

    fn cubie(&self, index: usize) -> u8 {
        self.s[index] & 0x0F
    }

    fn orientation(&self, index: usize) -> u8 {
        self.s[index] >> 4
    }

    pub fn cubies(&self) -> [u8; 12] {
        let mut c = [0u8; 12];
        for i in 0..12 {
            c[i] = self.cubie(i);
        }
        c
    }

    fn orientations(&self) -> [u8; 12] {
        let mut o = [0u8; 12];
        for i in 0..12 {
            o[i] = self.orientation(i);
        }
        o
    }

    pub fn solved() -> Self {
        Self::new(
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0; 12],
        )
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    pub fn is_even_permutation(&self) -> bool {
        is_even_permutation_array(&self.cubies())
    }
    
    /// Constructs an `Edges` instance from the given indices.
    /// - `slice_prm`: Slice permutation index (0 to SLICE_PRM_SIZE - 1).
    /// - `non_slice_prm`: Non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    /// - `slice_loc_index`: Slice location index (0 to SLICE_LOC_SIZE - 1).
    /// - `ori`: Orientation index (0 to ORI_SIZE - 1).
    pub fn from_index(slice_prm: u8, non_slice_prm: u16, slice_loc_index: u16, ori: u16) -> Self {
        let slice_loc = nth_combination(12, 4, slice_loc_index as i64);
        let non_slice = nth_permutation(non_slice_prm as i64, 8);
        let mut slice = nth_permutation(slice_prm as i64, 4);
        for i in 0..4 {
            slice[i] += 8;
        }

        let mut e = non_slice;
        for i in 0..4 {
            e.insert(slice_loc[i] as usize, slice[i]);
        }

        let mut o = [0u8; 12];
        for i in 0..11 {
            o[i] = ((ori >> i) & 1) as u8;
        }
        o[11] = (ori.count_ones() % 2) as u8;

        Self::new(e.try_into().unwrap(), o)
    }

    /// Get the slice permutation index (0 to SLICE_PRM_SIZE - 1).
    pub fn slice_prm_index(&self) -> u8 {
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
        permutation_index(&slice) as u8
    }

    /// Get the non-slice permutation index (0 to NON_SLICE_PRM_SIZE - 1).
    pub fn non_slice_prm_index(&self) -> u16 {
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
        permutation_index(&non_slice) as u16
    }

    /// Get the slice location index (0 to SLICE_LOC_SIZE - 1).
    pub fn slice_loc_index(&self) -> u16 {
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
        combination_index(12, &loc) as u16
    }

    /// Get the orientation index (0 to ORI_SIZE - 1).
    pub fn ori_index(&self) -> u16 {
        let mut index = 0;
        for i in 0..11 {
            index |= (self.orientation(i) as u16) << i;
        }
        index
    }
}

impl Twistable for Edges {
    fn twisted(&self, twist: Twist) -> Self {
        match twist {
            Twist::L1 => Edges{ s: ori_swap_l(shuffled(&self.s, 0, 1, 2, 3, 11, 5, 6, 8, 4, 9, 10, 7)) },
            Twist::L2 => Edges{ s: shuffled(&self.s, 0, 1, 2, 3, 7, 5, 6, 4, 11, 9, 10, 8) },
            Twist::L3 => Edges{ s: ori_swap_l(shuffled(&self.s, 0, 1, 2, 3, 8, 5, 6, 11, 7, 9, 10, 4)) },
            Twist::R1 => Edges{ s: ori_swap_r(shuffled(&self.s, 0, 1, 2, 3, 4, 9, 10, 7, 8, 6, 5, 11)) },
            Twist::R2 => Edges{ s: shuffled(&self.s, 0, 1, 2, 3, 4, 6, 5, 7, 8, 10, 9, 11) },
            Twist::R3 => Edges{ s: ori_swap_r(shuffled(&self.s, 0, 1, 2, 3, 4, 10, 9, 7, 8, 5, 6, 11)) },
            Twist::U1 => Edges{ s: shuffled(&self.s, 5, 4, 2, 3, 0, 1, 6, 7, 8, 9, 10, 11) },
            Twist::U2 => Edges{ s: shuffled(&self.s, 1, 0, 2, 3, 5, 4, 6, 7, 8, 9, 10, 11) },
            Twist::U3 => Edges{ s: shuffled(&self.s, 4, 5, 2, 3, 1, 0, 6, 7, 8, 9, 10, 11) },
            Twist::D1 => Edges{ s: shuffled(&self.s, 0, 1, 6, 7, 4, 5, 3, 2, 8, 9, 10, 11) },
            Twist::D2 => Edges{ s: shuffled(&self.s, 0, 1, 3, 2, 4, 5, 7, 6, 8, 9, 10, 11) },
            Twist::D3 => Edges{ s: shuffled(&self.s, 0, 1, 7, 6, 4, 5, 2, 3, 8, 9, 10, 11) },
            Twist::F1 => Edges{ s: shuffled(&self.s, 8, 1, 2, 9, 4, 5, 6, 7, 3, 0, 10, 11) },
            Twist::F2 => Edges{ s: shuffled(&self.s, 3, 1, 2, 0, 4, 5, 6, 7, 9, 8, 10, 11) },
            Twist::F3 => Edges{ s: shuffled(&self.s, 9, 1, 2, 8, 4, 5, 6, 7, 0, 3, 10, 11) },
            Twist::B1 => Edges{ s: shuffled(&self.s, 0, 10, 11, 3, 4, 5, 6, 7, 8, 9, 2, 1) },
            Twist::B2 => Edges{ s: shuffled(&self.s, 0, 2, 1, 3, 4, 5, 6, 7, 8, 9, 11, 10) },
            Twist::B3 => Edges{ s: shuffled(&self.s, 0, 11, 10, 3, 4, 5, 6, 7, 8, 9, 1, 2) },
        }
    }
}

impl fmt::Display for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", 
            self.cubies().iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "),
            self.orientations().iter().map(|o| o.to_string()).collect::<Vec<_>>().join(" ")
        )
    }
}

fn ori_swap_l(mut s: [u8; 12]) -> [u8; 12] {
    for i in [4, 7, 8, 11] {
        s[i] ^= 0x10;
    }
    s
}

fn ori_swap_r(mut s: [u8; 12]) -> [u8; 12] {
    for i in [5, 6, 9, 10] {
        s[i] ^= 0x10;
    }
    s
}

fn shuffled(s: &[u8; 12], a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, g: usize, h: usize, i: usize, j: usize, k: usize, l: usize) -> [u8; 12] {
    [s[a], s[b], s[c], s[d], s[e], s[f], s[g], s[h], s[i], s[j], s[k], s[l]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved() {
        let e = Edges::solved();
        assert!(e.is_solved());
        assert!(!e.twisted(Twist::L1).is_solved());
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
        let e = Edges::solved();
        assert_eq!(e.twisted(Twist::L2), e.twisted(Twist::L1).twisted(Twist::L1));
        assert_eq!(e.twisted(Twist::L3), e.twisted(Twist::L1).twisted(Twist::L1).twisted(Twist::L1));
        assert_eq!(e.twisted(Twist::R2), e.twisted(Twist::R1).twisted(Twist::R1));
        assert_eq!(e.twisted(Twist::R3), e.twisted(Twist::R1).twisted(Twist::R1).twisted(Twist::R1));
        assert_eq!(e.twisted(Twist::U2), e.twisted(Twist::U1).twisted(Twist::U1));
        assert_eq!(e.twisted(Twist::U3), e.twisted(Twist::U1).twisted(Twist::U1).twisted(Twist::U1));
        assert_eq!(e.twisted(Twist::D2), e.twisted(Twist::D1).twisted(Twist::D1));
        assert_eq!(e.twisted(Twist::D3), e.twisted(Twist::D1).twisted(Twist::D1).twisted(Twist::D1));
        assert_eq!(e.twisted(Twist::F2), e.twisted(Twist::F1).twisted(Twist::F1));
        assert_eq!(e.twisted(Twist::F3), e.twisted(Twist::F1).twisted(Twist::F1).twisted(Twist::F1));
        assert_eq!(e.twisted(Twist::B2), e.twisted(Twist::B1).twisted(Twist::B1));
        assert_eq!(e.twisted(Twist::B3), e.twisted(Twist::B1).twisted(Twist::B1).twisted(Twist::B1));
    }

    #[test] 
    fn test_inverse_twists() {
        for twist in Twists::all().iter() {
            assert!(Edges::solved().twisted(twist).twisted(inversed(twist)).is_solved(), "Inverse twist failed for {:?}", twist);
        }
    }

    #[test]
    fn test_twists_cycle() {
        for twist in Twists::all().iter() {
            let mut e = Edges::solved();
            for _ in 0..4 {
                e = e.twisted(twist);
            }
            assert_eq!(e, Edges::solved(), "4x twist failed for {:?}", twist);
        }
    }

    fn assert_twists_commute(a: Twist, b: Twist) {
        assert_eq!(
            Edges::solved().twisted(a).twisted(b),
            Edges::solved().twisted(b).twisted(a),
            "Twists {:?} and {:?} did not commute", a, b
        );
    }

    #[test]
    fn test_twist_commutation() {
        assert_twists_commute(Twist::L1, Twist::R1);
        assert_twists_commute(Twist::U1, Twist::D1);
        assert_twists_commute(Twist::F1, Twist::B1);
    }

    #[test]
    fn test_indexing() {
        let mut rnd = RandomTwistGen::new(181086, Twists::all());

        // Test from_index and prm_index/ori_index
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
            assert_eq!(e, Edges::from_index(slice_prm, non_slice_prm, slice_loc, ori));
        }
    }
}
