use integer::Integer;
use malachite_base::num::{EqMod, UnsignedAbs};

impl<'a> EqMod<i32, i32> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to an `i32` mod an `i32` `modulus`; that is,
    /// whether `self` - other is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(13).eq_mod(21i32, 8i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(321i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(322i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(679i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(680i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(321i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(322i32, -1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(679i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(680i32, -1_000i32), false);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-679i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-680i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-321i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-322i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-679i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-680i32, -1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-321i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-322i32, -1_000i32), false);
    /// }
    /// ```
    fn eq_mod(self, other: i32, modulus: i32) -> bool {
        if self.sign == (other >= 0) {
            self.abs
                .eq_mod(other.unsigned_abs(), modulus.unsigned_abs())
        } else {
            self.abs
                .eq_mod_neg_u32(other.unsigned_abs(), modulus.unsigned_abs())
        }
    }
}

impl<'a> EqMod<&'a Integer, i32> for i32 {
    /// Returns whether this `i32` is equivalent to an `Integer` mod an `i32` `modulus`; that is,
    /// whether other - `self` is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21i32.eq_mod(&Integer::from(13), 8i32), true);
    ///     assert_eq!(321i32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!(322i32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!(679i32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!(680i32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!(321i32.eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!(322i32.eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), false);
    ///     assert_eq!(679i32.eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!(680i32.eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), false);
    ///     assert_eq!((-679).eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!((-680).eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!((-321).eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!((-322).eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!((-679).eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!((-680).eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), false);
    ///     assert_eq!((-321).eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!((-322).eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), false);
    /// }
    /// ```
    fn eq_mod(self, other: &'a Integer, modulus: i32) -> bool {
        other.eq_mod(self, modulus)
    }
}