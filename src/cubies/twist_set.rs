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

        let index = self.bits.trailing_zeros();
        self.bits &= self.bits - 1;  // Clear the least significant set bit
        Some(Twist::try_from(index).unwrap())
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
        Self::new(!self.bits)
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
        self.bits |= 1 << (rhs as u32);
    }
}

impl std::ops::BitAndAssign<Twist> for TwistSet {
    fn bitand_assign(&mut self, rhs: Twist) {
        self.bits &= 1 << (rhs as u32);
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
    fn test_new() {
        let all_bits_set = TwistSet::new(u32::MAX);
        assert_eq!(all_bits_set, TwistSet::FULL);
    }

    #[test]
    fn test_from_twists() {
        let set = TwistSet::from_twists(&[Twist::L1, Twist::R2, Twist::F3]);
        assert!(set.contains(Twist::L1));
        assert!(set.contains(Twist::R2));
        assert!(set.contains(Twist::F3));
        assert!(!set.contains(Twist::U1));
        assert_eq!(set.count(), 3);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_as_u64() {
        let set = TwistSet::from_twists(&[Twist::D1, Twist::B2]);
        let bits = set.as_u64() as u32;
        assert_eq!(TwistSet::new(bits), set);
    }

    #[test]
    fn test_iter() {
        let set = TwistSet::from_twists(&[Twist::D1, Twist::B2]);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![Twist::D1, Twist::B2]);
        assert_eq!(TwistSet::EMPTY.iter().count(), 0);
        assert_eq!(TwistSet::FULL.iter().collect::<Vec<_>>(), ALL_TWISTS);
        assert_eq!(TwistSet::H0.iter().collect::<Vec<_>>(), H0_TWISTS);
    }

    #[test]
    fn test_not_operator() {
        let set = TwistSet::from_twists(&[Twist::L1, Twist::R1]);
        let inv = !set;
        assert!(!inv.contains(Twist::L1));
        assert!(!inv.contains(Twist::R1));
        assert_eq!(inv.count() + set.count(), TwistSet::FULL.count());
    }

    #[test]
    fn test_bitor_assign_twistset() {
        let mut a = TwistSet::from_twists(&[Twist::L1]);
        let b = TwistSet::from_twists(&[Twist::R1, Twist::U2]);
        a |= b;
        assert_eq!(a, TwistSet::from_twists(&[Twist::L1, Twist::R1, Twist::U2]));
    }

    #[test]
    fn test_bitand_assign_twistset() {
        let mut a = TwistSet::from_twists(&[Twist::L1, Twist::R1]);
        let b = TwistSet::from_twists(&[Twist::R1, Twist::F3]);
        a &= b;
        assert_eq!(a, TwistSet::from_twists(&[Twist::R1]));
    }

    #[test]
    fn test_bitor_assign_twist() {
        let mut set = TwistSet::EMPTY;
        set |= Twist::B3;
        assert_eq!(set, TwistSet::from_twists(&[Twist::B3]));
    }

    #[test]
    fn test_bitand_assign_twist() {
        let mut set = TwistSet::from_twists(&[Twist::L2, Twist::D3]);
        set &= Twist::D3;
        assert_eq!(set, TwistSet::from_twists(&[Twist::D3]));
    }
}
