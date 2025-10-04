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

pub fn inversed(t: Twist) -> Twist {
    match t {
        Twist::L1 => Twist::L3,
        Twist::L2 => Twist::L2,
        Twist::L3 => Twist::L1,
        Twist::R1 => Twist::R3,
        Twist::R2 => Twist::R2,
        Twist::R3 => Twist::R1,
        Twist::U1 => Twist::U3,
        Twist::U2 => Twist::U2,
        Twist::U3 => Twist::U1,
        Twist::D1 => Twist::D3,
        Twist::D2 => Twist::D2,
        Twist::D3 => Twist::D1,
        Twist::F1 => Twist::F3,
        Twist::F2 => Twist::F2,
        Twist::F3 => Twist::F1,
        Twist::B1 => Twist::B3,
        Twist::B2 => Twist::B2,
        Twist::B3 => Twist::B1,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Twists(u32);

impl std::str::FromStr for Twist {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Use Debug representation to match against all variants
        for twist in Twists::all().iter() {
            if format!("{:?}", twist) == s {
                return Ok(twist);
            }
        }
        Err(format!("Unknown twist: '{}'", s))
    }
}

/// Parse a string of space-separated twist names into a vector of Twist values.
pub fn parse_twists(input: &str) -> Result<Vec<Twist>, String> {
    input.split_whitespace().map(|s| s.parse()).collect()
}

impl Twists {
    pub fn from_bits(twists: u32) -> Self {
        Self(twists)
    }
    
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn all() -> Self {
        Self(0b111_111_111_111_111_111)
    }

    /// H0 = { L2, R2, U, D, F2, B2 }
    pub fn h0() -> Self {
        Self(0b010_010_111_111_010_010)
    }

    pub fn set(&mut self, t: Twist) {
        self.0 |= 1 << (t as u8);
    }

    pub fn unset(&mut self, t: Twist) {
        self.0 &= !(1 << (t as u8));
    }

    pub fn set_twists(&mut self, t: Twists) {
        self.0 |= t.0;
    }

    pub fn unset_twists(&mut self, t: Twists) {
        self.0 &= !t.0;
    }

    pub fn contains(&self, t: Twist) -> bool {
        self.0 & (1 << (t as u8)) != 0
    }

    pub fn size(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn iter(&self) -> impl Iterator<Item = Twist> {
        let mut bits = self.0;
        std::iter::from_fn(move || {
            if bits == 0 {
                None
            } else {
                let pos = bits.trailing_zeros() as u8;
                bits &= bits - 1; // Clear the lowest set bit
                Some(unsafe { std::mem::transmute(pos) })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inversed() {
        for twist in Twists::all().iter() {
            let inv = inversed(twist);
            let inv2 = inversed(inv);
            assert_eq!(twist, inv2, "Inversion failed for {:?}", twist);
        }
    }

    #[test]
    fn test_twists_h0() {
        let h0 = Twists::h0();
        assert!(h0.size() == 10);
        for twist in [
            Twist::L2,
            Twist::R2,
            Twist::U1,
            Twist::U2,
            Twist::U3,
            Twist::D1,
            Twist::D2,
            Twist::D3,
            Twist::F2,
            Twist::B2,
        ] {
            assert!(h0.contains(twist));
        }
    }

    #[test]
    fn test_state() {
        let mut twists = Twists::empty();
        assert!(twists.size() == 0);

        let twist = Twist::L3; // Arbitrary

        twists.set(twist);
        assert!(twists.size() == 1);
        assert!(twists.contains(twist));

        twists.unset(twist);
        assert!(twists.size() == 0);
        assert!(!twists.contains(twist));

        let multiple = Twists::from_bits(0b1010101); // Arbitrary
        twists.set_twists(multiple);
        assert!(twists.size() == 4);

        twists.unset_twists(multiple);
        assert!(twists.size() == 0);
    }

    #[test]
    fn test_iter() {
        assert_eq!(Twists::empty().iter().count(), 0);
        assert_eq!(Twists::all().iter().count(), 18);
    }

    #[test]
    fn test_twist_from_str() {
        // Test that FromStr works directly
        assert_eq!("L1".parse::<Twist>().unwrap(), Twist::L1);
        assert!("XX".parse::<Twist>().is_err());
    }

    #[test]
    fn test_parse_twists() {
        assert_eq!(parse_twists("").unwrap(), vec![]);
        assert_eq!(parse_twists("L1").unwrap(), vec![Twist::L1]);
        assert_eq!(parse_twists("L1 L2").unwrap(), vec![Twist::L1, Twist::L2]);
        assert!(parse_twists("XX").is_err());
    }
}
