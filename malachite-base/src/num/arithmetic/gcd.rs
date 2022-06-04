use num::arithmetic::traits::{Gcd, GcdAssign};
use num::basic::unsigneds::PrimitiveUnsigned;
use std::cmp::min;

#[cfg(feature = "test_build")]
pub fn gcd_euclidean<T: PrimitiveUnsigned>(x: T, y: T) -> T {
    if y == T::ZERO {
        x
    } else {
        gcd_euclidean(y, x % y)
    }
}

#[cfg(feature = "test_build")]
pub fn gcd_binary<T: PrimitiveUnsigned>(x: T, y: T) -> T {
    if x == y {
        x
    } else if x == T::ZERO {
        y
    } else if y == T::ZERO {
        x
    } else if x.even() {
        if y.odd() {
            gcd_binary(x >> 1, y)
        } else {
            gcd_binary(x >> 1, y >> 1) << 1
        }
    } else if y.even() {
        gcd_binary(x, y >> 1)
    } else if x > y {
        gcd_binary((x - y) >> 1, y)
    } else {
        gcd_binary((y - x) >> 1, x)
    }
}

// This is equivalent to the first version of `n_gcd` from `ulong_extras/gcd.c`, FLINT 2.7.1.
pub_test! {gcd_fast_a<T: PrimitiveUnsigned>(mut x: T, mut y: T) -> T {
    if x == T::ZERO {
        return y;
    }
    if y == T::ZERO {
        return x;
    }
    let x_zeros = x.trailing_zeros();
    let y_zeros = y.trailing_zeros();
    let f = min(x_zeros, y_zeros);
    x >>= x_zeros;
    y >>= y_zeros;
    while x != y {
        if x < y {
            y -= x;
            y >>= y.trailing_zeros();
        } else {
            x -= y;
            x >>= x.trailing_zeros();
        }
    }
    x << f
}}

#[cfg(feature = "test_build")]
// This is equivalent to the second version of `n_gcd` from `ulong_extras/gcd.c`, FLINT 2.7.1.
pub fn gcd_fast_b<T: PrimitiveUnsigned>(mut x: T, y: T) -> T {
    let mut v;
    if x >= y {
        v = y;
    } else {
        v = x;
        x = y;
    }
    let mut d;
    // x and y both have their top bit set.
    if (x & v).get_highest_bit() {
        d = x - v;
        x = v;
        v = d;
    }
    // The second value has its second-highest set.
    while (v << 1u32).get_highest_bit() {
        d = x - v;
        x = v;
        if d < v {
            v = d;
        } else if d < (v << 1) {
            v = d - x;
        } else {
            v = d - (x << 1);
        }
    }
    while v != T::ZERO {
        // Overflow is not possible due to top 2 bits of v not being set.
        // Avoid divisions when quotient < 4.
        if x < (v << 2) {
            d = x - v;
            x = v;
            if d < v {
                v = d;
            } else if d < (v << 1) {
                v = d - x;
            } else {
                v = d - (x << 1);
            }
        } else {
            let rem = x % v;
            x = v;
            v = rem;
        }
    }
    x
}

macro_rules! impl_gcd {
    ($t:ident) => {
        impl Gcd<$t> for $t {
            type Output = $t;

            /// Computes the GCD (greatest common divisor) of two numbers.
            ///
            /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which
            /// makes sense if we interpret "greatest" to mean "greatest by the divisibility
            /// order".
            ///
            /// $$
            /// f(x, y) = \gcd(x, y).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::gcd#gcd).
            #[inline]
            fn gcd(self, other: $t) -> $t {
                gcd_fast_a(self, other)
            }
        }

        impl GcdAssign<$t> for $t {
            /// Replaces another with the GCD (greatest common divisor) of it and another number.
            ///
            /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which
            /// makes sense if we interpret "greatest" to mean "greatest by the divisibility
            /// order".
            ///
            /// $$
            /// x \gets \gcd(x, y).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::gcd#gcd_assign).
            #[inline]
            fn gcd_assign(&mut self, other: $t) {
                *self = gcd_fast_a(*self, other);
            }
        }
    };
}
apply_to_unsigneds!(impl_gcd);