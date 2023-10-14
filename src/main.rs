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
pub fn first_n_palindromes(mut n: u32) -> Vec<u32> {
    if n > SEQ_A070199_1M_NUMBER {
        println!( "With {} numbers the palindrome number will exceed 1,000,000 limit.\nSo we only provide the numbers below that.", n);
        n = SEQ_A070199_1M_NUMBER;
    }
    (0..MAX_LIMIT_NUMBER)
        .filter(|x| is_palindrome(*x))
        .take(n as usize)
        .collect()
}

fn main() {
    let x = 121;
    let is_palindrome = is_palindrome(x);
    println!("{x} is a palindrome: {is_palindrome}");

    let first_10_palindromes = first_n_palindromes(SEQ_A070199_1M_NUMBER + 1);
    for x in first_10_palindromes {
        println!("{x} is a palindrome");
    }
}
