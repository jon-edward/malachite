use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen_var_1};
use std::cmp::Ordering;

fn abs_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.abs(), out);

        let mut n = n;
        n.abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
}

fn abs_primitive_float_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(NiceFloat(n.abs()), NiceFloat(out));

        let mut n = n;
        n.abs_assign();
        assert_eq!(NiceFloat(n), NiceFloat(out));
    };
    test(T::ZERO, T::ZERO);
    test(T::NEGATIVE_ZERO, T::ZERO);
    test(T::POSITIVE_INFINITY, T::POSITIVE_INFINITY);
    test(T::NEGATIVE_INFINITY, T::POSITIVE_INFINITY);
    test(T::NAN, T::NAN);
    test(T::ONE, T::ONE);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::from(100.0f32), T::from(100.0f32));
    test(T::from(-100.0f32), T::from(100.0f32));
}

#[test]
fn test_abs() {
    apply_fn_to_signeds!(abs_signed_helper);
    apply_fn_to_primitive_floats!(abs_primitive_float_helper);
}

fn abs_assign_properties_signed_helper<
    U,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() {
    signed_gen_var_1::<S>().test_properties(|n| {
        let mut abs = n;
        abs.abs_assign();
        assert_eq!(abs, n.abs());
        assert_eq!(abs.abs(), abs);
        assert_eq!(abs == n, n >= S::ZERO);
        assert_eq!(S::exact_from(n.unsigned_abs()), abs)
    });
}

fn abs_assign_properties_primitive_float_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        let mut abs = f;
        abs.abs_assign();
        assert_eq!(NiceFloat(abs), NiceFloat(f.abs()));
        assert_eq!(NiceFloat(abs.abs()), NiceFloat(abs));
        assert_eq!(NiceFloat(abs) == NiceFloat(f), f.sign() != Ordering::Less);
    });
}

#[test]
fn abs_assign_properties() {
    apply_fn_to_unsigned_signed_pairs!(abs_assign_properties_signed_helper);
    apply_fn_to_primitive_floats!(abs_assign_properties_primitive_float_helper);
}
