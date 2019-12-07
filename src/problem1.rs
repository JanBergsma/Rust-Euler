/// Multiples of 3 and 5
///
/// Problem 1
/// If we list all the natural numbers below 10 that are multiples of 3 or 5,
/// we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
/// ```
/// use euler::problem1::sum_of_multiples_of_3_or_5;
/// let result = sum_of_multiples_of_3_or_5(10);
/// assert_eq!(result, 23);
/// ```
pub fn sum_of_multiples_of_3_or_5(n: u64) -> u64 {
    let n = n - 1;
    sum_divisible_by(3, n) + sum_divisible_by(5, n) - sum_divisible_by(15, n)
}

/// Multiples of 3 and 5
///
/// Problem 1
/// If we list all the natural numbers below 10 that are multiples of 3 or 5,
/// we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
/// ```
/// use euler::problem1::sum_of_multiples_of_3_or_5_brute_force;
/// let result = sum_of_multiples_of_3_or_5_brute_force(10);
/// assert_eq!(result, 23);
/// ```
pub fn sum_of_multiples_of_3_or_5_brute_force(n: u64) -> u64 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

fn sum_divisible_by(d: u64, n: u64) -> u64 {
    let p = n / d;
    d * (p * (p + 1)) / 2
}

#[allow(dead_code)]
fn main() {
    println!("{}", sum_of_multiples_of_3_or_5(10));
    println!("{}", sum_of_multiples_of_3_or_5(1000));
    // see:
    // https://math.stackexchange.com/questions/9259/find-the-sum-of-all-the-multiples-of-3-or-5-below-1000
    let mut counts = (1..334).map(|k| 3 * k).sum::<u64>(); // count all threes
    counts += (1..200).map(|k| 5 * k).sum::<u64>(); // count all fives
    counts -= (1..67).map(|k| 15 * k).sum::<u64>(); // correct over counted 15 (3x5)
    println!(
        "sum_of_multiples_of_3_or_5(1000) = {} counts ={}",
        sum_of_multiples_of_3_or_5(1000),
        counts
    );

    println!(
        "sum_of_multiples_of_3_or_5(1) = {}",
        sum_of_multiples_of_3_or_5(1)
    );
    println!(
        "sum_of_multiples_of_3_or_5(1_000) = {}",
        sum_of_multiples_of_3_or_5(1_000)
    );
    println!(
        "sum_of_multiples_of_3_or_5(1_000_000) = {}",
        sum_of_multiples_of_3_or_5(1_000_000)
    );
    println!(
        "sum_of_multiples_of_3_or_5(1_000_000_000) = {}",
        sum_of_multiples_of_3_or_5(1_000_000_000)
    );
}
