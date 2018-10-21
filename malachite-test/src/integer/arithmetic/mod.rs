use common::DemoBenchRegistry;

pub mod abs;
pub mod add;
pub mod add_i32;
pub mod add_mul;
pub mod add_mul_i32;
pub mod add_mul_u32;
pub mod add_u32;
pub mod div_exact_u32;
pub mod div_mod_u32;
pub mod div_round_u32;
pub mod div_u32;
pub mod divisible_by_power_of_two;
pub mod divisible_by_u32;
pub mod eq_i32_mod_power_of_two;
pub mod eq_mod_power_of_two;
pub mod eq_u32_mod_power_of_two;
pub mod mod_power_of_two;
pub mod mod_u32;
pub mod mul;
pub mod mul_i32;
pub mod mul_u32;
pub mod neg;
pub mod parity;
pub mod shl_i;
pub mod shl_u;
pub mod shr_i;
pub mod shr_u;
pub mod sub;
pub mod sub_i32;
pub mod sub_mul;
pub mod sub_mul_i32;
pub mod sub_mul_u32;
pub mod sub_u32;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs::register(registry);
    add::register(registry);
    add_i32::register(registry);
    add_u32::register(registry);
    add_mul::register(registry);
    add_mul_i32::register(registry);
    add_mul_u32::register(registry);
    div_exact_u32::register(registry);
    div_mod_u32::register(registry);
    div_round_u32::register(registry);
    div_u32::register(registry);
    divisible_by_power_of_two::register(registry);
    divisible_by_u32::register(registry);
    eq_mod_power_of_two::register(registry);
    eq_i32_mod_power_of_two::register(registry);
    eq_u32_mod_power_of_two::register(registry);
    mod_power_of_two::register(registry);
    mod_u32::register(registry);
    mul::register(registry);
    mul_i32::register(registry);
    mul_u32::register(registry);
    neg::register(registry);
    parity::register(registry);
    shl_i::register(registry);
    shl_u::register(registry);
    shr_i::register(registry);
    shr_u::register(registry);
    sub::register(registry);
    sub_i32::register(registry);
    sub_u32::register(registry);
    sub_mul::register(registry);
    sub_mul_i32::register(registry);
    sub_mul_u32::register(registry);
}
