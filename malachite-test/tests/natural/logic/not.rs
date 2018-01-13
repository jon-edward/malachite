use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rugint_integer, rugint_integer_to_integer, GenerationMode};
use malachite_test::natural::logic::not::select_inputs;
use rugint;
use std::str::FromStr;

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rugint::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("1000000000000", "-1000000000001");
    test("2147483647", "-2147483648");
}

#[test]
fn not_properties() {
    // !x is equivalent for malachite and rugint.
    // !x is valid.
    //
    // !&x is equivalent for malachite and rugint.
    // !&x is valid.
    // !x and !&x are equivalent.
    //
    // !x < 0
    // !x == !(x.to_integer())
    // !x != x
    // !!x == x
    let one_natural = |x: Natural| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rugint_not = !natural_to_rugint_integer(&x);
        assert_eq!(rugint_integer_to_integer(&rugint_not), not);

        let not_2 = !&x;
        assert!(not_2.is_valid());

        assert_eq!(not_2, not);

        assert!(not < 0);
        assert_eq!(!x.to_integer(), not);
        assert_ne!(not, x);
        assert_eq!(!&not, x);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}