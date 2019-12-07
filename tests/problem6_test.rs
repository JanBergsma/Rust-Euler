use euler::problem6::{sum_square_difference, sum_square_difference_bf};

#[test]
fn test_sum_square_difference_for_10_should_be_240() {
    assert_eq!(sum_square_difference(10), 2640);
}

#[test]
fn test_sum_square_difference_bf_for_10_should_be_240() {
    assert_eq!(sum_square_difference_bf(10), 2640);
}

// todo how to do this rust!
// TESTS = list((f'n = {n}', n) for n in range(101))

// @pytest.mark.parametrize(
//     "name, n",
//     TESTS, ids=[test[0] for test in TESTS])
// def test_stress(name, n):
//     assert sum_square_difference(n) == sum_square_difference_bf(n)

#[test]
fn test_stress1() {
    assert!((0..=100).all(|n| sum_square_difference(n) == sum_square_difference_bf(n)));
}
