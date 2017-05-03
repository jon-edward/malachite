use malachite_native as native;
use malachite_gmp as gmp;
use std::str::FromStr;

#[test]
fn test_eq_integer() {
    let test = |u, v, out| {
        assert_eq!(native::natural::Natural::from_str(u).unwrap() ==
                   native::integer::Integer::from_str(v).unwrap(),
                   out);
        assert_eq!(gmp::natural::Natural::from_str(u).unwrap() ==
                   gmp::integer::Integer::from_str(v).unwrap(),
                   out);
    };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("123", "-123", false);
    test("123", "5", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("1000000000000", "-1000000000000", false);
}
