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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Corners {
    /// Internal state: each byte encodes a corner where:
    /// - bits 0-3: cubie position (0-7)
    /// - bits 4-7: orientation (0-2, stored << 4)
    s: [u8; 8],
}

impl Corners {
    /// Size constants for indexing
    pub const PRM_SIZE: u16 = factorial(8) as u16; // 40'320
    pub const ORI_SIZE: u16 = 3u16.pow(7); // 2'187
    pub const INDEX_SIZE: u32 = Self::PRM_SIZE as u32 * Self::ORI_SIZE as u32; // 88'179'840

    fn new(corners: [u8; 8], orientations: [u8; 8]) -> Self {
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
            ]
        }
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
        let mut c = [0u8; 8];
        for i in 0..8 {
            c[i] = self.cubie(i);
        }
        c
    }

    fn orientations(&self) -> [u8; 8] {
        let mut o = [0u8; 8];
        for i in 0..8 {
            o[i] = self.orientation(i);
        }
        o
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    /// Create Corners from permutation and orientation indices
    /// - `prm`: permutation index (0 to PRM_SIZE-1)
    /// - `ori`: orientation index (0 to ORI_SIZE-1)
    pub fn from_index(prm: u16, ori: u16) -> Self {
        let p: [u8; 8] = nth_permutation(prm as i64, 8).try_into().unwrap();

        // Decode orientations from base-3 representation
        let mut ori = ori;
        let o0 = (ori % 3) as u8; ori /= 3;
        let o1 = (ori % 3) as u8; ori /= 3;
        let o2 = (ori % 3) as u8; ori /= 3;
        let o3 = (ori % 3) as u8; ori /= 3;
        let o4 = (ori % 3) as u8; ori /= 3;
        let o5 = (ori % 3) as u8; ori /= 3;
        let o6 = (ori % 3) as u8;
        let o7 = ((12 + o0 - o1 - o2 + o3 - o4 + o5 + o6) % 3) as u8;

        Self::new(p, [o0, o1, o2, o3, o4, o5, o6, o7])
    }

    /// Get the permutation index (0 to PRM_SIZE-1)
    pub fn prm_index(&self) -> u16 {
        permutation_index(&self.cubies()) as u16
    }

    /// Get the orientation index (0 to ORI_SIZE-1)
    pub fn ori_index(&self) -> u16 {
        let o = self.orientations();
        o[0] as u16
        + o[1] as u16 * 3
        + o[2] as u16 * 9
        + o[3] as u16 * 27
        + o[4] as u16 * 81
        + o[5] as u16 * 243
        + o[6] as u16 * 729
    }

    /// Create Corners from a combined index (0 to INDEX_SIZE-1)
    pub fn from_combined_index(index: u32) -> Self {
        Self::from_index(
            (index / Self::ORI_SIZE as u32) as u16,
            (index % Self::ORI_SIZE as u32) as u16,
        )
    }

    /// Get the combined index (0 to INDEX_SIZE-1)
    pub fn index(&self) -> u32 {
        self.prm_index() as u32 * Self::ORI_SIZE as u32 + self.ori_index() as u32
    }
}

impl Twistable for Corners {
    fn twisted(&self, twist: Twist) -> Self {
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
}

impl fmt::Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", 
            self.cubies().iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "),
            self.orientations().iter().map(|o| o.to_string()).collect::<Vec<_>>().join(" ")
        )
    }
}

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
    fn test_solved() {
        let c = Corners::solved();
        assert!(c.is_solved());
        assert!(!c.twisted(Twist::L1).is_solved());
        assert_eq!(c.to_string(), "0 1 2 3 4 5 6 7 | 0 0 0 0 0 0 0 0");
    }

    #[test]
    fn test_twist_results() {
        let c = Corners::solved();
        assert_eq!(c.twisted(Twist::L1).to_string(), "2 1 6 3 0 5 4 7 | 2 0 2 0 2 0 2 0");
        assert_eq!(c.twisted(Twist::R1).to_string(), "0 5 2 1 4 7 6 3 | 0 2 0 2 0 2 0 2");
        assert_eq!(c.twisted(Twist::U1).to_string(), "1 3 0 2 4 5 6 7 | 0 0 0 0 0 0 0 0");
        assert_eq!(c.twisted(Twist::D1).to_string(), "0 1 2 3 6 4 7 5 | 0 0 0 0 0 0 0 0");
        assert_eq!(c.twisted(Twist::F1).to_string(), "4 0 2 3 5 1 6 7 | 1 1 0 0 1 1 0 0");
        assert_eq!(c.twisted(Twist::B1).to_string(), "0 1 3 7 4 5 2 6 | 0 0 1 1 0 0 1 1");
    }

    #[test]
    fn test_composed_twists() {
        let c = Corners::solved();
        assert_eq!(c.twisted(Twist::L2), c.twisted(Twist::L1).twisted(Twist::L1));
        assert_eq!(c.twisted(Twist::L3), c.twisted(Twist::L1).twisted(Twist::L1).twisted(Twist::L1));
        assert_eq!(c.twisted(Twist::R2), c.twisted(Twist::R1).twisted(Twist::R1));
        assert_eq!(c.twisted(Twist::R3), c.twisted(Twist::R1).twisted(Twist::R1).twisted(Twist::R1));
        assert_eq!(c.twisted(Twist::U2), c.twisted(Twist::U1).twisted(Twist::U1));
        assert_eq!(c.twisted(Twist::U3), c.twisted(Twist::U1).twisted(Twist::U1).twisted(Twist::U1));
        assert_eq!(c.twisted(Twist::D2), c.twisted(Twist::D1).twisted(Twist::D1));
        assert_eq!(c.twisted(Twist::D3), c.twisted(Twist::D1).twisted(Twist::D1).twisted(Twist::D1));
        assert_eq!(c.twisted(Twist::F2), c.twisted(Twist::F1).twisted(Twist::F1));
        assert_eq!(c.twisted(Twist::F3), c.twisted(Twist::F1).twisted(Twist::F1).twisted(Twist::F1));
        assert_eq!(c.twisted(Twist::B2), c.twisted(Twist::B1).twisted(Twist::B1));
        assert_eq!(c.twisted(Twist::B3), c.twisted(Twist::B1).twisted(Twist::B1).twisted(Twist::B1));
    }

    #[test] 
    fn test_inverse_twists() {
        for twist in Twists::all().iter() {
            assert!(Corners::solved().twisted(twist).twisted(inversed(twist)).is_solved(), "Inverse twist failed for {:?}", twist);
        }
    }

    #[test]
    fn test_twists_cycle() {
        for twist in Twists::all().iter() {
            let mut c = Corners::solved();
            for _ in 0..4 {
                c = c.twisted(twist);
            }
            assert_eq!(c, Corners::solved(), "4x twist failed for {:?}", twist);
        }
    }

    fn assert_twists_commute(a: Twist, b: Twist) {
        assert_eq!(
            Corners::solved().twisted(a).twisted(b),
            Corners::solved().twisted(b).twisted(a),
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
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            let prm = c.prm_index();
            let ori = c.ori_index();
            assert!(prm < Corners::PRM_SIZE);
            assert!(ori < Corners::ORI_SIZE);
            assert_eq!(c, Corners::from_index(prm, ori));
        }

        // Test from_combined_index and index
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            let index = c.index();
            assert!(index < Corners::INDEX_SIZE);
            assert_eq!(c, Corners::from_combined_index(index));
        }
    }
}
