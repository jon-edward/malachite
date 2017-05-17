use integer::Integer;
use std::ops::{Add, AddAssign};
use traits::Assign;

/// Adds a `u32` to an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(0) + 123u32).to_string(), "123");
/// assert_eq!((Integer::from(-123) + 0u32).to_string(), "-123");
/// assert_eq!((Integer::from(-123) + 456u32).to_string(), "333");
/// assert_eq!((Integer::from_str("-1000000000000").unwrap() + 123u32).to_string(),
///            "-999999999877");
/// ```
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, other: u32) -> Integer {
        self += other;
        self
    }
}

/// Adds an `Integer` to a `u32`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((123u32 + Integer::from(0)).to_string(), "123");
/// assert_eq!((0u32 + Integer::from(-123)).to_string(), "-123");
/// assert_eq!((456u32 + Integer::from(-123)).to_string(), "333");
/// assert_eq!((123u32 + Integer::from_str("-1000000000000").unwrap()).to_string(),
///            "-999999999877");
/// ```
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds a `u32` to an `Integer` in place.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// let mut x = Integer::from(-10);
/// x += 1;
/// x += 2;
/// x += 3;
/// x += 4;
/// assert_eq!(x.to_string(), "0");
/// ```
impl AddAssign<u32> for Integer {
    fn add_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        match *self {
            // e.g. 10 + 5; self stays positive
            Integer { sign: true, ref mut abs } => *abs += other,
            // e.g. -10 + 5; self stays negative
            Integer { sign: false, ref mut abs } if *abs > other => *abs -= other,
            // e.g. -5 + 10 or -5 + 5; self becomes non-negative
            Integer { ref mut sign, ref mut abs } => {
                *sign = true;
                let small_abs = abs.to_u32().unwrap();
                abs.assign(other - small_abs);
            }
        }
    }
}