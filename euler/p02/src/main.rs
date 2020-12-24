/**
 *
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

*/

fn main() {
    println!("Euler #2");

    let mut f1 = 1;
    let mut f2 = 2;
    let mut sum = 0;

    loop {
        let num = f1 +f2;
        if num > 4000000 { 
            break;
        }

        if num % 2 == 0 {
            sum = sum + num;
        }

       // println!("Num: {}", num);
        f1 = f2;
        f2 = num;
    }

    println!("QED: {}", sum);
}
