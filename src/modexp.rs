/// Calculates (base^exponent) % modulus using modular exponentiation.
///
/// # Arguments
///
/// *   `base` - The base.
/// *   `exponent` - The exponent.
/// *   `modulus` - The modulus.
pub fn mod_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut retval = 1;
    let mut exp = exponent;
    let mut b = base % modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            retval = (retval as u128 * b as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        b = (b as u128 * b as u128 % modulus as u128) as u64;
    }
    retval
}

/// Calculates the modular multiplicative inverse of `x` modulo `modulus`.
///
/// The modular inverse of `x` modulo `modulus` is an integer `y` such that
/// (x * y) % modulus == 1. It exists if and only if `x` and `modulus` are coprime
/// (i.e., their greatest common divisor is 1).
///
/// This function uses Fermat's Little Theorem, which states that if `modulus` is a prime number,
/// then for any integer `x` not divisible by `modulus`, x^(modulus-1) ≡ 1 (mod modulus).
/// Therefore, the modular inverse of `x` is x^(modulus-2) (mod modulus).
///
/// # Arguments
///
/// *   `x` - The number for which to calculate the inverse.
/// *   `modulus` - The modulus.
///
/// # Returns
///
/// The modular inverse of `x` modulo `modulus`.
///
/// # Panics
///
/// This function will panic if:
/// *   `modulus` is 0.
/// *   `x` and `modulus` are not coprime (their greatest common divisor is not 1).
pub fn mod_inv(x: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        panic!("Modulus cannot be zero.");
    }

    if x == 0 {
        panic!("x cannot be zero.");
    }

    if gcd(x, modulus) != 1 {
        panic!("x and modulus are not coprime. Inverse does not exist.");
    }
    mod_exp(x, modulus - 2, modulus)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        assert_eq!(mod_exp(2, 10, 1000000007), 1024);
        assert_eq!(mod_exp(3, 5, 1000000007), 243);
        assert_eq!(mod_exp(5, 3, 1000000007), 125);
        assert_eq!(mod_exp(10, 0, 1000000007), 1);
        assert_eq!(mod_exp(1255, 623, 1000000007), 152493811);
        assert_eq!(mod_exp(2, 10, 5), 4);
        assert_eq!(mod_exp(3, 5, 7), 5);
        assert_eq!(mod_exp(10, 0, 100), 1);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(3, 11), 4);
        assert_eq!(mod_inv(7, 13), 2);
        assert_eq!(mod_inv(5, 7), 3);
    }

    #[test]
    #[should_panic(expected = "x and modulus are not coprime. Inverse does not exist.")]
    fn test_mod_inv_not_coprime() {
        mod_inv(8, 12);
    }

    #[test]
    #[should_panic(expected = "Modulus cannot be zero.")]
    fn test_mod_inv_modulus_zero() {
        mod_inv(8, 0);
    }

    #[test]
    #[should_panic(expected = "x cannot be zero.")]
    fn test_mod_inv_x_zero() {
        mod_inv(0, 12);
    }
}