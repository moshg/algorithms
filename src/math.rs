use std::cmp;

pub fn digits10(mut n: i64) -> i64 {
    if n < 0 {
        n = -n;
    }

    let mut d = 1;
    let mut p = 10;
    while p <= n {
        p *= 10;
        d += 1;
    }
    d
}

pub fn permutation(mut n: i64, k: i64) -> i64 {
    assert!(k >= 0);
    assert!(n >= k);

    let mut p = 1;
    let end = n - k;
    while n > end {
        p *= n;
        n -= 1;
    }
    p
}

pub fn combination(n: i64, k: i64) -> i64 {
    assert!(k >= 0);
    assert!(n >= k);

    let k = cmp::min(k, n - k);
    let num = permutation(n, k);
    let mut den = 1;
    for i in 0..k {
        den = den * (i + 1);
    }
    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits10() {
        assert_eq!(digits10(1234), 4);
        assert_eq!(digits10(1000), 4);
        assert_eq!(digits10(999), 3);
    }

    #[test]
    fn test_permutation() {
        assert_eq!(permutation(4, 2), 12);
        assert_eq!(permutation(3, 0), 1);
        assert_eq!(permutation(5, 5), 120);
    }

    #[test]
    fn test_combination() {
        assert_eq!(combination(5, 2), 10);
        assert_eq!(combination(5, 3), 10);
        assert_eq!(combination(3, 0), 1);
        assert_eq!(combination(5, 5), 1);
    }
}
