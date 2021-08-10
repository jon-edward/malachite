use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_negative_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};

fn random_negative_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    let xs = random_negative_primitive_floats::<T>(EXAMPLE_SEED);
    let actual_values = xs.clone().take(20).map(NiceFloat).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone().map(NiceFloat));
    let actual_median = median(xs.clone().map(NiceFloat).take(1000000));
    let actual_moment_stats = moment_stats(xs.take(1000000));
    let (lo, hi) = expected_median;
    assert_eq!(
        (
            actual_values,
            actual_common_values.as_slice(),
            actual_median,
            actual_moment_stats
        ),
        (
            expected_values.iter().cloned().map(NiceFloat).collect_vec(),
            expected_common_values
                .iter()
                .map(|&(x, freq)| (NiceFloat(x), freq))
                .collect_vec()
                .as_slice(),
            (NiceFloat(lo), hi.map(NiceFloat)),
            expected_moment_stats
        )
    );
}

#[test]
fn test_random_negative_primitive_floats() {
    // f32
    let values = &[
        -2.3484665e-27,
        -0.010641627,
        -5.8060587e-9,
        -2.8182444e-31,
        -10462.533,
        -821.13,
        -6.3031636e33,
        -9.5037605e-15,
        -4.956113e-11,
        -8.565164e-22,
        -6667.2495,
        -8.6328766e18,
        -1222.5242,
        -2.825929e25,
        -8.408058e32,
        -35.376762,
        -5.331246e-35,
        -1.1431577e-12,
        -5.8021117e-15,
        -4.257878e-14,
    ];
    let common_values = &[
        (-10.31897, 2),
        (-539.9892, 2),
        (-60148.51, 2),
        (-7.546873, 2),
        (-7606.816, 2),
        (-776.3078, 2),
        (-1.3261846, 2),
        (-12496.083, 2),
        (-2.5221734, 2),
        (-2.9153142, 2),
    ];
    let sample_median = (-1.6313587, Some(-1.6313428));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.9935687709301062e36),
        standard_deviation: NiceFloat(1.8605031273250459e37),
        skewness: NiceFloat(-12.384332949727417),
        excess_kurtosis: NiceFloat(170.5284612507418),
    };
    random_negative_primitive_floats_helper::<f32>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64
    let values = &[
        -1.447005502560662e146,
        -6.24103868969363e-139,
        -3.2354168462712154e118,
        -2.848605957613258e243,
        -1.3007593490666566e274,
        -1.2678029865970891e-253,
        -6.298463311550518e114,
        -2.7114891905539105e49,
        -11845642624.881704,
        -6.049786340671397e-245,
        -9.268165817801776e-308,
        -7.20496059298362e278,
        -3.1529837422468573e-31,
        -2.908035118376895e-22,
        -5.397630931761669e186,
        -8.140466543599512e145,
        -1.5709219762031945e-140,
        -1.0805614483727636e203,
        -2.6257467421064508e300,
        -3.9223586797654796e20,
    ];
    let common_values = &[
        (-8417229.77625, 1),
        (-2.510958896e45, 1),
        (-3208093286.102, 1),
        (-362.2025556776, 1),
        (-46.55187627446, 1),
        (-466061517.8385, 1),
        (-5.171759573816, 1),
        (-6666772552.184, 1),
        (-675858.2909992, 1),
        (-732009733280.7, 1),
    ];
    let sample_median = (-1.5131202036927989, Some(-1.5119179328033963));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.324453738346271e305),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_negative_primitive_floats_helper::<f64>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}
