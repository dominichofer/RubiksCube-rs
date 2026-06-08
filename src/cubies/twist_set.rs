use crate::twist::*;

pub struct TwistBitsIter {
    bits: u32,
}

impl Iterator for TwistBitsIter {
    type Item = Twist;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }

        let index = self.bits.trailing_zeros() as i32;
        self.bits &= self.bits - 1;  // Clear the least significant set bit
        Some(Twist::from(index))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TwistSet {
    bits: u32, // Each bit represents whether a twist is included in the set
}

impl TwistSet {
    pub const EMPTY: Self = Self{ bits: 0 };
    pub const FULL: Self = Self::from_twists(&ALL_TWISTS);
    pub const H0: Self = Self::from_twists(&H0_TWISTS);

    pub const fn new(bits: u32) -> Self {
        Self { bits: bits & Self::FULL.bits } // Ensure we only keep valid bits
    }

    pub const fn from_twists(twists: &[Twist]) -> Self {
        let mut ret = Self::EMPTY;
        let mut i = 0;
        while i < twists.len() {
            ret.bits |= 1 << twists[i] as u32;
            i += 1;
        }
        ret
    }

    pub fn as_u64(&self) -> u64 {
        self.bits as u64
    }

    pub fn contains(&self, t: Twist) -> bool {
        self.bits & (1 << t as u32) != 0
    }

    pub fn count(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn iter(&self) -> TwistBitsIter {
        TwistBitsIter { bits: self.bits }
    }
}

impl std::ops::Not for TwistSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.bits & Self::FULL.bits) // Invert bits and mask with FULL to keep only valid bits
    }
}

impl std::ops::BitOrAssign for TwistSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl std::ops::BitAndAssign for TwistSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl std::ops::BitOrAssign<Twist> for TwistSet {
    fn bitor_assign(&mut self, rhs: Twist) {
        self.bits |= rhs as u32;
    }
}

impl std::ops::BitAndAssign<Twist> for TwistSet {
    fn bitand_assign(&mut self, rhs: Twist) {
        self.bits &= !(1 << rhs as u32);
    }
}

#[inline(always)]
pub fn unique_twists_after(twist: Twist) -> TwistSet {
    match twist {
        Twist::L1 | Twist::L2 | Twist::L3 => TwistSet::new(0b111_111_111_111_111_000),
        Twist::R1 | Twist::R2 | Twist::R3 => TwistSet::new(0b111_111_111_111_000_000),
        Twist::U1 | Twist::U2 | Twist::U3 => TwistSet::new(0b111_111_111_000_111_111),
        Twist::D1 | Twist::D2 | Twist::D3 => TwistSet::new(0b111_111_000_000_111_111),
        Twist::F1 | Twist::F2 | Twist::F3 => TwistSet::new(0b111_000_111_111_111_111),
        Twist::B1 | Twist::B2 | Twist::B3 => TwistSet::new(0b000_000_111_111_111_111),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut twists = TwistSet::EMPTY;
        assert!(twists.count() == 0);

        let twist = Twist::L3; // Arbitrary

        twists |= twist;
        assert!(twists.count() == 1);
        assert!(twists.contains(twist));


        let multiple = TwistSet::new(0b1010101); // Arbitrary
        twists |= multiple;
        assert!(twists.count() == 4);

        twists &= !multiple;
        assert!(twists.count() == 0);
    }

    #[test]
    fn test_iter() {
        assert_eq!(TwistSet::EMPTY.iter().count(), 0);
        assert_eq!(TwistSet::FULL.iter().collect::<Vec<_>>(), ALL_TWISTS);
    }
}
