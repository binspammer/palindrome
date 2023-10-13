# Palindrome number library prototype

## Definitions
A "palindrome" is a number that is the same when the digits are reversed.
For example, 121, 2332, and 6 are all palindromes.
But 10 is not a palindrome (since leading zeroes are not allowed).
Treat 0 as a palindrome, and use an unsigned integer type.

## Goal
Write crate that allows the user to:
1. check if a number is a palindrome
2. generate the first N palindromes

## Assumptions
1. No need to deal with numbers greater than 1,000,000.
2. Code may panic if it is called with any values that would result in a number greater than 1,000,000 being generated.
3. 3rd-party crates can be used, but they may not appear in your public API.

## Constrains
If a crate literally implements the question as-is, don't use it.

## Crate location
1. The code itself can be found on [github.com](https://github.com/binspammer/palindrome.git).
2. It is uploaded on [crates.io](https://crates.io/crates/bs_crate) and can be used directly as dependency in Cargo.toml:
```rust
...
[dependencies]
bs_crate = "0.1.0"
...
```

## Using
The crate should be used like this:
```rust
use bs_crate;

fn main() {
    let x = 123;
    let is_palindrome = bs_crate::is_palindrome(x);
    println!("{x} is a palindrome: {is_palindrome}");

    let first_10_palindromes = bs_crate::first_n_palindromes(10);

    for x in first_10_palindromes {
        println!("{x} is a palindrome");
    }
}
```
