/// Face twist, where the number indicates how many quarter turns to perform.
///      +---------+
///     /    ←B   /|
///    /   F→    / |
///   +---------+  |
///   | L  ←U   |  |
///   | ↓     ↑ |  +
///   |   D→  R | /
///   +---------+
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Twist {
    L1, L2, L3,
    R1, R2, R3,
    U1, U2, U3,
    D1, D2, D3,
    F1, F2, F3,
    B1, B2, B3,
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

impl Twist {
    pub fn from(value: u8) -> Self {
        match value {
            0 => Twist::L1,
            1 => Twist::L2,
            2 => Twist::L3,
            3 => Twist::R1,
            4 => Twist::R2,
            5 => Twist::R3,
            6 => Twist::U1,
            7 => Twist::U2,
            8 => Twist::U3,
            9 => Twist::D1,
            10 => Twist::D2,
            11 => Twist::D3,
            12 => Twist::F1,
            13 => Twist::F2,
            14 => Twist::F3,
            15 => Twist::B1,
            16 => Twist::B2,
            17 => Twist::B3,
            _ => panic!("Invalid twist value: {}", value),
        }
    }

    //TODO: Use this!
    pub fn to_index(&self) -> u8 {
        *self as u8
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("L1".parse::<Twist>().unwrap(), Twist::L1);
        assert!("XX".parse::<Twist>().is_err());
    }
}
