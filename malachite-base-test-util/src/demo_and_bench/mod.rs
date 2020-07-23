use malachite_base_test_util::runner::Runner;

macro_rules! unsigned_single_arg_demo {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
            for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
                println!(concat!("{}.", stringify!($f), "() = {}"), u, u.$f());
            }
        }
    };
}

macro_rules! signed_single_arg_demo {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
            for i in signed_gen::<T>().get(gm, &config).take(limit) {
                println!(concat!("({}).", stringify!($f), "() = {}"), i, i.$f());
            }
        }
    };
}

macro_rules! unsigned_single_arg_bench {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveUnsigned>(
            gm: GenMode,
            config: GenConfig,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("{}.significant_bits()", T::NAME),
                BenchmarkType::Single,
                unsigned_gen::<T>().get(gm, &config),
                gm.name(),
                limit,
                file_name,
                &(|&u| usize::exact_from(u.significant_bits())),
                "u",
                &mut [("malachite", &mut (|u| no_out!(u.$f())))],
            );
        }
    };
}

macro_rules! signed_single_arg_bench {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveSigned>(
            gm: GenMode,
            config: GenConfig,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("{}.significant_bits()", T::NAME),
                BenchmarkType::Single,
                signed_gen::<T>().get(gm, &config),
                gm.name(),
                limit,
                file_name,
                &(|&i| usize::exact_from(i.significant_bits())),
                "i",
                &mut [("malachite", &mut (|i| no_out!(i.$f())))],
            );
        }
    };
}

pub(crate) fn register(runner: &mut Runner) {
    bools::register(runner);
    num::register(runner);
}

pub mod bools;
pub mod num;
