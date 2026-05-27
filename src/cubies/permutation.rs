use crate::math::factorial;
use std::ops::Mul;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Permutation<const LEN: usize> {
    map: [usize; LEN],
}

impl<const LEN: usize> Permutation<LEN> {
    pub const fn new(map: [usize; LEN]) -> Self {
        Self { map }
    }

    pub const fn identity() -> Self {
        let mut map = [0usize; LEN];
        let mut i = 0;
        while i < LEN {
            map[i] = i;
            i += 1;
        }
        Self { map }
    }

    pub const fn data(&self) -> [usize; LEN] {
        self.map
    }

    pub fn inverse(&self) -> Self {
        let mut inverse_map = [0usize; LEN];
        for i in 0..LEN {
            inverse_map[self.map[i]] = i;
        }
        Self { map: inverse_map }
    }

    // Lexicographic index of the permutation (0 to N!-1)
    pub fn index(&self) -> usize {
        let size = self.map.len();
        let mut index = 0;
        let mut bitboard = 0;

        for i in 0..size {
            let mask: usize = 1usize << self.map[i];

            // Number of remaining elements smaller than the current element
            let smaller = self.map[i] - (bitboard & (mask - 1)).count_ones() as usize;

            // Total number of elements bigger than the current element
            let bigger = size - i - 1;

            index += smaller * factorial(bigger);
            bitboard |= mask;
        }
        index
    }

    pub fn from_index(mut index: usize) -> Self {
        let mut unused = 0xFFFFFFFFFFFFFFFFusize;
        let mut permutation = [0usize; LEN];

        for i in (0..LEN).rev() {
            let f = factorial(i);
            let pos = index / f;
            index %= f;

            // Find the pos-th set bit in unused
            let mut mask = unused;
            for _ in 0..pos {
                mask &= mask - 1; // Clear lowest set bit
            }
            let selected_bit = mask & (!mask + 1); // Get lowest set bit

            permutation[LEN - 1 - i] = selected_bit.trailing_zeros() as usize;
            unused ^= selected_bit;
        }
        Self { map: permutation }
    }
}

impl<const LEN: usize> Mul for Permutation<LEN> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { map: std::array::from_fn(|i| rhs.map[self.map[i]]) }
    }
}

impl<T: Copy, const N: usize> Mul<[T; N]> for Permutation<N> {
    type Output = [T; N];

    fn mul(self, rhs: [T; N]) -> [T; N] {
        std::array::from_fn(|i| rhs[self.map[i]])
    }
}

impl<const LEN: usize> fmt::Display for Permutation<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

pub fn is_even_permutation(lexicographical_index: usize) -> bool {
    // Convert the index to its factoradic representation and sum the digits.
    let mut index = lexicographical_index;
    let mut sum = 0;
    let mut i = 2;
    while index > 0 {
        sum += index % i;
        index /= i;
        i += 1;
    }
    sum % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_identity() {
        let prm = Permutation::new([1, 0, 2]); // Arbitrary
        assert_eq!(prm, Permutation::identity() * prm);
        assert_eq!(prm, prm * Permutation::identity());
    }

    #[test]
    fn test_inverse() {
        let prm = Permutation::new([2, 0, 1]); // Arbitrary
        let inv = prm.inverse();
        assert_eq!(Permutation::identity(), prm * inv);
        assert_eq!(Permutation::identity(), inv * prm);
    }

    #[test]
    fn test_mul() {
        let prm = Permutation::new([2, 0, 1]); // Arbitrary
        let arr = [0, 1, 1]; // Arbitrary
        assert_eq!(prm * arr, [1, 0, 1]);
    }

    #[test]
    fn test_compose() {
        let prm1 = Permutation::new([0, 2, 1]); // Arbitrary
        let prm2 = Permutation::new([1, 2, 0]); // Arbitrary
        let expected = Permutation::new([1, 0, 2]);
        assert_eq!(prm1 * prm2, expected);
    }

    // Test index and from_index
    #[test]
    fn test_index() {
        let base: Vec<usize> = (0..8).collect();
        for (index, perm) in base.iter().permutations(8).enumerate() {
            let prm_arr: [usize; 8] = perm.into_iter().copied().collect::<Vec<_>>().try_into().unwrap();
            let prm = Permutation::new(prm_arr);
            assert_eq!(prm.index(), index);
            assert_eq!(Permutation::from_index(index), prm);
        }
    }

    fn is_even_permutation_vec(perm: Vec<usize>) -> bool {
        let mut visited = vec![false; perm.len()];
        let mut cycles = 0;
        for i in 0..perm.len() {
            if !visited[i] {
                cycles += 1;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = perm[j];
                }
            }
        }
        (perm.len() - cycles) % 2 == 0
    }

    #[test]
    fn test_is_even_permutation() {
        // Test some known cases
        assert!(is_even_permutation_vec(vec![0, 1, 2])); // identity is even
        assert!(!is_even_permutation_vec(vec![0, 2, 1])); // one swap is odd
        assert!(!is_even_permutation_vec(vec![1, 0, 2])); // one swap is odd
        assert!(is_even_permutation_vec(vec![2, 0, 1])); // two swaps is even

        // Test that both methods give the same result
        for size in 1..=6 {
            let base: Vec<usize> = (0..size).collect();
            for (index, perm) in base.iter().permutations(size).enumerate() {
                let perm_vec: Vec<usize> = perm.into_iter().copied().collect();
                let even_from_array = is_even_permutation_vec(perm_vec);
                let even_from_index = is_even_permutation(index);
                assert_eq!(even_from_array, even_from_index);
            }
        }
    }

    #[test]
    fn test_permutation_index_half_is_bijection() {
        // Tests that permutation_index/2 is a bijection,
        // between even permutations and [0, factorial(n)/2 );
        // and between odd permutations and [0, factorial(n)/2 ).
        let mut even_permutations = 0;
        let mut odd_permutations = 0;

        for i in 0..factorial(8) {
            let p = Permutation::<8>::from_index(i);
            let index = p.index();
            if is_even_permutation(index) {
                assert_eq!(index / 2, even_permutations);
                even_permutations += 1;
            } else {
                assert_eq!(index / 2, odd_permutations);
                odd_permutations += 1;
            }
        }
    }
}
