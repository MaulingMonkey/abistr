//! # C ABI compatible string types
//!
//! While rust's stdlib has some good basic options for C-string support, it has some glaring issues when dealing with
//! C functions that accept or return arrays of structures containing C-string pointers or arrays.  This crate aims to
//! help fill that niche with safe, fast, ergonomic alternatives.
//!
//! | C type                                    | ABI compatible Rust                           | [null]            |
//! | ----------------------------------------- | --------------------------------------------- | ----------------- |
//! | `const char*`                             | <code>[abistr::CStrPtr]</code>                | `""`
//! | `const char*`                             | <code>[Option]<[abistr::CStrNonNull]></code>  | [`None`]
//! | `const char* __attribute__((nonnull))`    | <code>[abistr::CStrNonNull]</code>            | ❌ undefined behavior ❌
//! | `char struct_member[128];`                | <code>[abistr::CStrBuf]<\[[u8]; 128\]></code> | <span style="opacity: 33%">N/A</span>
//!
//! # Alternatives
//!
//! <code>\*const [std::os::raw::c_char]</code>
//! * Pro:  Can't get any simpler for basic interop!
//! * Con:  Requires `unsafe` to so much as shake a stick at.
//! * Con:  Easy to create undefined behavior by messing up edge cases involving [null].
//! * Con:  Easy to create undefined behavior by creating dangling poniters and other lifetime issues (raw pointers have no lifetimes.)
//! * Con:  Fairly unergonomic to use directly.
//!
//! <code>[`std::ffi::CStr`]</code>
//! * Pro:  Relatively safe!
//! * Con:  Immediate `O(n)` length check on construction, even if you never use the string.
//! * Con:  Being a [DST] (at least at the time of writing / rust 1.48.0), this isn't ABI compatible with `*const c_char` and thus cannot be embedded in zero-conversion structures.
//!
//! <code>[std::ffi::CString]</code> - per [`std::ffi::CStr`], but also:
//! * Pro:  Dynamically allocated!
//! * Con:  Dynamically allocated.
//!
//! [DST]:      https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts

#![warn(missing_docs)]

#[doc(hidden)] pub extern crate abistr_macros;

#[cfg(doc)] use crate as abistr;
#[cfg(doc)] use std::ptr::*;

#[macro_use] mod macros;

mod as_traits;                          pub use as_traits::*;
mod buffers;                            pub use buffers::*;
mod errors;                             pub use errors::*;
mod fmt;
mod pointers;                           pub use pointers::*;
mod try_into_as_traits;                 pub use try_into_as_traits::*;
