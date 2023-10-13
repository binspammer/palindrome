///
///
pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let digits: Vec<_> = s.chars().collect();
    let lenght = digits.len();
    for i in 0..(lenght / 2) {
        if &digits[i] != &digits[lenght - 1 - i] {
            return false;
        }
    }
    true
}

///
pub fn first_n_palindromes(n: u32) -> Vec<u32> {
    (0..n).filter(|x| is_palindrome(*x)).collect::<Vec<u32>>()
}

///
fn main() {
    let x = 121;
    let is_palindrome = is_palindrome(x);
    println!("{x} is a palindrome: {is_palindrome}");

    let first_10_palindromes = first_n_palindromes(30);

    for x in first_10_palindromes {
        println!("{x} is a palindrome");
    }
}
