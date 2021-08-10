use num::arithmetic::traits::XXSubYYIsZZ;
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{JoinHalves, SplitInHalf, WrappingFrom};

fn _implicit_xx_sub_yy_is_zz<DT: JoinHalves + PrimitiveUnsigned + SplitInHalf>(
    x_1: DT::Half,
    x_0: DT::Half,
    y_1: DT::Half,
    y_0: DT::Half,
) -> (DT::Half, DT::Half) {
    DT::join_halves(x_1, x_0)
        .wrapping_sub(DT::join_halves(y_1, y_0))
        .split_in_half()
}

pub fn _explicit_xx_sub_yy_is_zz<T: PrimitiveUnsigned>(x_1: T, x_0: T, y_1: T, y_0: T) -> (T, T) {
    let (z_0, borrow) = x_0.overflowing_sub(y_0);
    let mut z_1 = x_1.wrapping_sub(y_1);
    if borrow {
        z_1.wrapping_sub_assign(T::ONE);
    }
    (z_1, z_0)
}

macro_rules! implicit_xx_sub_yy_is_zz {
    ($t:ident, $dt:ident) => {
        impl XXSubYYIsZZ for $t {
            /// Subtracts two numbers, each composed of two `Self` values, returning the difference
            /// as a pair of `Self` values.
            ///
            /// The more significant value always comes first. Subtraction is wrapping, and
            /// overflow is not indicated.
            ///
            /// $$
            /// f(x_1, x_0, y_1, y_0) = (z_1, z_0),
            /// $$
            /// where $W$ is `Self::WIDTH`,
            ///
            /// $x_1, x_0, y_1, y_0, z_1, z_0 < 2^W$, and
            /// $$
            /// (2^Wx_1 + x_0) - (2^Wy_1 + y_0) \equiv 2^Wz_1 + z_0 \mod 2^{2W}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::xx_sub_yy_is_zz` module.
            ///
            /// This is sub_ddmmss from longlong.h, GMP 6.2.1, where (sh, sl) is returned.
            #[inline]
            fn xx_sub_yy_is_zz(x_1: $t, x_0: $t, y_1: $t, y_0: $t) -> ($t, $t) {
                _implicit_xx_sub_yy_is_zz::<$dt>(x_1, x_0, y_1, y_0)
            }
        }
    };
}

implicit_xx_sub_yy_is_zz!(u8, u16);
implicit_xx_sub_yy_is_zz!(u16, u32);
implicit_xx_sub_yy_is_zz!(u32, u64);
implicit_xx_sub_yy_is_zz!(u64, u128);

impl XXSubYYIsZZ for usize {
    /// Subtracts two numbers, each composed of two `usize` values, returning the difference as a
    /// pair of `usize` values.
    ///
    /// The more significant value always comes first. Subtraction is wrapping, and overflow is not
    /// indicated.
    ///
    /// $$
    /// f(x_1, x_0, y_1, y_0) = (z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_1, x_0, y_1, y_0, z_1, z_0 < 2^W$, and
    /// $$
    /// (2^Wx_1 + x_0) - (2^Wy_1 + y_0) \equiv 2^Wz_1 + z_0 \mod 2^{2W}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is sub_ddmmss from longlong.h, GMP 6.2.1, where (sh, sl) is returned.
    fn xx_sub_yy_is_zz(x_1: usize, x_0: usize, y_1: usize, y_0: usize) -> (usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_1, z_0) = u32::xx_sub_yy_is_zz(
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y_1),
                u32::wrapping_from(y_0),
            );
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        } else {
            let (z_1, z_0) = u64::xx_sub_yy_is_zz(
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y_1),
                u64::wrapping_from(y_0),
            );
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        }
    }
}

impl XXSubYYIsZZ for u128 {
    /// Subtracts two numbers, each composed of two `u128` values, returning the difference as a
    /// pair of `u128` values.
    ///
    /// The more significant value always comes first. Subtraction is wrapping, and overflow is not
    /// indicated.
    ///
    /// $$
    /// f(x_1, x_0, y_1, y_0) = (z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_1, x_0, y_1, y_0, z_1, z_0 < 2^W$, and
    /// $$
    /// (2^Wx_1 + x_0) - (2^Wy_1 + y_0) \equiv 2^Wz_1 + z_0 \mod 2^{2W}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is sub_ddmmss from longlong.h, GMP 6.2.1, where (sh, sl) is returned.
    #[inline]
    fn xx_sub_yy_is_zz(x_1: u128, x_0: u128, y_1: u128, y_0: u128) -> (u128, u128) {
        _explicit_xx_sub_yy_is_zz(x_1, x_0, y_1, y_0)
    }
}
