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

type_alias! { "c_intmax_t.md", c_intmax_t = i64, NonZero_c_intmax_t = NonZeroI64;
#[unstable(feature = "c_size_t", issue = "88345")]
}
type_alias! { "c_uintmax_t.md", c_uintmax_t = u64, NonZero_c_uintmax_t = NonZeroU64;
#[unstable(feature = "c_size_t", issue = "88345")]
}

type_alias! { "c_intptr_t.md", c_intptr_t = isize, NonZero_c_intptr_t = NonZeroIsize;
#[unstable(feature = "c_size_t", issue = "88345")]
}
type_alias! { "c_uintptr_t.md", c_uintptr_t = usize, NonZero_c_uintptr_t = NonZeroUsize;
#[unstable(feature = "c_size_t", issue = "88345")]
}

type_alias! { ".md", c_wchar_t = isize, NonZero_c_wchar_t = NonZeroIsize;
#[unstable(feature = "c_size_t", issue = "88345")]
}

type_alias! { "c_wchar_t.md", c_wchar_t = c_wchar_t_definition::c_wchar_t, NonZero_c_wchar_t = c_wchar_t_definition::NonZero_c_wchar_t;
#[doc(cfg(all()))]
#[unstable(feature = "c_size_t", issue = "88345")]
}

type_alias_no_nz! { "c_float.md", c_float = f32;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}
type_alias_no_nz! { "c_double.md", c_double = f64;
#[stable(feature = "core_ffi_c", since = "1.64.0")]
}

type_alias! { "c_size_t.md", c_size_t = usize, NonZero_c_size_t = NonZeroUsize;
#[unstable(feature = "c_size_t", issue = "88345")]
}
type_alias! { "c_ptrdiff_t.md", c_ptrdiff_t = isize, NonZero_c_ptrdiff_t = NonZeroIsize;
#[unstable(feature = "c_size_t", issue = "88345")]
}
type_alias! { "c_ssize_t.md", c_ssize_t = isize, NonZero_c_ssize_t = NonZeroIsize;
#[unstable(feature = "c_size_t", issue = "88345")]
}

mod c_char_definition {
    cfg_if! {
        // These are the targets on which c_char is unsigned.
        if #[cfg(any(
            all(
                target_os = "linux",
                any(
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "hexagon",
                    target_arch = "powerpc",
                    target_arch = "powerpc64",
                    target_arch = "s390x",
                    target_arch = "riscv64",
                    target_arch = "riscv32"
                )
            ),
            all(target_os = "android", any(target_arch = "aarch64", target_arch = "arm")),
            all(target_os = "l4re", target_arch = "x86_64"),
            all(
                any(target_os = "freebsd", target_os = "openbsd"),
                any(
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "powerpc",
                    target_arch = "powerpc64",
                    target_arch = "riscv64"
                )
            ),
            all(
                target_os = "netbsd",
                any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc")
            ),
            all(
                target_os = "vxworks",
                any(
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "powerpc64",
                    target_arch = "powerpc"
                )
            ),
            all(
                target_os = "fuchsia",
                any(target_arch = "aarch64", target_arch = "riscv64")
            ),
            all(target_os = "nto", target_arch = "aarch64"),
            target_os = "horizon"
        ))] {
            pub type c_char = u8;
            pub type NonZero_c_char = crate::num::NonZeroU8;
        } else {
            // On every other target, c_char is signed.
            pub type c_char = i8;
            pub type NonZero_c_char = crate::num::NonZeroI8;
        }
    }
}

mod c_int_definition {
    cfg_if! {
        if #[cfg(any(target_arch = "avr", target_arch = "msp430"))] {
            pub type c_int = i16;
            pub type NonZero_c_int = crate::num::NonZeroI16;
            pub type c_uint = u16;
            pub type NonZero_c_uint = crate::num::NonZeroU16;
        } else {
            pub type c_int = i32;
            pub type NonZero_c_int = crate::num::NonZeroI32;
            pub type c_uint = u32;
            pub type NonZero_c_uint = crate::num::NonZeroU32;
        }
    }
}

mod c_long_definition {
    cfg_if! {
        if #[cfg(all(target_pointer_width = "64", not(windows)))] {
            pub type c_long = i64;
            pub type NonZero_c_long = crate::num::NonZeroI64;
            pub type c_ulong = u64;
            pub type NonZero_c_ulong = crate::num::NonZeroU64;
        } else {
            // The minimal size of `long` in the C standard is 32 bits
            pub type c_long = i32;
            pub type NonZero_c_long = crate::num::NonZeroI32;
            pub type c_ulong = u32;
            pub type NonZero_c_ulong = crate::num::NonZeroU32;
        }
    }
}

mod c_wchar_t_definition {
    cfg_if! {
        if #[cfg(windows)] {
            pub type c_wchar_t = u16;
            pub type NonZero_c_wchar_t = crate::num::NonZeroU16;
        } else {
            pub type c_wchar_t = u32;
            pub type NonZero_c_wchar_t = crate::num::NonZeroU16;
        }
    }
}

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
