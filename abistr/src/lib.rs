//! # C ABI compatible string types
//!
//! While rust's stdlib has some good basic options for C-string support, it has some glaring issues when dealing with
//! C functions that accept or return arrays of structures containing C-string pointers or arrays.  This crate aims to
//! help fill that niche with safe, fast, ergonomic alternatives.
//!
//! | C type                                        | ABI compatible Rust                                       | [null]            |
//! | --------------------------------------------- | --------------------------------------------------------- | ----------------- |
//! | `const char *`                                | <code>[abistr::CStrPtr]<[encoding::Unknown8]></code>      <br> <code>[abistr::CStrPtr]<[encoding::Utf8ish] ></code>        | `""`
//! | `const char *`                                | <code>[Option]<[abistr::CStrNonNull]<[Unknown8]>></code>  <br> <code>[Option]<[abistr::CStrNonNull]<[Utf8ish] >></code>    | [`None`]
//! | `const char * __attribute__((nonnull))`       | <code>[abistr::CStrNonNull]<[Unknown8]></code>            <br> <code>[abistr::CStrNonNull]<[Utf8ish] ></code>              | ❌ undef ❌
//! | `char struct_member[128];`                    | <code>[abistr::CStrBuf]<[Unknown8], 128></code>           <br> <code>[abistr::CStrBuf]<[Utf8ish] , 128></code>             | <span style="opacity: 33%">N/A</span>
//! | **C++20**                                     | **ABI compatible Rust**
//! | `const char8_t  *`                            | <code>[abistr::CStrPtr]<[encoding::Utf8ish] ></code>      | `""`
//! | `const char16_t *`                            | <code>[abistr::CStrPtr]<[encoding::Utf16ish]></code>      | `""`
//! | `const char32_t *`                            | <code>[abistr::CStrPtr]<[encoding::Utf32ish]></code>      | `""`
//! | `const char8_t  *`                            | <code>[Option]<[abistr::CStrNonNull]<[Utf8ish] >></code>  | [`None`]
//! | `const char16_t *`                            | <code>[Option]<[abistr::CStrNonNull]<[Utf16ish]>></code>  | [`None`]
//! | `const char32_t *`                            | <code>[Option]<[abistr::CStrNonNull]<[Utf32ish]>></code>  | [`None`]
//! | `const char8_t  * __attribute__((nonnull))`   | <code>[abistr::CStrNonNull]<[Utf8ish] ></code>            | ❌ undef ❌
//! | `const char16_t * __attribute__((nonnull))`   | <code>[abistr::CStrNonNull]<[Utf16ish]></code>            | ❌ undef ❌
//! | `const char32_t * __attribute__((nonnull))`   | <code>[abistr::CStrNonNull]<[Utf32ish]></code>            | ❌ undef ❌
//! | `char8_t  struct_member[128];`                | <code>[abistr::CStrBuf]<[Utf8ish],  128></code>           | <span style="opacity: 33%">N/A</span>
//! | `char16_t struct_member[128];`                | <code>[abistr::CStrBuf]<[Utf16ish], 128></code>           | <span style="opacity: 33%">N/A</span>
//! | `char32_t struct_member[128];`                | <code>[abistr::CStrBuf]<[Utf32ish], 128></code>           | <span style="opacity: 33%">N/A</span>
//! | **iOS, OS X**                                 | **ABI compatible Rust**
//! | `const unichar *`                             | <code>[abistr::CStrPtr]<[encoding::Utf16ish]></code>      | `""`
//! | `const wchar_t *`                             | <code>[abistr::CStrPtr]<[encoding::Utf32ish]></code>      | `""`
//! | `const unichar *`                             | <code>[Option]<[abistr::CStrPtr]<[Utf16ish]>></code>      | [`None`]
//! | `const wchar_t *`                             | <code>[Option]<[abistr::CStrPtr]<[Utf32ish]>></code>      | [`None`]
//! | `const unichar * __attribute__((nonnull))`    | <code>[abistr::CStrNonNull]<[Utf16ish]></code>            | ❌ undef ❌
//! | `const wchar_t * __attribute__((nonnull))`    | <code>[abistr::CStrNonNull]<[Utf32ish]></code>            | ❌ undef ❌
//! | `unichar struct_member[128];`                 | <code>[abistr::CStrBuf]<[Utf16ish], 128></code>           | <span style="opacity: 33%">N/A</span>
//! | `wchar_t struct_member[128];`                 | <code>[abistr::CStrBuf]<[Utf32ish], 128></code>           | <span style="opacity: 33%">N/A</span>
//! | **Linux**                                     | **ABI compatible Rust**
//! | `const wchar_t *`                             | <code>[abistr::CStrPtr]<[encoding::Utf32ish]></code>      | `""`
//! | `const wchar_t *`                             | <code>[Option]<[abistr::CStrPtr]<[Utf32ish]>></code>      | [`None`]
//! | `const wchar_t * __attribute__((nonnull))`    | <code>[abistr::CStrNonNull]<[Utf32ish]></code>            | ❌ undef ❌
//! | `wchar_t struct_member[128];`                 | <code>[abistr::CStrBuf]<[Utf32ish], 128></code>           | <span style="opacity: 33%">N/A</span>
//! | **Windows**                                   | **ABI compatible Rust**
//! | `const wchar_t *`                             | <code>[abistr::CStrPtr]<[encoding::Utf16ish]></code>      | `""`
//! | `const wchar_t *`                             | <code>[Option]<[abistr::CStrPtr]<[Utf16ish]>></code>      | [`None`]
//! | `const wchar_t * __attribute__((nonnull))`    | <code>[abistr::CStrNonNull]<[Utf16ish]></code>            | ❌ undef ❌
//! | `wchar_t struct_member[128];`                 | <code>[abistr::CStrBuf]<[Utf16ish], 128></code>           | <span style="opacity: 33%">N/A</span>
//!
//! # Alternatives
//!
//! <code>\*const [c_char]</code>
//! * Pro:  Can't get any simpler for basic interop!
//! * Con:  Requires `unsafe` to so much as shake a stick at.
//! * Con:  Easy to create undefined behavior by messing up edge cases involving [null].
//! * Con:  Easy to create undefined behavior by creating dangling pointers and other lifetime issues (raw pointers have no lifetimes.)
//! * Con:  Fairly unergonomic to use directly.
//!
//! <code>&[std::ffi::CStr]</code>
//! * Pro:  Relatively safe!
//! * Con:  Immediate `O(n)` length check on construction, even if you never use the string.
//! * Con:  Being a [DST] (at least at the time of writing / rust 1.48.0), this isn't ABI compatible with `*const c_char` and thus cannot be embedded in zero-conversion structures.
//!
//! <code>[std::ffi::CString]</code> - per <code>&[std::ffi::CStr]</code>, but also:
//! * Pro:  Dynamically allocated!
//! * Con:  Dynamically allocated.
//!
//! [DST]:      https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts

#![debugger_visualizer(natvis_file = "../abistr.natvis")]
#![no_std]
#![warn(missing_docs)]
#![warn(unsafe_op_in_unsafe_fn)]

#[doc(hidden)] pub extern crate abistr_macros;
#[cfg(feature = "alloc" )] extern crate alloc;
#[cfg(feature = "std"   )] extern crate std;

#[cfg(doc)] use crate as abistr;
#[cfg(doc)] use core::ptr::*;
#[cfg(doc)] #[doc(hidden)] pub use encoding::*;
#[cfg(doc)] #[cfg(feature = "alloc")] use alloc::string::String;
#[cfg(doc)] #[cfg(feature = "std"  )] use std::ffi::*;

#[macro_use] mod macros;

mod as_traits;                          pub use as_traits::*;
//mod buffers;                            pub use buffers::*;
mod errors;                             pub use errors::*;
mod estring;                            pub use estring::*;
pub mod encoding;                       pub use encoding::Encoding; use encoding::*;
mod fmt;
mod pointers;                           pub use pointers::*;
mod try_into_as_traits;                 pub use try_into_as_traits::*;
mod unit;                               pub use unit::*;

pub(crate) mod private {
    pub use crate::unit::private::*;
}
