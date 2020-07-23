use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{triples_of_signeds_var_3, triples_of_unsigneds_var_4};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_sub_mul);
    register_demo!(registry, demo_u16_sub_mul);
    register_demo!(registry, demo_u32_sub_mul);
    register_demo!(registry, demo_u64_sub_mul);
    register_demo!(registry, demo_usize_sub_mul);
    register_demo!(registry, demo_u8_sub_mul_assign);
    register_demo!(registry, demo_u16_sub_mul_assign);
    register_demo!(registry, demo_u32_sub_mul_assign);
    register_demo!(registry, demo_u64_sub_mul_assign);
    register_demo!(registry, demo_usize_sub_mul_assign);
    register_demo!(registry, demo_i8_sub_mul);
    register_demo!(registry, demo_i16_sub_mul);
    register_demo!(registry, demo_i32_sub_mul);
    register_demo!(registry, demo_i64_sub_mul);
    register_demo!(registry, demo_isize_sub_mul);
    register_demo!(registry, demo_i8_sub_mul_assign);
    register_demo!(registry, demo_i16_sub_mul_assign);
    register_demo!(registry, demo_i32_sub_mul_assign);
    register_demo!(registry, demo_i64_sub_mul_assign);
    register_demo!(registry, demo_isize_sub_mul_assign);

    register_bench!(registry, None, benchmark_u8_sub_mul);
    register_bench!(registry, None, benchmark_u16_sub_mul);
    register_bench!(registry, None, benchmark_u32_sub_mul);
    register_bench!(registry, None, benchmark_u64_sub_mul);
    register_bench!(registry, None, benchmark_usize_sub_mul);
    register_bench!(registry, None, benchmark_u8_sub_mul_assign);
    register_bench!(registry, None, benchmark_u16_sub_mul_assign);
    register_bench!(registry, None, benchmark_u32_sub_mul_assign);
    register_bench!(registry, None, benchmark_u64_sub_mul_assign);
    register_bench!(registry, None, benchmark_usize_sub_mul_assign);
    register_bench!(registry, None, benchmark_i8_sub_mul);
    register_bench!(registry, None, benchmark_i16_sub_mul);
    register_bench!(registry, None, benchmark_i32_sub_mul);
    register_bench!(registry, None, benchmark_i64_sub_mul);
    register_bench!(registry, None, benchmark_isize_sub_mul);
    register_bench!(registry, None, benchmark_i8_sub_mul_assign);
    register_bench!(registry, None, benchmark_i16_sub_mul_assign);
    register_bench!(registry, None, benchmark_i32_sub_mul_assign);
    register_bench!(registry, None, benchmark_i64_sub_mul_assign);
    register_bench!(registry, None, benchmark_isize_sub_mul_assign);
}

fn demo_sub_mul_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, z) in triples_of_unsigneds_var_4::<T>(gm).take(limit) {
        println!("{}.sub_mul({}, {}) = {}", x, y, z, x.sub_mul(y, z));
    }
}

fn demo_sub_mul_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y, z) in triples_of_signeds_var_3::<T>(gm).take(limit) {
        println!("({}).sub_mul({}, {}) = {}", x, y, z, x.sub_mul(y, z));
    }
}

fn demo_sub_mul_assign_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y, z) in triples_of_unsigneds_var_4::<T>(gm).take(limit) {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!("x := {}; x.sub_mul_assign({}, {}); x = {}", old_x, y, z, x);
    }
}

fn demo_sub_mul_assign_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y, z) in triples_of_signeds_var_3::<T>(gm).take(limit) {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!("x := {}; x.sub_mul_assign({}, {}); x = {}", old_x, y, z, x);
    }
}

fn benchmark_sub_mul_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y, z)| no_out!(x.sub_mul(y, z))))],
    );
}

fn benchmark_sub_mul_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.sub_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signeds_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y, z)| no_out!(x.sub_mul(y, z))))],
    );
}

fn benchmark_sub_mul_assign_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y, z)| x.sub_mul_assign(y, z)))],
    );
}

fn benchmark_sub_mul_assign_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.sub_mul_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signeds_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y, z)| x.sub_mul_assign(y, z)))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_sub_mul_unsigned::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_sub_mul_assign_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_sub_mul_unsigned::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_sub_mul_assign_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_sub_mul,
    demo_u8_sub_mul_assign,
    benchmark_u8_sub_mul,
    benchmark_u8_sub_mul_assign
);
unsigned!(
    u16,
    demo_u16_sub_mul,
    demo_u16_sub_mul_assign,
    benchmark_u16_sub_mul,
    benchmark_u16_sub_mul_assign
);
unsigned!(
    u32,
    demo_u32_sub_mul,
    demo_u32_sub_mul_assign,
    benchmark_u32_sub_mul,
    benchmark_u32_sub_mul_assign
);
unsigned!(
    u64,
    demo_u64_sub_mul,
    demo_u64_sub_mul_assign,
    benchmark_u64_sub_mul,
    benchmark_u64_sub_mul_assign
);
unsigned!(
    usize,
    demo_usize_sub_mul,
    demo_usize_sub_mul_assign,
    benchmark_usize_sub_mul,
    benchmark_usize_sub_mul_assign
);

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_sub_mul_signed::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_sub_mul_assign_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_sub_mul_signed::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_sub_mul_assign_signed::<$t>(gm, limit, file_name);
        }
    };
}

signed!(
    i8,
    demo_i8_sub_mul,
    demo_i8_sub_mul_assign,
    benchmark_i8_sub_mul,
    benchmark_i8_sub_mul_assign
);
signed!(
    i16,
    demo_i16_sub_mul,
    demo_i16_sub_mul_assign,
    benchmark_i16_sub_mul,
    benchmark_i16_sub_mul_assign
);
signed!(
    i32,
    demo_i32_sub_mul,
    demo_i32_sub_mul_assign,
    benchmark_i32_sub_mul,
    benchmark_i32_sub_mul_assign
);
signed!(
    i64,
    demo_i64_sub_mul,
    demo_i64_sub_mul_assign,
    benchmark_i64_sub_mul,
    benchmark_i64_sub_mul_assign
);
signed!(
    isize,
    demo_isize_sub_mul,
    demo_isize_sub_mul_assign,
    benchmark_isize_sub_mul,
    benchmark_isize_sub_mul_assign
);
