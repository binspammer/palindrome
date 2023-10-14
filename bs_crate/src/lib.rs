///
const SEQ_A070199_1M_NUMBER: u32 = 1_999; // Number of palindromes of length <= 1,000,000 (https://oeis.org/A070199)
const MAX_LIMIT_NUMBER: u32 = 1_000_000; // Hardcoded requested limit

///
pub fn is_palindrome(n: u32) -> bool {
    if n > MAX_LIMIT_NUMBER {
        panic!("Number must not be greater than 1,000,000, got {}.", n);
    }
    let digits = n.to_string();
    let front = digits.chars();
    let back = front.clone().rev();
    front.eq(back)
}

///
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
