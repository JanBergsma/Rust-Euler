use euler::problem1::{sum_of_multiples_of_3_or_5, sum_of_multiples_of_3_or_5_brute_force};

#[test]
fn test_sum_of_multiples_of_3_or_5() {
    let ar = [(10u64, 23u64), (1000u64, 233_168u64)];
    for (n, expected) in ar.iter() {
        let result = sum_of_multiples_of_3_or_5(*n);
        assert_eq!(
            result, *expected,
            "sum_of_muliiples_of_or_5({}) should be {}!",
            n, expected
        );
    }
}
#[test]
fn test_sum_of_multiples_of_3_or_5_brute_force() {
    let ar = [(10u64, 23u64), (1000u64, 233_168u64)];
    for (n, expected) in ar.iter() {
        let result = sum_of_multiples_of_3_or_5_brute_force(*n);
        assert_eq!(
            result, *expected,
            "sum_of_multiples_of_3_or_5_brute_force({}) should be {}!",
            n, expected
        );
    }
}

#[test]
fn test_against_brute_force() {
    for n in 1..1000 {
        let som = sum_of_multiples_of_3_or_5(n);
        let brute = sum_of_multiples_of_3_or_5_brute_force(n);
        assert_eq!(
            som, brute,
            "sum({}) = {} and bruteforce({})={}!",
            n, som, n, brute
        );
    }
}
