pub const fn factorial(n: i64) -> i64 {
    const PRECOMPUTED: [i64; 21] = [
        1,
        1,
        2,
        6,
        24,
        120,
        720,
        5040,
        40320,
        362880,
        3628800,
        39916800,
        479001600,
        6227020800,
        87178291200,
        1307674368000,
        20922789888000,
        355687428096000,
        6402373705728000,
        121645100408832000,
        2432902008176640000,
    ];

    PRECOMPUTED[n as usize]
}

pub fn binomial(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    if n < 13 {
        const PASCAL: [[i16; 13]; 13] = [
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 3, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 4, 6, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 5, 10, 10, 5, 1, 0, 0, 0, 0, 0, 0, 0],
            [1, 6, 15, 20, 15, 6, 1, 0, 0, 0, 0, 0, 0],
            [1, 7, 21, 35, 35, 21, 7, 1, 0, 0, 0, 0, 0],
            [1, 8, 28, 56, 70, 56, 28, 8, 1, 0, 0, 0, 0],
            [1, 9, 36, 84, 126, 126, 84, 36, 9, 1, 0, 0, 0],
            [1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1, 0, 0],
            [1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1, 0],
            [1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1],
        ];
        return PASCAL[n as usize][k as usize] as i64;
    }

    let k = k.min(n - k); // Take advantage of symmetry
    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

// Returns the index of the combination
// in the lexicographically sorted list of all possible
// combinations of n elements taken k at a time.
pub fn combination_index<T>(n: i64, combination: &[T]) -> i64
where
    T: Copy + Into<i64>,
{
    let mut index = 0;
    let mut j = 0;
    let k = combination.len() as i64;

    for i in 0..k {
        j += 1;
        while j < combination[i as usize].into() + 1 {
            index += binomial(n - j, k - i - 1);
            j += 1;
        }
    }
    index
}

pub fn nth_combination(n: i64, k: i64, index: i64) -> Vec<u8>{
    if k < 1 || k > n {
        return vec![];
    }

    let mut size = 0;
    let mut index = index;
    let mut combination = vec![0u8; k as usize];

    for i in 0..n {
        let count = binomial(n - 1 - i, k - size - 1);
        if count > index {
            combination[size as usize] = i as u8;
            size += 1;
            if size == k {
                break;
            }
        } else {
            index -= count;
        }
    }
    combination
}

pub fn nth_permutation(index: i64, size: usize) -> Vec<u8> {
    let mut unused = 0xFFFFFFFFFFFFFFFFusize;
    let mut index = index;
    let mut permutation = vec![0u8; size];

    for i in (0..size).rev() {
        let f = factorial(i as i64);
        let pos = index / f;
        index %= f;

        // Find the pos-th set bit in unused
        let mut mask = unused;
        for _ in 0..pos {
            mask &= mask - 1; // Clear lowest set bit
        }
        let selected_bit = mask & (!mask + 1); // Get lowest set bit

        permutation[size - 1 - i] = selected_bit.trailing_zeros() as u8;
        unused ^= selected_bit;
    }
    permutation
}

pub fn permutation_index(permutation: &[u8]) -> i64 {
    let size = permutation.len();
    let mut index = 0;
    let mut bitboard = 0;

    for i in 0..size {
        let mask: u64 = 1u64 << permutation[i];

        // Number of remaining elements smaller than the current element
        let smaller = permutation[i] as i64 - (bitboard & (mask - 1)).count_ones() as i64;

        // Total number of elements bigger than the current element
        let bigger = size - i - 1;

        index += smaller * factorial(bigger as i64);
        bitboard |= mask;
    }
    index
}

pub fn is_even_permutation_array<T>(permutation: &[T]) -> bool
where
    T: PartialOrd,
{
    let size = permutation.len();
    let mut count = 0u32;

    for i in 0..size {
        for j in (i + 1)..size {
            if permutation[i] > permutation[j] {
                count += 1;
            }
        }
    }
    count % 2 == 0
}

