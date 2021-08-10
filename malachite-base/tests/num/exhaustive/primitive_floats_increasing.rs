use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::primitive_floats_increasing;
use malachite_base::num::float::NiceFloat;

fn primitive_floats_increasing_helper<T: PrimitiveFloat>(first_20: &[T], last_20: &[T]) {
    let xs = primitive_floats_increasing::<T>();
    assert_eq!(
        xs.clone().take(20).map(NiceFloat).collect_vec(),
        first_20.iter().copied().map(NiceFloat).collect_vec()
    );
    let mut reversed = xs.rev().take(20).map(NiceFloat).collect_vec();
    reversed.reverse();
    assert_eq!(
        reversed,
        last_20.iter().copied().map(NiceFloat).collect_vec()
    );
}

#[test]
fn test_primitive_floats_increasing() {
    primitive_floats_increasing_helper::<f32>(
        &[
            f32::NEGATIVE_INFINITY,
            -3.4028235e38,
            -3.4028233e38,
            -3.402823e38,
            -3.4028229e38,
            -3.4028227e38,
            -3.4028225e38,
            -3.4028222e38,
            -3.402822e38,
            -3.4028218e38,
            -3.4028216e38,
            -3.4028214e38,
            -3.4028212e38,
            -3.402821e38,
            -3.4028208e38,
            -3.4028206e38,
            -3.4028204e38,
            -3.4028202e38,
            -3.40282e38,
            -3.4028198e38,
        ],
        &[
            3.4028198e38,
            3.40282e38,
            3.4028202e38,
            3.4028204e38,
            3.4028206e38,
            3.4028208e38,
            3.402821e38,
            3.4028212e38,
            3.4028214e38,
            3.4028216e38,
            3.4028218e38,
            3.402822e38,
            3.4028222e38,
            3.4028225e38,
            3.4028227e38,
            3.4028229e38,
            3.402823e38,
            3.4028233e38,
            3.4028235e38,
            f32::POSITIVE_INFINITY,
        ],
    );
    primitive_floats_increasing_helper::<f64>(
        &[
            f64::NEGATIVE_INFINITY,
            -1.7976931348623157e308,
            -1.7976931348623155e308,
            -1.7976931348623153e308,
            -1.7976931348623151e308,
            -1.797693134862315e308,
            -1.7976931348623147e308,
            -1.7976931348623145e308,
            -1.7976931348623143e308,
            -1.7976931348623141e308,
            -1.797693134862314e308,
            -1.7976931348623137e308,
            -1.7976931348623135e308,
            -1.7976931348623133e308,
            -1.7976931348623131e308,
            -1.797693134862313e308,
            -1.7976931348623127e308,
            -1.7976931348623125e308,
            -1.7976931348623123e308,
            -1.7976931348623121e308,
        ],
        &[
            1.7976931348623121e308,
            1.7976931348623123e308,
            1.7976931348623125e308,
            1.7976931348623127e308,
            1.797693134862313e308,
            1.7976931348623131e308,
            1.7976931348623133e308,
            1.7976931348623135e308,
            1.7976931348623137e308,
            1.797693134862314e308,
            1.7976931348623141e308,
            1.7976931348623143e308,
            1.7976931348623145e308,
            1.7976931348623147e308,
            1.797693134862315e308,
            1.7976931348623151e308,
            1.7976931348623153e308,
            1.7976931348623155e308,
            1.7976931348623157e308,
            f64::POSITIVE_INFINITY,
        ],
    );
}
