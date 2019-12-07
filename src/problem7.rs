/// 10001st prime
///
/// Problem 7
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
/// we can see that the 6th prime is 13.
///
/// What is the 10 001st prime number
/// ```
/// use euler::problem7::nth_prime;
/// let result = nth_prime(6);
/// assert_eq!(result, 13);
/// ```
pub fn nth_prime(n: u64) -> u64 {
    primes(n).max().unwrap()
}

pub fn primes(n: u64) -> impl Iterator<Item = u64> {
    (0..)
        .map(|i| if i == 0 { 2 } else { 2 * i + 1 })
        .filter(|i| is_prime(*i))
        .take(n as usize)
}

pub fn is_prime(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    if p == 2 || p == 3 {
        return true;
    }
    if p % 2 == 0 || p % 3 == 0 {
        return false;
    }
    (5..=(p as f64).sqrt() as u64)
        .step_by(6)
        .all(|i| p % i != 0 && p % (i + 2) != 0)
}

#[allow(dead_code)]
fn main() {
    println!("nth_rime(6) = {}", nth_prime(6));
    println!("nth_rime(10_001) = {}", nth_prime(10_001));
    println!(
        "primes(10_001) = {:?}",
        primes(10_001).collect::<Vec<u64>>()
    );
}
