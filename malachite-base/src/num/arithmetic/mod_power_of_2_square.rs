use crate::num::arithmetic::traits::{
    ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Square, ModPowerOf2SquareAssign,
};

macro_rules! impl_mod_power_of_2_square {
    ($t:ident) => {
        impl ModPowerOf2Square for $t {
            type Output = $t;

            /// Squares a number modulo another number $2^k$. Assumes the input is already reduced
            /// modulo $2^k$.
            ///
            /// $f(x, k) = y$, where $x, y < 2^k$ and $x^2 \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_square#mod_power_of_2_square).
            #[inline]
            fn mod_power_of_2_square(self, pow: u64) -> $t {
                self.mod_power_of_2_mul(self, pow)
            }
        }

        impl ModPowerOf2SquareAssign for $t {
            /// Squares a number modulo another number $2^k$, in place. Assumes the input is
            /// already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $x^2 \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_square#mod_power_of_2_square_assign).
            #[inline]
            fn mod_power_of_2_square_assign(&mut self, pow: u64) {
                self.mod_power_of_2_mul_assign(*self, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_square);
