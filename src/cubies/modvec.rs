use super::permutation::*;
use std::ops::{Add, Mul, RangeTo, Index};

/// A vector (in the mathematical sense) of integers modulo a divisor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModVec<const LEN: usize, const DIVISOR: usize> {
    values: [usize; LEN],
}

impl<const LEN: usize, const DIVISOR: usize> ModVec<LEN, DIVISOR> {
    pub const fn new(values: [usize; LEN]) -> Self {
        Self { values }
    }

    pub const fn identity() -> Self {
        Self { values: [0; LEN] }
    }

    pub fn inverse(&self) -> Self {
        Self { values: self.values.map(|v| (DIVISOR - v) % DIVISOR) }
    }
}

/// ModVec[..index]
impl<const LEN: usize, const DIVISOR: usize> Index<RangeTo<usize>> for ModVec<LEN, DIVISOR> {
    type Output = [usize];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.values[index]
    }
}

/// ModVec + ModVec
impl<const LEN: usize, const DIVISOR: usize> Add for ModVec<LEN, DIVISOR> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { values: std::array::from_fn(|i| (self.values[i] + rhs.values[i]) % DIVISOR) }
    }
}

/// Permutation * ModVec
impl<const LEN: usize, const DIVISOR: usize> Mul<ModVec<LEN, DIVISOR>> for Permutation<LEN> {
    type Output = ModVec<LEN, DIVISOR>;

    fn mul(self, rhs: ModVec<LEN, DIVISOR>) -> ModVec<LEN, DIVISOR> {
        ModVec { values: self * rhs.values }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let a = ModVec::<4, 3>::new([0, 1, 2, 0]);
        let b = a.inverse();
        assert_eq!(b, ModVec::<4, 3>::new([0, 2, 1, 0]));
    }

    #[test]
    fn test_range_to_index() {
        let a = ModVec::<4, 3>::new([0, 1, 2, 0]);
        let slice = &a[..2];
        assert_eq!(slice, &[0, 1]);
    }

    #[test]
    fn test_add() {
        let a = ModVec::<4, 3>::new([0, 1, 2, 0]);
        let b = ModVec::<4, 3>::new([1, 2, 0, 1]);
        let c = a + b;
        assert_eq!(c, ModVec::<4, 3>::new([1, 0, 2, 1]));
    }

    #[test]
    fn test_modvec_permutation_mul() {
        let p = Permutation::<4>::new([2, 0, 3, 1]);
        let v = ModVec::<4, 3>::new([0, 1, 2, 0]);
        let result = p * v;
        assert_eq!(result, ModVec::<4, 3>::new([2, 0, 0, 1]));
    }
}
