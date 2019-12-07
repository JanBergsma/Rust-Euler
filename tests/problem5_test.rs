use euler::problem5::{smallest_multiple, smallest_multiple_bf};

macro_rules! smallest_multiple_tests {
    ($func:ident, $($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, $func(input));
        }
    )*
    }
}

smallest_multiple_tests! {
    smallest_multiple,
    smallest_multiple_of_10_should_be_2520: (10, 2520),
    smallest_multiple_of_20_should_be_232792560: (20, 232_792_560),
}

smallest_multiple_tests! {
    smallest_multiple_bf,
    smallest_multiple_brute_force_of_10_should_be_2520: (10, 2520),
    smallest_multiple_brute_force_of_20_should_be_232792560: (20, 232_792_560),
}
