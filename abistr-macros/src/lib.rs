extern crate proc_macro;

mod cstr; use cstr::*;

//#[proc_macro] pub fn ascii( input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<ASCII>(input) }

#[proc_macro] pub fn unknown8( input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Unknown8 >(input) }
#[proc_macro] pub fn unknown16(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Unknown16>(input) }
#[proc_macro] pub fn unknown32(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Unknown32>(input) }

#[proc_macro] pub fn utf8( input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Utf8 >(input) }
#[proc_macro] pub fn utf16(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Utf16>(input) }
#[proc_macro] pub fn utf32(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Utf32>(input) }

#[proc_macro] pub fn utf8ish( input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Utf8ish >(input) }
#[proc_macro] pub fn utf16ish(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Utf16ish>(input) }
#[proc_macro] pub fn utf32ish(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<Utf32ish>(input) }
