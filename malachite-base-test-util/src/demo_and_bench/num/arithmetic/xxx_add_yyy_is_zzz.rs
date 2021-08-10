use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::sextuple_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_sextuple_gen_var_1;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_xxx_add_yyy_is_zzz);
    register_unsigned_benches!(runner, benchmark_xxx_add_yyy_is_zzz);
}

fn demo_xxx_add_yyy_is_zzz<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x_2, x_1, x_0, y_2, y_1, y_0) in unsigned_sextuple_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "[{}, {}, {}] + [{}, {}, {}] = {:?}",
            x_2,
            x_1,
            x_0,
            y_2,
            y_1,
            y_0,
            T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0)
        );
    }
}

fn benchmark_xxx_add_yyy_is_zzz<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::xxx_add_yyy_is_zzz({}, {}, {}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_sextuple_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &sextuple_max_bit_bucketer("x_2", "x_1", "x_0", "y_2", "y_1", "y_0"),
        &mut [("default", &mut |(x_2, x_1, x_0, y_2, y_1, y_0)| {
            no_out!(T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0))
        })],
    );
}
