use crate::math::{factorial, permutation_index, nth_permutation};
use crate::twist::Twist;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Corners {
    /// Internal state: each byte encodes a corner where:
    /// - bits 0-3: cubie position (0-7)
    /// - bits 4-7: orientation (0-2, stored << 4)
    s: [u8; 8],
}

impl Corners {
    /// Size constants for indexing
    pub const PRM_SIZE: u64 = factorial(8); // 40'320
    pub const ORI_SIZE: u64 = 3u64.pow(7); // 2'187
    pub const INDEX_SIZE: u64 = Self::PRM_SIZE * Self::ORI_SIZE; // 88'179'840

    fn new(corners: [u8; 8], orientations: [u8; 8]) -> Self {
        let mut s = [0u8; 8];
        for i in 0..8 {
            s[i] = (orientations[i] << 4) | corners[i];
        }
        Self { s }
    }

    pub fn solved() -> Self {
        Self {
            s: [0, 1, 2, 3, 4, 5, 6, 7],
        }
    }

    fn cubie(&self, index: usize) -> u8 {
        self.s[index] & 0x0F
    }

    fn orientation(&self, index: usize) -> u8 {
        self.s[index] >> 4
    }

    fn cubies(&self) -> [u8; 8] {
        let mut ret = [0u8; 8];
        for i in 0..8 {
            ret[i] = self.cubie(i);
        }
        ret
    }

    fn orientations(&self) -> [u8; 8] {
        let mut ret = [0u8; 8];
        for i in 0..8 {
            ret[i] = self.orientation(i);
        }
        ret
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        match twist {
            Twist::L1 => Corners{ s: ori_swap_l(shuffled(&self.s, 2, 1, 6, 3, 0, 5, 4, 7)) },
            Twist::L2 => Corners{ s: shuffled(&self.s, 6, 1, 4, 3, 2, 5, 0, 7) },
            Twist::L3 => Corners{ s: ori_swap_l(shuffled(&self.s, 4, 1, 0, 3, 6, 5, 2, 7)) },
            Twist::R1 => Corners{ s: ori_swap_r(shuffled(&self.s, 0, 5, 2, 1, 4, 7, 6, 3)) },
            Twist::R2 => Corners{ s: shuffled(&self.s, 0, 7, 2, 5, 4, 3, 6, 1) },
            Twist::R3 => Corners{ s: ori_swap_r(shuffled(&self.s, 0, 3, 2, 7, 4, 1, 6, 5)) },
            Twist::U1 => Corners{ s: ori_swap_u(shuffled(&self.s, 1, 3, 0, 2, 4, 5, 6, 7)) },
            Twist::U2 => Corners{ s: shuffled(&self.s, 3, 2, 1, 0, 4, 5, 6, 7) },
            Twist::U3 => Corners{ s: ori_swap_u(shuffled(&self.s, 2, 0, 3, 1, 4, 5, 6, 7)) },
            Twist::D1 => Corners{ s: ori_swap_d(shuffled(&self.s, 0, 1, 2, 3, 6, 4, 7, 5)) },
            Twist::D2 => Corners{ s: shuffled(&self.s, 0, 1, 2, 3, 7, 6, 5, 4) },
            Twist::D3 => Corners{ s: ori_swap_d(shuffled(&self.s, 0, 1, 2, 3, 5, 7, 4, 6)) },
            Twist::F1 => Corners{ s: ori_swap_f(shuffled(&self.s, 4, 0, 2, 3, 5, 1, 6, 7)) },
            Twist::F2 => Corners{ s: shuffled(&self.s, 5, 4, 2, 3, 1, 0, 6, 7) },
            Twist::F3 => Corners{ s: ori_swap_f(shuffled(&self.s, 1, 5, 2, 3, 0, 4, 6, 7)) },
            Twist::B1 => Corners{ s: ori_swap_b(shuffled(&self.s, 0, 1, 3, 7, 4, 5, 2, 6)) },
            Twist::B2 => Corners{ s: shuffled(&self.s, 0, 1, 7, 6, 4, 5, 3, 2) },
            Twist::B3 => Corners{ s: ori_swap_b(shuffled(&self.s, 0, 1, 6, 2, 4, 5, 7, 3)) },
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    pub fn from_index(prm: u64, ori: u64) -> Self {
        let mut c = [0u8; 8];
        nth_permutation(prm, &mut c);
        let corners: [u8; 8] = [
            c[0], c[1], c[2], c[3],
            c[4], c[5], c[6], c[7],
        ];

        // Decode orientations from base-3 representation
        let mut ori = ori;
        let o6 = (ori % 3) as u8; ori /= 3;
        let o5 = (ori % 3) as u8; ori /= 3;
        let o4 = (ori % 3) as u8; ori /= 3;
        let o3 = (ori % 3) as u8; ori /= 3;
        let o2 = (ori % 3) as u8; ori /= 3;
        let o1 = (ori % 3) as u8; ori /= 3;
        let o0 = (ori % 3) as u8;
        let o7 = ((12 + o0 - o1 - o2 + o3 - o4 + o5 + o6) % 3) as u8;
        
        Self::new(corners, [o0, o1, o2, o3, o4, o5, o6, o7])
    }

    /// Gets the permutation index (0 to PRM_SIZE-1)
    pub fn prm_index(&self) -> u16 {
        permutation_index(&self.cubies()) as u16
    }

    /// Gets the orientation index (0 to ORI_SIZE-1)
    pub const fn ori_index(&self) -> u16 {
        let mut ret = 0u16;
        let mut i = 0;
        while i < 7 {
            ret = ret * 3 + self.orientation(i) as u16;
            i += 1;
        }
        ret
    }

    /// Creates Corners from combined index
    pub fn from_combined_index(index: u32) -> Self {
        Self::from_index(
            (index / Self::ORI_SIZE as u32) as u16,
            (index % Self::ORI_SIZE as u32) as u16,
        )
    }

    /// Gets the combined index
    pub fn index(&self) -> u32 {
        self.prm_index() as u32 * Self::ORI_SIZE as u32 + self.ori_index() as u32
    }
}

impl fmt::Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{} {} {} {} {} {} {} {} | {} {} {} {} {} {} {} {}", 
            c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7],
            o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7],
        )
    }
}

