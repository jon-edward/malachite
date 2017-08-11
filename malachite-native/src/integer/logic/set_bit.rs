use integer::Integer;
use natural::arithmetic::sub_u32::large_sub_u32;
use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Integer {
    /// Sets the `index`th bit of a `Integer`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    ///
    /// let mut x = Integer::new();
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x.to_string(), "100");
    ///
    /// let mut x = Integer::from(-256);
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x.to_string(), "-156");
    /// ```
    pub fn set_bit(&mut self, index: u64) {
        match *self {
            Integer {
                sign: true,
                ref mut abs,
            } => abs.set_bit(index),
            Integer {
                sign: false,
                ref mut abs,
            } => abs.set_bit_neg(index),
        }
    }
}

impl Natural {
    // self cannot be zero
    fn set_bit_neg(&mut self, index: u64) {
        match *self {
            Small(ref mut small) => {
                if index < LIMB_BITS as u64 {
                    *small = ((*small - 1) & !(1 << index)) + 1;
                }
                return;
            }
            Large(ref mut limbs) => {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index >= limbs.len() {
                    return;
                }
                let mask = 1 << ((index & LIMB_BITS_MASK as u64) as u32);
                let mut zero_bound = 0;
                // No index upper bound on this loop; we're sure there's a nonzero limb sooner or
                // later.
                while limbs[zero_bound] == 0 {
                    zero_bound += 1;
                }
                if limb_index > zero_bound {
                    limbs[limb_index] &= !mask;
                } else if limb_index == zero_bound {
                    limbs[limb_index] = ((limbs[limb_index] - 1) & !mask) + 1;
                } else {
                    large_sub_u32(&mut limbs[limb_index..], mask);
                }
            }
        }
        self.trim();
    }
}
