use common::GenerationMode;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

type It = Iterator<Item = Natural>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_naturals())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_naturals(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_not(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

pub fn demo_natural_not_ref(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

pub fn benchmark_natural_not(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} !Natural", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|n: Natural| !n),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "-Natural",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_not_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} !Natural evaluation strategy", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: Natural| !n),
        function_g: &(|n: Natural| !&n),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "-Natural",
        g_name: "-\\\\&Natural",
        title: "-Natural evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}