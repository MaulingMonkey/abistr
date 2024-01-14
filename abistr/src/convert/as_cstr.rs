use crate::*;
use crate::encoding::*;



/// Treat `self` as a C-style string
///
/// ### Safety
///
/// By implementing this trait, you promise that:
///
/// *   The returned pointer points to a valid `\0`-terminated string.
/// *   Said string remains valid and immutable until `self` is dropped or a `&mut self` method is called.
pub unsafe trait AsCStr<E: Encoding> {
    /// Returns a `\0`-terminated C string
    fn as_cstr(&self) -> *const E::Unit;
}

/* abistr */ const _ : () = {
    unsafe impl<E: Encoding> AsCStr<E>      for CStrNonNull<'_, E>          { fn as_cstr(&self) -> *const E::Unit { self.as_ptr().cast() } }

    unsafe impl AsCStr<Utf8ish >            for CStrNonNull<'_, Utf8>       { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown8>            for CStrNonNull<'_, Utf8>       { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown8>            for CStrNonNull<'_, Utf8ish>    { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }

    unsafe impl AsCStr<Utf16ish >           for CStrNonNull<'_, Utf16>      { fn as_cstr(&self) -> *const u16 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown16>           for CStrNonNull<'_, Utf16>      { fn as_cstr(&self) -> *const u16 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown16>           for CStrNonNull<'_, Utf16ish>   { fn as_cstr(&self) -> *const u16 { self.as_ptr().cast() } }

    unsafe impl AsCStr<Utf32ish >           for CStrNonNull<'_, Utf32>      { fn as_cstr(&self) -> *const u32 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown32>           for CStrNonNull<'_, Utf32>      { fn as_cstr(&self) -> *const u32 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown32>           for CStrNonNull<'_, Utf32ish>   { fn as_cstr(&self) -> *const u32 { self.as_ptr().cast() } }
};

/* core */ const _ : () = {
    unsafe impl AsCStr<Unknown8 >           for &'_ core::ffi::CStr         { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
};

/* core */ #[cfg(feature = "assume-core-ffi-cstr-utf8ish")] const _ : () = {
    unsafe impl AsCStr<Utf8ish  >           for &'_ core::ffi::CStr         { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
};

#[cfg(feature = "alloc")] const _ : () = {
    unsafe impl AsCStr<Unknown8 >           for &'_ alloc::ffi::CString     { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Unknown8 >           for     alloc::ffi::CString     { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
};

#[cfg(feature = "alloc")] #[cfg(feature = "assume-core-ffi-cstr-utf8ish")] const _ : () = {
    unsafe impl AsCStr<Utf8ish  >           for &'_ alloc::ffi::CString     { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
    unsafe impl AsCStr<Utf8ish  >           for     alloc::ffi::CString     { fn as_cstr(&self) -> *const u8 { self.as_ptr().cast() } }
};

#[cfg(feature = "widestring")] const _ : () = {
    unsafe impl AsCStr<Unknown16>           for &'_ widestring::U16CStr     { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
    unsafe impl AsCStr<Unknown32>           for &'_ widestring::U32CStr     { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }
    unsafe impl AsCStr<Unknown16>           for &'_ widestring::U16CString  { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
    unsafe impl AsCStr<Unknown32>           for &'_ widestring::U32CString  { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }
    unsafe impl AsCStr<Unknown16>           for     widestring::U16CString  { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
    unsafe impl AsCStr<Unknown32>           for     widestring::U32CString  { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }

    // while there is a Utf{16,32}String, there is no Utf{16,32}CString which would be appropriate for encoding::UTF{16,32}ish.
};

#[cfg(all(feature = "widestring", feature = "assume-widestring-utfish"))] const _ : () = {
    unsafe impl AsCStr<Utf16ish >           for &'_ widestring::U16CStr     { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
    unsafe impl AsCStr<Utf32ish >           for &'_ widestring::U32CStr     { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }
    unsafe impl AsCStr<Utf16ish >           for &'_ widestring::U16CString  { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
    unsafe impl AsCStr<Utf32ish >           for &'_ widestring::U32CString  { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }
    unsafe impl AsCStr<Utf16ish >           for     widestring::U16CString  { fn as_cstr(&self) -> *const u16 { self.as_ptr() } }
    unsafe impl AsCStr<Utf32ish >           for     widestring::U32CString  { fn as_cstr(&self) -> *const u32 { self.as_ptr() } }
};
