use super::permutation::*;
use std::ops::{Add, Mul};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Orientation<const LEN: usize, const BASE: usize> {
    values: [usize; LEN],
}

impl<const LEN: usize, const BASE: usize> Orientation<LEN, BASE> {
    pub const fn new(values: [usize; LEN]) -> Self {
        Self { values }
    }

    pub const fn identity() -> Self {
        Self { values: [0; LEN] }
    }

    pub fn inverse(&self) -> Self {
        Self { values: self.values.map(|v| (BASE - v) % BASE) }
    }

    pub fn data(&self) -> [usize; LEN] {
        self.values
    }
}

impl<const LEN: usize, const BASE: usize> Add for Orientation<LEN, BASE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { values: std::array::from_fn(|i| (self.values[i] + rhs.values[i]) % BASE) }
    }
}

impl<const LEN: usize, const BASE: usize> Add<[usize; LEN]> for Orientation<LEN, BASE> {
    type Output = Self;

    fn add(self, rhs: [usize; LEN]) -> Self {
        self + Self { values: rhs }
    }
}

impl<const LEN: usize, const BASE: usize> Mul<Orientation<LEN, BASE>> for Permutation<LEN> {
    type Output = Orientation<LEN, BASE>;

    fn mul(self, rhs: Orientation<LEN, BASE>) -> Orientation<LEN, BASE> {
        Orientation { values: self * rhs.values }
    }
}

impl<const LEN: usize, const BASE: usize> fmt::Display for Orientation<LEN, BASE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.values)
    }
}
