use euler::problem3::{factor_brute_force, largest_prime_factor};

#[test]
fn test_factor_brute_force() {
    assert_eq!(factor_brute_force(13195), [5, 7, 13, 29]);
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195), 29);
}

#[test]
fn test_largest_prime_factor_for_600851475143() {
    assert_eq!(largest_prime_factor(600_851_475_143), 6857);
}
