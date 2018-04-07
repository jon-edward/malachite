use integer::Integer;
use malachite_base::num::{BitAccess, PrimitiveInteger};
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::conversion::to_limbs::LimbIterator;
use natural::logic::not::limbs_not_in_place;
use natural::Natural;
use std::u32;

/// A double-ended iterator over the two's complement limbs of the negative of a `Natural`. The
/// forward order is ascending (least-significant first). There may be at most one implicit
/// most-significant `u32::MAX` limb.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NegativeLimbIterator<'a> {
    pub(crate) limbs: LimbIterator<'a>,
    first_nonzero_index: Option<usize>,
}

impl<'a> Iterator for NegativeLimbIterator<'a> {
    type Item = u32;

    /// A function to iterate through the two's complement limbs of the negative of a `Natural` in
    /// ascending order (least-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn next(&mut self) -> Option<u32> {
        let previous_i = self.limbs.i as u64;
        self.limbs.next().map(|limb| {
            if let Some(first_nonzero_index) = self.first_nonzero_index {
                if previous_i <= first_nonzero_index as u64 {
                    limb.wrapping_neg()
                } else {
                    !limb
                }
            } else {
                if limb != 0 {
                    self.first_nonzero_index = Some(previous_i as usize);
                }
                limb.wrapping_neg()
            }
        })
    }

    /// A function that returns the length of the negative limbs iterator; that is, the `Natural`'s
    /// negative limb count (this is the same as its limb count). The format is (lower bound,
    /// Option<upper bound>), but in this case it's trivial to always have an exact bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.limbs.limb_count, Some(self.limbs.limb_count))
    }
}

impl<'a> DoubleEndedIterator for NegativeLimbIterator<'a> {
    /// A function to iterate through the two's complement limbs of the negative of a `Natural` in
    /// descending order (most-significant first). This is worst-case linear since in first
    /// `next_back` call needs to determine the index of the least-significant nonzero limb.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    fn next_back(&mut self) -> Option<u32> {
        let previous_j = self.limbs.j as u64;
        self.limbs.next_back().map(|limb| {
            if self.first_nonzero_index.is_none() {
                let mut i = 0;
                while self.limbs[i] == 0 {
                    i += 1;
                }
                self.first_nonzero_index = Some(i);
            }
            let first_nonzero_index = self.first_nonzero_index.unwrap();
            if previous_j <= first_nonzero_index as u64 {
                limb.wrapping_neg()
            } else {
                !limb
            }
        })
    }
}

trait SignExtendedLimbIterator: DoubleEndedIterator<Item = u32> {
    const EXTENSION: u32;

    fn needs_sign_extension(&self) -> bool;

    fn iterate_forward(&mut self, extension_checked: &mut bool) -> Option<u32> {
        let next = self.next();
        if next.is_none() {
            if *extension_checked {
                None
            } else {
                *extension_checked = true;
                if self.needs_sign_extension() {
                    Some(Self::EXTENSION)
                } else {
                    None
                }
            }
        } else {
            next
        }
    }

    fn iterate_backward(&mut self, extension_checked: &mut bool) -> Option<u32> {
        if !*extension_checked {
            *extension_checked = true;
            if self.needs_sign_extension() {
                return Some(Self::EXTENSION);
            }
        }
        self.next_back()
    }
}

impl<'a> SignExtendedLimbIterator for LimbIterator<'a> {
    const EXTENSION: u32 = 0;

    fn needs_sign_extension(&self) -> bool {
        self[self.limb_count - 1].get_bit(u64::from(u32::WIDTH) - 1)
    }
}

impl<'a> SignExtendedLimbIterator for NegativeLimbIterator<'a> {
    const EXTENSION: u32 = u32::MAX;

    fn needs_sign_extension(&self) -> bool {
        let limbs = self.limbs.n.limbs();
        let mut i = 0;
        while limbs[i] == 0 {
            i += 1;
        }
        let last_limb_index = self.limbs.limb_count - 1;
        let last_limb = limbs[last_limb_index];
        let twos_complement_limb = if i == last_limb_index {
            last_limb.wrapping_neg()
        } else {
            !last_limb
        };
        !twos_complement_limb.get_bit(u64::from(u32::WIDTH) - 1)
    }
}

