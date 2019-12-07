#[path = "problem7.rs"]
mod problem7;
use problem7::is_prime;

#[path = "timeit.rs"]
mod timeit;
use timeit::timeit;
const RUN_WITH_TIMING: bool = false;
/// Summation of primes
///
/// Problem 10
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
/// >>> sum_primes_below(10)
/// 17
/// ```
/// use euler::problem10::sum_primes_below;
/// let result = sum_primes_below(10);
/// assert_eq!(result, 17);
/// ```
pub fn sum_primes_below(n: u64) -> u64 {
    (0..)
        .map(|i| if i == 0 { 2 } else { 2 * i + 1 })
        .take_while(move |i| *i < n)
        .filter(|i| is_prime(*i))
        .sum()
}

/// Summation of primes
///
/// Problem 10
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
/// >>> sum_primes_below(10)
/// 17
/// ```
/// use euler::problem10::sum_primes_below2;
/// let result = sum_primes_below2(10);
/// assert_eq!(result, 17);
/// ```
pub fn sum_primes_below2(n: u64) -> u64 {
    let n = n as usize;
    let mut sum = 0;
    let mut a = vec![true; n]; // Initialize the primality list
    a[0] = false;
    a[1] = false;
    for i in 0..n {
        if a[i] {
            sum += i;
            for n in (i * i..n).step_by(i) {
                // Mark factors non-prime
                a[n] = false;
            }
        }
    }
    sum as u64
}

/// Summation of primes
///
/// Problem 10
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
/// >>> sum_primes_below(10)
/// 17
/// ```
/// use euler::problem10::sieve_of_eratosthenes;
/// let result = sieve_of_eratosthenes(10);
/// assert_eq!(result, 17);
/// ```
pub fn sieve_of_eratosthenes(limit: u64) -> u64 {
    if limit < 2 {
        return 0;
    }
    let sieve_bound: usize = ((limit as usize) - 1) / 2 + 1;
    let mut sieve = vec![false; sieve_bound];
    let crosslimit = (((limit as f64).sqrt() as usize) - 1) / 2 + 1;

    for i in 1..crosslimit {
        if !sieve[i] {
            for j in (2 * i * (i + 1)..sieve_bound).step_by(2 * i + 1) {
                sieve[j] = true;
            }
        }
    }
    2u64 + (1..sieve_bound)
        .filter(|i| !sieve[*i])
        .map(|i| (2 * i + 1) as u64)
        .sum::<u64>()
}

#[allow(dead_code)]
fn main() {
    println!("sum_primes_below(10) = {}", sum_primes_below(10));
    println!("sum_primes_below2(10) = {}", sum_primes_below2(10));
    println!("sieve_of_eratosthenes(10) = {}", sieve_of_eratosthenes(10));
    println!(
        "sum_primes_below(2_000_000) = {}",
        sum_primes_below(2_000_000)
    );
    println!(
        "sum_primes_below2(2_000_000) = {}",
        sum_primes_below2(2_000_000)
    );
    println!(
        "sieve_of_eratosthenes(2_000_000) = {}",
        sieve_of_eratosthenes(2_000_000)
    );

    if !RUN_WITH_TIMING {
        return;
    }

    println!(
        "sum_primes_below(2_000_000): {}",
        timeit(
            || {
                sum_primes_below(2_000_000);
            },
            10
        )
    );
    println!(
        "sum_primes_below2(2_000_000): {}",
        timeit(
            || {
                sum_primes_below2(2_000_000);
            },
            10
        )
    );
    println!(
        "sieve_of_eratosthenes(2_000_000): {}",
        timeit(
            || {
                sieve_of_eratosthenes(2_000_000);
            },
            10
        )
    );
}
