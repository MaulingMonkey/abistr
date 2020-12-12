use crate::*;

use std::ffi::*;
use std::os::raw::c_char;
use std::ptr::*;



/// Treat `self` as a C-style string
///
/// ### Safety
///
/// By implementing this trait, you promise that:
///
/// *   The returned pointer points to a valid `\0`-terminated string.
/// *   Said string remains valid and immutable until `self` is dropped or a `&mut self` method is called.
pub unsafe trait AsCStr {
    /// Returns a `\0`-terminated C string
    fn as_cstr(&self) -> *const c_char;
}

unsafe impl AsCStr for CStrNonNull<'_> {
    fn as_cstr(&self) -> *const c_char { self.as_ptr() }
}

unsafe impl AsCStr for &'_ CStr {
    fn as_cstr(&self) -> *const c_char { self.as_ptr() }
}

unsafe impl AsCStr for CString {
    fn as_cstr(&self) -> *const c_char { self.as_ptr() }
}



/// Treat `self` as a C-style string or <code>[null]\(\)</code>
///
/// ### Safety
///
/// By implementing this trait, you promise that:
///
/// *   The returned pointer is either <code>[null]\(\)</code>, or points to a `\0`-terminated string.
/// *   If pointing to a string, said string remains valid and immutable until `self` is dropped or a `&mut self` method is called.
pub unsafe trait AsOptCStr {
    /// Returns a `\0`-terminated C string, or <code>[null]\(\)</code>.
    fn as_opt_cstr(&self) -> *const c_char;
}

unsafe impl AsOptCStr for () {
    fn as_opt_cstr(&self) -> *const c_char { null() }
}

unsafe impl AsOptCStr for CStrPtr<'_> {
    fn as_opt_cstr(&self) -> *const c_char { self.as_ptr() }
}

unsafe impl<T: AsCStr> AsOptCStr for T {
    fn as_opt_cstr(&self) -> *const c_char { self.as_cstr() }
}

unsafe impl<T: AsCStr> AsOptCStr for Option<T> {
    fn as_opt_cstr(&self) -> *const c_char { self.as_ref().map_or(null(), |s| s.as_cstr()) }
}
