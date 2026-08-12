use crate::math::*;
use crate::twist::*;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

/// Face numbering:
///              +----------+
///              |    Up    |
///              | 00 01 02 |
///              | 03 04 05 |
///              | 06 07 08 |
///   +----------+----------+----------+----------+
///   |   Left   |   Front  |   Right  |   Back   |
///   | 36 37 38 | 18 19 20 | 09 10 11 | 45 46 47 |
///   | 39 40 41 | 21 22 23 | 12 13 14 | 48 49 50 |
///   | 42 43 44 | 24 25 26 | 15 16 17 | 51 52 53 |
///   +----------+----------+----------+----------+
///              |   Down   |
///              | 27 28 29 |
///              | 30 31 32 |
///              | 33 34 35 |
///              +----------+
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FaceCube {
    prm: Permutation<54>,
}

impl FaceCube {
    fn rotation(a: usize, b: usize, c: usize, d: usize) -> Self {
        Self { prm: Permutation::rotation([a, b, c, d]) }
    }

    pub fn solved() -> Self {
        Self { prm: Permutation::identity() }
    }

    pub fn twist(twist: Twist) -> Self {
        match twist {
            Twist::L1 => 
                Self::rotation(36, 38, 44, 42) *
                Self::rotation(37, 41, 43, 39) *
                Self::rotation(00, 18, 27, 53) *
                Self::rotation(03, 21, 30, 50) *
                Self::rotation(06, 24, 33, 47),
            Twist::R1 =>
                Self::rotation(09, 11, 17, 15) *
                Self::rotation(10, 14, 16, 12) *
                Self::rotation(02, 51, 29, 20) *
                Self::rotation(05, 48, 32, 23) *
                Self::rotation(08, 45, 35, 26),
            Twist::U1 =>
                Self::rotation(00, 02, 08, 06) *
                Self::rotation(01, 05, 07, 03) *
                Self::rotation(18, 36, 45, 09) *
                Self::rotation(19, 37, 46, 10) *
                Self::rotation(20, 38, 47, 11),
            Twist::D1 =>
                Self::rotation(27, 29, 35, 33) *
                Self::rotation(28, 32, 34, 30) *
                Self::rotation(24, 15, 51, 42) *
                Self::rotation(25, 16, 52, 43) *
                Self::rotation(26, 17, 53, 44),
            Twist::F1 =>
                Self::rotation(18, 20, 26, 24) *
                Self::rotation(19, 23, 25, 21) *
                Self::rotation(06, 09, 29, 44) *
                Self::rotation(07, 12, 28, 41) *
                Self::rotation(08, 15, 27, 38),
            Twist::B1 =>
                Self::rotation(45, 47, 53, 51) *
                Self::rotation(46, 50, 52, 48) *
                Self::rotation(00, 42, 35, 11) *
                Self::rotation(01, 39, 34, 14) *
                Self::rotation(02, 36, 33, 17),

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

    pub fn string_1d(&self) -> String {
        let mut out = String::new();
        for i in 0..54 {
            let face_char = match self.prm[i] {
                0..=8 => 'U',
                9..=17 => 'R',
                18..=26 => 'F',
                27..=35 => 'D',
                36..=44 => 'L',
                45..=53 => 'B',
                _ => unreachable!(),
            };
            out.push(face_char);
        }
        out
    }

    pub fn string_2d(&self) -> String {
        // face characters
        let c = self.prm.iter().map(|i| match i {
            0..=8 => 'U',
            9..=17 => 'R',
            18..=26 => 'F',
            27..=35 => 'D',
            36..=44 => 'L',
            45..=53 => 'B',
            _ => unreachable!(),
        }).collect::<Vec<_>>();

        let mut out = String::new();
        out.push_str(&format!("           +----------+\n"));
        out.push_str(&format!("           |    Up    |\n"));
        out.push_str(&format!("           | {:02} {:02} {:02} |\n", c[0], c[1], c[2]));
        out.push_str(&format!("           | {:02} {:02} {:02} |\n", c[3], c[4], c[5]));
        out.push_str(&format!("           | {:02} {:02} {:02} |\n", c[6], c[7], c[8]));
        out.push_str(&format!("+----------+----------+----------+----------+\n"));
        out.push_str(&format!("|   Left   |   Front  |   Right  |   Back   |\n"));
        out.push_str(&format!("| {:02} {:02} {:02} | {:02} {:02} {:02} | {:02} {:02} {:02} | {:02} {:02} {:02} |\n", c[36], c[37], c[38], c[18], c[19], c[20], c[9], c[10], c[11], c[45], c[46], c[47]));
        out.push_str(&format!("| {:02} {:02} {:02} | {:02} {:02} {:02} | {:02} {:02} {:02} | {:02} {:02} {:02} |\n", c[39], c[40], c[41], c[21], c[22], c[23], c[12], c[13], c[14], c[48], c[49], c[50]));
        out.push_str(&format!("| {:02} {:02} {:02} | {:02} {:02} {:02} | {:02} {:02} {:02} | {:02} {:02} {:02} |\n", c[42], c[43], c[44], c[24], c[25], c[26], c[15], c[16], c[17], c[51], c[52], c[53]));
        out.push_str(&format!("+----------+----------+----------+----------+\n"));
        out.push_str(&format!("           |   Down   |\n"));
        out.push_str(&format!("           | {:02} {:02} {:02} |\n", c[27], c[28], c[29]));
        out.push_str(&format!("           | {:02} {:02} {:02} |\n", c[30], c[31], c[32]));
        out.push_str(&format!("           | {:02} {:02} {:02} |\n", c[33], c[34], c[35]));
        out.push_str(&format!("           +----------+"));
        out
    }
}

/// FaceCube * FaceCube
impl Mul<FaceCube> for FaceCube {
    type Output = FaceCube;

    fn mul(self, r: FaceCube) -> FaceCube {
        FaceCube { prm: self.prm * r.prm }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle_length(twists: &[Twist]) -> usize {
        let mut current = FaceCube::twists(twists);
        let mut length = 1;
        while current != FaceCube::solved() {
            current = FaceCube::twists(twists) * current;
            length += 1;
        }
        length
    }

    #[test]
    fn test_twist_commutation() {
        let l = FaceCube::twist(Twist::L1);
        let r = FaceCube::twist(Twist::R1);
        let u = FaceCube::twist(Twist::U1);
        let d = FaceCube::twist(Twist::D1);
        let f = FaceCube::twist(Twist::F1);
        let b = FaceCube::twist(Twist::B1);
        
        assert_eq!(l * r, r * l, "L and R should commutate");
        assert_eq!(u * d, d * u, "U and D should commutate");
        assert_eq!(f * b, b * f, "F and B should commutate");
    }

    #[test]
    fn test_twist_cycles() {
        for t in [Twist::L2, Twist::R2, Twist::U2, Twist::D2, Twist::F2, Twist::B2] {
            assert_eq!(cycle_length(&[t]), 2, "Twist {:?} should have cycle length 2", t);
        }
        for t in [Twist::L1, Twist::L3, Twist::R1, Twist::R3, Twist::U1, Twist::U3, Twist::D1, Twist::D3, Twist::F1, Twist::F3, Twist::B1, Twist::B3] {
            assert_eq!(cycle_length(&[t]), 4, "Twist {:?} should have cycle length 4", t);
        }
        for t in [Twist::U1, Twist::D1, Twist::F1, Twist::B1] {
            assert_eq!(cycle_length(&[Twist::L1, t]), 105, "Twists L1 and {:?} should have cycle length 105", t);
        }
        for t in [Twist::U2, Twist::D2, Twist::F2, Twist::B2] {
            assert_eq!(cycle_length(&[Twist::L1, t]), 30, "Twists L1 and {:?} should have cycle length 30", t);
        }
        for t in [Twist::U3, Twist::D3, Twist::F3, Twist::B3] {
            assert_eq!(cycle_length(&[Twist::L1, t]), 63, "Twists L1 and {:?} should have cycle length 63", t);
        }
        assert_eq!(cycle_length(&[Twist::R1, Twist::U2, Twist::D3, Twist::B1, Twist::D3]), 1260);
    }
}