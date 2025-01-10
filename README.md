# math-comb

Combinatorial and modular arithmetic library.

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
    let comb = Comb::new(1000000007, 5);
    println!("5 choose 2: {}", comb.nCr(5, 2)); // Output: 10
    println!("5 permute 2: {}", comb.nPr(5, 2)); // Output: 60
}
```

```rust
use math_comb::Modexp;

fn main() {
    let base = 2;
    let exponent = 10;
    let modulus = 1000000007;
    println!("2^10 % 1000000007: {}", Modexp::mod_exp(base, exponent, modulus)); // Output: 1024

    let x = 3;
    let modulus = 11;
    println!("Modular inverse of 3 mod 11: {}", Modexp::mod_inv(x, modulus)); // Output: 4
}
```

## License

This library is licensed under MIT License.