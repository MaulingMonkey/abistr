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
pub unsafe trait AsCStr<C = c_char> {
    /// Returns a `\0`-terminated C string
    fn as_cstr(&self) -> *const C;
}

unsafe impl AsCStr<i8   > for CStrNonNull<'_, u8    > { fn as_cstr(&self) -> *const i8  { self.as_ptr().cast() } }
unsafe impl AsCStr<u8   > for CStrNonNull<'_, u8    > { fn as_cstr(&self) -> *const u8  { self.as_ptr().cast() } }
unsafe impl AsCStr<u16  > for CStrNonNull<'_, u16   > { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
unsafe impl AsCStr<u32  > for CStrNonNull<'_, u32   > { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }

unsafe impl AsCStr<i8> for &'_ CStr { fn as_cstr(&self) -> *const i8 { self.as_ptr().cast() } }
unsafe impl AsCStr<u8> for &'_ CStr { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }

unsafe impl AsCStr<i8> for CString { fn as_cstr(&self) -> *const i8 { self.as_ptr().cast() } }
unsafe impl AsCStr<u8> for CString { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }

#[cfg(feature = "widestring")] unsafe impl AsCStr<u16> for &'_ widestring::U16CStr { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
#[cfg(feature = "widestring")] unsafe impl AsCStr<u32> for &'_ widestring::U32CStr { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }

#[cfg(feature = "widestring")] unsafe impl AsCStr<u16> for widestring::U16CString { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
#[cfg(feature = "widestring")] unsafe impl AsCStr<u32> for widestring::U32CString { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }



/// Treat `self` as a C-style string or <code>[null]\(\)</code>
///
/// ### Safety
///
/// By implementing this trait, you promise that:
///
/// *   The returned pointer is either <code>[null]\(\)</code>, or points to a `\0`-terminated string.
/// *   If pointing to a string, said string remains valid and immutable until `self` is dropped or a `&mut self` method is called.
pub unsafe trait AsOptCStr<C = c_char> {
    /// Returns a `\0`-terminated C string, or <code>[null]\(\)</code>.
    fn as_opt_cstr(&self) -> *const C;
}

unsafe impl AsOptCStr<i8    > for () { fn as_opt_cstr(&self) -> *const i8     { null() } }
unsafe impl AsOptCStr<u8    > for () { fn as_opt_cstr(&self) -> *const u8     { null() } }
unsafe impl AsOptCStr<u16   > for () { fn as_opt_cstr(&self) -> *const u16    { null() } }
unsafe impl AsOptCStr<u32   > for () { fn as_opt_cstr(&self) -> *const u32    { null() } }

unsafe impl AsOptCStr<i8    > for CStrPtr<'_, u8 > { fn as_opt_cstr(&self) -> *const i8     { self.as_ptr().cast() } }
unsafe impl AsOptCStr<u8    > for CStrPtr<'_, u8 > { fn as_opt_cstr(&self) -> *const u8     { self.as_ptr().cast() } }
unsafe impl AsOptCStr<u16   > for CStrPtr<'_, u16> { fn as_opt_cstr(&self) -> *const u16    { self.as_ptr() } }
unsafe impl AsOptCStr<u32   > for CStrPtr<'_, u32> { fn as_opt_cstr(&self) -> *const u32    { self.as_ptr() } }

unsafe impl<T: AsCStr<i8    >> AsOptCStr<i8    > for T { fn as_opt_cstr(&self) -> *const i8     { self.as_cstr() } }
unsafe impl<T: AsCStr<u8    >> AsOptCStr<u8    > for T { fn as_opt_cstr(&self) -> *const u8     { self.as_cstr() } }
unsafe impl<T: AsCStr<u16   >> AsOptCStr<u16   > for T { fn as_opt_cstr(&self) -> *const u16    { self.as_cstr() } }
unsafe impl<T: AsCStr<u32   >> AsOptCStr<u32   > for T { fn as_opt_cstr(&self) -> *const u32    { self.as_cstr() } }

unsafe impl<T: AsCStr<i8    >> AsOptCStr<i8    > for Option<T> { fn as_opt_cstr(&self) -> *const i8     { self.as_ref().map_or(null(), |s| s.as_cstr()) } }
unsafe impl<T: AsCStr<u8    >> AsOptCStr<u8    > for Option<T> { fn as_opt_cstr(&self) -> *const u8     { self.as_ref().map_or(null(), |s| s.as_cstr()) } }
unsafe impl<T: AsCStr<u16   >> AsOptCStr<u16   > for Option<T> { fn as_opt_cstr(&self) -> *const u16    { self.as_ref().map_or(null(), |s| s.as_cstr()) } }
unsafe impl<T: AsCStr<u32   >> AsOptCStr<u32   > for Option<T> { fn as_opt_cstr(&self) -> *const u32    { self.as_ref().map_or(null(), |s| s.as_cstr()) } }
