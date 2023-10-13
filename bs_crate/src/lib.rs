// bs_crate/src/lib.rs
//
// The main lib file for the Rust palindrome number library.
//
// This file is part of the Rust 'bs_crate' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.

//! Palindrome number methods
//!
//! # An introduction to palindrome number
//!
//! A palindromic number (also known as a numeral palindrome or a numeric palindrome) is a number (such as 16461)
//! that remains the same when its digits are reversed. In other words, it has reflectional symmetry across a
//! vertical axis. The term palindromic is derived from palindrome, which refers to a word (such as rotor or racecar)
//! whose spelling is unchanged when its letters are reversed. The first 30 palindromic numbers (in decimal) are:
//!    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44, 55, 66, 77, 88, 99, 101, 111, 121, 131, 141, 151, 161, 171, 181, 191, 202, …
//! (sequence [A070199](https://oeis.org/A070199) in the [OEIS](https://oeis.org/)).
//!
//! Palindromic numbers receive most attention in the realm of recreational mathematics.
//! A typical problem asks for numbers that possess a certain property and are palindromic. For instance:
//!  - The palindromic primes are 2, 3, 5, 7, 11, 101, 131, 151, ...
//!  - The palindromic square numbers are 0, 1, 4, 9, 121, 484, 676, 10201, 12321, ...
//! (sequence [A070199](https://oeis.org/A070199) in the [OEIS](https://oeis.org/)).
//!
//! It is obvious that in any base there are infinitely many palindromic numbers, since in any base the infinite sequence
//! of numbers written (in that base) as 101, 1001, 10001, 100001, etc. consists solely of palindromic numbers.
//!
//! ## Decimal palindromic numbers
//!
//! All numbers in base 10 (and indeed in any base) with one digit are palindromic, so there are
//!  - ten decimal palindromic numbers with one digit:
//!    {0, 1, 2, 3, 4, 5, 6, 7, 8, 9}.
//!  - There are 9 palindromic numbers with two digits:
//!    {11, 22, 33, 44, 55, 66, 77, 88, 99}.
//!  - There are 90 palindromic numbers with three digits:
//!    {101, 111, 121, 131, 141, 151, 161, 171, 181, 191, …, 909, 919, 929, 939, 949, 959, 969, 979, 989, 999}
//!    ...
//!    and so on.
//!
//! Detailed properties of the palindrome numbers can be found on [A070199](https://oeis.org/A070199) in the [OEIS](https://oeis.org/).
//!
//! ## Crate Features
//!
//! ### Default behaviour
//!
//! The crate allows the user to:
//!  - check if a number is a palindrome
//!  - generate the first N palindromes
//!
//! ### Limits and Assumptions
//!
//!  - Assuming that you don't need to deal with numbers greater than 1,000,000.
//!  - Code may panic if it is called with any values that would result in a number greater than 1,000,000 being generated.
//!

// clippy: do not warn about things like "bs_crate" inside the docs
#![allow(clippy::doc_markdown)]

/// Hardcoded requested limit
const MAX_LIMIT_NUMBER: u32 = 1_000_000;

/// Number of palindromes of length <= 1,000,000 [A070199](https://oeis.org/A070199)
const SEQ_A070199_1M_NUMBER: u32 = 1_999;

/// Check if a number is a palindrome
///
/// # Examples
///
/// ```
/// let x = 121;
/// assert!(is_palindrome(x));
/// ```
pub fn is_palindrome(n: u32) -> bool {
    if n > MAX_LIMIT_NUMBER {
        panic!("Number must not be greater than 1,000,000, got {}.", n);
    }
    let digits = n.to_string();
    let front = digits.chars();
    let back = front.clone().rev();
    front.eq(back)
}

/// Generate the first N palindromes
///
/// # Examples
///
/// ```
/// let first_10_palindromes = first_n_palindromes(10);
/// assert_eq!(first_10_palindromes.first(), Some(&0));
/// ```
pub fn first_n_palindromes(n: u32) -> Vec<u32> {
    (0..MAX_LIMIT_NUMBER)
        .filter(|x| is_palindrome(*x))
        .take(
            if n > SEQ_A070199_1M_NUMBER {
                println!( "With {} numbers the palindrome number will exceed 1,000,000 limit.\nSo we only provide the numbers below that.", n);
                SEQ_A070199_1M_NUMBER
            } else {
                n
            } as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_check_random() {
        let x = 121;
        let is_palindrome = is_palindrome(x);
        assert!(is_palindrome);
    }

    #[test]
    fn is_palindrome_check_zero() {
        let x = 0;
        let is_palindrome = is_palindrome(x);
        assert!(is_palindrome);
    }

    #[test]
    #[should_panic]
    fn is_palindrome_check_above_limit() {
        let x = MAX_LIMIT_NUMBER + 10;
        let is_palindrome = is_palindrome(x);
        assert!(is_palindrome);
    }

    #[test]
    fn first_10_palindromes_check_first_one() {
        let first_10_palindromes = first_n_palindromes(10);
        assert_eq!(first_10_palindromes.first(), Some(&0));
    }

    #[test]
    fn first_10_palindromes_check_first_three() {
        let all_eq = vec![0, 1, 2];
        let first_3_palindromes = first_n_palindromes(3);
        assert!(first_3_palindromes.iter().eq(&all_eq));
    }

    #[test]
    fn first_10_palindromes_check_last_one() {
        let first_10_palindromes = first_n_palindromes(SEQ_A070199_1M_NUMBER);
        assert_eq!(first_10_palindromes.last(), Some(&999999));
    }

    #[test]
    fn first_10_palindromes_check_above_limit() {
        let first_10_palindromes = first_n_palindromes(SEQ_A070199_1M_NUMBER + 1);
        assert_eq!(first_10_palindromes.last(), Some(&999999));
    }
}
