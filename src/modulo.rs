/// Returns `x` + `y` mod `modulo`.
///
/// `x < modulo` and `y < modulo` must hold.
#[inline]
pub fn add(x: u32, y: u32, modulo: u32) -> u32 {
    debug_assert!(modulo > 0);
    debug_assert!(x < modulo && y < modulo);
    let sum = x as u64 + y as u64;
    if sum <= modulo as u64 {
        sum as u32
    } else {
        (sum as u32).wrapping_sub(modulo)
    }
}

/// Returns `x` - `y` mod `modulo`.
///
/// `x < modulo` and `y < modulo` must hold.
#[inline]
pub fn sub(x: u32, y: u32, modulo: u32) -> u32 {
    debug_assert!(0 < modulo);
    debug_assert!(x < modulo && y < modulo);
    if x >= y {
        x - y
    } else {
        modulo + x - y
    }
}

/// Returns `x` * `y` mod `modulo`.
#[inline]
pub fn mul(x: u32, y: u32, modulo: u32) -> u32 {
    ((x as u64 * y as u64) % modulo as u64) as u32
}

/// Returns `x`^ `y` mod `modulo`.
pub fn pow(x: u32, mut y: u32, modulo: u32) -> u32 {
    debug_assert!(0 < modulo);
    let mut p = x;
    let mut ret = 1;

    while y != 0 {
        if y & 1 == 1 {
            ret = mul(ret, p, modulo);
        }
        p = mul(p, p, modulo);
        y >>= 1;
    }
    ret
}

fn is_prime(x: u32) -> bool {
    let sqrt = (x as f32).sqrt() as u32;
    for factor in 2..sqrt {
        if x % factor == 0 {
            return false;
        }
    }
    true
}

/// Returns 1 / `x`.
///
/// `modulo` must be a prime number.
#[inline]
fn reciprocal(x: u32, modulo: u32) -> u32 {
    debug_assert!(0 < modulo && is_prime(modulo));
    pow(x, modulo - 2, modulo)
}

/// Returns `x` / `y` mod `modulo`.
///
/// `modulo` must be a prime number.
#[inline]
pub fn div(x: u32, y: u32, modulo: u32) -> u32 {
    debug_assert!(0 < modulo && is_prime(modulo));
    mul(x, reciprocal(y, modulo), modulo)
}

/// Returns `n`P`k` mod `modulo`.
pub fn permutation(n: u32, k: u32, modulo: u32) -> u32 {
    let mut p = 1;
    for i in 0..k {
        p = mul(p, n - i, modulo);
    }
    p
}

/// Returns `n`C`k` mod `modulo`.
pub fn combination(n: u32, k: u32, modulo: u32) -> u32 {
    use std::cmp;

    let k = cmp::min(k, n - k);
    let num = permutation(n, k, modulo);
    let mut den = 1;
    for i in 0..k {
        den = mul(den, i + 1, modulo);
    }
    div(num, den, modulo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let sum = add(1 << 31, 1 << 31, u32::max_value());
        assert_eq!(sum, 1);
    }

    #[test]
    fn test_sub() {
        let diff = sub(0, 1 << 31, u32::max_value());
        assert_eq!(diff, (1 << 31) - 1);
    }

    #[test]
    fn test_mul() {
        let prod = mul(1 << 31, 1 <<31, 2);
        assert_eq!(prod, 0);
    }

    #[test]
    fn test_pow() {
        let power = pow(3, 0, 2);
        assert_eq!(power, 1);

        let power = pow(u32::max_value() - 1, 0b101010, u32::max_value());
        assert_eq!(power, 1);
    }

    #[test]
    fn test_div() {
        let q = div(1, 5, 7);
        assert_eq!(q, 3);
    }

    #[test]
    fn test_permutation() {
        let p = permutation(8, 3, 5);
        assert_eq!(p, 1);
    }

    #[test]
    fn test_combination() {
        let c = combination(10, 3, 7);
        assert_eq!(c, 1);
        let c = combination(10, 7, 7);
        assert_eq!(c, 1);
    }
}