/// A double-ended iterator over the twos-complement limbs of an `Integer`. The forward order is
/// ascending (least- significant first). The most significant bit of the most significant limb
/// corresponds to the sign of the `Integer`; `false` for non-negative and `true` for negative. This
/// means that there may be a single most-significant sign-extension limb that is 0 or `u32::MAX`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TwosComplementLimbIterator<'a> {
    Zero,
    Positive(LimbIterator<'a>, bool),
    Negative(NegativeLimbIterator<'a>, bool),
}

impl<'a> Iterator for TwosComplementLimbIterator<'a> {
    type Item = u32;

    /// A function to iterate through the twos-complement limbs of an `Integer` in ascending order
    /// (least-significant first). The last limb may be a sign-extension limb.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().next(), None);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::trillion();
    ///     let mut limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.next(), Some(727379968));
    ///     assert_eq!(limbs.next(), Some(4294967063));
    ///     assert_eq!(limbs.next(), None);
    /// }
    /// ```
    fn next(&mut self) -> Option<u32> {
        match *self {
            TwosComplementLimbIterator::Zero => None,
            TwosComplementLimbIterator::Positive(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_forward(extension_checked)
            }
            TwosComplementLimbIterator::Negative(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_forward(extension_checked)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for TwosComplementLimbIterator<'a> {
    /// A function to iterate through the twos-complement limbs of an `Integer` in descending order
    /// (most-significant first). The first limb may be a sign-extension limb.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().next(), None);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::trillion();
    ///     let mut limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.next_back(), Some(4294967063));
    ///     assert_eq!(limbs.next_back(), Some(727379968));
    ///     assert_eq!(limbs.next_back(), None);
    /// }
    /// ```
    fn next_back(&mut self) -> Option<u32> {
        match *self {
            TwosComplementLimbIterator::Zero => None,
            TwosComplementLimbIterator::Positive(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_backward(extension_checked)
            }
            TwosComplementLimbIterator::Negative(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_backward(extension_checked)
            }
        }
    }
}

/// Given the limbs, or base-2<sup>32</sup> digits, of a non-negative `Integer`, in ascending order,
/// checks whether the most significant bit is `false`; if it isn't, appends an extra zero bit. This
/// way the `Integer`'s non-negativity is preserved in its limbs.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_to_twos_complement_limbs_non_negative(&mut limbs);
/// assert_eq!(limbs, &[1, 2, 3]);
///
/// let mut limbs = vec![1, 2, 0xffff_ffff];
/// limbs_to_twos_complement_limbs_non_negative(&mut limbs);
/// assert_eq!(limbs, &[1, 2, 0xffff_ffff, 0]);
/// ```
pub fn limbs_to_twos_complement_limbs_non_negative(limbs: &mut Vec<u32>) {
    if !limbs.is_empty() && limbs.last().unwrap().get_bit(u64::from(u32::WIDTH) - 1) {
        // Sign-extend with an extra 0 limb to indicate a positive Integer
        limbs.push(0);
    }
}

/// Given the limbs, or base-2<sup>32</sup> digits, of the absolute value of a negative `Integer`,
/// in ascending order, converts the limbs to two's complement. Returns whether there is a carry
/// left over from the two's complement conversion process.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = &mut [1, 2, 3];
/// assert!(!limbs_slice_to_twos_complement_limbs_negative(limbs));
/// assert_eq!(limbs, &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
///
/// let mut limbs = &mut [0, 0, 0];
/// assert!(limbs_slice_to_twos_complement_limbs_negative(limbs));
/// assert_eq!(limbs, &[0, 0, 0]);
/// ```
pub fn limbs_slice_to_twos_complement_limbs_negative(limbs: &mut [u32]) -> bool {
    limbs_not_in_place(limbs);
    mpn_add_1_in_place(limbs, 1)
}

/// Given the limbs, or base-2<sup>32</sup> digits, of the absolute value of a negative `Integer`,
/// in ascending order, converts the limbs to two's complement and checks whether the most
/// significant bit is `true`; if it isn't, appends an extra `u32::MAX` bit. This way the
/// `Integer`'s negativity is preserved in its limbs. The limbs cannot be empty or contain only
/// zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` contains only zeros.
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
/// assert_eq!(limbs, &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
///
/// let mut limbs = vec![0, 0xffff_ffff];
/// limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
/// assert_eq!(limbs, &[0, 1, 0xffff_ffff]);
/// ```
pub fn limbs_vec_to_twos_complement_limbs_negative(limbs: &mut Vec<u32>) {
    assert!(!limbs_slice_to_twos_complement_limbs_negative(limbs));
    if !limbs.last().unwrap().get_bit(u64::from(u32::WIDTH) - 1) {
        // Sign-extend with an extra !0 limb to indicate a negative Integer
        limbs.push(u32::MAX);
    }
}

impl Integer {
    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// `into_twos_complement_limbs_asc` is more efficient.
    ///
    /// This method is more efficient than `to_twos_complement_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_asc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).to_twos_complement_limbs_asc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().to_twos_complement_limbs_asc(), vec![3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).to_twos_complement_limbs_asc(),
    ///         vec![727379968, 4294967063]);
    /// }
    /// ```
    pub fn to_twos_complement_limbs_asc(&self) -> Vec<u32> {
        let mut limbs = self.abs.to_limbs_asc();
        if self.sign {
            limbs_to_twos_complement_limbs_non_negative(&mut limbs);
        } else {
            limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
        }
        limbs
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// `into_twos_complement_limbs_desc` is more efficient.
    ///
    /// This method is less efficient than `to_twos_complement_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_desc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).to_twos_complement_limbs_desc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().to_twos_complement_limbs_desc(), vec![232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).to_twos_complement_limbs_desc(),
    ///         vec![4294967063, 727379968]);
    /// }
    pub fn to_twos_complement_limbs_desc(&self) -> Vec<u32> {
        let mut limbs = self.to_twos_complement_limbs_asc();
        limbs.reverse();
        limbs
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_twos_complement_limbs_asc`.
    ///
    /// This method is more efficient than `into_twos_complement_limbs_desc`.
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_asc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).into_twos_complement_limbs_asc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().into_twos_complement_limbs_asc(), vec![3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).into_twos_complement_limbs_asc(),
    ///         vec![727379968, 4294967063]);
    /// }
    /// ```
    pub fn into_twos_complement_limbs_asc(self) -> Vec<u32> {
        let mut limbs = self.abs.into_limbs_asc();
        if self.sign {
            limbs_to_twos_complement_limbs_non_negative(&mut limbs);
        } else {
            limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
        }
        limbs
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_twos_complement_limbs_desc`.
    ///
    /// This method is less efficient than `into_twos_complement_limbs_asc`.
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_desc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).into_twos_complement_limbs_desc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().into_twos_complement_limbs_desc(),
    ///         vec![232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).into_twos_complement_limbs_desc(),
    ///         vec![4294967063, 727379968]);
    /// }
    pub fn into_twos_complement_limbs_desc(self) -> Vec<u32> {
        let mut limbs = self.into_twos_complement_limbs_asc();
        limbs.reverse();
        limbs
    }

    /// Returns a double-ended iterator over the twos-complement limbs of an `Integer`. The forward
    /// order is ascending, so that less significant limbs appear first. There may be a
    /// most-significant sign-extension limb.
    ///
    /// If it's necessary to get a `Vec` of all the twos_complement limbs, consider using
    /// `to_twos_complement_limbs_asc`,
    /// `to_twos_complement_limbs_desc`, `into_twos_complement_limbs_asc`, or
    /// `into_twos_complement_limbs_desc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.twos_complement_limbs().next().is_none());
    ///     assert_eq!(Integer::from(123).twos_complement_limbs().collect::<Vec<u32>>(), vec![123]);
    ///     assert_eq!(Integer::from(-123).twos_complement_limbs().collect::<Vec<u32>>(),
    ///         vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().twos_complement_limbs().collect::<Vec<u32>>(),
    ///         vec![3567587328, 232]);
    ///     // Sign-extension for a non-negative `Integer`
    ///     assert_eq!(Integer::from(0xffff_ffffu32).twos_complement_limbs().collect::<Vec<u32>>(),
    ///         vec![0xffff_ffff, 0]);
    ///     assert_eq!((-Integer::trillion()).twos_complement_limbs().collect::<Vec<u32>>(),
    ///         vec![727379968, 4294967063]);
    ///     // Sign-extension for a negative `Integer`
    ///     assert_eq!((-Integer::from(0xffff_ffffu32)).twos_complement_limbs()
    ///         .collect::<Vec<u32>>(), vec![1, 0xffff_ffff]);
    ///
    ///     assert!(Integer::ZERO.twos_complement_limbs().rev().next().is_none());
    ///     assert_eq!(Integer::from(123).twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         vec![123]);
    ///     assert_eq!(Integer::from(-123).twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         vec![232, 3567587328]);
    ///     // Sign-extension for a non-negative `Integer`
    ///     assert_eq!(Integer::from(0xffff_ffffu32).twos_complement_limbs().rev()
    ///         .collect::<Vec<u32>>(), vec![0, 0xffff_ffff]);
    ///     assert_eq!((-Integer::trillion()).twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         vec![4294967063, 727379968]);
    ///     // Sign-extension for a negative `Integer`
    ///     assert_eq!((-Integer::from(0xffff_ffffu32)).twos_complement_limbs().rev()
    ///         .collect::<Vec<u32>>(), vec![0xffff_ffff, 1]);
    /// }
    /// ```
    pub fn twos_complement_limbs(&self) -> TwosComplementLimbIterator {
        if *self == 0 {
            TwosComplementLimbIterator::Zero
        } else if self.sign {
            TwosComplementLimbIterator::Positive(self.abs.limbs(), false)
        } else {
            TwosComplementLimbIterator::Negative(self.abs.negative_limbs(), false)
        }
    }
}

impl Natural {
    /// Returns a double-ended iterator over the two's complement limbs of the negative of a
    /// `Natural`. The forward order is ascending, so that less significant limbs appear first.
    /// There may be at most one trailing `u32::MAX` limb going forward, or leading `u32::MAX` limb
    /// going backward. The `Natural` cannot be zero.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn negative_limbs(&self) -> NegativeLimbIterator {
        assert_ne!(*self, 0, "Cannot get negative limbs of 0.");
        NegativeLimbIterator {
            limbs: self.limbs(),
            first_nonzero_index: None,
        }
    }
}
