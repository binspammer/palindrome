///
///
pub fn is_palindrome(n: u32) -> bool {
    let num = n.to_string();
    let mut it = num.chars();
    while let (Some(front), Some(back)) = (it.next(), it.next_back()) {
        if front != back {
            return false;
        }
    }
    true
}

///
pub fn is_palindrome1(n: u32) -> bool {
    let num = n.to_string();
    let front = num.chars();
    let back = front.clone().rev();
    front.eq(back)
}

///
pub fn first_n_palindromes(n: usize) -> Vec<u32> {
    (0..).filter(|x| is_palindrome(*x)).take(n).collect()
}

///
fn main() {
    let x = 121;
    let is_palindrome = is_palindrome(x);
    println!("{x} is a palindrome: {is_palindrome}");

    let first_10_palindromes = first_n_palindromes(10);
    for x in first_10_palindromes {
        println!("{x} is a palindrome");
    }

    println!("compare 2 itarators:");
    let numb = 11;
    let txt = numb.to_string();
    let chrs = txt.chars();
    let chrsrv = chrs.clone().rev();
    let res = chrs.eq(chrsrv);
    println!("{res}");
}
