/// This module contains functions for getting the absolute value of a number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AbsAssign;
///
/// let mut x = 0i8;
/// x.abs_assign();
/// assert_eq!(x, 0i8);
///
/// let mut x = 100i64;
/// x.abs_assign();
/// assert_eq!(x, 100i64);
///
/// let mut x = -100i64;
/// x.abs_assign();
/// assert_eq!(x, 100i64);
/// ```
pub mod abs;
/// This module contains functions for adding a number and the product of two other numbers.
///
/// Here are usage examples of the macro-generated functions:
///
/// # add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AddMul;
///
/// assert_eq!(2u8.add_mul(3, 7), 23);
/// assert_eq!(127i8.add_mul(-2, 100), -73);
/// ```
///
/// # add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
///
/// let mut x = 2u8;
/// x.add_mul_assign(3, 7);
/// assert_eq!(x, 23);
///
/// let mut x = 127i8;
/// x.add_mul_assign(-2, 100);
/// assert_eq!(x, -73);
/// ```
pub mod add_mul;
/// This module contains functions for left-shifting a number and checking whether the result is
/// representable.
///
/// Here are usage examples of the macro-generated functions:
///
/// # arithmetic_checked_shl
/// ```
/// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
///
/// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
/// assert_eq!(3u8.arithmetic_checked_shl(7), None);
/// assert_eq!(3u8.arithmetic_checked_shl(100), None);
/// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
///
/// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
/// assert_eq!(3u8.arithmetic_checked_shl(7), None);
/// assert_eq!(3u8.arithmetic_checked_shl(100), None);
/// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
/// assert_eq!(100u8.arithmetic_checked_shl(-3), Some(12u8));
/// assert_eq!(100u8.arithmetic_checked_shl(-100), Some(0u8));
///
/// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
/// assert_eq!(3i8.arithmetic_checked_shl(6), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
/// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
/// assert_eq!(3i8.arithmetic_checked_shl(100), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
/// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
///
/// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
/// assert_eq!(3i8.arithmetic_checked_shl(6), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
/// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
/// assert_eq!(3i8.arithmetic_checked_shl(100), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
/// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
/// assert_eq!(100i8.arithmetic_checked_shl(-3), Some(12i8));
/// assert_eq!((-100i8).arithmetic_checked_shl(-3), Some(-13i8));
/// assert_eq!(100i8.arithmetic_checked_shl(-100), Some(0i8));
/// assert_eq!((-100i8).arithmetic_checked_shl(-100), Some(-1i8));
/// ```
pub mod arithmetic_checked_shl;
/// This module contains functions for right-shifting a number and checking whether the result is
/// representable.
///
/// Here are usage examples of the macro-generated functions:
///
/// # arithmetic_checked_shr
/// ```
/// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
///
/// assert_eq!(100u8.arithmetic_checked_shr(3), Some(12u8));
/// assert_eq!(100u8.arithmetic_checked_shr(100), Some(0u8));
/// assert_eq!(3u8.arithmetic_checked_shr(-6), Some(192u8));
/// assert_eq!(3u8.arithmetic_checked_shr(-7), None);
/// assert_eq!(3u8.arithmetic_checked_shr(-100), None);
/// assert_eq!(0u8.arithmetic_checked_shr(-100), Some(0u8));
///
/// assert_eq!(100i8.arithmetic_checked_shr(3), Some(12i8));
/// assert_eq!((-100i8).arithmetic_checked_shr(3), Some(-13i8));
/// assert_eq!(100i8.arithmetic_checked_shr(100), Some(0i8));
/// assert_eq!((-100i8).arithmetic_checked_shr(100), Some(-1i8));
/// assert_eq!(3i8.arithmetic_checked_shr(-5), Some(96i8));
/// assert_eq!(3i8.arithmetic_checked_shr(-6), None);
/// assert_eq!((-3i8).arithmetic_checked_shr(-5), Some(-96i8));
/// assert_eq!((-3i8).arithmetic_checked_shr(-6), None);
/// assert_eq!(3i8.arithmetic_checked_shr(-100), None);
/// assert_eq!((-3i8).arithmetic_checked_shr(-100), None);
/// assert_eq!(0i8.arithmetic_checked_shr(-100), Some(0i8));
/// ```
pub mod arithmetic_checked_shr;
/// This module wraps the `checked_abs` function into an implementation of `CheckedAbs`.
pub mod checked_abs;
/// This module wraps the `checked_add` function into an implementation of `CheckedAdd`.
pub mod checked_add;
/// This module contains functions for adding a number and the product of two other numbers, and
/// checking whether the result is representable.
///
/// Here are usage examples of the macro-generated functions:
///
/// # checked_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedAddMul;
///
/// assert_eq!(2u8.checked_add_mul(3, 7), Some(23));
/// assert_eq!(2u8.checked_add_mul(20, 20), None);
///
/// assert_eq!(127i8.checked_add_mul(-2, 100), Some(-73));
/// assert_eq!((-127i8).checked_add_mul(-2, 100), None);
/// ```
pub mod checked_add_mul;
/// This module wraps the `checked_div` function into an implementation of `CheckedDiv`.
pub mod checked_div;
/// This module wraps the `checked_mul` function into an implementation of `CheckedMul`.
pub mod checked_mul;
/// This module wraps the `checked_neg` function into an implementation of `CheckedNeg`.
pub mod checked_neg;
/// This module wraps the `checked_next_power_of_two` function into an implementation of
/// `CheckedNextPowerOfTwo`.
pub mod checked_next_power_of_two;
/// This module wraps the `checked_pow` function into an implementation of `CheckedPow`.
pub mod checked_pow;
/// This module contains functions for squaring a number and checking whether the result is
/// representable.
///
/// Here are usage examples of the macro-generated functions:
///
/// # checked_square
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedSquare;
///
/// assert_eq!(3u8.checked_square(), Some(9));
/// assert_eq!((-1000i32).checked_square(), Some(1000000));
/// assert_eq!((1000u16).checked_square(), None);
/// ```
pub mod checked_square;
/// This module wraps the `checked_sub` function into an implementation of `CheckedSub`.
pub mod checked_sub;
/// This module contains functions for subtracting the product of two numbers from another number,
/// and checking whether the result is representable.
///
/// Here are usage examples of the macro-generated functions:
///
/// # checked_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedSubMul;
///
/// assert_eq!(60u8.checked_sub_mul(5, 10), Some(10));
/// assert_eq!(2u8.checked_sub_mul(10, 5), None);
///
/// assert_eq!(127i8.checked_sub_mul(2, 100), Some(-73));
/// assert_eq!((-127i8).checked_sub_mul(2, 100), None);
/// ```
pub mod checked_sub_mul;
/// This module contains functions for unchecked exact division.
///
/// Here are usage examples of the macro-generated functions:
///
/// # div_exact
/// ```
/// use malachite_base::num::arithmetic::traits::DivExact;
///
/// // 123 * 456 = 56088
/// assert_eq!(56088u32.div_exact(456), 123);
///
/// // -123 * -456 = 56088
/// assert_eq!(56088i64.div_exact(-456), -123);
/// ```
///
/// # div_exact_assign
/// ```
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
///
/// // 123 * 456 = 56088
/// let mut x = 56088u32;
/// x.div_exact_assign(456);
/// assert_eq!(x, 123);
///
/// // -123 * -456 = 56088
/// let mut x = 56088i64;
/// x.div_exact_assign(-456);
/// assert_eq!(x, -123);
/// ```
pub mod div_exact;
/// This module contains functions for simultaneously finding the quotient and remainder of a
/// number, subject to various rounding rules.
///
/// Here are usage examples of the macro-generated functions:
///
/// # div_mod
/// ```
/// use malachite_base::num::arithmetic::traits::DivMod;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.div_mod(10), (2, 3));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.div_mod(5), (9, 0));
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.div_mod(10), (2, 3));
///
/// // -3 * -10 + -7 = 23
/// assert_eq!(23i16.div_mod(-10), (-3, -7));
///
/// // -3 * 10 + 7 = -23
/// assert_eq!((-23i32).div_mod(10), (-3, 7));
///
/// // 2 * -10 + -3 = -23
/// assert_eq!((-23i64).div_mod(-10), (2, -3));
/// ```
///
/// # div_assign_mod
/// ```
/// use malachite_base::num::arithmetic::traits::DivAssignMod;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// assert_eq!(x.div_assign_mod(10), 3);
/// assert_eq!(x, 2);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// assert_eq!(x.div_assign_mod(5), 0);
/// assert_eq!(x, 9);
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23i8;
/// assert_eq!(x.div_assign_mod(10), 3);
/// assert_eq!(x, 2);
///
/// // -3 * -10 + -7 = 23
/// let mut x = 23i16;
/// assert_eq!(x.div_assign_mod(-10), -7);
/// assert_eq!(x, -3);
///
/// // -3 * 10 + 7 = -23
/// let mut x = -23i32;
/// assert_eq!(x.div_assign_mod(10), 7);
/// assert_eq!(x, -3);
///
/// // 2 * -10 + -3 = -23
/// let mut x = -23i64;
/// assert_eq!(x.div_assign_mod(-10), -3);
/// assert_eq!(x, 2);
/// ```
///
/// # div_rem
/// ```
/// use malachite_base::num::arithmetic::traits::DivRem;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.div_rem(10), (2, 3));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.div_rem(5), (9, 0));
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.div_rem(10), (2, 3));
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.div_rem(-10), (-2, 3));
///
/// // -2 * 10 + -3 = -23
/// assert_eq!((-23i32).div_rem(10), (-2, -3));
///
/// // 2 * -10 + -3 = -23
/// assert_eq!((-23i64).div_rem(-10), (2, -3));
/// ```
///
/// # div_assign_rem
/// ```
/// use malachite_base::num::arithmetic::traits::DivAssignRem;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// assert_eq!(x.div_assign_rem(10), 3);
/// assert_eq!(x, 2);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// assert_eq!(x.div_assign_rem(5), 0);
/// assert_eq!(x, 9);
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23i8;
/// assert_eq!(x.div_assign_rem(10), 3);
/// assert_eq!(x, 2);
///
/// // -2 * -10 + 3 = 23
/// let mut x = 23i16;
/// assert_eq!(x.div_assign_rem(-10), 3);
/// assert_eq!(x, -2);
///
/// // -2 * 10 + -3 = -23
/// let mut x = -23i32;
/// assert_eq!(x.div_assign_rem(10), -3);
/// assert_eq!(x, -2);
///
/// // 2 * -10 + -3 = -23
/// let mut x = -23i64;
/// assert_eq!(x.div_assign_rem(-10), -3);
/// assert_eq!(x, 2);
/// ```
///
/// # ceiling_div_neg_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
///
/// // 3 * 10 - 7 = 23
/// assert_eq!(23u8.ceiling_div_neg_mod(10), (3, 7));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.ceiling_div_neg_mod(5), (9, 0));
/// ```
///
/// # ceiling_div_assign_neg_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
///
/// // 3 * 10 - 7 = 23
/// let mut x = 23u8;
/// assert_eq!(x.ceiling_div_assign_neg_mod(10), 7);
/// assert_eq!(x, 3);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// assert_eq!(x.ceiling_div_assign_neg_mod(5), 0);
/// assert_eq!(x, 9);
/// ```
///
/// # ceiling_div_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivMod;
///
/// // 3 * 10 + -7 = 23
/// assert_eq!(23i8.ceiling_div_mod(10), (3, -7));
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.ceiling_div_mod(-10), (-2, 3));
///
/// // -2 * 10 + -3 = -23
/// assert_eq!((-23i32).ceiling_div_mod(10), (-2, -3));
///
/// // 3 * -10 + 7 = -23
/// assert_eq!((-23i64).ceiling_div_mod(-10), (3, 7));
/// ```
///
/// # ceiling_div_assign_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivAssignMod;
///
/// // 3 * 10 + -7 = 23
/// let mut x = 23i8;
/// assert_eq!(x.ceiling_div_assign_mod(10), -7);
/// assert_eq!(x, 3);
///
/// // -2 * -10 + 3 = 23
/// let mut x = 23i16;
/// assert_eq!(x.ceiling_div_assign_mod(-10), 3);
/// assert_eq!(x, -2);
///
/// // -2 * 10 + -3 = -23
/// let mut x = -23i32;
/// assert_eq!(x.ceiling_div_assign_mod(10), -3);
/// assert_eq!(x, -2);
///
/// // 3 * -10 + 7 = -23
/// let mut x = -23i64;
/// assert_eq!(x.ceiling_div_assign_mod(-10), 7);
/// assert_eq!(x, 3);
/// ```
pub mod div_mod;
/// This module contains functions dividing two numbers according to a specified `RoundingMode`.
///
/// Here are usage examples of the macro-generated functions:
///
/// # div_round
/// ```
/// use malachite_base::num::arithmetic::traits::DivRound;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(10u8.div_round(4, RoundingMode::Down), 2);
/// assert_eq!(10u16.div_round(4, RoundingMode::Up), 3);
/// assert_eq!(10u32.div_round(5, RoundingMode::Exact), 2);
/// assert_eq!(10u64.div_round(3, RoundingMode::Nearest), 3);
/// assert_eq!(20u128.div_round(3, RoundingMode::Nearest), 7);
/// assert_eq!(10usize.div_round(4, RoundingMode::Nearest), 2);
/// assert_eq!(14u8.div_round(4, RoundingMode::Nearest), 4);
///
/// assert_eq!((-10i8).div_round(4, RoundingMode::Down), -2);
/// assert_eq!((-10i16).div_round(4, RoundingMode::Up), -3);
/// assert_eq!((-10i32).div_round(5, RoundingMode::Exact), -2);
/// assert_eq!((-10i64).div_round(3, RoundingMode::Nearest), -3);
/// assert_eq!((-20i128).div_round(3, RoundingMode::Nearest), -7);
/// assert_eq!((-10isize).div_round(4, RoundingMode::Nearest), -2);
/// assert_eq!((-14i8).div_round(4, RoundingMode::Nearest), -4);
///
/// assert_eq!((-10i16).div_round(-4, RoundingMode::Down), 2);
/// assert_eq!((-10i32).div_round(-4, RoundingMode::Up), 3);
/// assert_eq!((-10i64).div_round(-5, RoundingMode::Exact), 2);
/// assert_eq!((-10i128).div_round(-3, RoundingMode::Nearest), 3);
/// assert_eq!((-20isize).div_round(-3, RoundingMode::Nearest), 7);
/// assert_eq!((-10i8).div_round(-4, RoundingMode::Nearest), 2);
/// assert_eq!((-14i16).div_round(-4, RoundingMode::Nearest), 4);
/// ```
///
/// # div_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::DivRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// let mut x = 10u8;
/// x.div_round_assign(4, RoundingMode::Down);
/// assert_eq!(x, 2);
///
/// let mut x = 10u16;
/// x.div_round_assign(4, RoundingMode::Up);
/// assert_eq!(x, 3);
///
/// let mut x = 10u32;
/// x.div_round_assign(5, RoundingMode::Exact);
/// assert_eq!(x, 2);
///
/// let mut x = 10u64;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, 3);
///
/// let mut x = 20u128;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, 7);
///
/// let mut x = 10usize;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, 2);
///
/// let mut x = 14u8;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, 4);
///
/// let mut x = -10i8;
/// x.div_round_assign(4, RoundingMode::Down);
/// assert_eq!(x, -2);
///
/// let mut x = -10i16;
/// x.div_round_assign(4, RoundingMode::Up);
/// assert_eq!(x, -3);
///
/// let mut x = -10i32;
/// x.div_round_assign(5, RoundingMode::Exact);
/// assert_eq!(x, -2);
///
/// let mut x = -10i64;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, -3);
///
/// let mut x = -20i128;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, -7);
///
/// let mut x = -10isize;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, -2);
///
/// let mut x = -14i8;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, -4);
///
/// let mut x = -10i16;
/// x.div_round_assign(-4, RoundingMode::Down);
/// assert_eq!(x, 2);
///
/// let mut x = -10i32;
/// x.div_round_assign(-4, RoundingMode::Up);
/// assert_eq!(x, 3);
///
/// let mut x = -10i64;
/// x.div_round_assign(-5, RoundingMode::Exact);
/// assert_eq!(x, 2);
///
/// let mut x = -10i128;
/// x.div_round_assign(-3, RoundingMode::Nearest);
/// assert_eq!(x, 3);
///
/// let mut x = -20isize;
/// x.div_round_assign(-3, RoundingMode::Nearest);
/// assert_eq!(x, 7);
///
/// let mut x = -10i8;
/// x.div_round_assign(-4, RoundingMode::Nearest);
/// assert_eq!(x, 2);
///
/// let mut x = -14i16;
/// x.div_round_assign(-4, RoundingMode::Nearest);
/// assert_eq!(x, 4);
/// ```
pub mod div_round;
pub mod divisible_by;
pub mod divisible_by_power_of_two;
pub mod eq_mod;
pub mod eq_mod_power_of_two;
pub mod is_power_of_two;
pub mod mod_add;
pub mod mod_is_reduced;
pub mod mod_mul;
pub mod mod_neg;
pub mod mod_op;
pub mod mod_pow;
pub mod mod_power_of_two;
pub mod mod_power_of_two_add;
pub mod mod_power_of_two_is_reduced;
pub mod mod_power_of_two_mul;
pub mod mod_power_of_two_neg;
pub mod mod_power_of_two_pow;
pub mod mod_power_of_two_shl;
pub mod mod_power_of_two_shr;
pub mod mod_power_of_two_square;
pub mod mod_power_of_two_sub;
pub mod mod_shl;
pub mod mod_shr;
pub mod mod_square;
pub mod mod_sub;
pub mod neg;
pub mod next_power_of_two;
pub mod overflowing_abs;
pub mod overflowing_add;
pub mod overflowing_add_mul;
pub mod overflowing_div;
pub mod overflowing_mul;
pub mod overflowing_neg;
pub mod overflowing_pow;
pub mod overflowing_square;
pub mod overflowing_sub;
pub mod overflowing_sub_mul;
pub mod parity;
pub mod pow;
pub mod power_of_two;
pub mod round_to_multiple;
pub mod round_to_multiple_of_power_of_two;
pub mod saturating_abs;
pub mod saturating_add;
pub mod saturating_add_mul;
pub mod saturating_mul;
pub mod saturating_neg;
pub mod saturating_pow;
pub mod saturating_square;
pub mod saturating_sub;
pub mod saturating_sub_mul;
pub mod shl_round;
pub mod shr_round;
pub mod sign;
pub mod square;
pub mod sub_mul;
pub mod traits;
pub mod unsigneds;
pub mod wrapping_abs;
pub mod wrapping_add;
pub mod wrapping_add_mul;
pub mod wrapping_div;
pub mod wrapping_mul;
pub mod wrapping_neg;
pub mod wrapping_pow;
pub mod wrapping_square;
pub mod wrapping_sub;
pub mod wrapping_sub_mul;
pub mod x_mul_y_is_zz;
pub mod xx_add_yy_is_zz;
pub mod xx_div_mod_y_is_qr;
pub mod xx_sub_yy_is_zz;
pub mod xxx_add_yyy_is_zzz;
pub mod xxx_sub_yyy_is_zzz;
pub mod xxxx_add_yyyy_is_zzzz;