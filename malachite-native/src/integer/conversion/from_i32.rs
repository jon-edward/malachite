use integer::Integer;
use natural::Natural;

/// Converts an `i32` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!(Integer::from(123).to_string(), "123");
/// assert_eq!(Integer::from(-123).to_string(), "-123");
/// ```
impl From<i32> for Integer {
    fn from(i: i32) -> Integer {
        Integer {
            sign: i >= 0,
            abs: Natural::from(i.wrapping_abs() as u32),
        }
    }
}
