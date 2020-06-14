use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};

use natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left,
};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

impl Natural {
    fn checked_sub_mul_limb_ref_ref(&self, b: &Natural, c: Limb) -> Option<Natural> {
        match (self, b, c) {
            (a, _, 0) | (a, natural_zero!(), _) => Some(a.clone()),
            (a, b, 1) => a.checked_sub(b),
            (a, b @ Natural(Small(_)), c) => a.checked_sub(b * Natural::from(c)),
            (Natural(Small(_)), _, _) => None,
            (&Natural(Large(ref a_limbs)), &Natural(Large(ref b_limbs)), c) => {
                if a_limbs.len() >= b_limbs.len() {
                    limbs_sub_mul_limb_greater(a_limbs, b_limbs, c)
                        .map(Natural::from_owned_limbs_asc)
                } else {
                    None
                }
            }
        }
    }

    //TODO clean
    fn sub_mul_assign_limb_no_panic(&mut self, b: Natural, c: Limb) -> bool {
        if c == 0 || b == 0 {
            false
        } else if c == 1 {
            self.sub_assign_no_panic(b)
        } else if self.limb_count() < b.limb_count() {
            true
        } else {
            let (borrow, fallback) = match (&mut *self, &b) {
                (&mut Natural(Large(ref mut a_limbs)), &Natural(Large(ref b_limbs))) => (
                    limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0,
                    false,
                ),
                _ => (false, true),
            };
            if fallback {
                self.sub_assign_no_panic(b * Natural::from(c))
            } else if borrow {
                true
            } else {
                self.trim();
                false
            }
        }
    }

    fn sub_mul_assign_limb_ref_no_panic(&mut self, b: &Natural, c: Limb) -> bool {
        if c == 0 || *b == 0 {
            false
        } else if c == 1 {
            self.sub_assign_ref_no_panic(b)
        } else if self.limb_count() < b.limb_count() {
            true
        } else {
            let (borrow, fallback) = match (&mut *self, &b) {
                (&mut Natural(Large(ref mut a_limbs)), &Natural(Large(ref b_limbs))) => (
                    limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0,
                    false,
                ),
                _ => (false, true),
            };
            if fallback {
                self.sub_assign_no_panic(b * Natural::from(c))
            } else if borrow {
                true
            } else {
                self.trim();
                false
            }
        }
    }

    fn sub_mul_assign_helper(&mut self, b: &Natural, c: &Natural) -> bool {
        {
            if let Natural(Large(ref mut a_limbs)) = *self {
                if let Natural(Large(ref b_limbs)) = *b {
                    if let Natural(Large(ref c_limbs)) = *c {
                        if limbs_sub_mul_in_place_left(a_limbs, b_limbs, c_limbs) {
                            return true;
                        }
                    }
                }
            }
        }
        self.trim();
        false
    }

    pub(crate) fn sub_mul_assign_no_panic(&mut self, b: Natural, c: Natural) -> bool {
        if let Natural(Small(small_b)) = b {
            self.sub_mul_assign_limb_no_panic(c, small_b)
        } else if let Natural(Small(small_c)) = c {
            self.sub_mul_assign_limb_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(&b, &c)
        }
    }

    pub(crate) fn sub_mul_assign_val_ref_no_panic(&mut self, b: Natural, c: &Natural) -> bool {
        if let Natural(Small(small_b)) = b {
            self.sub_mul_assign_limb_ref_no_panic(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.sub_mul_assign_limb_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(&b, c)
        }
    }

    pub(crate) fn sub_mul_assign_ref_val_no_panic(&mut self, b: &Natural, c: Natural) -> bool {
        if let Natural(Small(small_b)) = *b {
            self.sub_mul_assign_limb_no_panic(c, small_b)
        } else if let Natural(Small(small_c)) = c {
            self.sub_mul_assign_limb_ref_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(b, &c)
        }
    }

    pub(crate) fn sub_mul_assign_ref_ref_no_panic(&mut self, b: &Natural, c: &Natural) -> bool {
        if let Natural(Small(small_b)) = *b {
            self.sub_mul_assign_limb_ref_no_panic(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.sub_mul_assign_limb_ref_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(b, c)
        }
    }
}

impl CheckedSubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self`, y, and z by value. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(Natural::from(3u32), Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32),
    ///     Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     Natural::from(0x1_0000u32), Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: Natural, c: Natural) -> Option<Natural> {
        if self.sub_mul_assign_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self` and y by value and z by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(Natural::from(3u32), &Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32),
    ///     &Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: Natural, c: &'a Natural) -> Option<Natural> {
        if self.sub_mul_assign_val_ref_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self` and z by value and y by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(&Natural::from(3u32), Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32),
    ///     Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     &Natural::from(0x1_0000u32), Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: &'a Natural, c: Natural) -> Option<Natural> {
        if self.sub_mul_assign_ref_val_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, 'b> CheckedSubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self` by value and y and z by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32),
    ///     &Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if self.sub_mul_assign_ref_ref_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, 'b, 'c> CheckedSubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self`, y, and z by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", (&Natural::from(20u32))
    ///     .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", (&Natural::from(10u32)).checked_sub_mul(&Natural::from(3u32),
    ///     &Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", (&Natural::trillion()).checked_sub_mul(
    ///     &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if let Natural(Small(small_b)) = *b {
            self.checked_sub_mul_limb_ref_ref(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.checked_sub_mul_limb_ref_ref(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            None
        } else {
            if let Natural(Large(ref a_limbs)) = *self {
                if let Natural(Large(ref b_limbs)) = *b {
                    if let Natural(Large(ref c_limbs)) = *c {
                        return limbs_sub_mul(a_limbs, b_limbs, c_limbs)
                            .map(Natural::from_owned_limbs_asc);
                    }
                }
            }
            unreachable!();
        }
    }
}
