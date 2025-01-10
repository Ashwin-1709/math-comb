# math-comb

[![Rust Check](https://github.com/Ashwin-1709/math-comb/actions/workflows/rust.yml/badge.svg)](https://github.com/Ashwin-1709/math-comb/actions/workflows/rust.yml) Combinatorial and modular arithmetic library.

## Description

This library provides utilities for combinatorial calculations and modular arithmetic, including combinations, permutations, modular exponentiation, and modular inverses.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
math-comb = "0.1.0"
```

### Examples
```rust
use math_comb::Comb;

fn main() {
    let comb: Comb = Comb::new(1000000007, 5);
    println!("5 choose 2: {}", comb.nCr(5, 2)); // Output: 10
    println!("5 permute 2: {}", comb.nPr(5, 2)); // Output: 60
}
```

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

## License

This library is licensed under MIT License.