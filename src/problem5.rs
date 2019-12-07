use std::collections::HashMap;
#[path = "problem3.rs"]
mod problem3;
use problem3::factor_brute_force;

/// Smallest multiple
///
/// Problem 5
/// 2520 is the smallest number that can be divided by each of
/// the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible
/// by all of the numbers from 1 to 20?
/// ```
/// use euler::problem5::smallest_multiple;
/// let result = smallest_multiple(10);
/// assert_eq!(result, 2520);
/// ```
pub fn smallest_multiple(n: u64) -> u64 {
    let mut ps = primes(n).iter().fold(HashMap::new(), |mut map, p| {
        map.insert(*p, 1);
        map
    });
    for i in 2..=n {
        if !ps.contains_key(&(i as u64)) {
            let c = count(factor_brute_force(i as u64));
            for (k, v) in &c {
                if *v > ps[k] {
                    *ps.get_mut(k).unwrap() = *v;
                }
            }
        }
    }
    ps.iter().fold(1, |acc, i| acc * i.0.pow(*i.1 as u32))
}

fn count(v: Vec<u64>) -> HashMap<u64, u64> {
    v.iter().fold(HashMap::new(), |mut map, p| {
        *map.entry(*p).or_insert(0) += 1;
        map
    })
}

/// Smallest multiple
///
/// Problem 5
/// 2520 is the smallest number that can be divided by each of
/// the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible
/// by all of the numbers from 1 to 20?
/// ```
/// use euler::problem5::smallest_multiple_bf;
/// let result = smallest_multiple_bf(10);
/// assert_eq!(result, 2520);
/// ```
pub fn smallest_multiple_bf(n: u64) -> u64 {
    let mut result: u64 = 1;
    for i in 1..=n {
        result = result * i / gcd(result, i);
    }
    result
}

fn primes(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let mut primes = vec![];
    let mut i = 3;
    let j = (upper_bound as f64).sqrt() as u64;
    while i < upper_bound {
        if primes.iter().filter(|p| **p <= j).all(|p| i % p != 0) {
            primes.push(i);
        }
        i += 2;
    }
    let mut tmp = vec![2];
    tmp.append(&mut primes);
    tmp
}

/// Calculate the Greatest Common Divisor of a and b.
///
/// Unless b==0, the result will have the same sign as b (so that when
/// b is divided by it, the result comes out positive).
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

#[path = "timeit.rs"]
mod timeit;
use timeit::timeit;

#[allow(dead_code)]
fn main() {
    println!("primes(10) = {:?}", primes(10));
    println!("primes(20) = {:?}", primes(20));
    println!(
        "smallest_multiple(10) = {} and should be 2520",
        smallest_multiple(10)
    );
    println!(
        "smallest_multiple(20) = {} and should be 232792560",
        smallest_multiple(20)
    );
    println!(
        "smallest_multiple_bf(10) = {} and should be 2520",
        smallest_multiple_bf(10)
    );
    println!(
        "smallest_multiple_bf(20) = {} and should be 232792560",
        smallest_multiple_bf(20)
    );
    println!("primes(100_000) = {:?}", primes(100_000));
    // println!("smallest_multiple(100_000) = {}", smallest_multiple(100_000));
    println!(
        "smallest_multiple(20): {}",
        timeit(
            || {
                smallest_multiple(20);
            },
            1000
        )
    );
    println!(
        "smallest_multiple_bf(20): {}",
        timeit(
            || {
                smallest_multiple_bf(20);
            },
            1000
        )
    );
}
