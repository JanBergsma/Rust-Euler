///
/// Special Pythagorean triplet
///
/// Problem 9
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for
/// which,
/// a² + b² = c²
/// For example, 3² + 4² = 9 + 16 = 25 = 5².
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
/// ```
/// use euler::problem9::find_pythagorean_triple;
/// let result = find_pythagorean_triple(1000);
/// assert_eq!(result, (375, 200, 425));
/// ```
pub fn find_pythagorean_triple(n: u32) -> (u32, u32, u32) {
    for c in 3..=n {
        for b in 2..c {
            let a = n - b - c;
            if a > 0 && a.pow(2) + b.pow(2) == c.pow(2) {
                return (a, b, c);
            }
        }
    }
    (0, 0, 0)
}

#[allow(dead_code)]
fn main() {
    let (a, b, c) = find_pythagorean_triple(1000);
    println!("'a = {}, b = {} and c = {} == 1000", a, b, c);
    println!("'a + b + c = {} == 1000", a + b + c);
    println!(
        "a**2 + b**2 == c**2 => {} == {}",
        a.pow(2) + b.pow(2),
        c.pow(2)
    );
    println!("a * b * c = {}", a * b * c);
}
