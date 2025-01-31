use crate::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogBase, CeilingLogBase2,
    CeilingLogBasePowerOf2, CheckedDoubleFactorial, CheckedFactorial, CheckedLcm, CheckedLogBase,
    CheckedLogBase2, CheckedLogBasePowerOf2, CheckedMultifactorial, CheckedNextPowerOf2,
    CheckedPrimorial, CheckedSubfactorial, CoprimeWith, DoubleFactorial, ExtendedGcd, Factorial,
    FloorLogBase, FloorLogBase2, FloorLogBasePowerOf2, Gcd, GcdAssign, IsPowerOf2, Lcm, LcmAssign,
    ModAdd, ModAddAssign, ModInverse, ModIsReduced, ModMul, ModMulAssign, ModMulPrecomputed,
    ModMulPrecomputedAssign, ModNeg, ModNegAssign, ModPow, ModPowAssign, ModPowPrecomputed,
    ModPowPrecomputedAssign, ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2Inverse,
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Neg,
    ModPowerOf2NegAssign, ModPowerOf2Pow, ModPowerOf2PowAssign, ModPowerOf2Shl,
    ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, ModPowerOf2Square,
    ModPowerOf2SquareAssign, ModPowerOf2Sub, ModPowerOf2SubAssign, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSquarePrecomputedAssign, ModSub, ModSubAssign, Multifactorial, NegMod,
    NegModAssign, NegModPowerOf2, NegModPowerOf2Assign, NextPowerOf2, NextPowerOf2Assign,
    Primorial, RootAssignRem, RootRem, SqrtAssignRem, SqrtRem, Subfactorial, XMulYToZZ,
    XXAddYYToZZ, XXDivModYToQR, XXSubYYToZZ, XXXAddYYYToZZZ, XXXSubYYYToZZZ, XXXXAddYYYYToZZZZ,
};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::conversion::traits::{
    Digits, FromOtherTypeSlice, IntegerMantissaAndExponent, PowerOf2DigitIterable, PowerOf2Digits,
    SciMantissaAndExponent, VecFromOtherType, VecFromOtherTypeSlice,
};
use crate::num::factorization::primes::{PrimesIterator, PrimesLessThanIterator};
use crate::num::factorization::traits::Primes;
use crate::num::logic::traits::{BitBlockAccess, HammingDistance};

