/// Sum square difference
///
/// Problem 6
/// The sum of the squares of the first ten natural numbers is,
///
/// 12 + 22 + ... + 102 = 385
/// The square of the sum of the first ten natural numbers is,
///
/// (1 + 2 + ... + 10)2 = 552 = 3025
/// Hence the difference between the sum of the squares of the first ten
/// natural numbers and the square of the sum is 3025 − 385 = 2640.
///
/// Find the difference between the sum of the squares of the first one
/// hundred natural numbers and the square of the sum.
///
/// ```
/// use euler::problem6::sum_square_difference;
/// let result = sum_square_difference(10);
/// assert_eq!(result, 2640);
/// ```

pub fn sum_square_difference(n: u128) -> u128 {
    if n == 0 {
        0
    } else {
        n * (n - 1) * (n + 1) * (3 * n + 2) / 12
    }
}

/// Sum square difference
///
/// Problem 6
/// The sum of the squares of the first ten natural numbers is,
///
/// 12 + 22 + ... + 102 = 385
/// The square of the sum of the first ten natural numbers is,
///
/// (1 + 2 + ... + 10)2 = 552 = 3025
/// Hence the difference between the sum of the squares of the first ten
/// natural numbers and the square of the sum is 3025 − 385 = 2640.
///
/// Find the difference between the sum of the squares of the first one
/// hundred natural numbers and the square of the sum.
///
/// ```
/// use euler::problem6::sum_square_difference_bf;
/// let result = sum_square_difference_bf(10);
/// assert_eq!(result, 2640);
/// ```
pub fn sum_square_difference_bf(n: u128) -> u128 {
    (1..=n).sum::<u128>().pow(2) - (1..=n).map(|i| i.pow(2)).sum::<u128>()
}

#[allow(dead_code)]
fn main() {
    println!(
        "sum_square_difference_bf(0) = {}",
        sum_square_difference_bf(0)
    );
    println!("sum_square_difference(0) = {}", sum_square_difference(0));
    println!(
        "sum_square_difference_bf(10) = {}",
        sum_square_difference_bf(10)
    );
    println!("sum_square_difference(1) = {}", sum_square_difference(1));
    println!("sum_square_difference(10) = {}", sum_square_difference(10));
    println!(
        "sum_square_difference(100) = {}",
        sum_square_difference(100)
    );
    println!(
        "sum_square_difference(1000) - {}",
        sum_square_difference(1000)
    );
    println!(
        "sum_square_difference(10_000) = {}",
        sum_square_difference(10_000)
    );
    println!(
        "sum_square_difference(100_000) = {}",
        sum_square_difference(100_000)
    );
    println!(
        "sum_square_difference(1_000_000) = {}",
        sum_square_difference(1_000_000)
    );
}
