//! Platform-specific types, as defined by C.
//!
//! Code that interacts via FFI will almost certainly be using the
//! base types provided by C, which aren't nearly as nicely defined
//! as Rust's primitive types. This module provides types which will
//! match those defined by C, so that code that interacts with C will
//! refer to the correct types.

#![stable(feature = "core_ffi", since = "1.30.0")]
#![allow(non_camel_case_types)]

#[doc(inline)]
#[stable(feature = "core_c_str", since = "1.64.0")]
pub use self::c_str::CStr;
#[doc(inline)]
#[stable(feature = "cstr_from_bytes_until_nul", since = "1.69.0")]
pub use self::c_str::FromBytesUntilNulError;
#[doc(inline)]
#[stable(feature = "core_c_str", since = "1.64.0")]
pub use self::c_str::FromBytesWithNulError;
use crate::fmt;

#[stable(feature = "c_str_module", since = "1.88.0")]
pub mod c_str;

#[unstable(
    feature = "c_variadic",
    issue = "44930",
    reason = "the `c_variadic` feature has not been properly tested on all supported platforms"
)]
pub use self::va_list::{VaArgSafe, VaList, VaListImpl};

macro_rules! type_alias_no_nz {
    {
      $Docfile:tt, $Alias:ident = $Real:ty;
      $( $Cfg:tt )*
    } => {
        #[doc = include_str!($Docfile)]
        $( $Cfg )*
        pub type $Alias = $Real;
    }
}

// To verify that the NonZero types in this file's macro invocations correspond
//
//  perl -n < library/std/src/os/raw/mod.rs -e 'next unless m/type_alias\!/; die "$_ ?" unless m/, (c_\w+) = (\w+), NonZero_(\w+) = NonZero(\w+)/; die "$_ ?" unless $3 eq $1 and $4 eq ucfirst $2'
//
// NB this does not check that the main c_* types are right.

macro_rules! type_alias {
    {
      $Docfile:tt, $Alias:ident = $Real:ty, $NZAlias:ident = $NZReal:ty;
      $( $Cfg:tt )*
    } => {
        type_alias_no_nz! { $Docfile, $Alias = $Real; $( $Cfg )* }

        #[doc = concat!("Type alias for `NonZero` version of [`", stringify!($Alias), "`]")]
        #[unstable(feature = "raw_os_nonzero", issue = "82363")]
        $( $Cfg )*
        pub type $NZAlias = $NZReal;
    }
}

type_alias! { "c_char.md", c_char = c_char_definition::c_char, NonZero_c_char = c_char_definition::NonZero_c_char;
// Make this type alias appear cfg-dependent so that Clippy does not suggest
// replacing `0 as c_char` with `0_i8`/`0_u8`. This #[cfg(all())] can be removed
// after the false positive in https://github.com/rust-lang/rust-clippy/issues/8093
// is fixed.
#[cfg(all())]
#[doc(cfg(all()))]
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

type_alias! { "c_schar.md", c_schar = i8, NonZero_c_schar = NonZeroI8;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias! { "c_uchar.md", c_uchar = u8, NonZero_c_uchar = NonZeroU8;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias! { "c_short.md", c_short = i16, NonZero_c_short = NonZeroI16;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias! { "c_ushort.md", c_ushort = u16, NonZero_c_ushort = NonZeroU16;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

type_alias! { "c_int.md", c_int = c_int_definition::c_int, NonZero_c_int = c_int_definition::NonZero_c_int;
#[doc(cfg(all()))]
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias! { "c_uint.md", c_uint = c_int_definition::c_uint, NonZero_c_uint = c_int_definition::NonZero_c_uint;
#[doc(cfg(all()))]
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

type_alias! { "c_long.md", c_long = c_long_definition::c_long, NonZero_c_long = c_long_definition::NonZero_c_long;
#[doc(cfg(all()))]
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias! { "c_ulong.md", c_ulong = c_long_definition::c_ulong, NonZero_c_ulong = c_long_definition::NonZero_c_ulong;
#[doc(cfg(all()))]
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

type_alias! { "c_longlong.md", c_longlong = i64, NonZero_c_longlong = NonZeroI64;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias! { "c_ulonglong.md", c_ulonglong = u64, NonZero_c_ulonglong = NonZeroU64;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

type_alias_no_nz! { "c_float.md", c_float = f32;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias_no_nz! { "c_double.md", c_double = f64;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

/// Equivalent to C's `size_t` type, from `stddef.h` (or `cstddef` for C++).
///
/// This type is currently always [`usize`], however in the future there may be
/// platforms where this is not the case.
#[unstable(feature = "c_size_t", issue = "88345")]
pub type c_size_t = usize;

/// Equivalent to C's `ptrdiff_t` type, from `stddef.h` (or `cstddef` for C++).
// N.B., for LLVM to recognize the void pointer type and by extension
//     functions like malloc(), we need to have it represented as i8* in
//     LLVM bitcode. The enum used here ensures this and prevents misuse
//     of the "raw" type by only having private variants. We need two
//     variants, because the compiler complains about the repr attribute
//     otherwise and we need at least one variant as otherwise the enum
//     would be uninhabited and at least dereferencing such pointers would
//     be UB.
#[doc = include_str!("c_void.md")]
#[lang = "c_void"]
#[cfg_attr(not(doc), repr(u8))] // An implementation detail we don't want to show up in rustdoc
#[stable(feature = "core_c_void", since = "1.30.0")]
pub enum c_void {
    #[unstable(
        feature = "c_void_variant",
        reason = "temporary implementation detail",
        issue = "none"
    )]
    #[doc(hidden)]
    __variant1,
    #[unstable(
        feature = "c_void_variant",
        reason = "temporary implementation detail",
        issue = "none"
    )]
    #[doc(hidden)]
    __variant2,
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl fmt::Debug for c_void {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("c_void").finish()
    }
}

// Link the MSVC default lib
#[cfg(all(windows, target_env = "msvc"))]
#[link(
    name = "/defaultlib:msvcrt",
    modifiers = "+verbatim",
    cfg(not(target_feature = "crt-static"))
)]
#[link(name = "/defaultlib:libcmt", modifiers = "+verbatim", cfg(target_feature = "crt-static"))]
unsafe extern "C" {}
