use euler::problem10::{sieve_of_eratosthenes, sum_primes_below, sum_primes_below2};

#[test]
fn test_sum_primes_below_10_should_be_17() {
    assert_eq!(sum_primes_below(10), 17);
}

#[test]
#[ignore]
fn test_sum_primes_below_2_000_000_should_be_142_913_828_922() {
    assert_eq!(sum_primes_below(2_000_000), 142_913_828_922);
}

#[test]
fn test_sum_primes_below2_10_should_be_17() {
    assert_eq!(sum_primes_below2(10), 17);
}

#[test]
#[ignore]
fn test_sum_primes_below2_2_000_000_should_be_142_913_828_922() {
    assert_eq!(sum_primes_below2(2_000_000), 142_913_828_922);
}

#[test]
fn test_sieve_of_eratosthenes_10_should_be_17() {
    assert_eq!(sieve_of_eratosthenes(10), 17);
}

#[test]
fn test_sieve_of_eratosthenes_2_000_000_should_be_142_913_828_922() {
    assert_eq!(sieve_of_eratosthenes(2_000_000), 142_913_828_922);
}