pub fn is_even_permutation(lexicographical_index: i64) -> bool {
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

pub fn is_odd_permutation(lexicographical_index: i64) -> bool {
    !is_even_permutation(lexicographical_index)
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_factorial() {
        for i in 0..=20 {
            assert_eq!(factorial(i), (1..=i).product::<i64>());
        }
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(10, 3), 120);
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(6, 0), 1);
        assert_eq!(binomial(6, 6), 1);
        assert_eq!(binomial(6, 7), 0);
        assert_eq!(binomial(12, 6), 924);
        assert_eq!(binomial(20, 10), 184756);
    }

    #[test]
    fn test_combination_index() {
        assert_eq!(combination_index(1, &[0]), 0);

        assert_eq!(combination_index(2, &[0]), 0);
        assert_eq!(combination_index(2, &[1]), 1);
        assert_eq!(combination_index(2, &[0, 1]), 0);

        assert_eq!(combination_index(5, &[0, 1]), 0);
        assert_eq!(combination_index(5, &[0, 2]), 1);
        assert_eq!(combination_index(5, &[0, 3]), 2);
        assert_eq!(combination_index(5, &[0, 4]), 3);
        assert_eq!(combination_index(5, &[1, 2]), 4);
        assert_eq!(combination_index(5, &[1, 3]), 5);
        assert_eq!(combination_index(5, &[1, 4]), 6);
        assert_eq!(combination_index(5, &[2, 3]), 7);
        assert_eq!(combination_index(5, &[2, 4]), 8);
        assert_eq!(combination_index(5, &[3, 4]), 9);
        assert_eq!(combination_index(5, &[0, 1, 2]), 0);
        assert_eq!(combination_index(5, &[0, 1, 3]), 1);
        assert_eq!(combination_index(5, &[0, 1, 4]), 2);
        assert_eq!(combination_index(5, &[0, 2, 3]), 3);
        assert_eq!(combination_index(5, &[0, 2, 4]), 4);
        assert_eq!(combination_index(5, &[0, 3, 4]), 5);
        assert_eq!(combination_index(5, &[1, 2, 3]), 6);
        assert_eq!(combination_index(5, &[1, 2, 4]), 7);
        assert_eq!(combination_index(5, &[1, 3, 4]), 8);
        assert_eq!(combination_index(5, &[2, 3, 4]), 9);
        assert_eq!(combination_index(5, &[0, 1, 2, 3]), 0);
        assert_eq!(combination_index(5, &[0, 1, 2, 4]), 1);
        assert_eq!(combination_index(5, &[0, 1, 3, 4]), 2);
        assert_eq!(combination_index(5, &[0, 2, 3, 4]), 3);
        assert_eq!(combination_index(5, &[1, 2, 3, 4]), 4);
        assert_eq!(combination_index(5, &[0, 1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_nth_combination() {
        for n in 1..=10 {
            for k in 1..=n {
                let size = binomial(n, k);
                for index in 0..size {
                    let comb = nth_combination(n, k, index);
                    assert_eq!(combination_index(n, &comb), index);
                }
            }
        }
    }

    #[test]
    fn test_nth_permutation() {
        // Test against itertools reference implementation
        for size in 1..=8 {
            let base: Vec<usize> = (0..size).collect();
            
            for (index, expected_perm) in base.iter().permutations(size).enumerate() {
                let computed_perm = nth_permutation(index as i64, size);
                let expected_u8: Vec<u8> = expected_perm.into_iter().map(|&x| x as u8).collect();
                assert_eq!(computed_perm, expected_u8);
            }
        }
    }

    #[test]
    fn test_permutation_index() {
        for size in 1..=8 {
            let base: Vec<usize> = (0..size).collect();
            for (index, perm) in base.iter().permutations(size).enumerate() {
                let perm_vec: Vec<u8> = perm.into_iter().map(|&x| x as u8).collect();
                assert_eq!(permutation_index(&perm_vec), index as i64);
            }
        }
    }

    #[test]
    fn test_is_even_permutation() {
        // Test that both methods give the same result
        for size in 1..=6 {
            let base: Vec<usize> = (0..size).collect();
            for (index, perm) in base.iter().permutations(size).enumerate() {
                let perm_vec: Vec<usize> = perm.into_iter().copied().collect();
                let even_from_array = is_even_permutation_array(&perm_vec);
                let even_from_index = is_even_permutation(index as i64);
                assert_eq!(even_from_array, even_from_index);
            }
        }

        // Test specific known cases
        assert!(is_even_permutation_array(&[0, 1, 2])); // identity is even
        assert!(!is_even_permutation_array(&[0, 2, 1])); // one swap is odd
        assert!(!is_even_permutation_array(&[1, 0, 2])); // one swap is odd  
        assert!(is_even_permutation_array(&[2, 0, 1])); // two swaps is even
    }

    #[test]
    fn test_permutation_index_half_is_bijection() {
        // Tests that permutation_index/2 is a bijection,
        // between even permutations and [0, factorial(n)/2);
        // and between odd permutations and [0, factorial(n)/2).
        for size in 0..=10 {
            let mut even_permutations = 0;
            let mut odd_permutations = 0;
            
            for i in 0..factorial(size as i64) {
                let p = nth_permutation(i, size);
                if is_even_permutation_array(&p) {
                    assert_eq!(permutation_index(&p) / 2, even_permutations);
                    even_permutations += 1;
                } else {
                    assert_eq!(permutation_index(&p) / 2, odd_permutations);
                    odd_permutations += 1;
                }
            }
        }
    }
}