/// Helper functions for orientation swaps

fn ori_swap_0_1(state: u8) -> u8 {
    (((!state) & 0x20) >> 1) ^ state
}

fn ori_swap_0_2(state: u8) -> u8 {
    (0x20 - (state & 0x30)) | (state & 0x0F)
}

fn ori_swap_1_2(state: u8) -> u8 {
    let l = (state & 0x20) >> 1;
    let r = (state & 0x10) << 1;
    (state & 0x0F) | l | r
}

fn ori_swap_l(mut s: [u8; 8]) -> [u8; 8] {
    for i in [0, 2, 4, 6] {
        s[i] = ori_swap_0_2(s[i]);
    }
    s
}

fn ori_swap_r(mut s: [u8; 8]) -> [u8; 8] {
    for i in [1, 3, 5, 7] {
        s[i] = ori_swap_0_2(s[i]);
    }
    s
}

fn ori_swap_u(mut s: [u8; 8]) -> [u8; 8] {
    for i in [0, 1, 2, 3] {
        s[i] = ori_swap_1_2(s[i]);
    }
    s
}

fn ori_swap_d(mut s: [u8; 8]) -> [u8; 8] {
    for i in [4, 5, 6, 7] {
        s[i] = ori_swap_1_2(s[i]);
    }
    s
}

fn ori_swap_f(mut s: [u8; 8]) -> [u8; 8] {
    for i in [0, 1, 4, 5] {
        s[i] = ori_swap_0_1(s[i]);
    }
    s
}

fn ori_swap_b(mut s: [u8; 8]) -> [u8; 8] {
    for i in [2, 3, 6, 7] {
        s[i] = ori_swap_0_1(s[i]);
    }
    s
}

