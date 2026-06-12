/// Face twist, where the number indicates how many quarter turns to perform.
///      +---------+
///     /    ←B   /|
///    /   F→    / |
///   +---------+  |
///   | L  ←U   |  |
///   | ↓     ↑ |  +
///   |   D→  R | /
///   +---------+
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Twist {
    L1, L2, L3, // Left face
    R1, R2, R3, // Right face
    U1, U2, U3, // Up face
    D1, D2, D3, // Down face
    F1, F2, F3, // Front face
    B1, B2, B3, // Back face
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X, Y, Z,
}

pub const ALL_TWISTS: [Twist; 18] = [
    Twist::L1, Twist::L2, Twist::L3,
    Twist::R1, Twist::R2, Twist::R3,
    Twist::U1, Twist::U2, Twist::U3,
    Twist::D1, Twist::D2, Twist::D3,
    Twist::F1, Twist::F2, Twist::F3,
    Twist::B1, Twist::B2, Twist::B3,
];

pub const H0_TWISTS: [Twist; 10] = [
    Twist::L2,
    Twist::R2,
    Twist::U1, Twist::U2, Twist::U3,
    Twist::D1, Twist::D2, Twist::D3,
    Twist::F2,
    Twist::B2,
];

impl TryFrom<u32> for Twist {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Twist::L1),
            1 => Ok(Twist::L2),
            2 => Ok(Twist::L3),
            3 => Ok(Twist::R1),
            4 => Ok(Twist::R2),
            5 => Ok(Twist::R3),
            6 => Ok(Twist::U1),
            7 => Ok(Twist::U2),
            8 => Ok(Twist::U3),
            9 => Ok(Twist::D1),
            10 => Ok(Twist::D2),
            11 => Ok(Twist::D3),
            12 => Ok(Twist::F1),
            13 => Ok(Twist::F2),
            14 => Ok(Twist::F3),
            15 => Ok(Twist::B1),
            16 => Ok(Twist::B2),
            17 => Ok(Twist::B3),
            _ => Err(()),
        }
    }
}

impl Twist {
    pub fn inverse(&self) -> Self {
        match self {
            Twist::L2 | Twist::R2 | Twist::U2 | Twist::D2 | Twist::F2 | Twist::B2 => *self,
            Twist::L1 => Twist::L3,
            Twist::L3 => Twist::L1,
            Twist::R1 => Twist::R3,
            Twist::R3 => Twist::R1,
            Twist::U1 => Twist::U3,
            Twist::U3 => Twist::U1,
            Twist::D1 => Twist::D3,
            Twist::D3 => Twist::D1,
            Twist::F1 => Twist::F3,
            Twist::F3 => Twist::F1,
            Twist::B1 => Twist::B3,
            Twist::B3 => Twist::B1,
        }
    }

    pub fn conjugate_by_inv(&self, rot: Axis) -> Self {
        match rot {
            Axis::X => match self {
                Twist::L1 | Twist::L2 | Twist::L3 | Twist::R1 | Twist::R2 | Twist::R3 => *self,
                Twist::U1 => Twist::B1,
                Twist::U2 => Twist::B2,
                Twist::U3 => Twist::B3,
                Twist::D1 => Twist::F1,
                Twist::D2 => Twist::F2,
                Twist::D3 => Twist::F3,
                Twist::F1 => Twist::U1,
                Twist::F2 => Twist::U2,
                Twist::F3 => Twist::U3,
                Twist::B1 => Twist::D1,
                Twist::B2 => Twist::D2,
                Twist::B3 => Twist::D3,
            },
            Axis::Y => match self {
                Twist::F1 | Twist::F2 | Twist::F3 | Twist::B1 | Twist::B2 | Twist::B3 => *self,
                Twist::L1 => Twist::D1,
                Twist::L2 => Twist::D2,
                Twist::L3 => Twist::D3,
                Twist::R1 => Twist::U1,
                Twist::R2 => Twist::U2,
                Twist::R3 => Twist::U3,
                Twist::U1 => Twist::L1,
                Twist::U2 => Twist::L2,
                Twist::U3 => Twist::L3,
                Twist::D1 => Twist::R1,
                Twist::D2 => Twist::R2,
                Twist::D3 => Twist::R3,
            },
            Axis::Z => match self {
                Twist::U1 | Twist::U2 | Twist::U3 | Twist::D1 | Twist::D2 | Twist::D3 => *self,
                Twist::L1 => Twist::B1,
                Twist::L2 => Twist::B2,
                Twist::L3 => Twist::B3,
                Twist::R1 => Twist::F1,
                Twist::R2 => Twist::F2,
                Twist::R3 => Twist::F3,
                Twist::F1 => Twist::L1,
                Twist::F2 => Twist::L2,
                Twist::F3 => Twist::L3,
                Twist::B1 => Twist::R1,
                Twist::B2 => Twist::R2,
                Twist::B3 => Twist::R3,
            },
        }
    }
}

impl std::str::FromStr for Twist {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Use Debug representation to match against all variants
        for twist in ALL_TWISTS {
            if format!("{:?}", twist) == s {
                return Ok(twist);
            }
        }
        Err(format!("Unknown twist: '{}'", s))
    }
}

pub fn inverse(twists: &[Twist]) -> Vec<Twist> {
    twists.iter().rev().map(|t| t.inverse()).collect()
}

pub fn conjugate_by_inv(twists: &[Twist], rot: Axis) -> Vec<Twist> {
    twists.iter().map(|t| t.conjugate_by_inv(rot)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("L1".parse::<Twist>().unwrap(), Twist::L1);
        assert!("XX".parse::<Twist>().is_err());
    }

    #[test]
    fn test_inverse() {
        for twist in ALL_TWISTS {
            assert_eq!(twist.inverse().inverse(), twist);
        }

        let sequence = [Twist::L1, Twist::U2, Twist::F3]; // Arbitrary
        assert_eq!(inverse(&inverse(&sequence)), sequence);
    }

    #[test]
    fn test_conjugation() {
        for twist in ALL_TWISTS {
            for rot in [Axis::X, Axis::Y, Axis::Z] {
                let conjugated_twist = twist.conjugate_by_inv(rot).conjugate_by_inv(rot).conjugate_by_inv(rot).conjugate_by_inv(rot);
                assert_eq!(conjugated_twist, twist, "Failed for twist {:?} and rotation {:?}", twist, rot);
            }
        }
    }
}
