use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_square);
    register_unsigned_demos!(runner, demo_mod_square_assign);
    register_unsigned_benches!(runner, benchmark_mod_square);
    register_unsigned_benches!(runner, benchmark_mod_square_assign);
    register_unsigned_benches!(runner, benchmark_mod_square_precomputed_algorithms);
}

fn demo_mod_square<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, m) in unsigned_pair_gen_var_16::<T>().get(gm, &config).take(limit) {
        println!("{}.square() ≡ {} mod {}", x, x.mod_square(m), m);
    }
}

fn demo_mod_square_assign<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, m) in unsigned_pair_gen_var_16::<T>().get(gm, &config).take(limit) {
        let old_x = x;
        x.mod_square_assign(m);
        println!("x := {}; x.mod_square_assign({}); x = {}", old_x, m, x);
    }
}

fn benchmark_mod_square<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_square({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_16::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [("Malachite", &mut |(x, m)| no_out!(x.mod_square(m)))],
    );
}

fn benchmark_mod_square_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_square({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_16::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut x, m)| x.mod_square_assign(m))],
    );
}

fn benchmark_mod_square_precomputed_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_square({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_16::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, m)| {
                for _ in 0..10 {
                    x.mod_square(m);
                }
            }),
            ("precomputed", &mut |(x, m)| {
                let data = T::precompute_mod_pow_data(&m);
                for _ in 0..10 {
                    x.mod_square_precomputed(m, &data);
                }
            }),
        ],
    );
}
