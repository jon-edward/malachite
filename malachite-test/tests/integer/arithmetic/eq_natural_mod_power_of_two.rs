use common::test_properties;
use malachite_base::num::{DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo, Zero};
use malachite_nz::integer::arithmetic::eq_natural_mod_power_of_two::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_integer_and_small_unsigned,
    triples_of_integer_natural_and_small_unsigned,
    triples_of_integer_natural_and_small_unsigned_var_1,
    triples_of_integer_natural_and_small_unsigned_var_2,
};
use malachite_test::inputs::natural::triples_of_natural_natural_and_small_unsigned;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_two_neg_pos() {
    let test = |xs, ys, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_two_neg_pos(xs, ys, pow), out);
    };
    test(&[0b111_1011, 0b1_1100_1000], &[0b1_0101], 4, true);
    test(&[0b111_1011, 0b1_1100_1000], &[0b1_0101], 5, false);
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b1111_1111_1111_1111_1111_1111_1000_0101, 0b1111],
        35,
        true,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b1111_1111_1111_1111_1111_1111_1000_0101, 0b1111],
        36,
        false,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b1111_1111_1111_1111_1111_1111_1000_0101, 0b1111],
        100,
        false,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b1111_1111_1111_1111_1111_1111_1000_0101, 0b1_0111],
        37,
        true,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b1111_1111_1111_1111_1111_1111_1000_0101, 0b1_0111],
        38,
        false,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b1111_1111_1111_1111_1111_1111_1000_0101, 0b1_0111],
        100,
        false,
    );

    test(
        &[0xabcd_abcd, 0x1234_1234],
        &[0x5432_5433, 0xedcb_edcb],
        64,
        true,
    );
    test(
        &[0xabcd_abcd, 0x1234_1234],
        &[0x0000_0000, 0xedcb_edcb],
        64,
        false,
    );
    test(
        &[0xabcd_abcd, 0x1234_1234],
        &[0x5432_5433, 0xedcb_edcb],
        65,
        false,
    );
    test(
        &[0xabcd_abcd, 0x1234_1234],
        &[0x5432_5433, 0xedcb_edcb],
        128,
        false,
    );
    test(&[0, 0, 0x1234_1234], &[0, 0, 0x1234_edcc], 80, true);

    test(
        &[0x5432_5433, 0xedcb_edcb],
        &[0xabcd_abcd, 0x1234_1234],
        64,
        true,
    );
    test(
        &[0x0000_0000, 0xedcb_edcb],
        &[0xabcd_abcd, 0x1234_1234],
        64,
        false,
    );
    test(
        &[0x5432_5433, 0xedcb_edcb],
        &[0xabcd_abcd, 0x1234_1234],
        65,
        false,
    );
    test(
        &[0x5432_5433, 0xedcb_edcb],
        &[0xabcd_abcd, 0x1234_1234],
        128,
        false,
    );
    test(&[0, 0, 0x1234_edcc], &[0, 0, 0x1234_1234], 80, true);
}

#[test]
fn test_eq_natural_mod_power_of_two() {
    let test = |x, y, pow, out| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .eq_mod_power_of_two(&Natural::from_str(y).unwrap(), pow),
            out
        );
        assert_eq!(
            Natural::from_str(y)
                .unwrap()
                .eq_mod_power_of_two(&Integer::from_str(x).unwrap(), pow),
            out
        );
    };
    test("0", "256", 8, true);
    test("0", "256", 9, false);

    test("13", "21", 0, true);
    test("13", "21", 1, true);
    test("13", "21", 2, true);
    test("13", "21", 3, true);
    test("13", "21", 4, false);
    test("13", "21", 100, false);
    test("1000000000001", "1", 12, true);
    test("1000000000001", "1", 13, false);
    test("4294967295", "4294967295", 32, true);
    test("281474976710672", "844424930131984", 49, true);
    test("281474976710672", "844424930131984", 50, false);

    test("-13", "27", 0, true);
    test("-13", "27", 1, true);
    test("-13", "27", 2, true);
    test("-13", "27", 3, true);
    test("-13", "27", 4, false);
    test("-13", "27", 100, false);
    test("-1000000000001", "4095", 13, true);
    test("-1000000000001", "4095", 14, false);
    test("-1", "4294967295", 32, true);

    test("-1311693408901639117", "17135050664807912499", 64, true);
    test("-1311693408901639117", "17135050663395328000", 64, false);
    test("-1311693408901639117", "17135050664807912499", 65, false);
    test("-1311693408901639117", "17135050664807912499", 128, false);
    test(
        "-5633680281231555440641310720",
        "5634717283396403096794955776",
        80,
        true,
    );
}

#[test]
fn limbs_eq_mod_power_of_two_neg_pos_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
        |&(ref xs, ref ys, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_two_neg_pos(xs, ys, pow),
                (-Natural::from_limbs_asc(xs))
                    .eq_mod_power_of_two(&Integer::from(Natural::from_limbs_asc(ys)), pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_two_properties() {
    test_properties(
        triples_of_integer_natural_and_small_unsigned,
        |&(ref x, ref y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            assert_eq!(y.eq_mod_power_of_two(x, pow), eq_mod_power_of_two);
            assert_eq!(
                x.eq_mod_power_of_two(&Integer::from(y), pow),
                eq_mod_power_of_two
            );
            assert_eq!(
                x.mod_power_of_two(pow) == y.mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(
        triples_of_integer_natural_and_small_unsigned_var_1::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(x.eq_mod_power_of_two(y, pow));
            assert!(y.eq_mod_power_of_two(x, pow));
            assert_eq!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(
        triples_of_integer_natural_and_small_unsigned_var_2::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(!x.eq_mod_power_of_two(y, pow));
            assert!(!y.eq_mod_power_of_two(x, pow));
            assert_ne!(x.mod_power_of_two(pow), y.mod_power_of_two(pow),);
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            n.eq_mod_power_of_two(&Natural::ZERO, pow),
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(
            Natural::ZERO.eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow)
        );
    });

    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        assert!(x.eq_mod_power_of_two(y, 0));
    });

    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, pow)| {
            assert_eq!(
                x.eq_mod_power_of_two(y, pow),
                Integer::from(x).eq_mod_power_of_two(y, pow),
            );
            assert_eq!(
                x.eq_mod_power_of_two(y, pow),
                x.eq_mod_power_of_two(&Integer::from(y), pow),
            );
        },
    );
}
