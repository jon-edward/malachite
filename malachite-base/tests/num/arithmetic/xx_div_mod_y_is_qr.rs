use malachite_base::num::arithmetic::xx_div_mod_y_is_qr::_explicit_xx_div_mod_y_is_qr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{unsigned_gen_var_1, unsigned_triple_gen_var_15};
use std::panic::catch_unwind;

#[test]
fn test_xx_div_mod_y_is_qr() {
    fn test<T: PrimitiveUnsigned>(x_1: T, x_0: T, y: T, q: T, r: T) {
        assert_eq!(T::xx_div_mod_y_is_qr(x_1, x_0, y), (q, r));
        assert_eq!(_explicit_xx_div_mod_y_is_qr(x_1, x_0, y), (q, r));
    }
    test::<u8>(0, 0, 1, 0, 0);
    test::<u32>(0, 1, 1, 1, 0);
    test::<u16>(1, 0, 2, 0x8000, 0);
    test::<u16>(1, 7, 2, 0x8003, 1);
    test::<u8>(0x78, 0x9a, 0xbc, 0xa4, 0x2a);
    test::<u64>(0x12, 0x34, 0x33, 0x5a5a5a5a5a5a5a5b, 0x13);
}

fn xx_div_mod_y_is_qr_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::xx_div_mod_y_is_qr(
        T::exact_from(3),
        T::exact_from(5),
        T::ZERO
    ));
    assert_panic!(T::xx_div_mod_y_is_qr(
        T::exact_from(3),
        T::exact_from(5),
        T::TWO
    ));
}

#[test]
fn xx_div_mod_y_is_qr_fail() {
    apply_fn_to_unsigneds!(xx_div_mod_y_is_qr_fail_helper);
}

fn xx_div_mod_y_is_qr_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_15::<T, T>().test_properties(|(x_1, x_0, y)| {
        let (q, r) = T::xx_div_mod_y_is_qr(x_1, x_0, y);
        assert_eq!(_explicit_xx_div_mod_y_is_qr(x_1, x_0, y), (q, r));

        assert!(r < y);
        let (product_1, product_0) = T::x_mul_y_is_zz(q, y);
        assert_eq!(
            T::xx_add_yy_is_zz(product_1, product_0, T::ZERO, r),
            (x_1, x_0)
        );
    });

    unsigned_gen_var_1::<T>().test_properties(|a| {
        assert_eq!(
            T::xx_div_mod_y_is_qr(T::ZERO, T::ZERO, a),
            (T::ZERO, T::ZERO)
        );
        assert_eq!(T::xx_div_mod_y_is_qr(T::ZERO, a, a), (T::ONE, T::ZERO));
    });
}

#[test]
fn xx_div_mod_y_is_qr_properties() {
    apply_fn_to_unsigneds!(xx_div_mod_y_is_qr_properties_helper);
}
