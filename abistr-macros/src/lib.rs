extern crate proc_macro;

mod cstr;

#[proc_macro]
pub fn cstr_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream { cstr::cstr_impl(input) }
