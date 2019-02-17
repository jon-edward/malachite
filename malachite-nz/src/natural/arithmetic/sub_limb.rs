use malachite_base::num::{CheckedSub, OverflowingSubAssign};
use natural::Natural;
use platform::Limb;
use std::fmt::Display;
use std::ops::{Sub, SubAssign};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Limb` from the `Natural`. Returns a pair consisting of the limbs of the result, and whether
/// there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_limb::limbs_sub_limb;
///
/// assert_eq!(limbs_sub_limb(&[123, 456], 78), (vec![45, 456], false));
/// assert_eq!(limbs_sub_limb(&[123, 456], 789), (vec![4_294_966_630, 455], false));
/// assert_eq!(limbs_sub_limb(&[1], 2), (vec![4_294_967_295], true));
/// ```
pub fn limbs_sub_limb(limbs: &[Limb], mut limb: Limb) -> (Vec<Limb>, bool) {
    let len = limbs.len();
    let mut result_limbs = Vec::with_capacity(len);
    for i in 0..len {
        let (difference, overflow) = limbs[i].overflowing_sub(limb);
        result_limbs.push(difference);
        if overflow {
            limb = 1;
        } else {
            limb = 0;
            result_limbs.extend_from_slice(&limbs[i + 1..]);
            break;
        }
    }
    (result_limbs, limb != 0)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Limb` from the `Natural`, writing the `in_limbs.len()` limbs of the result to an output slice.
/// Returns whether there was a borrow left over; that is, whether the `Limb` was greater than the
/// `Natural`. The output slice must be at least as long as the input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_limb::limbs_sub_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_limb_to_out(&mut out, &[123, 456], 78), false);
/// assert_eq!(out, &[45, 456, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_limb_to_out(&mut out, &[123, 456], 789), false);
/// assert_eq!(out, &[4_294_966_630, 455, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_limb_to_out(&mut out, &[1], 2), true);
/// assert_eq!(out, &[4_294_967_295, 0, 0]);
/// ```
pub fn limbs_sub_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], mut limb: Limb) -> bool {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    for i in 0..len {
        let (difference, overflow) = in_limbs[i].overflowing_sub(limb);
        out[i] = difference;
        if overflow {
            limb = 1;
        } else {
            limb = 0;
            let copy_index = i + 1;
            out[copy_index..len].copy_from_slice(&in_limbs[copy_index..]);
            break;
        }
    }
    limb != 0
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Limb` from the `Natural` and writes the limbs of the result to the input slice. Returns whether
/// there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_sub_limb_in_place(&mut limbs, 78), false);
/// assert_eq!(limbs, &[45, 456]);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_sub_limb_in_place(&mut limbs, 789), false);
/// assert_eq!(limbs, &[4_294_966_630, 455]);
///
/// let mut limbs = vec![1];
/// assert_eq!(limbs_sub_limb_in_place(&mut limbs, 2), true);
/// assert_eq!(limbs, &[4_294_967_295]);
/// ```
pub fn limbs_sub_limb_in_place(limbs: &mut [Limb], mut limb: Limb) -> bool {
    for x in limbs.iter_mut() {
        if x.overflowing_sub_assign(limb) {
            limb = 1;
        } else {
            return false;
        }
    }
    limb != 0
}

fn sub_panic<S: Display, T: Display>(x: S, y: T) -> ! {
    panic!(
        "Cannot subtract a Limb from a smaller Natural. self: {}, other: {}",
        x, y
    );
}

/// Subtracts a `Limb` from a `Natural`, taking the `Natural` by value. Panics if the `Limb` is
/// greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(123u32) - 123).to_string(), "0");
/// assert_eq!((Natural::from(123u32) - 0).to_string(), "123");
/// assert_eq!((Natural::from(456u32) - 123).to_string(), "333");
/// assert_eq!((Natural::trillion() - 123).to_string(), "999999999877");
/// ```
impl Sub<Limb> for Natural {
    type Output = Natural;

    fn sub(self, other: Limb) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Limb from a smaller Natural")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Sub<u32> for Natural {
    type Output = Natural;

    #[inline]
    fn sub(self, other: u32) -> Natural {
        self - Limb::from(other)
    }
}

/// Subtracts a `Limb` from a `Natural`, taking the `Natural` by reference. Panics if the `Limb` is
/// greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(123u32) - 123).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) - 0).to_string(), "123");
/// assert_eq!((&Natural::from(456u32) - 123).to_string(), "333");
/// assert_eq!((&Natural::trillion() - 123).to_string(), "999999999877");
/// ```
impl<'a> Sub<Limb> for &'a Natural {
    type Output = Natural;

    fn sub(self, other: Limb) -> Natural {
        self.checked_sub(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Sub<u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn sub(self, other: u32) -> Natural {
        self - Limb::from(other)
    }
}

/// Subtracts a `Limb` from a `Natural` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Example
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(15u32);
/// x -= 1;
/// x -= 2;
/// x -= 3;
/// x -= 4;
/// assert_eq!(x.to_string(), "5");
/// ```
impl SubAssign<Limb> for Natural {
    fn sub_assign(&mut self, other: Limb) {
        if self.sub_assign_limb_no_panic(other) {
            sub_panic(self, other);
        }
    }
}

/// Subtracts a `Natural` from a `Limb`, taking the `Natural` by value. Panics if the `Natural` is
/// greater than the `Limb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123 - Natural::from(123u32), 0);
///     assert_eq!(123 - Natural::ZERO, 123);
///     assert_eq!(456 - Natural::from(123u32), 333);
/// }
/// ```
impl Sub<Natural> for Limb {
    type Output = Limb;

    fn sub(self, other: Natural) -> Limb {
        CheckedSub::checked_sub(self, &other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

/// Subtracts a `Natural` from a `Limb`, taking the `Natural` by reference. Panics if the `Natural`
/// is greater than the `Limb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123 - &Natural::from(123u32), 0);
///     assert_eq!(123 - &Natural::ZERO, 123);
///     assert_eq!(456 - &Natural::from(123u32), 333);
/// }
/// ```
impl<'a> Sub<&'a Natural> for Limb {
    type Output = Limb;

    fn sub(self, other: &'a Natural) -> Limb {
        CheckedSub::checked_sub(self, other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

/// Subtracts a `Natural` from a `u32` in place, taking the `Natural` by value. Panics if the
/// `Natural` is greater than the `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = 123;
///     x -= Natural::from(123u32);
///     assert_eq!(x, 0);
///
///     let mut x = 123;
///     x -= Natural::ZERO;
///     assert_eq!(x, 123);
///
///     let mut x = 456;
///     x -= Natural::from(123u32);
///     assert_eq!(x, 333);
/// }
/// ```
impl SubAssign<Natural> for Limb {
    fn sub_assign(&mut self, other: Natural) {
        *self = *self - other;
    }
}

/// Subtracts a `Natural` from a `Limb` in place, taking the `Natural` by reference. Panics if the
/// `Natural` is greater than the `Limb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = 123;
///     x -= &Natural::from(123u32);
///     assert_eq!(x, 0);
///
///     let mut x = 123;
///     x -= &Natural::ZERO;
///     assert_eq!(x, 123);
///
///     let mut x = 456;
///     x -= &Natural::from(123u32);
///     assert_eq!(x, 333);
/// }
/// ```
impl<'a> SubAssign<&'a Natural> for Limb {
    fn sub_assign(&mut self, other: &'a Natural) {
        *self = *self - other;
    }
}