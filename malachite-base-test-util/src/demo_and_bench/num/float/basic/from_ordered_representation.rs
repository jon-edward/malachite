use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_gen_var_13;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_from_ordered_representation);
    register_primitive_float_benches!(runner, benchmark_from_ordered_representation);
}

fn demo_from_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_13::<T>().get(gm, &config).take(limit) {
        println!(
            "from_ordered_representation({}) = {}",
            x,
            NiceFloat(T::from_ordered_representation(x))
        );
    }
}

fn benchmark_from_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_ordered_representation(u64)", T::NAME,),
        BenchmarkType::Single,
        unsigned_gen_var_13::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| {
            no_out!(T::from_ordered_representation(x))
        })],
    );
}
