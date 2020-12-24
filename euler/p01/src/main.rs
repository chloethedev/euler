/**
 *

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

*/

fn main() {
    println!("Euler #1");
    
    let sum: i64 = multiples(1000).iter().sum();
    println!("Sum: {}", sum);
}

fn multiples(num: i64) -> Vec<i64> {
    let mut stack: Vec<i64> = Vec::new();

    for x in 1..num {
        if x % 3 == 0 || x % 5 == 0 {
            stack.push(x)
        }   
    }
    return stack;
}

#[test]
fn test_multiples() {
    assert_eq!( multiples(10), [3,5,6,9] );
}