fn shuffled(s: &[u8; 8], a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, g: usize, h: usize) -> [u8; 8] {
    [s[a], s[b], s[c], s[d], s[e], s[f], s[g], s[h]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_corners() {
        let corners = Corners::solved();
        assert!(corners.is_solved());
        assert_eq!(corners.to_string(), "0 1 2 3 4 5 6 7 | 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_twist_results() {
        let corners = Corners::solved();
        assert_eq!(corners.twisted(Twist::L1).to_string(), "2 1 6 3 0 5 4 7 | 2 0 2 0 2 0 2 0");
        assert_eq!(corners.twisted(Twist::R1).to_string(), "0 5 2 1 4 7 6 3 | 0 2 0 2 0 2 0 2");
        assert_eq!(corners.twisted(Twist::U1).to_string(), "1 3 0 2 4 5 6 7 | 0 0 0 0 0 0 0 0");
        assert_eq!(corners.twisted(Twist::D1).to_string(), "0 1 2 3 6 4 7 5 | 0 0 0 0 0 0 0 0");
        assert_eq!(corners.twisted(Twist::F1).to_string(), "4 0 2 3 5 1 6 7 | 1 1 0 0 1 1 0 0");
        assert_eq!(corners.twisted(Twist::B1).to_string(), "0 1 3 7 4 5 2 6 | 0 0 1 1 0 0 1 1");
    }

    #[test]
    fn test_composed_twists() {
        let c = Corners::solved();
        assert_eq!(c.twisted(Twist::L2), c.twisted_by(&[Twist::L1, Twist::L1]));
        assert_eq!(c.twisted(Twist::L3), c.twisted_by(&[Twist::L1, Twist::L1, Twist::L1]));
        assert_eq!(c.twisted(Twist::R2), c.twisted_by(&[Twist::R1, Twist::R1]));
        assert_eq!(c.twisted(Twist::R3), c.twisted_by(&[Twist::R1, Twist::R1, Twist::R1]));
        assert_eq!(c.twisted(Twist::U2), c.twisted_by(&[Twist::U1, Twist::U1]));
        assert_eq!(c.twisted(Twist::U3), c.twisted_by(&[Twist::U1, Twist::U1, Twist::U1]));
        assert_eq!(c.twisted(Twist::D2), c.twisted_by(&[Twist::D1, Twist::D1]));
        assert_eq!(c.twisted(Twist::D3), c.twisted_by(&[Twist::D1, Twist::D1, Twist::D1]));
        assert_eq!(c.twisted(Twist::F2), c.twisted_by(&[Twist::F1, Twist::F1]));
        assert_eq!(c.twisted(Twist::F3), c.twisted_by(&[Twist::F1, Twist::F1, Twist::F1]));
        assert_eq!(c.twisted(Twist::B2), c.twisted_by(&[Twist::B1, Twist::B1]));
        assert_eq!(c.twisted(Twist::B3), c.twisted_by(&[Twist::B1, Twist::B1, Twist::B1]));
    }

    #[test] 
    fn test_inverse_twists() {
        use crate::twist::Twist::*;
        let all_twists = [
            L1, L2, L3, R1, R2, R3, U1, U2, U3, 
            D1, D2, D3, F1, F2, F3, B1, B2, B3
        ];
        let inverses = [
            L3, L2, L1, R3, R2, R1, U3, U2, U1,
            D3, D2, D1, F3, F2, F1, B3, B2, B1
        ];

        for (twist, inverse) in all_twists.iter().zip(inverses.iter()) {
            assert!(Corners::solved().twisted_by(&[*twist, *inverse]).is_solved());
        }
    }

    #[test]
    fn test_twists_cycle() {
        use crate::twist::Twist::*;
        let all_twists = [
            L1, L2, L3, R1, R2, R3, U1, U2, U3, 
            D1, D2, D3, F1, F2, F3, B1, B2, B3
        ];

        for twist in all_twists {
            assert!(Corners::solved().twisted_by(&[twist, twist, twist, twist]).is_solved());
        }
    }

    fn expect_twists_commute(a: Twist, b: Twist) {
        assert_eq!(
            Corners::solved().twisted_by(&[a, b]), 
            Corners::solved().twisted_by(&[b, a])
        );
    }

    #[test]
    fn test_twist_commutation() {
        expect_twists_commute(Twist::L1, Twist::R1);
        expect_twists_commute(Twist::U1, Twist::D1);
        expect_twists_commute(Twist::F1, Twist::B1);
    }

    #[test]
    fn test_index_bijection() {
        // Test that from_index and index are inverses
        for prm in 0..100 { // Test first 100 permutations
            for ori in 0..100 { // Test first 100 orientations
                if prm < Corners::PRM_SIZE && ori < Corners::ORI_SIZE {
                    let c = Corners::from_index(prm, ori);
                    assert_eq!(c.prm_index(), prm);
                    assert_eq!(c.ori_index(), ori);
                }
            }
        }

        // Test combined index
        for index in 0..1000 { // Test first 1000 indices
            let c = Corners::from_combined_index(index);
            assert_eq!(c.index(), index);
        }
    }

    #[test]
    fn test_is_solved() {
        assert!(Corners::solved().is_solved());
        assert!(!Corners::solved().twisted(Twist::L1).is_solved());
    }
}
