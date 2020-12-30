/**
 *
 A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

*/


fn main() {
    println!("Euler 4");

    let mut max = 0;
    for x in 10..1000 {
        for y in 10..1000 {
            let result = x * y;
            if is_palindrome(result) {
                if result > max {
                    max = result;
                }
            }
        }
    }
    println!("QED: {}", max);
}

fn is_palindrome(num: u32) -> bool {
    let str = num.to_string();
    let mid = str.len() / 2;
    let adj = if str.len() % 2 == 0 { 0 } else { 1 };
    let a: Vec<char> = str.get(..mid+adj).unwrap().chars().collect();
    let b: Vec<char> = str.get(mid..).unwrap().chars().rev().collect();

    return a == b;
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(9009), true);
    assert_eq!(is_palindrome(90109), true);
    assert_eq!(is_palindrome(109), false);
}
