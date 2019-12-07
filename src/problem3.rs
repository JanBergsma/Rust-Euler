/// Largest prime factor
///
/// Problem 3
/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
/// ```
/// use euler::problem3::largest_prime_factor;
/// let result: u64 = largest_prime_factor(13195);
/// assert_eq!(result, 29);
/// ```

pub fn largest_prime_factor(i: u64) -> u64 {
    *factor_brute_force(i).iter().max().unwrap()
}

/// Brute force integer factorizing.    
/// ```
/// use euler::problem3::factor_brute_force;
/// let result = factor_brute_force(13195);
/// assert_eq!(result, vec![5, 7, 13, 29]);
/// ```
pub fn factor_brute_force(mut x: u64) -> Vec<u64> {
    let mut roots = Vec::new();
    for i in 2..=((x as f64).sqrt() as u64) {
        while x > 1 && x % i == 0 {
            x /= i;
            roots.push(i);
        }
    }
    if x > 1 {
        roots.push(x);
    }
    roots
}

#[allow(dead_code)]
fn main() {
    println!(
        "factor_brute_force(13195) = {:?}",
        factor_brute_force(13195)
    );
    println!(
        "factor_brute_force(600851475143) = {:?}",
        factor_brute_force(600_851_475_143)
    );
    println!(
        "largest_prime_factor(13195) = {}",
        largest_prime_factor(13195)
    );
    println!(
        "largest_prime_factor(600851475143) = {}",
        largest_prime_factor(600_851_475_143)
    );
}
