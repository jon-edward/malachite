use common::GenerationMode;
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, exhaustive_natural_integers,
                                       random_integers, random_natural_integers};
use rust_wheels::iterators::integers_geometric::{i32s_geometric, natural_u32s_geometric};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::{exhaustive_i, exhaustive_u};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples, exhaustive_triples_from_single,
                                     lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
                                     random_triples, random_triples_from_single};

pub fn integers(gm: GenerationMode) -> Box<Iterator<Item = Integer>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_integers()),
        GenerationMode::Random(scale) => Box::new(random_integers(&EXAMPLE_SEED, scale)),
    }
}

pub fn pairs_of_integers(gm: GenerationMode) -> Box<Iterator<Item = (Integer, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_integers())),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(random_integers(
            &EXAMPLE_SEED,
            scale,
        ))),
    }
}

pub fn triples_of_integers(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_integers()))
        }
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(random_integers(
            &EXAMPLE_SEED,
            scale,
        ))),
    }
}

pub fn natural_integers(gm: GenerationMode) -> Box<Iterator<Item = Integer>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_natural_integers()),
        GenerationMode::Random(scale) => Box::new(random_natural_integers(&EXAMPLE_SEED, scale)),
    }
}

fn random_pairs_of_integer_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

fn random_pairs_of_primitive_and_integer<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Integer)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

pub fn pairs_of_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_i()))
        }
        GenerationMode::Random(scale) => random_pairs_of_integer_and_primitive(scale),
    }
}

pub fn pairs_of_signed_and_integer<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_i(), exhaustive_integers()))
        }
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_integer(scale),
    }
}

pub fn pairs_of_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_u()))
        }
        GenerationMode::Random(scale) => random_pairs_of_integer_and_primitive(scale),
    }
}

pub fn pairs_of_unsigned_and_integer<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_u(), exhaustive_integers()))
        }
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_integer(scale),
    }
}

fn random_triples_of_integer_integer_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn triples_of_integer_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_i(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_integer_and_primitive(scale),
    }
}

pub fn triples_of_integer_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_integer_and_primitive(scale),
    }
}

fn log_pairs_of_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_u()))
}

pub fn pairs_of_integer_and_small_u32(gm: GenerationMode) -> Box<Iterator<Item = (Integer, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        )),
    }
}

pub fn pairs_of_integer_and_small_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32)>> {
    Box::new(pairs_of_integer_and_small_u32(gm).map(|(n, u)| (n << u, u)))
}

pub fn pairs_of_integer_and_small_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32)>> {
    Box::new(pairs_of_integer_and_small_u32(gm).filter(|&(ref n, u)| !n.divisible_by_power_of_2(u)))
}

pub fn pairs_of_integer_and_small_u64(gm: GenerationMode) -> Box<Iterator<Item = (Integer, u64)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
        )),
    }
}

pub fn triples_of_integer_small_u32_and_small_u32(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, u32)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_integers(),
            exhaustive_pairs_from_single(exhaustive_u()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        )),
    }
}

fn log_pairs_of_integer_and_signed<T: 'static + PrimitiveSigned>(
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_i()))
}

pub fn pairs_of_integer_and_small_i32(gm: GenerationMode) -> Box<Iterator<Item = (Integer, i32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_signed(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        )),
    }
}

fn random_triples_of_integer_primitive_and_integer<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

fn random_triples_of_primitive_integer_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn triples_of_integer_unsigned_and_integer<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_u(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_primitive_and_integer(scale),
    }
}

pub fn triples_of_unsigned_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_u(),
            exhaustive_integers(),
            exhaustive_u(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_integer_and_primitive(scale),
    }
}

pub fn triples_of_integer_signed_and_integer<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_i(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_primitive_and_integer(scale),
    }
}

pub fn triples_of_signed_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_i(),
            exhaustive_integers(),
            exhaustive_i(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_integer_and_primitive(scale),
    }
}

pub fn pairs_of_integer_and_natural(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_integers(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
        )),
    }
}

pub fn pairs_of_natural_and_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
        )),
    }
}

pub fn pairs_of_natural_and_natural_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_natural_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_natural_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_integer_natural_and_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Natural, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_naturals(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_natural_integer_and_natural(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Integer, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_integers(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
        )),
    }
}

pub fn triples_of_integer_small_u64_and_bool(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u64, bool)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_integers(), exhaustive_u()),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random_x(seed)),
        )),
    }
}

pub fn pairs_of_integer_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_integers(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

fn triples_of_integer_small_i32_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_integer_and_signed(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn triples_of_integer_small_i32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i >= 0 || rm != RoundingMode::Exact
                || n.divisible_by_power_of_2(i.wrapping_neg() as u32)
        }),
    )
}

pub fn triples_of_integer_small_i32_and_rounding_mode_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
    )
}

fn triples_of_integer_small_u32_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_integer_and_unsigned(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn triples_of_integer_small_u32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_u32_and_rounding_mode(gm)
            .filter(|&(ref n, u, rm)| rm != RoundingMode::Exact || n.divisible_by_power_of_2(u)),
    )
}