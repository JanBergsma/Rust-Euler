use euler::problem9::find_pythagorean_triple;

#[test]
fn test_product_of_pytagorian_triplet_which_sum_of_a_b_c_is_1000_should_be_31875000() {
    let (a, b, c) = find_pythagorean_triple(1000);
    assert_eq!(a + b + c, 1000);
    assert_eq!(a.pow(2) + b.pow(2), c.pow(2));
    assert_eq!(a * b * c, 31_875_000);
}
