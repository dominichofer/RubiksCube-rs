use crate::twist::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TwistSet(u32);

impl TwistSet {
    pub fn from_bits(twists: u32) -> Self {
        Self(twists)
    }

    pub fn bits(&self) -> u32 {
        self.0
    }
    
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn full() -> Self {
        Self(0b111_111_111_111_111_111)
    }

    pub const fn full_and_none() -> Self {
        Self(0b1_111_111_111_111_111_111)
    }

    /// H0 = { L2, R2, U, D, F2, B2 }
    pub const fn h0() -> Self {
        Self(0b010_010_111_111_010_010)
    }

    pub fn set_twist(&mut self, t: Twist) {
        self.0 |= 1 << (t as u8);
    }

    pub fn unset_twist(&mut self, t: Twist) {
        self.0 &= !(1 << (t as u8));
    }

    pub fn set_twists(&mut self, t: TwistSet) {
        self.0 |= t.0;
    }

    pub fn unset_twists(&mut self, t: TwistSet) {
        self.0 &= !t.0;
    }

    pub fn keep_only(&mut self, t: TwistSet) {
        self.0 &= t.0;
    }

    pub fn contains(&self, t: Twist) -> bool {
        self.0 & (1 << (t as u8)) != 0
    }

    pub const fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn nth(&self, n: usize) -> Twist {
        // TODO: Try to optimize this using bit manipulation tricks, e.g. using _pdep_u64 on x86 to directly get the nth set bit.
        // Twist::from(_pdep_u64(1 << n, self.0 as u64).trailing_zeros() as u8)
        let mut bits = self.0;
        for _ in 0..n {
            bits &= bits - 1; // Clear the lowest set bit
        }
        let pos = bits.trailing_zeros();
        Twist::from(pos as u8)
    }

    pub fn iter(&self) -> impl Iterator<Item = Twist> {
        (0..32).filter_map(|i| {
            if self.0 & (1 << i) != 0 {
                Some(Twist::from(i as u8))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h0() {
        let h0 = TwistSet::h0();
        assert!(h0.count() == H0_TWISTS.len());
        for twist in H0_TWISTS {
            assert!(h0.contains(twist));
        }
    }

    #[test]
    fn test_state() {
        let mut twists = TwistSet::empty();
        assert!(twists.count() == 0);

        let twist = Twist::L3; // Arbitrary

        twists.set_twist(twist);
        assert!(twists.count() == 1);
        assert!(twists.contains(twist));

        twists.unset_twist(twist);
        assert!(twists.count() == 0);
        assert!(!twists.contains(twist));

        let multiple = TwistSet::from_bits(0b1010101); // Arbitrary
        twists.set_twists(multiple);
        assert!(twists.count() == 4);

        twists.unset_twists(multiple);
        assert!(twists.count() == 0);
    }

    #[test]
    fn test_iter() {
        assert_eq!(TwistSet::empty().iter().count(), 0);
        assert_eq!(TwistSet::full().iter().collect::<Vec<_>>(), ALL_TWISTS);
    }
}
