use super::twist::Twist;

///      +---------+
///     /         /|
///    /   F→    / |
///   +---------+  |
///   |  L      |  |
///   |  ↓  ←U  |  +
///   |         | /
///   +---------+
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    L,
    U,
    F,
}

/// Simplifies the sequence (rot, twists, rot^-1) to an equivalent twist.
pub fn simplify_rot_twist(rot: Rotation, twist: Twist) -> Twist {
    match rot {
        Rotation::L => match twist {
            Twist::L1 => Twist::L1,
            Twist::L2 => Twist::L2,
            Twist::L3 => Twist::L3,
            Twist::R1 => Twist::R1,
            Twist::R2 => Twist::R2,
            Twist::R3 => Twist::R3,
            Twist::U1 => Twist::F1,
            Twist::U2 => Twist::F2,
            Twist::U3 => Twist::F3,
            Twist::D1 => Twist::B1,
            Twist::D2 => Twist::B2,
            Twist::D3 => Twist::B3,
            Twist::F1 => Twist::D1,
            Twist::F2 => Twist::D2,
            Twist::F3 => Twist::D3,
            Twist::B1 => Twist::U1,
            Twist::B2 => Twist::U2,
            Twist::B3 => Twist::U3,
        },
        Rotation::U => match twist {
            Twist::L1 => Twist::B1,
            Twist::L2 => Twist::B2,
            Twist::L3 => Twist::B3,
            Twist::R1 => Twist::F1,
            Twist::R2 => Twist::F2,
            Twist::R3 => Twist::F3,
            Twist::U1 => Twist::U1,
            Twist::U2 => Twist::U2,
            Twist::U3 => Twist::U3,
            Twist::D1 => Twist::D1,
            Twist::D2 => Twist::D2,
            Twist::D3 => Twist::D3,
            Twist::F1 => Twist::L1,
            Twist::F2 => Twist::L2,
            Twist::F3 => Twist::L3,
            Twist::B1 => Twist::R1,
            Twist::B2 => Twist::R2,
            Twist::B3 => Twist::R3,
        },
        Rotation::F => match twist {
            Twist::L1 => Twist::U1,
            Twist::L2 => Twist::U2,
            Twist::L3 => Twist::U3,
            Twist::R1 => Twist::D1,
            Twist::R2 => Twist::D2,
            Twist::R3 => Twist::D3,
            Twist::U1 => Twist::R1,
            Twist::U2 => Twist::R2,
            Twist::U3 => Twist::R3,
            Twist::D1 => Twist::L1,
            Twist::D2 => Twist::L2,
            Twist::D3 => Twist::L3,
            Twist::F1 => Twist::F1,
            Twist::F2 => Twist::F2,
            Twist::F3 => Twist::F3,
            Twist::B1 => Twist::B1,
            Twist::B2 => Twist::B2,
            Twist::B3 => Twist::B3,
        },
    }
}

/// Simplifies the sequence (rot, twists, rot^-1) to an equivalent sequence of twists.
pub fn simplify_rot_twists(rot: Rotation, twists: &[Twist]) -> Vec<Twist> {
    twists.iter().map(|&t| simplify_rot_twist(rot, t)).collect()
}

/// Simplifies the sequence (rots, twists, rots^-1) to an equivalent sequence of twists.
pub fn simplify_rots_twists(rots: &[Rotation], twists: &[Twist]) -> Vec<Twist> {
    rots.iter()
        .rev()
        .fold(twists.to_vec(), |ts, &rot| simplify_rot_twists(rot, &ts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_rot_twist() {
        assert_eq!(simplify_rot_twist(Rotation::L, Twist::U1), Twist::F1);
    }
}
