use bs_crate;

fn main() {
    let x = 121;
    let is_palindrome = bs_crate::is_palindrome(x);
    println!("{x} is a palindrome: {is_palindrome}");

    let first_10_palindromes = bs_crate::first_n_palindromes(10);
    for x in first_10_palindromes {
        println!("{x} is a palindrome");
    }
}
