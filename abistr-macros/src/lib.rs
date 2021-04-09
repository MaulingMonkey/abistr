extern crate proc_macro;

mod cstr;

#[proc_macro] pub fn cstr8_impl( input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<u8 >(input) }
#[proc_macro] pub fn cstr16_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<u16>(input) }
#[proc_macro] pub fn cstr32_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl::<u32>(input) }
