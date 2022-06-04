use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen_var_8;
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_from_float);
    register_primitive_float_benches!(runner, benchmark_rational_from_float);
    register_bench!(runner, benchmark_rational_from_f32_library_comparison);
    register_bench!(runner, benchmark_rational_from_f64_library_comparison);
}

fn demo_rational_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: From<T>,
{
    for f in primitive_float_gen_var_8::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("Rational::from({}) = {}", NiceFloat(f), Rational::from(f));
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: From<T>,
{
    run_benchmark(
        &format!("Rational::from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_8::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Rational::from(f)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_f32_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(f32)",
        BenchmarkType::LibraryComparison,
        primitive_float_gen_var_8::<f32>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("Malachite", &mut |f| no_out!(Rational::from(f))),
            ("rug", &mut |f| no_out!(rug::Rational::from_f32(f))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_f64_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(f64)",
        BenchmarkType::LibraryComparison,
        primitive_float_gen_var_8::<f64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("Malachite", &mut |f| no_out!(Rational::from(f))),
            ("rug", &mut |f| no_out!(rug::Rational::from_f64(f))),
        ],
    );
}