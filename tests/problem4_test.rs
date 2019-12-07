use euler::problem4::{
    largest_palindrome_product, largest_palindrome_product_bf, A002113Iter, A002113,
};

#[test]
fn test_a002113() {
    assert_eq!(
        A002113.to_vec(),
        A002113Iter::new(1000)
            .take(A002113.len())
            .collect::<Vec<u64>>()
    );
}

#[test]
fn test_largest_palindrome_product_size_2() {
    assert_eq!(largest_palindrome_product(2), 9009)
}

#[test]
fn test_largest_palindrome_product_size_3() {
    assert_eq!(largest_palindrome_product(3), 906_609)
}

#[test]
fn test_largest_palindrome_product_bf_size_2() {
    assert_eq!(largest_palindrome_product_bf(2), 9009);
}

#[test]
fn test_largest_palindrome_product_bf_size_3() {
    assert_eq!(largest_palindrome_product_bf(3), 906_609);
}
