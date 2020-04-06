use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_nz::natural::logic::low_mask::limbs_low_mask;
use malachite_nz::natural::Natural;

use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_limbs_low_mask);
    register_ns_demo!(registry, demo_natural_low_mask);
    register_ns_bench!(registry, Small, benchmark_limbs_low_mask);
    register_ns_bench!(registry, Large, benchmark_natural_low_mask_algorithms);
}

fn demo_limbs_low_mask(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        println!("limbs_low_mask({}) = {:?}", bits, limbs_low_mask(bits));
    }
}

fn demo_natural_low_mask(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        println!("Natural::low_mask({}) = {}", bits, Natural::low_mask(bits));
    }
}

fn benchmark_limbs_low_mask(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        &format!("limbs_low_mask(u64)"),
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [("malachite", &mut (|bits| no_out!(limbs_low_mask(bits))))],
    );
}

fn benchmark_natural_low_mask_algorithms(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("Natural.low_mask(u64)"),
        BenchmarkType::Algorithms,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [
            (
                "Natural.low_mask(u64)",
                &mut (|bits| no_out!(Natural::low_mask(bits))),
            ),
            (
                "Natural.power_of_two(u64) - 1",
                &mut (|bits| no_out!(Natural::power_of_two(bits) - Natural::ONE)),
            ),
        ],
    );
}