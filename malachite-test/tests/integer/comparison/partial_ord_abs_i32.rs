use common::LARGE_LIMIT;
use malachite_base::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::comparison::partial_ord_abs_i32::select_inputs_1;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_i32_abs() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            PartialOrdAbs::partial_cmp_abs(&v, &Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Equal));
    test("-123", -123, Some(Ordering::Equal));
    test("-123", -122, Some(Ordering::Greater));
    test("-123", -124, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", -123, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_i32_properties() {
    // n.partial_cmp_abs(&Integer::from(i)) is equivalent to n.partial_cmp_abs(&i).
    // n.partial_cmp_abs(&i) == n.abs().partial_cmp(&i.abs())
    //
    // Integer::from(i).partial_cmp_abs(&n) is equivalent to i.partial_cmp_abs(&n).
    // i.partial_cmp_abs(&n) == i.abs().partial_cmp(&n.abs())
    //
    // n.lt_abs(u) <=> u.gt_abs(n) and n.gt_abs(u) <=> u.lt_abs(n).
    let integer_and_i32 = |n: Integer, i: i32| {
        let cmp_1 = n.partial_cmp_abs(&i);
        assert_eq!(n.partial_cmp_abs(&Integer::from(i)), cmp_1);
        assert_eq!(n.abs_ref().partial_cmp(&(i.wrapping_abs() as u32)), cmp_1);

        let cmp_2 = PartialOrdAbs::partial_cmp_abs(&i, &n);
        assert_eq!(Integer::from(i).partial_cmp_abs(&n), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!((i.wrapping_abs() as u32).partial_cmp(&n.abs_ref()), cmp_2);
    };

    // n.lt_abs(i) and i.lt_abs(m) => n.lt_abs(m)
    // n.gt_abs(i) and i.gt_abs(m) => n.gt_abs(m)
    let integer_i32_and_integer = |n: Integer, i: i32, m: Integer| {
        if n.lt_abs(&i) && PartialOrdAbs::lt_abs(&i, &m) {
            assert!(n.lt_abs(&m));
        } else if n.gt_abs(&i) && PartialOrdAbs::gt_abs(&i, &m) {
            assert!(n.gt_abs(&m));
        }
    };

    // i.lt_abs(n) and n.lt_abs(j) => i < j
    // i.gt_abs(n) and n.gt_abs(j) => i > j
    let i32_integer_and_i32 = |i: i32, n: Integer, j: i32| {
        if PartialOrdAbs::lt_abs(&i, &n) && n.lt_abs(&j) {
            assert!((i.wrapping_abs() as u32) < (j.wrapping_abs() as u32));
        } else if PartialOrdAbs::gt_abs(&i, &n) && n.gt_abs(&j) {
            assert!((i.wrapping_abs() as u32) > (j.wrapping_abs() as u32));
        }
    };

    for (n, i) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i, m) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_i::<i32>(),
        exhaustive_integers(),
    ).take(LARGE_LIMIT)
    {
        integer_i32_and_integer(n, i, m);
    }

    for (n, i, m) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_i32_and_integer(n, i, m);
    }

    for (i, n, j) in exhaustive_triples(
        exhaustive_i::<i32>(),
        exhaustive_integers(),
        exhaustive_i::<i32>(),
    ).take(LARGE_LIMIT)
    {
        i32_integer_and_i32(i, n, j);
    }

    for (i, n, j) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(LARGE_LIMIT)
    {
        i32_integer_and_i32(i, n, j);
    }
}