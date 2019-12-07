///  A014445 = Even Fibonacci numbers; or, Fibonacci(3*n).
/// ```
/// use euler::problem2::a014445;
/// let result = a014445(4);
/// assert_eq!(result, 144);
/// ```
pub fn a014445(n: u64) -> u64 {
    2 * a001076(n)
}

///  A014445 = Even Fibonacci numbers; or, Fibonacci(3*n).
/// ```
/// use euler::problem2::a014445_iter;
/// let result: u64 = a014445_iter(4_000_000).sum();
/// assert_eq!(result, 4613732);
/// ```
pub fn a014445_iter(upperbound: u64) -> impl Iterator<Item = u64> {
    a001076_iter()
        .map(|a| 2 * a)
        .take_while(move |x| *x <= upperbound)
}

///  A001076 Denominators of continued fraction convergents to sqrt(5).
///
/// a(n) = 4*a(n-1) + a(n-2), n > 1. a(0)=0, a(1)=1.
fn a001076(n: u64) -> u64 {
    let mut a = (1, 0);
    for _ in 0..n {
        a = (4 * a.0 + a.1, a.0);
    }
    a.1
}

///  A001076 Denominators of continued fraction convergents to sqrt(5).
///
/// a(n) = 4*a(n-1) + a(n-2), n > 1. a(0)=0, a(1)=1.
fn a001076_iter() -> impl Iterator<Item = u64> {
    let mut a = (1, 0);
    (0..).map(move |_| {
        let t = a.1;
        a = (4 * a.0 + a.1, a.0);
        t
    })
}

pub const A014445: [u64; 24] = [
    0,
    2,
    8,
    34,
    144,
    610,
    2_584,
    10_946,
    46_368,
    196_418,
    832_040,
    3_524_578,
    14_930_352,
    63_245_986,
    267_914_296,
    1_134_903_170,
    4_807_526_976,
    20_365_011_074,
    86_267_571_272,
    365_435_296_162,
    1_548_008_755_920,
    6_557_470_319_842,
    27_777_890_035_288,
    117_669_030_460_994,
];

#[allow(dead_code)]
fn main() {
    let expected: u64 = A014445.iter().take_while(move |&&x| x <= 4_000_000).sum();
    let result: u64 = a014445_iter(4_000_000).sum();
    println!("expected = {}, result = {}", expected, result);
}
