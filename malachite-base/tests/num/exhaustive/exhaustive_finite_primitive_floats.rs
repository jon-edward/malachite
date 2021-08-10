use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_finite_primitive_floats;
use malachite_base::num::float::NiceFloat;

fn exhaustive_finite_primitive_floats_helper<T: PrimitiveFloat>(out: &[T]) {
    assert_eq!(
        exhaustive_finite_primitive_floats::<T>()
            .take(50)
            .map(NiceFloat)
            .collect_vec(),
        out.iter().copied().map(NiceFloat).collect_vec()
    );
}

#[test]
fn test_exhaustive_finite_primitive_floats() {
    exhaustive_finite_primitive_floats_helper::<f32>(&[
        0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75,
        4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5,
        1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625, -0.625,
        1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0,
    ]);
    exhaustive_finite_primitive_floats_helper::<f64>(&[
        0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75,
        4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5,
        1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625, -0.625,
        1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0,
    ]);
}
