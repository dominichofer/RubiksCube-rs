use crate::twist::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TwistSet(u32);

impl TwistSet {
    pub const EMPTY: Self = Self(0b000_000_000_000_000_000);
    pub const FULL: Self = Self(0b111_111_111_111_111_111);
    pub const H0: Self = Self(0b010_010_111_111_010_010); // H0 = { L2, R2, U, D, F2, B2 }

    pub fn bits(&self) -> u32 {
        self.0
    }

    pub fn set_twist(&mut self, t: Twist) {
        self.0 |= 1 << t.to_index();
    }

    pub fn set_twists(&mut self, t: TwistSet) {
        self.0 |= t.0;
    }

    pub fn unset_twist(&mut self, t: Twist) {
        self.0 &= !(1 << t.to_index());
    }

    pub fn unset_twists(&mut self, t: TwistSet) {
        self.0 &= !t.0;
    }

    pub fn keep_only(&mut self, t: TwistSet) {
        self.0 &= t.0;
    }

    pub fn contains(&self, t: Twist) -> bool {
        self.0 & (1 << t.to_index()) != 0
    }

    pub const fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Twist> {
        (0..18u8).filter_map(|i| {
            if self.0 & (1 << i) != 0 {
                Some(Twist::from(i))
            } else {
                None
            }
        })
    }
}

impl From<u32> for TwistSet {
    fn from(bits: u32) -> Self {
        TwistSet(bits)
    }
}

impl From<Twist> for TwistSet {
    fn from(twist: Twist) -> Self {
        TwistSet(1 << twist.to_index())
    }
}

impl From<&[Twist]> for TwistSet {
    fn from(twists: &[Twist]) -> Self {
        let mut set = TwistSet::EMPTY;
        for &twist in twists {
            set.set_twist(twist);
        }
        set
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

        twists.set_twist(twist);
        assert!(twists.count() == 1);
        assert!(twists.contains(twist));

        twists.unset_twist(twist);
        assert!(twists.count() == 0);
        assert!(!twists.contains(twist));

        let multiple = TwistSet::from(0b1010101); // Arbitrary
        twists.set_twists(multiple);
        assert!(twists.count() == 4);

        twists.unset_twists(multiple);
        assert!(twists.count() == 0);
    }

    #[test]
    fn test_iter() {
        assert_eq!(TwistSet::EMPTY.iter().count(), 0);
        assert_eq!(TwistSet::FULL.iter().collect::<Vec<_>>(), ALL_TWISTS);
    }
}
