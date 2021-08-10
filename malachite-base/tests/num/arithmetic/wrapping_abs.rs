use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base_test_util::generators::signed_gen;

fn wrapping_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.wrapping_abs(), out);

        let mut n = n;
        n.wrapping_abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::MAX, T::MAX);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MIN);
}

#[test]
fn test_wrapping_abs() {
    apply_fn_to_signeds!(wrapping_abs_helper);
}

fn wrapping_abs_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let mut abs = n;
        abs.wrapping_abs_assign();
        assert_eq!(abs, n.wrapping_abs());
        assert_eq!(abs.wrapping_abs(), abs);
        if n != T::MIN {
            assert_eq!(n.abs(), abs);
        }
        assert_eq!(abs == n, n >= T::ZERO || n == T::MIN);
    });
}

#[test]
fn wrapping_abs_properties() {
    apply_fn_to_signeds!(wrapping_abs_properties_helper);
}
