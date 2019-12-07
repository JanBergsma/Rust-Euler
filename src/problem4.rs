/// Problem 4
/// A palindromic number reads the same both ways.
/// The largest palindrome made from the product of two 2-digit numbers
/// is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// A002113 Palindromes in base 10.
/// ```
/// use euler::problem4::largest_palindrome_product;
/// let result: u64 = largest_palindrome_product(3);
/// assert_eq!(result, 906609);
/// ```
pub fn largest_palindrome_product(number_of_digits: u32) -> u64 {
    let mut max_palindrone: u64 = 0;

    let n = (0..number_of_digits).map(|i| 9 * 10u64.pow(i)).sum::<u64>();
    let m = n / 11 * 11;

    for i in (n / 10 + 1..n).rev() {
        let n1 = if i % 11 == 0 { n } else { m };
        let st = if i % 11 == 0 { 1 } else { 11 };

        for j in (i - 2..=n1).rev().step_by(st) {
            let ij: u64 = i * j;
            if ij > max_palindrone && is_palindrome(&ij.to_string()) {
                max_palindrone = ij;
            }
        }
    }
    max_palindrone
}

fn is_palindrome(string: &str) -> bool {
    let half_len = string.len() / 2;
    string
        .chars()
        .take(half_len)
        .eq(string.chars().rev().take(half_len))
}
/// Problem 4
/// A palindromic number reads the same both ways.
/// The largest palindrome made from the product of two 2-digit numbers
/// is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// A002113 Palindromes in base 10.
/// ```
/// use euler::problem4::largest_palindrome_product_bf;
/// let result: u64 = largest_palindrome_product_bf(2);
/// assert_eq!(result, 9009);
/// ```
pub fn largest_palindrome_product_bf(number_of_digits: u32) -> u64 {
    let max_number = (0..number_of_digits)
        .map(|_| "9")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    A002113Iter::new(max_number).max().unwrap()
}

/// A002113	Palindromes in base 10.
pub struct A002113Iter {
    n: u64,
    i: u64,
    j: u64,
}

impl A002113Iter {
    pub fn new(n: u64) -> A002113Iter {
        A002113Iter { n, i: 0, j: 1 }
    }
}

impl Iterator for A002113Iter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.i == 0 {
            self.i = 1;
            Some(0)
        } else {
            while self.i <= self.n {
                while self.j <= self.n {
                    let ij = self.i * self.j;
                    self.j += 1;
                    if is_palindrome(&ij.to_string()) {
                        return Some(ij);
                    }
                }
                self.i += 1;
                self.j = self.i;
            }
            None
        }
    }
}

pub const A002113: [u64; 61] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44, 55, 66, 77, 88, 99, 101, 111, 121, 131, 141, 151,
    161, 171, 181, 191, 202, 212, 222, 232, 242, 252, 262, 272, 282, 292, 303, 313, 323, 333, 343,
    353, 363, 373, 383, 393, 404, 414, 424, 434, 444, 454, 464, 474, 484, 494, 505, 515,
];

#[path = "timeit.rs"]
mod timeit;
use timeit::timeit;

#[allow(dead_code)]
fn main() {
    println!(
        "largest_palindrome_product(2) = {}, largest_palindrome_product_bf(2) ={}",
        largest_palindrome_product(2),
        largest_palindrome_product_bf(2)
    );
    println!(
        "largest_palindrome_product(3) = {}, largest_palindrome_product_bf(3) ={}",
        largest_palindrome_product(3),
        largest_palindrome_product_bf(3)
    );
    println!(
        "largest_palindrome_product(4) = {}",
        largest_palindrome_product(4)
    );
    println!(
        "A002113Iter = {:?}",
        A002113Iter::new(99).collect::<Vec<u64>>()
    );
    println!(
        "{:?} should be {:?}",
        largest_palindrome_product_bf(2),
        9009
    );
    println!(
        "largest_palindrome_product: {}",
        timeit(
            || {
                largest_palindrome_product(3);
            },
            100
        )
    );
    println!(
        "largest_palindrome_product_bf: {}",
        timeit(
            || {
                largest_palindrome_product_bf(3);
            },
            100
        )
    );
}
