/**
 * 

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

*/

fn main() {
    println!("Euler #3");

    let qed = largest_factor(600851475143);
    println!("QED: {}", qed);
}

fn largest_factor(num: u64) -> u64 {

    let mut max_factor = 0;
    let primes = find_primes(num);
    for prime in primes {
        if num % prime == 0 {
           // println!("Factor: {}", prime);
            max_factor = prime;
        }
    }
    return max_factor;
}

fn find_primes(num: u64) -> Vec<u64> {

    let mut primes: Vec<u64> = vec![ 2, 3, 5];
    
    // Sieve of Erastothenes
    let max = (num as f64).sqrt();
    for x in 7..(max as u64) {
        let mut is_prime = true;
        for prime in &primes {
            if x % prime == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            primes.push(x);
        }
    }
            
    return primes;

}
#[test]
fn test_largest_factor() {
    assert_eq!(largest_factor(13195), 29);
}

