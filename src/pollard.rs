use crate::modexp;

/// Performs modular multiplication of `a` and `b` under modulus `modulus`.
///
/// # Arguments
///
/// * `a` - The first operand.
/// * `b` - The second operand.
/// * `modulus` - The modulus.
///
/// # Returns
///
/// The result of `(a * b) % modulus`.
pub fn modmul(a: u64, b: u64, modulus: u64) -> u64 {
    let mut ret: u64 = a % modulus;
    ret = (ret * (b % modulus)) % modulus;
    return ret;
}

/// Checks if `n` is a prime number.
///
/// # Arguments
///
/// * `n` - The number to check.
///
/// # Returns
///
/// `true` if `n` is prime, `false` otherwise.
pub fn is_prime(n: u64) -> bool {
    if n < 2 || n % 6 % 4 != 1 {
        return (n | 3) == 3;
    }
    let A: Vec<u64> = vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    let s: u64 = (n - 1).trailing_zeros() as u64;
    let d: u64 = n >> s;
    for &a in &A {
        let mut p = modexp::mod_exp(a % n, d, n);
        let mut i = s;
        while p != 1 && p != n - 1 && a % n != 0 && i != 0 {
            p = modmul(p, p, n);
            i -= 1;
        }
        if p != n - 1 && i != s {
            return false;
        }
    }
    return true;
}

/// Pollard's rho algorithm for integer factorization.
///
/// # Arguments
///
/// * `n` - The number to factorize.
///
/// # Returns
///
/// A non-trivial factor of `n`.
pub fn pollard(n: u64) -> u64 {
    let f = |x| modmul(x, x, n) + 1;
    let mut x = 0;
    let mut y = 0;
    let mut t = 30;
    let mut prd = 2;
    let mut i = 1;
    while t % 40 != 0 || modexp::gcd(prd, n) == 1 {
        if x == y {
            x = i;
            i += 1;
            y = f(x);
        }
        let q = modmul(prd, (x.max(y) - x.min(y)) % n, n);
        if q != 0 {
            prd = q;
        }
        x = f(x);
        y = f(f(y));
        t += 1;
    }
    return modexp::gcd(prd, n);
}

/// Factorizes `n` into its prime factors.
///
/// # Arguments
///
/// * `n` - The number to factorize.
///
/// # Returns
///
/// A vector containing the prime factors of `n` in sorted order.
pub fn factor(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }
    if is_prime(n) {
        return vec![n];
    }
    let x = pollard(n);
    let mut l = factor(x);
    let mut r = factor(n / x);
    l.append(&mut r);
    l.sort();
    return l;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(1000000007), true);
        assert_eq!(is_prime(21), false);
        assert_eq!(is_prime(10079), true);
        assert_eq!(is_prime(1000429), true);
        assert_eq!(is_prime(1000013), false);
        assert_eq!(is_prime(1000067), false);
    }

    #[test]
    fn test_get_factors() {
        assert_eq!(factor(1000429 * 15), vec![3, 5, 1000429]);
        assert_eq!(factor(24), vec![2, 2, 2, 3]);
        assert_eq!(factor(45), vec![3, 3, 5]);
        assert_eq!(factor(1000000007), vec![1000000007]);
        assert_eq!(factor(346789), vec![239, 1451]);
        assert_eq!(factor(34486788), vec![2, 2, 3, 7, 7, 89, 659]);
    }
}