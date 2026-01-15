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
    /// - bits 0 to 3: cubie position (values 0 to 7)
    /// - bits 4 to 5: orientation (values 0 to 2)
    s: [u8; 8],
}

impl Corners {
    /// Size constants for indexing
    pub const PRM_SIZE: usize = factorial(8); // 40'320
    pub const ORI_SIZE: usize = 3usize.pow(7); // 2'187
    pub const INDEX_SIZE: usize = Self::PRM_SIZE * Self::ORI_SIZE; // 88'179'840

    pub const fn solved() -> Self {
        Self { s: [0, 1, 2, 3, 4, 5, 6, 7] }
    }

    fn cubie(&self, index: usize) -> u8 {
        self.s[index] & 0x0F
    }

    fn orientation(&self, index: usize) -> u8 {
        self.s[index] >> 4
    }

    /// Create Corners from permutation and orientation indices
    /// - `prm`: permutation index (0 to PRM_SIZE-1)
    /// - `ori`: orientation index (0 to ORI_SIZE-1)
    pub fn from_indices(prm: usize, ori: usize) -> Self {
        // Decode orientations from base-3 representation
        let mut ori = ori;
        let o0 = ori % 3; ori /= 3;
        let o1 = ori % 3; ori /= 3;
        let o2 = ori % 3; ori /= 3;
        let o3 = ori % 3; ori /= 3;
        let o4 = ori % 3; ori /= 3;
        let o5 = ori % 3; ori /= 3;
        let o6 = ori % 3;
        let o7 = (12 + o0 - o1 - o2 + o3 - o4 + o5 + o6) % 3;
        let c = nth_permutation(prm, 8);
        Self {
            s: [
                c[0] | (o0 << 4) as u8,
                c[1] | (o1 << 4) as u8,
                c[2] | (o2 << 4) as u8,
                c[3] | (o3 << 4) as u8,
                c[4] | (o4 << 4) as u8,
                c[5] | (o5 << 4) as u8,
                c[6] | (o6 << 4) as u8,
                c[7] | (o7 << 4) as u8,
            ]
        }
    }

    /// Get the permutation index (0 to PRM_SIZE-1)
    pub fn prm_index(&self) -> usize {
        let mut cubies = [0u8; 8];
        for i in 0..8 {
            cubies[i] = self.cubie(i);
        }
        permutation_index(&cubies)
    }

    /// Get the orientation index (0 to ORI_SIZE-1)
    pub fn ori_index(&self) -> usize {
        self.orientation(0) as usize
        + self.orientation(1) as usize * 3
        + self.orientation(2) as usize * 9
        + self.orientation(3) as usize * 27
        + self.orientation(4) as usize * 81
        + self.orientation(5) as usize * 243
        + self.orientation(6) as usize * 729
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        match twist {
            Twist::L1 => self.shuffled(2, 1, 6, 3, 0, 5, 4, 7).ori_swap_0_2([0, 2, 4, 6]),
            Twist::L2 => self.shuffled(6, 1, 4, 3, 2, 5, 0, 7),
            Twist::L3 => self.shuffled(4, 1, 0, 3, 6, 5, 2, 7).ori_swap_0_2([0, 2, 4, 6]),
            Twist::R1 => self.shuffled(0, 5, 2, 1, 4, 7, 6, 3).ori_swap_0_2([1, 3, 5, 7]),
            Twist::R2 => self.shuffled(0, 7, 2, 5, 4, 3, 6, 1),
            Twist::R3 => self.shuffled(0, 3, 2, 7, 4, 1, 6, 5).ori_swap_0_2([1, 3, 5, 7]),
            Twist::U1 => self.shuffled(1, 3, 0, 2, 4, 5, 6, 7).ori_swap_1_2([0, 1, 2, 3]),
            Twist::U2 => self.shuffled(3, 2, 1, 0, 4, 5, 6, 7),
            Twist::U3 => self.shuffled(2, 0, 3, 1, 4, 5, 6, 7).ori_swap_1_2([0, 1, 2, 3]),
            Twist::D1 => self.shuffled(0, 1, 2, 3, 6, 4, 7, 5).ori_swap_1_2([4, 5, 6, 7]),
            Twist::D2 => self.shuffled(0, 1, 2, 3, 7, 6, 5, 4),
            Twist::D3 => self.shuffled(0, 1, 2, 3, 5, 7, 4, 6).ori_swap_1_2([4, 5, 6, 7]),
            Twist::F1 => self.shuffled(4, 0, 2, 3, 5, 1, 6, 7).ori_swap_0_1([0, 1, 4, 5]),
            Twist::F2 => self.shuffled(5, 4, 2, 3, 1, 0, 6, 7),
            Twist::F3 => self.shuffled(1, 5, 2, 3, 0, 4, 6, 7).ori_swap_0_1([0, 1, 4, 5]),
            Twist::B1 => self.shuffled(0, 1, 3, 7, 4, 5, 2, 6).ori_swap_0_1([2, 3, 6, 7]),
            Twist::B2 => self.shuffled(0, 1, 7, 6, 4, 5, 3, 2),
            Twist::B3 => self.shuffled(0, 1, 6, 2, 4, 5, 7, 3).ori_swap_0_1([2, 3, 6, 7]),
            Twist::None => *self,
        }
    }

