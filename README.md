# Rust take-home assignment

A "palindrome" is a number that is the same when the digits are reversed.
For example, 121, 2332, and 6 are all palindromes.
But 10 is not a palindrome (since leading zeroes are not allowed).
Treat 0 as a palindrome, and use an unsigned integer type.

Write a crate that allows the user to:
 - check if a number is a palindrome
 - generate the first N palindromes

You can assume that you don't need to deal with numbers greater than 1,000,000.
Your code may panic if it is called with any values that would result in a number greater than 1,000,000 being generated.

You can use whatever 3rd-party crates you want, but they may not appear in your public API.
Also, if a crate literally implements the question as-is, don't use it.

The main focus of the exercise is to develop a high-quality crate.
The exact meaning of "high-quality" is left deliberately vague, but use your best judgement.
Think about the crates you've had the best experience with, and try to emulate those.
The exact details of the API are up to you - there are no hard requirements about function signatures, etc (other than there being no 3rd-party types present in the API).

Your crate should be able to be used roughly like this:
```rust
fn main() {
    let x = 123;
    let is_palindrome = your_crate::is_palindrome(x);
    println!("{x} is a palindrome: {is_palindrome}");

    let first_10_palindromes = your_crate::first_n_palindromes(10);

    for x in first_10_palindromes {
        println!("{x} is a palindrome");
    }
}
```

# Implementation

Please see the crate implemented in ./bs_crate directory.

# Using

Please read the [README.md](https://github.com/binspammer/palindrome/blob/master/bs_crate/README.md)
