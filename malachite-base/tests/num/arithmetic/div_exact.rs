use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_div_exact() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.div_exact(y), out);

        let mut x = x;
        x.div_exact_assign(y);
        assert_eq!(x, out);
    };
    test::<u8>(0, 123, 0);
    test::<u16>(123, 1, 123);
    test::<u32>(123, 123, 1);
    test::<usize>(56_088, 123, 456);
    test::<u64>(0, 1_000_000_000_000, 0);
    test::<u128>(1_000_000_000_000, 1, 1_000_000_000_000);
    test::<usize>(1_000_000_000_000, 1_000_000_000_000, 1);
    test::<usize>(123_000_000_000_000, 1_000_000_000_000, 123);
    test::<usize>(123_000_000_000_000, 123, 1_000_000_000_000);
    test::<u128>(
        121_932_631_112_635_269_000_000,
        123_456_789_000,
        987_654_321_000,
    );
    test::<u64>(0x1_ffff_fffe, 0xffff_ffff, 2);
    test::<u64>(18_446_744_065_119_617_025, 0xffff_ffff, 0xffff_ffff);

    test::<i8>(0, -123, 0);
    test::<i16>(123, -1, -123);
    test::<i32>(123, -123, -1);
    test::<isize>(56_088, -123, -456);
    test::<i64>(0, -1_000_000_000_000, 0);
    test::<i128>(1_000_000_000_000, -1, -1_000_000_000_000);
    test::<isize>(1_000_000_000_000, -1_000_000_000_000, -1);
    test::<isize>(123_000_000_000_000, -1_000_000_000_000, -123);
    test::<isize>(123_000_000_000_000, -123, -1_000_000_000_000);
    test::<i128>(
        121_932_631_112_635_269_000_000,
        -123_456_789_000,
        -987_654_321_000,
    );
    test::<i64>(0x1_ffff_fffe, -0xffff_ffff, -2);
    test::<i128>(18_446_744_065_119_617_025, -0xffff_ffff, -0xffff_ffff);

    test::<i16>(-123, 1, -123);
    test::<i32>(-123, 123, -1);
    test::<isize>(-56_088, 123, -456);
    test::<i128>(-1_000_000_000_000, 1, -1_000_000_000_000);
    test::<isize>(-1_000_000_000_000, 1_000_000_000_000, -1);
    test::<isize>(-123_000_000_000_000, 1_000_000_000_000, -123);
    test::<isize>(-123_000_000_000_000, 123, -1_000_000_000_000);
    test::<i128>(
        -121_932_631_112_635_269_000_000,
        123_456_789_000,
        -987_654_321_000,
    );
    test::<i64>(-0x1_ffff_fffe, 0xffff_ffff, -2);
    test::<i128>(-18_446_744_065_119_617_025, 0xffff_ffff, -0xffff_ffff);

    test::<i16>(-123, -1, 123);
    test::<i32>(-123, -123, 1);
    test::<isize>(-56_088, -123, 456);
    test::<i128>(-1_000_000_000_000, -1, 1_000_000_000_000);
    test::<isize>(-1_000_000_000_000, -1_000_000_000_000, 1);
    test::<isize>(-123_000_000_000_000, -1_000_000_000_000, 123);
    test::<isize>(-123_000_000_000_000, -123, 1_000_000_000_000);
    test::<i128>(
        -121_932_631_112_635_269_000_000,
        -123_456_789_000,
        987_654_321_000,
    );
    test::<i64>(-0x1_ffff_fffe, -0xffff_ffff, 2);
    test::<i128>(-18_446_744_065_119_617_025, -0xffff_ffff, 0xffff_ffff);
}

fn div_exact_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.div_exact(T::ZERO));
    assert_panic!(T::ONE.div_exact_assign(T::ZERO));
}

#[test]
pub fn div_exact_fail() {
    apply_fn_to_primitive_ints!(div_exact_fail_helper);
}