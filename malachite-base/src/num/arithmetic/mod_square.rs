use crate::num::arithmetic::traits::{
    ModPow, ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSquarePrecomputedAssign,
};

macro_rules! impl_mod_square {
    ($t:ident) => {
        impl ModSquare for $t {
            type Output = $t;

            /// Squares a number modulo another number $m$. Assumes the input is already reduced
            /// modulo $m$.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square).
            #[inline]
            fn mod_square(self, m: $t) -> $t {
                self.mod_pow(2, m)
            }
        }

        impl ModSquareAssign for $t {
            /// Squares a number modulo another number $m$, in place. Assumes the input is already
            /// reduced modulo $m$.
            ///
            /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square_assign).
            #[inline]
            fn mod_square_assign(&mut self, m: $t) {
                self.mod_pow_assign(2, m);
            }
        }

        impl ModSquarePrecomputed<u64, $t> for $t {
            /// Squares a number modulo another number $m$. Assumes the input is already reduced
            /// modulo $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular squarings with the same modulus. The precomputed data should be obtained
            /// using [`precompute_mod_pow_data`](super::traits::ModPowPrecomputed).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square_precomputed).
            #[inline]
            fn mod_square_precomputed(self, m: $t, data: &Self::Data) -> Self::Output {
                self.mod_pow_precomputed(2, m, data)
            }
        }

        impl ModSquarePrecomputedAssign<u64, $t> for $t {
            /// Squares a number modulo another number $m$, in place. Assumes the input is already
            /// reduced modulo $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular squarings with the same modulus. The precomputed data should be obtained
            /// using [`precompute_mod_pow_data`](super::traits::ModPowPrecomputed).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square_precomputed_assign).
            #[inline]
            fn mod_square_precomputed_assign(&mut self, m: $t, data: &Self::Data) {
                self.mod_pow_precomputed_assign(2, m, data);
            }
        }
    };
}
apply_to_unsigneds_wo_u64!(impl_mod_square);