/// Defines functions on primitive unsigned integer types: uxx and usize.
pub trait PrimitiveUnsigned:
    BitBlockAccess<Bits = Self>
    + CeilingDivAssignNegMod<Self, ModOutput = Self>
    + CeilingDivNegMod<Self, DivOutput = Self, ModOutput = Self>
    + CheckedDoubleFactorial
    + CheckedFactorial
    + CheckedMultifactorial
    + CheckedPrimorial
    + CheckedSubfactorial
    + CheckedLcm<Self, Output = Self>
    + CheckedNextPowerOf2<Output = Self>
    + CoprimeWith<Self>
    + DoubleFactorial
    + Digits<u8>
    + Digits<u16>
    + Digits<u32>
    + Digits<usize>
    + ExtendedGcd<Self, Gcd = Self>
    + Factorial
    + From<u8>
    + FromOtherTypeSlice<u8>
    + FromOtherTypeSlice<u16>
    + FromOtherTypeSlice<u32>
    + FromOtherTypeSlice<usize>
    + Gcd<Self, Output = Self>
    + GcdAssign<Self>
    + HammingDistance
    + IsPowerOf2
    + Lcm<Self, Output = Self>
    + LcmAssign<Self>
    + ModIsReduced<Self>
    + ModAdd<Self, Self, Output = Self>
    + ModAddAssign<Self, Self>
    + ModInverse<Self, Output = Self>
    + ModMul<Self, Self, Output = Self>
    + ModMulAssign<Self, Self>
    + ModMulPrecomputed<Self, Self, Output = Self>
    + ModMulPrecomputedAssign<Self, Self>
    + ModNeg<Self, Output = Self>
    + ModNegAssign<Self>
    + ModPowerOf2<Output = Self>
    + ModPowerOf2Add<Self, Output = Self>
    + ModPowerOf2AddAssign<Self>
    + ModPowerOf2Inverse<Output = Self>
    + ModPowerOf2IsReduced
    + ModPowerOf2Mul<Self, Output = Self>
    + ModPowerOf2MulAssign<Self>
    + ModPowerOf2Neg<Output = Self>
    + ModPowerOf2NegAssign
    + ModPowerOf2Shl<i8, Output = Self>
    + ModPowerOf2Shl<i16, Output = Self>
    + ModPowerOf2Shl<i32, Output = Self>
    + ModPowerOf2Shl<u8, Output = Self>
    + ModPowerOf2Shl<u16, Output = Self>
    + ModPowerOf2Shl<u32, Output = Self>
    + ModPowerOf2ShlAssign<u8>
    + ModPowerOf2ShlAssign<u16>
    + ModPowerOf2ShlAssign<u32>
    + ModPowerOf2ShlAssign<usize>
    + ModPowerOf2ShlAssign<i8>
    + ModPowerOf2ShlAssign<i16>
    + ModPowerOf2ShlAssign<i32>
    + ModPowerOf2ShlAssign<isize>
    + ModPowerOf2Shr<i8, Output = Self>
    + ModPowerOf2Shr<i16, Output = Self>
    + ModPowerOf2Shr<i32, Output = Self>
    + ModPowerOf2ShrAssign<i8>
    + ModPowerOf2ShrAssign<i16>
    + ModPowerOf2ShrAssign<i32>
    + ModPowerOf2ShrAssign<isize>
    + ModPowerOf2Square<Output = Self>
    + ModPowerOf2SquareAssign
    + ModPowerOf2Sub<Self, Output = Self>
    + ModPowerOf2SubAssign<Self>
    + ModSquare<Self, Output = Self>
    + ModSquareAssign<Self>
    + ModSub<Self, Self, Output = Self>
    + ModSubAssign<Self, Self>
    + Multifactorial
    + NegMod<Self, Output = Self>
    + NegModAssign<Self>
    + NegModPowerOf2<Output = Self>
    + NegModPowerOf2Assign
    + NextPowerOf2<Output = Self>
    + NextPowerOf2Assign
    + PowerOf2Digits<u8>
    + PowerOf2Digits<u16>
    + PowerOf2Digits<u32>
    + PowerOf2Digits<usize>
    + PowerOf2DigitIterable<u8>
    + PowerOf2DigitIterable<u16>
    + PowerOf2DigitIterable<u32>
    + PowerOf2DigitIterable<usize>
    + Primes<I = PrimesIterator<Self>, LI = PrimesLessThanIterator<Self>>
    + PrimitiveInt
    + Primorial
    + SqrtRem<SqrtOutput = Self, RemOutput = Self>
    + SqrtAssignRem<RemOutput = Self>
    + Subfactorial
    + VecFromOtherType<u8>
    + VecFromOtherType<u16>
    + VecFromOtherType<u32>
    + VecFromOtherType<usize>
    + VecFromOtherTypeSlice<u8>
    + VecFromOtherTypeSlice<u16>
    + VecFromOtherTypeSlice<u32>
    + VecFromOtherTypeSlice<usize>
    + XXAddYYToZZ
    + XXDivModYToQR
    + XXSubYYToZZ
    + XMulYToZZ
    + XXXAddYYYToZZZ
    + XXXSubYYYToZZZ
    + XXXXAddYYYYToZZZZ
{
}

macro_rules! impl_basic_traits {
    ($u:ident) => {
        impl PrimitiveUnsigned for $u {}
    };
}
apply_to_unsigneds!(impl_basic_traits);
