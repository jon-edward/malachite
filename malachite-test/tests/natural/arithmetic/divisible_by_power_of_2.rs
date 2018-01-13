use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::natural::arithmetic::divisible_by_power_of_2::select_inputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use std::str::FromStr;

#[test]
fn test_divisible_by_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().divisible_by_power_of_2(pow),
            out
        );
    };
    test("0", 0, true);
    test("0", 10, true);
    test("0", 100, true);
    test("123", 0, true);
    test("123", 1, false);
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
}

#[test]
fn divisible_by_power_of_2_properties() {
    // if x != 0, x.divisible_by_power_of_2(pow) == (x.trailing_zeros().unwrap() >= pow)
    // (-x).divisible_by_power_of_2(pow) == x.divisible_by_power_of_2()
    // (x << pow).divisible_by_power_of_2(pow)
    // x.divisible_by_power_of_2(pow) == (x >> pow << pow == x)
    let natural_and_u32 = |x: Natural, pow: u32| {
        let divisible = x.divisible_by_power_of_2(pow);
        if x != 0 {
            assert_eq!(x.trailing_zeros().unwrap() >= pow.into(), divisible);
        }
        assert_eq!((-&x).divisible_by_power_of_2(pow), divisible);
        assert!((&x << pow as u32).divisible_by_power_of_2(pow));
        assert_eq!(&x >> pow << pow == x, divisible);
    };

    // x.divisible_by_power_of_2(0)
    let one_natural = |x: Natural| {
        assert!(x.divisible_by_power_of_2(0));
    };

    // 0.divisible_by_power_of_2(pow)
    let one_u32 = |pow: u32| {
        assert!(Natural::ZERO.divisible_by_power_of_2(pow));
    };

    for (x, pow) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(x, pow);
    }

    for (x, pow) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(x, pow);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(n);
    }
}