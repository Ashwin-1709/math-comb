mod modexp;

/// A struct that provides methods for modular exponentiation and modular inverse calculations.
pub struct Modexp {}

impl Modexp {
    /// Calculates (base^exponent) % modulus using modular exponentiation.
    ///
    /// # Arguments
    ///
    /// *   `base` - The base.
    /// *   `exponent` - The exponent.
    /// *   `modulus` - The modulus.
    pub fn mod_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
        return modexp::mod_exp(base, exponent, modulus);
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
        return modexp::mod_inv(x, modulus);
    }
}

/// A struct for pre-calculating factorials and their modular inverses,
/// useful for efficient combination and permutation calculations under mod.
pub struct Comb {
    mod_value: u64,
    max_fact: usize,
    fact: Vec<u64>,
    inv_fact: Vec<u64>
}

impl Comb {
    /// Creates a new `Comb` instance, pre-calculating factorials and their
    /// modular inverses up to `max_fact`.
    ///
    /// This pre-computation allows for fast calculation of combinations and permutations
    /// modulo `mod_value`.
    ///
    /// # Arguments
    ///
    /// *   `mod_value` - The modulus to use for calculations.
    /// *   `max_fact` - The maximum number for which factorials and inverse
    ///                  factorials will be pre-calculated. This determines the
    ///                  range of `n` that can be used in `nCr` and `nPr` without
    ///                  requiring further calculations.
    /// # Panics
    /// 
    /// Panics if modulus is not prime.
    pub fn new(mod_value: u64, max_fact: usize) -> Comb {
        if !Self::check_prime(mod_value) {
            panic!("modulus is not prime!");
        }

        let mut fact: Vec<u64> = vec![0; max_fact + 1];
        let mut inv_fact: Vec<u64> = vec![0; max_fact + 1];
        
        fact[0] = 1;

        for i in 1..=max_fact {
            fact[i] = (fact[i - 1] * (i as u64)) % mod_value;
        }
        inv_fact[max_fact] = Modexp::mod_inv(fact[max_fact] as u64, mod_value);
        for i in (0..max_fact).rev() {
            inv_fact[i] = (inv_fact[i + 1] * ((i + 1) as u64)) % mod_value;
        }

        Comb { 
            mod_value: mod_value, 
            max_fact: max_fact, 
            fact: fact, 
            inv_fact: inv_fact
        }
    }
    
    /// Calculates nPr (n permutations of r) under mod.
    ///
    /// # Arguments
    ///
    /// *   `n` - The total number of items.
    /// *   `r` - The number of items to choose.
    ///
    /// # Panics
    ///
    /// Panics if `n` is less than `r` or `n` > `max_fact`.
    pub fn nPr(&self, n: u64, r: u64) -> u64 {
        if n < r {
            panic!("n cannot be less than r!")
        } else if n > (self.max_fact as u64) {
            panic!("n cannot be greater than {}!", self.max_fact);
        } else {
            return (self.fact[n as usize] * self.inv_fact[r as usize]) % self.mod_value;
        }
    }

    /// Calculates nCr (n combinations of r) under mod.
    ///
    /// # Arguments
    ///
    /// *   `n` - The total number of items.
    /// *   `r` - The number of items to choose.
    ///
    /// # Panics
    ///
    /// Panics if `n` is less than `r` or `n` > `max_fact`.
    pub fn nCr(&self, n: u64, r: u64) -> u64 {
        if n < r {
            panic!("n cannot be less than r!");
        } else if n > (self.max_fact as u64) {
            panic!("n cannot be greater than {}!", self.max_fact);
        } else {
            return (self.nPr(n, r) * self.inv_fact[(n - r) as usize]) % self.mod_value;
        }
    }

    fn check_prime(n: u64) -> bool {
        let mut _x: u64 = 2;
        while _x * _x <= n {
            if n % _x == 0 {
                return false
            }
            _x = _x + 1;
        }
        return true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ncr() {
        let comb: Comb = Comb::new(1000000007, 5);
        assert_eq!(comb.nCr(5, 2), 10);
        assert_eq!(comb.nCr(4, 0), 1);
        assert_eq!(comb.nCr(4, 4), 1);
    }

    #[test]
    fn test_npr() {
        let comb: Comb = Comb::new(1000000007, 5);
        assert_eq!(comb.nPr(5, 2), 60);
        assert_eq!(comb.nPr(5, 0), 120);
        assert_eq!(comb.nPr(5, 5), 1);
    }

    #[test]
    fn test_ncr_small_mod() {
        let comb: Comb = Comb::new(7, 5);
        assert_eq!(comb.nCr(5, 2), 3);
        assert_eq!(comb.nCr(4, 0), 1);
        assert_eq!(comb.nCr(4, 3), 4);
        assert_eq!(comb.nCr(5, 0), 1);

        let comb: Comb = Comb::new(2, 1);
        assert_eq!(comb.nCr(1, 1), 1);
        assert_eq!(comb.nCr(1, 0), 1);
        assert_eq!(comb.nCr(0, 0), 1);
    }

    #[test]
    fn test_npr_small_mod() {
        let comb: Comb = Comb::new(7, 5);
        assert_eq!(comb.nPr(5, 2), 4);
        assert_eq!(comb.nPr(4, 0), 3);
        assert_eq!(comb.nPr(4, 3), 4);
        let comb: Comb = Comb::new(2, 1);
        assert_eq!(comb.nPr(1, 1), 1);
        assert_eq!(comb.nPr(1, 0), 1);
        assert_eq!(comb.nPr(0, 0), 1);
    }

    #[test]
    #[should_panic(expected = "n cannot be less than r!")]
    fn test_ncr_panic() {
        let comb = Comb::new(1000000007, 5);
        comb.nCr(2, 5);
    }

    #[test]
    #[should_panic(expected = "n cannot be less than r!")]
    fn test_npr_panic() {
        let comb = Comb::new(1000000007, 5);
        comb.nPr(2, 5);
    }

    #[test]
    #[should_panic(expected = "n cannot be greater than 5!")]
    fn test_n_greater_than_max_fact_panic() {
        let comb: Comb = Comb::new(13, 5);
        comb.nCr(10, 3);
    }

    #[test]
    #[should_panic(expected = "modulus is not prime!")]
    fn test_composite_mod() {
        let comb: Comb = Comb::new(4, 14);
    }
}