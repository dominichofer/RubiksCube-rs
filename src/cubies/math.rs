pub fn encode(data: &[usize], base: usize) -> usize {
    let mut result = 0;
    for &value in data {
        result = result * base + value;
    }
    result
}

pub fn decode(mut value: usize, base: usize, length: usize) -> Vec<usize> {
    let mut result = vec![0; length];
    for i in (0..length).rev() {
        result[i] = value % base;
        value /= base;
    }
    result
}

pub const fn factorial(n: usize) -> usize {
    const PRECOMPUTED: [usize; 21] = [
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

    PRECOMPUTED[n]
}

pub const fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if n < 13 {
        const PASCAL: [[usize; 13]; 13] = [
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
        return PASCAL[n][k];
    }

    let k = if k > n - k { n - k } else { k }; // Take advantage of symmetry
    let mut result = 1;
    let mut i = 0;
    while i < k {
        result = result * (n - i) / (i + 1);
        i += 1;
    }
    result
}

// Returns the index of the combination
// in the lexicographically sorted list of all possible
// combinations of n elements taken k at a time.
pub const fn combination_index(n: usize, combination: &[usize]) -> usize {
    let mut index = 0;
    let mut j = 0;
    let k = combination.len();
    let mut i = 0;
    while i < k {
        j += 1;
        while j < combination[i] + 1 {
            index += binomial(n - j, k - i - 1);
            j += 1;
        }
        i += 1;
    }
    index
}

pub fn nth_combination(n: usize, k: usize, index: usize) -> Vec<usize> {
    if k < 1 || k > n {
        return vec![];
    }

    let mut size = 0;
    let mut index = index;
    let mut combination = vec![0usize; k as usize];

    for i in 0..n {
        let count = binomial(n - 1 - i, k - size - 1);
        if count > index {
            combination[size] = i;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        for i in 0..=20 {
            assert_eq!(factorial(i), (1..=i).product::<usize>());
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
        assert_eq!(combination_index(1, &[0usize]), 0);

        assert_eq!(combination_index(2, &[0usize]), 0);
        assert_eq!(combination_index(2, &[1usize]), 1);
        assert_eq!(combination_index(2, &[0usize, 1usize]), 0);

        assert_eq!(combination_index(5, &[0usize, 1usize]), 0);
        assert_eq!(combination_index(5, &[0usize, 2usize]), 1);
        assert_eq!(combination_index(5, &[0usize, 3usize]), 2);
        assert_eq!(combination_index(5, &[0usize, 4usize]), 3);
        assert_eq!(combination_index(5, &[1usize, 2usize]), 4);
        assert_eq!(combination_index(5, &[1usize, 3usize]), 5);
        assert_eq!(combination_index(5, &[1usize, 4usize]), 6);
        assert_eq!(combination_index(5, &[2usize, 3usize]), 7);
        assert_eq!(combination_index(5, &[2usize, 4usize]), 8);
        assert_eq!(combination_index(5, &[3usize, 4usize]), 9);
        assert_eq!(combination_index(5, &[0usize, 1usize, 2usize]), 0);
        assert_eq!(combination_index(5, &[0usize, 1usize, 3usize]), 1);
        assert_eq!(combination_index(5, &[0usize, 1usize, 4usize]), 2);
        assert_eq!(combination_index(5, &[0usize, 2usize, 3usize]), 3);
        assert_eq!(combination_index(5, &[0usize, 2usize, 4usize]), 4);
        assert_eq!(combination_index(5, &[0usize, 3usize, 4usize]), 5);
        assert_eq!(combination_index(5, &[1usize, 2usize, 3usize]), 6);
        assert_eq!(combination_index(5, &[1usize, 2usize, 4usize]), 7);
        assert_eq!(combination_index(5, &[1usize, 3usize, 4usize]), 8);
        assert_eq!(combination_index(5, &[2usize, 3usize, 4usize]), 9);
        assert_eq!(combination_index(5, &[0usize, 1usize, 2usize, 3usize]), 0);
        assert_eq!(combination_index(5, &[0usize, 1usize, 2usize, 4usize]), 1);
        assert_eq!(combination_index(5, &[0usize, 1usize, 3usize, 4usize]), 2);
        assert_eq!(combination_index(5, &[0usize, 2usize, 3usize, 4usize]), 3);
        assert_eq!(combination_index(5, &[1usize, 2usize, 3usize, 4usize]), 4);
        assert_eq!(combination_index(5, &[0usize, 1usize, 2usize, 3usize, 4usize]), 0);
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
}
