use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_even_unsigned);
    register_signed_demos!(runner, demo_even_signed);
    register_unsigned_demos!(runner, demo_odd_unsigned);
    register_signed_demos!(runner, demo_odd_signed);

    register_unsigned_benches!(runner, benchmark_even_unsigned);
    register_signed_benches!(runner, benchmark_even_signed);
    register_unsigned_benches!(runner, benchmark_odd_unsigned);
    register_signed_benches!(runner, benchmark_odd_signed);
}

fn demo_even_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        if u.even() {
            println!("{} is even", u);
        } else {
            println!("{} is not even", u);
        }
    }
}

fn demo_even_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        if i.even() {
            println!("{} is even", i);
        } else {
            println!("{} is not even", i);
        }
    }
}

fn demo_odd_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        if u.odd() {
            println!("{} is odd", u);
        } else {
            println!("{} is not od", u);
        }
    }
}

fn demo_odd_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        if i.odd() {
            println!("{} is odd", i);
        } else {
            println!("{} is not odd", i);
        }
    }
}

fn benchmark_even_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.even()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.even()))],
    );
}

fn benchmark_even_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.even()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.even()))],
    );
}

fn benchmark_odd_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.odd()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.odd()))],
    );
}

fn benchmark_odd_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.odd()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.odd()))],
    );
}
