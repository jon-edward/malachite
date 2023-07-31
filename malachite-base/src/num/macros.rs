#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_unsigneds {
    ($m: tt) => {
        $m!(u8);
        $m!(u16);
        $m!(u32);
        $m!(usize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_unsigneds_wo_u64 {
    ($m: tt) => {
        $m!(u8);
        $m!(u16);
        $m!(u32);
        $m!(usize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_signeds {
    ($m: tt) => {
        $m!(i8);
        $m!(i16);
        $m!(i32);
        $m!(isize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_primitive_ints {
    ($m: tt) => {
        apply_to_unsigneds!($m);
        apply_to_signeds!($m);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_unsigned_signed_pairs {
    ($m: tt) => {
        $m!(u8, i8);
        $m!(u16, i16);
        $m!(u32, i32);
        $m!(usize, isize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds {
    ($f: ident) => {
        $f::<u8>();
        $f::<u16>();
        $f::<u32>();
        $f::<usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds {
    ($f: ident) => {
        $f::<i8>();
        $f::<i16>();
        $f::<i32>();
        $f::<isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints {
    ($f: ident) => {
        apply_fn_to_unsigneds!($f);
        apply_fn_to_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigned_signed_pairs {
    ($f: ident) => {
        $f::<u8, i8>();
        $f::<u16, i16>();
        $f::<u32, i32>();
        $f::<usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_unsigneds {
    ($f: ident) => {
        $f::<u8, u8>();
        $f::<u8, u16>();
        $f::<u8, u32>();
        $f::<u8, usize>();
        $f::<u16, u8>();
        $f::<u16, u16>();
        $f::<u16, u32>();
        $f::<u16, usize>();
        $f::<u32, u8>();
        $f::<u32, u16>();
        $f::<u32, u32>();
        $f::<u32, usize>();
        $f::<usize, u8>();
        $f::<usize, u16>();
        $f::<usize, u32>();
        $f::<usize, usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_signeds {
    ($f: ident) => {
        $f::<u8, i8>();
        $f::<u8, i16>();
        $f::<u8, i32>();
        $f::<u8, isize>();
        $f::<u16, i8>();
        $f::<u16, i16>();
        $f::<u16, i32>();
        $f::<u16, isize>();
        $f::<u32, i8>();
        $f::<u32, i16>();
        $f::<u32, i32>();
        $f::<u32, isize>();
        $f::<usize, i8>();
        $f::<usize, i16>();
        $f::<usize, i32>();
        $f::<usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_unsigned_signed_pairs {
    ($f: ident) => {
        $f::<u8, u8, i8>();
        $f::<u8, u16, i16>();
        $f::<u8, u32, i32>();
        $f::<u8, usize, isize>();
        $f::<u16, u8, i8>();
        $f::<u16, u16, i16>();
        $f::<u16, u32, i32>();
        $f::<u16, usize, isize>();
        $f::<u32, u8, i8>();
        $f::<u32, u16, i16>();
        $f::<u32, u32, i32>();
        $f::<u32, usize, isize>();
        $f::<usize, u8, i8>();
        $f::<usize, u16, i16>();
        $f::<usize, u32, i32>();
        $f::<usize, usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_primitive_floats {
    ($f: ident) => {
        $f::<u8, f32>();
        $f::<u16, f32>();
        $f::<u32, f32>();
        $f::<usize, f32>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds_and_primitive_floats {
    ($f: ident) => {
        $f::<i8, f32>();
        $f::<i16, f32>();
        $f::<i32, f32>();
        $f::<isize, f32>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_primitive_floats {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_primitive_floats!($f);
        apply_fn_to_signeds_and_primitive_floats!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats_and_unsigneds {
    ($f: ident) => {
        $f::<f32, u8>();
        $f::<f32, u16>();
        $f::<f32, u32>();
        $f::<f32, usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats_and_signeds {
    ($f: ident) => {
        $f::<f32, i8>();
        $f::<f32, i16>();
        $f::<f32, i32>();
        $f::<f32, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats_and_unsigned_signed_pairs {
    ($f: ident) => {
        $f::<f32, u8, i8>();
        $f::<f32, u16, i16>();
        $f::<f32, u32, i32>();
        $f::<f32, usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds_and_unsigneds {
    ($f: ident) => {
        $f::<i8, u8>();
        $f::<i8, u16>();
        $f::<i8, u32>();
        $f::<i8, usize>();
        $f::<i16, u8>();
        $f::<i16, u16>();
        $f::<i16, u32>();
        $f::<i16, usize>();
        $f::<i32, u8>();
        $f::<i32, u16>();
        $f::<i32, u32>();
        $f::<i32, usize>();
        $f::<isize, u8>();
        $f::<isize, u16>();
        $f::<isize, u32>();
        $f::<isize, usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds_and_signeds {
    ($f: ident) => {
        $f::<i8, i8>();
        $f::<i8, i16>();
        $f::<i8, i32>();
        $f::<i8, isize>();
        $f::<i16, i8>();
        $f::<i16, i16>();
        $f::<i16, i32>();
        $f::<i16, isize>();
        $f::<i32, i8>();
        $f::<i32, i16>();
        $f::<i32, i32>();
        $f::<i32, isize>();
        $f::<isize, i8>();
        $f::<isize, i16>();
        $f::<isize, i32>();
        $f::<isize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_unsigneds {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_unsigneds!($f);
        apply_fn_to_signeds_and_unsigneds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_signeds {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_signeds!($f);
        apply_fn_to_signeds_and_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_primitive_ints {
    ($f: ident) => {
        apply_fn_to_primitive_ints_and_unsigneds!($f);
        apply_fn_to_primitive_ints_and_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_primitive_floats {
    ($m: tt) => {
        $m!(f32);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_primitive_float_unsigned_pairs {
    ($m: tt) => {
        $m!(f32, u32);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats {
    ($f: ident) => {
        $f::<f32>();
    };
}
