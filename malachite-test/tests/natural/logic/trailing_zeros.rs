use common::test_properties;
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::vecs_of_u32_var_1;
use malachite_test::inputs::natural::naturals;
use malachite_test::natural::logic::trailing_zeros::natural_trailing_zeros_alt;
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_trailing_zeros() {
    let test = |limbs, out| {
        assert_eq!(limbs_trailing_zeros(limbs), out);
    };
    test(&[4], 2);
    test(&[0, 4], 34);
    test(&[1, 2, 3], 0);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_trailing_zeros_fail_1() {
    limbs_trailing_zeros(&[]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn limbs_trailing_zeros_fail_2() {
    limbs_trailing_zeros(&[0, 0, 0]);
}

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().trailing_zeros(), out);
        assert_eq!(
            natural_trailing_zeros_alt(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", None);
    test("123", Some(0));
    test("1000000000000", Some(12));
    test("4294967295", Some(0));
    test("4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
}

#[test]
fn limbs_trailing_zeros_properties() {
    test_properties(vecs_of_u32_var_1, |limbs| {
        assert_eq!(
            Some(limbs_trailing_zeros(limbs)),
            Natural::from_limbs_asc(limbs).trailing_zeros()
        );
    });
}

#[test]
fn trailing_zeros_properties() {
    test_properties(naturals, |x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(natural_trailing_zeros_alt(x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), *x == 0);
        if *x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            if trailing_zeros <= u64::from(u32::MAX) {
                let trailing_zeros = trailing_zeros as u32;
                assert!((x >> trailing_zeros).is_odd());
                assert_eq!(x >> trailing_zeros << trailing_zeros, *x);
            }
        }
    });
}