    pub fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists.iter().fold(self.clone(), |cube, &twist| {
            cube.twisted(twist)
        })
    }

    fn shuffled(&self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, g: usize, h: usize) -> Self {
        Self { s: [self.s[a], self.s[b], self.s[c], self.s[d], self.s[e], self.s[f], self.s[g], self.s[h]] }
    }
    
    fn ori_swap_0_1(mut self, indices: [usize; 4]) -> Self {
        for &i in &indices {
            self.s[i] ^= (!self.s[i] & 0x20) >> 1;
        }
        self
    }
    
    fn ori_swap_0_2(mut self, indices: [usize; 4]) -> Self {
        for &i in &indices {
            self.s[i] = (0x20 - (self.s[i] & 0x30)) | (self.s[i] & 0x0F);
        }
        self
    }
    
    fn ori_swap_1_2(mut self, indices: [usize; 4]) -> Self {
        for &i in &indices {
            let l = (self.s[i] & 0x20) >> 1;
            let r = (self.s[i] & 0x10) << 1;
            self.s[i] = (self.s[i] & 0x0F) | l | r;
        }
        self
    }
}

impl fmt::Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", 
            (0..8).map(|i| self.cubie(i).to_string()).collect::<Vec<String>>().join(" "),
            (0..8).map(|i| self.orientation(i).to_string()).collect::<Vec<String>>().join(" ")
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved() {
        let c = Corners::solved();
        assert_ne!(c.twisted(Twist::L1), Corners::solved());
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
        let mut rnd = RandomTwistGen::new(4729365, TwistSet::full());
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
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
    }

    #[test] 
    fn test_inverse_twists() {
        let mut rnd = RandomTwistGen::new(4729365, TwistSet::full());
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            for twist in TwistSet::full().iter() {
                let t1 = twist;
                let t2 = inversed(twist);
                assert_eq!(c.twisted_by(&[t1, t2]), c, "Inverse twist failed for {:?}", twist);
            }
        }
    }

    #[test]
    fn test_twists_cycle() {
        let mut rnd = RandomTwistGen::new(3423598, TwistSet::full());
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            for t in TwistSet::full().iter() {
                assert_eq!(c.twisted_by(&[t, t, t, t]), c, "4x twist failed for {:?}", t);
            }
        }
    }

    fn assert_twists_commute(a: Twist, b: Twist) {
        let mut rnd = RandomTwistGen::new(32468723, TwistSet::full());
        let mut c = Corners::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            assert_eq!(
                c.twisted_by(&[a, b]),
                c.twisted_by(&[b, a]),
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
