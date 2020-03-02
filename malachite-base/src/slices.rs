use num::basic::traits::Zero;

/// Tests whether all values in a slice are equal to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_test_zero;
///
/// assert!(slice_test_zero::<u32>(&[0, 0, 0]));
/// assert!(!slice_test_zero::<u32>(&[0, 1, 0]));
/// ```
///
/// This is mpn_zero_p from gmp.h, GMP 6.1.2.
pub fn slice_test_zero<T: Copy + Eq + Zero>(xs: &[T]) -> bool {
    xs.iter().all(|&x| x == T::ZERO)
}

/// Sets all values in a slice to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_set_zero;
///
/// let mut xs = [1, 2, 3, 4, 5];
/// slice_set_zero::<u32>(&mut xs[1..4]);
/// assert_eq!(xs, [1, 0, 0, 0, 5]);
/// ```
///
/// This is mpn_zero from mpn/generic/zero.c, GMP 6.1.2. Note that this is needed less often in
/// Malachite than in GMP, since Malachite generally initializes new memory with zeros.
pub fn slice_set_zero<T: Zero>(xs: &mut [T]) {
    for x in xs.iter_mut() {
        *x = T::ZERO;
    }
}

/// Counts the number of zeros that a slice starts with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_leading_zeros;
///
/// assert_eq!(slice_leading_zeros::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(slice_leading_zeros::<u32>(&[0, 0, 0, 1, 2, 3]), 3);
/// ```
pub fn slice_leading_zeros<T: Eq + Zero>(xs: &[T]) -> usize {
    let zero = T::ZERO;
    xs.iter().take_while(|&x| x == &zero).count()
}

/// Counts the number of zeros that a slice ends with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_trailing_zeros;
///
/// assert_eq!(slice_trailing_zeros::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(slice_trailing_zeros::<u32>(&[1, 2, 3, 0, 0, 0]), 3);
/// ```
pub fn slice_trailing_zeros<T: Eq + Zero>(xs: &[T]) -> usize {
    let zero = T::ZERO;
    xs.iter().rev().take_while(|&x| x == &zero).count()
}

/// Given a slice `xs` and an amount, copies the contents of `&xs[amount..]` to
/// `&xs[..xs.len() - amount]`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `amount` is greater than the length of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_move_left;
///
/// let xs = &mut [1, 2, 3, 4, 5, 6];
/// slice_move_left::<u32>(xs, 2);
/// assert_eq!(xs, &[3, 4, 5, 6, 5, 6]);
/// ```
#[inline]
pub fn slice_move_left<T: Copy>(xs: &mut [T], amount: usize) {
    xs.copy_within(amount..xs.len(), 0);
}