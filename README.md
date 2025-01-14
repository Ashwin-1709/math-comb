# math-comb 
![crates.io](https://img.shields.io/crates/v/math-comb.svg) ![Build Passing](https://github.com/Ashwin-1709/math-comb/actions/workflows/rust.yml/badge.svg)

Math library for Combinatorics, Modular arithmetic & Prime Factorization.

---

## Description

This library provides a collection of mathematical utilities:
- Combinatorics:
    - Combinations
    - Permutations
- Modular Arithemetic:
    - Modular Exponentiation
    - Modular Inverses
- Number Theory:
    - Primality checking
    - Prime factorization (using the `Pollard-Rho` algorithm)
    - Smallest prime factor sieve (SPF)

---

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
math-comb = "0.2.3"
```

---

### Examples

#### Permutations & Combinations (Modular)

```rust
use math_comb::Comb;

fn main() {
    let comb = Comb::new(
        /* modulus = */ 1000000007,
        /* max_fact = */ 5
    );
    println!("5 choose 2: {}", comb.nCr(5, 2)); // Output: 10
    println!("5 permute 2: {}", comb.nPr(5, 2)); // Output: 60
}
```

#### Modular Exponentiation & Inverse

```rust
use math_comb::Modexp;

fn main() {
    let base: u64 = 2;
    let exponent: u64 = 10;
    let modulus: u64 = 1000000007;
    println!("2^10 % 1000000007: {}", Modexp::mod_exp(base, exponent, modulus)); // Output: 1024

    let x: u64 = 3;
    let modulus: u64 = 11;
    println!("Modular inverse of 3 mod 11: {}", Modexp::mod_inv(x, modulus)); // Output: 4
}
```

#### Prime factorization & Primality checks

```rust
use math_comb::Prime;

fn main() {
    let n: u64 = 15006435;
    println!("A non-trivial factor of {}: {}", n, Prime::pollard(n)); // Output: A non-trivial factor of 15006435: 3 or 5 or 1000429

    let factors = Prime::factor(n);
    println!("Prime factors of {}: {:?}", n, factors); // Output: Prime factors of 15006435: [3, 5, 1000429]

    let a: u64 = 1000000007;
    println!("Is {} prime? {}", a, Prime::is_prime(a)); // Output: Is 1000000007 prime? true

    let b: u64 = 21;
    println!("Is {} prime? {}", b, Prime::is_prime(b)); // Output: Is 21 prime? false
}
```

#### Smallest Prime Factors (SPF) & Prime Factorization

```rust
use math_comb::Spf;

fn main() {
    let spf = Spf::new(
        /*max_limit = */ 10000000
    );

    // Get the smallest prime factor of a number
    let number: u64 = 81;
    println!("Smallest prime factor of {}: {}", number, spf.get_spf(number)); // Output: Smallest prime factor of 81: 3

    // Factorize a number into its prime factors. This is O(logn) since we have precomputed spfs.
    let number: u64 = 45;
    let factors = spf.factorize(number);
    println!("Prime factors of {}: {:?}", number, factors); // Output: Prime factors of 45: [3, 3, 5]
}
```

---

## License

This library is licensed under MIT License.