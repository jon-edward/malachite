use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_wrapping_sub_mul() {
    fn test<T>(x: T, y: T, z: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.wrapping_sub_mul(y, z), out);

        let mut x = x;
        x.wrapping_sub_mul_assign(y, z);
        assert_eq!(x, out);
    };
    test::<u8>(100, 3, 7, 79);
    test::<u32>(60, 5, 10, 10);
    test::<u64>(1_000_000, 456, 789, 640_216);
    test::<i32>(123, -456, 789, 359_907);
    test::<i128>(-123, 456, 789, -359_907);
    test::<i8>(127, 2, 100, -73);
    test::<i8>(-127, -2, 100, 73);
    test::<i8>(-128, 1, 0, -128);

    test::<u8>(2, 10, 5, 208);
    test::<i8>(-127, 2, 100, -71);
    test::<i8>(-127, 1, 100, 29);
    test::<i8>(127, -1, 100, -29);
    test::<i8>(127, -10, 100, 103);
}