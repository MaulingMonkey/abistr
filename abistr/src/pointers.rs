use crate::*;

#[cfg(feature = "widestring-0-4")] use widestring_0_4::*;

use std::borrow::Cow;
use std::ffi::*;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
#[allow(unused_imports)] use std::os::raw::c_char;
use std::ptr::*;
use std::str::Utf8Error;



/// <code>[CStrPtr]&lt;[Unit]&gt;</code> is ABI compatible with <code>*const [Unit]</code>.  <code>[null]\(\)</code> is treated as an empty string.
///
/// If you want to treat <code>[null]\(\)</code> as [`None`], use <code>[Option]<[CStrNonNull]></code> instead.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CStrPtr<'s, U: Unit = u8> {
    ptr:        *const U::CChar,
    phantom:    PhantomData<&'s U::CChar>,
}

impl<'s, U: Unit> CStrPtr<'s, U> {
    /// A <code>[null]\(\)</code> [CStrPtr].
    pub const NULL : Self = Self { ptr: 0 as *const _, phantom: PhantomData };

    /// Convert a raw C-string into a [`CStrPtr`].  Note that the lifetime of the returned reference is unbounded!
    ///
    /// ### Safety
    /// *   `ptr` cannot be null
    /// *   `ptr` must point to a `\0`-terminated C string
    /// *   The underlying C-string cannot change for the duration of the lifetime `'s`.
    /// *   The lifetime `'s` is unbounded by this fn.  Very easy to accidentally extend.  Be careful!
    pub unsafe fn from_ptr_unbounded(ptr: *const U::CChar) -> Self { Self { ptr, phantom: PhantomData } }

    /// Convert a raw slice of units into a [`CStrPtr`].  `units` should end with `\0`, but contain no interior `\0`s otherwise.
    pub fn from_units_with_nul(units: &'s [U]) -> Result<Self, FromUnitsWithNulError> {
        let (nul, interior) = units.split_last().ok_or(FromUnitsWithNulError(()))?;
        if *nul != U::NUL { return Err(FromUnitsWithNulError(())); }
        if interior.contains(&U::NUL) { return Err(FromUnitsWithNulError(())); }
        Ok(unsafe { Self::from_ptr_unbounded(units.as_ptr().cast()) })
    }

    /// Convert a raw slice of units to a [`CStrPtr`].  The resulting string will be terminated at the first `\0` in `units`.
    ///
    /// ### Safety
    /// *   `units` must contain at least one `\0`.
    pub unsafe fn from_units_with_nul_unchecked(units: &'s [U]) -> Self {
        debug_assert!(units.contains(&U::NUL), "Undefined Behavior: `units` contained no `\0`!");
        Self::from_ptr_unbounded(units.as_ptr() as *const _)
    }

    /// Treat `self` as a raw, possibly <code>[null]\(\)</code> C string.
    pub fn as_ptr(&self) -> *const U::CChar { self.ptr.cast() }

    /// Checks if `self` is <code>[null]\(\)</code>.
    pub fn is_null(&self) -> bool { self.ptr.is_null() }

    /// Checks if `self` is empty (either null, or the first character is `\0`.)
    pub fn is_empty(&self) -> bool { self.ptr.is_null() || U::NUL == unsafe { *self.ptr.cast() } }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, **excluding** the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units(&self) -> &'s [U] {
        if self.ptr.is_null() { return &[]; }
        let start = self.ptr.cast();
        unsafe { std::slice::from_raw_parts(start, strlen(start)) }
    }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, including the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units_with_nul(&self) -> &'s [U] {
        if self.ptr.is_null() { return U::EMPTY; }
        let start = self.ptr.cast();
        unsafe { std::slice::from_raw_parts(start, strlen(start) + 1) }
    }

    /// Convert `self` to a <code>[Cow]\<[str]\></code>.
    ///
    /// `O(n)` to find the terminal `\0` and validate, and to convert UTF8ish data to UTF8 if necesssary.
    pub fn to_string_lossy(&self) -> Cow<'s, str> { U::to_string_lossy(self.to_units()) }
}

impl<'s> CStrPtr<'s, u8> {
    #[doc(hidden)] pub fn from_bytes_with_nul(bytes: &'s [u8]) -> Result<Self, FromBytesWithNulError> { CStr::from_bytes_with_nul(bytes).map(Self::from) }
    #[doc(hidden)] pub unsafe fn from_bytes_with_nul_unchecked(bytes: &'s [u8]) -> Self { Self::from_units_with_nul_unchecked(bytes) }
    #[doc(hidden)] pub fn to_bytes(&self) -> &'s [u8] { self.to_units() }
    #[doc(hidden)] pub fn to_bytes_with_nul(&self) -> &'s [u8] { self.to_units_with_nul() }

    /// Convert `self` to a [`std::ffi::CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_cstr(&self) -> &'s CStr {
        if self.ptr.is_null() {
            unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") }
        } else {
            unsafe { CStr::from_ptr(self.ptr) }
        }
    }

    /// Convert `self` to a <code>&[str]</code>.
    ///
    /// `O(n)` to find the terminal `\0` and validate UTF8.
    pub fn to_str(&self) -> Result<&'s str, Utf8Error> { self.to_cstr().to_str() }
}

#[cfg(feature = "widestring-0-4")] impl<'s> CStrPtr<'s, u16> {
    /// Convert `self` to a [`U16CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_u16cstr(&self) -> &'s U16CStr {
        if self.ptr.is_null() {
            Default::default()
        } else {
            unsafe { U16CStr::from_ptr_with_nul(self.ptr, strlen(self.ptr)) }
        }
    }

    /// Convert `self` to a [`U16Str`].
    ///
    /// `O(n)` to find the terminal `\0`
    pub fn to_u16str(&self) -> &'s U16Str { U16Str::from_slice(self.to_units()) }
}

#[cfg(feature = "widestring-0-4")] impl<'s> CStrPtr<'s, u32> {
    /// Convert `self` to a [`U32CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_u32cstr(&self) -> &'s U32CStr {
        if self.ptr.is_null() {
            Default::default()
        } else {
            unsafe { U32CStr::from_ptr_with_nul(self.ptr, strlen(self.ptr)) }
        }
    }

    /// Convert `self` to a [`U32Str`].
    ///
    /// `O(n)` to find the terminal `\0`
    pub fn to_u32str(&self) -> &'s U32Str { U32Str::from_slice(self.to_units()) }
}

impl<U: Unit> Debug for CStrPtr<'_, U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { U::debug(self.to_units(), f) }
}

impl<U: Unit> Default for CStrPtr<'_, U> {
    fn default() -> Self { Self { ptr: U::EMPTY.as_ptr().cast(), phantom: PhantomData } }
}

impl<'s> From<CStrPtr<'s>> for &'s CStr {
    fn from(s: CStrPtr<'s>) -> Self { s.to_cstr() }
}

impl<'s> From<&'s CStr> for CStrPtr<'s> {
    fn from(s: &'s CStr) -> Self { unsafe { CStrPtr::from_ptr_unbounded(s.as_ptr().cast()) } }
}



/// <code>[Option]&lt;[CStrNonNull]&lt;[Unit]&gt;&gt;</code> is ABI compatible with <code>*const [Unit]</code>.
///
/// If you want to treat <code>[null]\(\)</code> as `""`, use [`CStrPtr`] instead.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CStrNonNull<'s, U: Unit = u8> {
    ptr:        NonNull<U::CChar>,
    phantom:    PhantomData<&'s U::CChar>,
}

impl<'s, U: Unit> CStrNonNull<'s, U> {
    /// Convert a raw C-string into a [`CStrPtr`].  Note that the lifetime of the returned reference is unbounded!
    ///
    /// ### Safety
    /// *   `ptr` cannot be null
    /// *   `ptr` must point to a `\0`-terminated C string
    /// *   The underlying C-string cannot change for the duration of the lifetime `'s`.
    /// *   The lifetime `'s` is unbounded by this fn.  Very easy to accidentally extend.  Be careful!
    pub unsafe fn from_ptr_unchecked_unbounded(ptr: *const U::CChar) -> Self { Self { ptr: NonNull::new_unchecked(ptr as *mut _), phantom: PhantomData } }

    /// Convert a raw slice of units into a [`CStrNonNull`].  `units` should end with `\0`, but contain no interior `\0`s otherwise.
    pub fn from_units_with_nul(units: &'s [U]) -> Result<Self, FromUnitsWithNulError> {
        let (nul, interior) = units.split_last().ok_or(FromUnitsWithNulError(()))?;
        if *nul != U::NUL { return Err(FromUnitsWithNulError(())); }
        if interior.contains(&U::NUL) { return Err(FromUnitsWithNulError(())); }
        Ok(unsafe { Self::from_ptr_unchecked_unbounded(units.as_ptr().cast()) })
    }

    /// Convert a raw slice of units to a [`CStrNonNull`].  The resulting string will be terminated at the first `\0` in `units`.
    ///
    /// ### Safety
    /// *   `units` must contain at least one `\0`.
    pub unsafe fn from_units_with_nul_unchecked(units: &'s [U]) -> Self {
        debug_assert!(units.contains(&U::NUL), "Undefined Behavior: `units` contained no `\0`!");
        Self::from_ptr_unchecked_unbounded(units.as_ptr() as *const _)
    }

    /// Use [`from_units_with_nul_unchecked`](Self::from_units_with_nul_unchecked) or [`cstr!`] instead!
    #[doc(hidden)] // This fn only exists to allow the use of the totally safe `cstr!` macro in `#![forbid(unsafe_code)]` codebases.
    pub fn zzz_unsound_do_not_call_this_directly_from_macro_units_with_nul(units: &'s [U]) -> Self {
        unsafe { Self::from_units_with_nul_unchecked(units) }
    }

    /// Treat `self` as a raw C string.
    pub fn as_ptr(&self) -> *const U::CChar { self.ptr.as_ptr().cast() }

    /// Treat `self` as a [`NonNull`] C string.
    pub fn as_non_null(&self) -> NonNull<U::CChar> { self.ptr }

    /// Checks if `self` is empty (either <code>[null]\(\)</code>, or the first character is `\0`.)
    pub fn is_empty(&self) -> bool { U::NUL == unsafe { *self.ptr.as_ptr().cast() } }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, **excluding** the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units(&self) -> &'s [U] {
        let start = self.ptr.as_ptr().cast();
        unsafe { std::slice::from_raw_parts(start, strlen(start) + 0) }
    }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, including the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units_with_nul(&self) -> &'s [U] {
        let start = self.ptr.as_ptr().cast();
        unsafe { std::slice::from_raw_parts(start, strlen(start) + 1) }
    }

    /// Convert `self` to a <code>[Cow]\<[str]\></code>.
    ///
    /// `O(n)` to find the terminal `\0` and validate, and to convert UTF8ish data to UTF8 if necesssary.
    pub fn to_string_lossy(&self) -> Cow<'s, str> { U::to_string_lossy(self.to_units()) }
}

impl<'s> CStrNonNull<'s, u8> {
    #[doc(hidden)] pub fn from_bytes_with_nul(bytes: &'s [u8]) -> Result<Self, FromBytesWithNulError> { CStr::from_bytes_with_nul(bytes).map(Self::from) }
    #[doc(hidden)] pub unsafe fn from_bytes_with_nul_unchecked(bytes: &'s [u8]) -> Self { Self::from_units_with_nul_unchecked(bytes) }
    #[doc(hidden)] pub fn to_bytes(&self) -> &'s [u8] { self.to_cstr().to_bytes() }
    #[doc(hidden)] pub fn to_bytes_with_nul(&self) -> &'s [u8] { self.to_cstr().to_bytes_with_nul() }

    /// Convert `self` to a [`std::ffi::CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_cstr(&self) -> &'s CStr { unsafe { CStr::from_ptr(self.as_ptr()) } }

    /// Convert `self` to a <code>&[str]</code>.
    ///
    /// `O(n)` to find the terminal `\0` and validate UTF8.
    pub fn to_str(&self) -> Result<&'s str, Utf8Error> { self.to_cstr().to_str() }
}

#[cfg(feature = "widestring-0-4")] impl<'s> CStrNonNull<'s, u16> {
    /// Convert `self` to a [`U16CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_u16cstr(&self) -> &'s U16CStr { unsafe { U16CStr::from_ptr_with_nul(self.as_ptr(), strlen(self.as_ptr())) } }

    /// Convert `self` to a [`U16Str`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_u16str(&self) -> &'s U16Str { U16Str::from_slice(self.to_units()) }
}

#[cfg(feature = "widestring-0-4")] impl<'s> CStrNonNull<'s, u32> {
    /// Convert `self` to a [`U32CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_u32cstr(&self) -> &'s U32CStr { unsafe { U32CStr::from_ptr_with_nul(self.as_ptr(), strlen(self.as_ptr())) } }

    /// Convert `self` to a [`U32Str`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_u32str(&self) -> &'s U32Str { U32Str::from_slice(self.to_units()) }
}

impl<U: Unit> Debug for CStrNonNull<'_, U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { U::debug(self.to_units(), f) }
}

impl<U: Unit> Default for CStrNonNull<'_, U> {
    fn default() -> Self { Self { ptr: unsafe { NonNull::new_unchecked(U::EMPTY.as_ptr() as *mut _) }, phantom: PhantomData } }
}

impl<'s> From<CStrNonNull<'s>> for &'s CStr {
    fn from(s: CStrNonNull<'s>) -> Self { s.to_cstr() }
}

impl<'s> From<&'s CStr> for CStrNonNull<'s> {
    fn from(s: &'s CStr) -> Self { unsafe { CStrNonNull::from_ptr_unchecked_unbounded(s.as_ptr().cast()) } }
}



#[test] fn abi_layout() {
    assert_abi_compatible!(CStrPtr,             *const c_char);
    assert_abi_compatible!(Option<CStrNonNull>, *const c_char);
    assert_abi_compatible!(CStrNonNull,         NonNull<c_char>);

    assert_abi_compatible!(CStrPtr<u16>,             *const u16);
    assert_abi_compatible!(Option<CStrNonNull<u16>>, *const u16);
    assert_abi_compatible!(CStrNonNull<u16>,         NonNull<u16>);
}



#[test] fn struct_interop_narrow() {
    use std::mem::*;
    use std::os::raw::c_char;

    #[repr(C)] struct C {
        null:           *const c_char,
        empty:          *const c_char,
        example:        *const c_char,
        not_unicode:    *const c_char,
    }
    let c = C {
        null:           null(),
        empty:          b"\0".as_ptr().cast(),
        example:        b"example\0".as_ptr().cast(),
        not_unicode:    b"\xFF\xFF\0".as_ptr().cast(),
    };

    assert_abi_compatible!(R1, C);
    #[repr(C)] struct R1 {
        null:           CStrPtr<'static>,
        empty:          CStrPtr<'static>,
        example:        CStrPtr<'static>,
        not_unicode:    CStrPtr<'static>,
    }
    let r1 : &R1 = unsafe { transmute(&c) };

    assert_abi_compatible!(R2, C);
    #[repr(C)] struct R2 {
        null:           Option<CStrNonNull<'static>>,
        empty:          Option<CStrNonNull<'static>>,
        example:        Option<CStrNonNull<'static>>,
        not_unicode:    Option<CStrNonNull<'static>>,
    }
    let r2 : &R2 = unsafe { transmute(&c) };

    assert_eq!(r1.null          .as_ptr(), c.null.cast());
    assert_eq!(r1.empty         .as_ptr(), c.empty.cast());
    assert_eq!(r1.example       .as_ptr(), c.example.cast());
    assert_eq!(r1.not_unicode   .as_ptr(), c.not_unicode.cast());

    assert_eq!(r2.null          .is_none(), true);
    assert_eq!(r2.empty         .as_ref().unwrap().as_ptr(), c.empty.cast());
    assert_eq!(r2.example       .as_ref().unwrap().as_ptr(), c.example.cast());
    assert_eq!(r2.not_unicode   .as_ref().unwrap().as_ptr(), c.not_unicode.cast());
    assert_eq!(r2.empty         .as_ref().unwrap().as_non_null().as_ptr() as *const c_char, c.empty);
    assert_eq!(r2.example       .as_ref().unwrap().as_non_null().as_ptr() as *const c_char, c.example);
    assert_eq!(r2.not_unicode   .as_ref().unwrap().as_non_null().as_ptr() as *const c_char, c.not_unicode);

    assert_eq!(r1.null          .is_null(), true);
    assert_eq!(r1.empty         .is_null(), false);
    assert_eq!(r1.example       .is_null(), false);
    assert_eq!(r1.not_unicode   .is_null(), false);

    assert_eq!(r2.null          .is_none(), true);
    assert_eq!(r2.empty         .is_none(), false);
    assert_eq!(r2.example       .is_none(), false);
    assert_eq!(r2.not_unicode   .is_none(), false);

    assert_eq!(r1.null          .is_empty(), true);
    assert_eq!(r1.empty         .is_empty(), true);
    assert_eq!(r1.example       .is_empty(), false);
    assert_eq!(r1.not_unicode   .is_empty(), false);

    assert_eq!(r2.null          .as_ref().map_or(true, |s| s.is_empty()), true);
    assert_eq!(r2.empty         .as_ref().map_or(true, |s| s.is_empty()), true);
    assert_eq!(r2.example       .as_ref().map_or(true, |s| s.is_empty()), false);
    assert_eq!(r2.not_unicode   .as_ref().map_or(true, |s| s.is_empty()), false);

    assert_eq!(r1.null          .to_bytes(), b"");
    assert_eq!(r1.empty         .to_bytes(), b"");
    assert_eq!(r1.example       .to_bytes(), b"example");
    assert_eq!(r1.not_unicode   .to_bytes(), b"\xFF\xFF");

    assert_eq!(r2.null          .as_ref().map_or(&b""[..], |s| s.to_bytes()), &b""[..]);
    assert_eq!(r2.empty         .as_ref().map_or(&b""[..], |s| s.to_bytes()), &b""[..]);
    assert_eq!(r2.example       .as_ref().map_or(&b""[..], |s| s.to_bytes()), &b"example"[..]);
    assert_eq!(r2.not_unicode   .as_ref().map_or(&b""[..], |s| s.to_bytes()), &b"\xFF\xFF"[..]);

    assert_eq!(r1.null          .to_bytes_with_nul(), b"\0");
    assert_eq!(r1.empty         .to_bytes_with_nul(), b"\0");
    assert_eq!(r1.example       .to_bytes_with_nul(), b"example\0");
    assert_eq!(r1.not_unicode   .to_bytes_with_nul(), b"\xFF\xFF\0");

    assert_eq!(r2.null          .as_ref().map_or(&b"\0"[..], |s| s.to_bytes_with_nul()), &b"\0"[..]);
    assert_eq!(r2.empty         .as_ref().map_or(&b"\0"[..], |s| s.to_bytes_with_nul()), &b"\0"[..]);
    assert_eq!(r2.example       .as_ref().map_or(&b"\0"[..], |s| s.to_bytes_with_nul()), &b"example\0"[..]);
    assert_eq!(r2.not_unicode   .as_ref().map_or(&b"\0"[..], |s| s.to_bytes_with_nul()), &b"\xFF\xFF\0"[..]);

    assert_eq!(r1.null          .to_cstr(), CStr::from_bytes_with_nul(b"\0").unwrap());
    assert_eq!(r1.empty         .to_cstr(), CStr::from_bytes_with_nul(b"\0").unwrap());
    assert_eq!(r1.example       .to_cstr(), CStr::from_bytes_with_nul(b"example\0").unwrap());
    assert_eq!(r1.not_unicode   .to_cstr(), CStr::from_bytes_with_nul(b"\xFF\xFF\0").unwrap());

    let empty = CStr::from_bytes_with_nul(b"\0").unwrap();
    assert_eq!(r2.null          .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"\0").unwrap());
    assert_eq!(r2.empty         .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"\0").unwrap());
    assert_eq!(r2.example       .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"example\0").unwrap());
    assert_eq!(r2.not_unicode   .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"\xFF\xFF\0").unwrap());

    assert_eq!(r1.null          .to_str(), Ok(""));
    assert_eq!(r1.empty         .to_str(), Ok(""));
    assert_eq!(r1.example       .to_str(), Ok("example"));
    assert_eq!(r1.not_unicode   .to_str().is_err(), true);

    assert_eq!(r2.null          .as_ref().map_or(Ok(""),    |s| s.to_str()), Ok(""));
    assert_eq!(r2.empty         .as_ref().map_or(Ok(""),    |s| s.to_str()), Ok(""));
    assert_eq!(r2.example       .as_ref().map_or(Ok(""),    |s| s.to_str()), Ok("example"));
    assert_eq!(r2.not_unicode   .as_ref().map_or(false,     |s| s.to_str().is_err()), true);

    assert_eq!(r1.null          .to_string_lossy(), "");
    assert_eq!(r1.empty         .to_string_lossy(), "");
    assert_eq!(r1.example       .to_string_lossy(), "example");
    assert_eq!(r1.not_unicode   .to_string_lossy(), "\u{FFFD}\u{FFFD}");

    assert_eq!(r2.null          .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "");
    assert_eq!(r2.empty         .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "");
    assert_eq!(r2.example       .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "example");
    assert_eq!(r2.not_unicode   .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "\u{FFFD}\u{FFFD}");

    assert_eq!(format!("{:?}", r1.null          ), "\"\"" );
    assert_eq!(format!("{:?}", r1.empty         ), "\"\"" );
    assert_eq!(format!("{:?}", r1.example       ), "\"example\"" );
    assert_eq!(format!("{:?}", r1.not_unicode   ), "\"\\xff\\xff\"" );

    assert_eq!(format!("{:?}", r2.null          ), "None" );
    assert_eq!(format!("{:?}", r2.empty         ), "Some(\"\")" );
    assert_eq!(format!("{:?}", r2.example       ), "Some(\"example\")" );
    assert_eq!(format!("{:?}", r2.not_unicode   ), "Some(\"\\xff\\xff\")" );
}

#[test] fn struct_interop_wide() {
    use std::mem::*;

    let u_empty : &[u16; 0] = &[];
    let u_empty0 = &[0u16];

    let u_example  = &[b'e' as u16, b'x' as u16, b'a' as u16, b'm' as u16, b'p' as u16, b'l' as u16, b'e' as u16];
    let u_example0 = &[b'e' as u16, b'x' as u16, b'a' as u16, b'm' as u16, b'p' as u16, b'l' as u16, b'e' as u16, 0u16];

    // UTF16 encodes surrogates with: `[high, low]`
    // FFFF is a valid non-surrogate code point
    // use the following invalid `[low, low]` sequence instead:
    let u_not_unicode  = &[0xDC00, 0xDC00];
    let u_not_unicode0 = &[0xDC00, 0xDC00, 0];
    // ref: https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF

    #[repr(C)] struct C {
        null:           *const u16,
        empty:          *const u16,
        example:        *const u16,
        not_unicode:    *const u16,
    }
    let c = C {
        null:           null(),
        empty:          u_empty0.as_ptr(),
        example:        u_example0.as_ptr(),
        not_unicode:    u_not_unicode0.as_ptr(),
    };

    assert_abi_compatible!(R1, C);
    #[repr(C)] struct R1 {
        null:           CStrPtr<'static, u16>,
        empty:          CStrPtr<'static, u16>,
        example:        CStrPtr<'static, u16>,
        not_unicode:    CStrPtr<'static, u16>,
    }
    let r1 : &R1 = unsafe { transmute(&c) };

    assert_abi_compatible!(R2, C);
    #[repr(C)] struct R2 {
        null:           Option<CStrNonNull<'static, u16>>,
        empty:          Option<CStrNonNull<'static, u16>>,
        example:        Option<CStrNonNull<'static, u16>>,
        not_unicode:    Option<CStrNonNull<'static, u16>>,
    }
    let r2 : &R2 = unsafe { transmute(&c) };

    assert_eq!(r1.null          .as_ptr(), c.null);
    assert_eq!(r1.empty         .as_ptr(), c.empty);
    assert_eq!(r1.example       .as_ptr(), c.example);
    assert_eq!(r1.not_unicode   .as_ptr(), c.not_unicode);

    assert_eq!(r2.null          .is_none(), true);
    assert_eq!(r2.empty         .as_ref().unwrap().as_ptr(), c.empty);
    assert_eq!(r2.example       .as_ref().unwrap().as_ptr(), c.example);
    assert_eq!(r2.not_unicode   .as_ref().unwrap().as_ptr(), c.not_unicode);
    assert_eq!(r2.empty         .as_ref().unwrap().as_non_null().as_ptr() as *const u16, c.empty);
    assert_eq!(r2.example       .as_ref().unwrap().as_non_null().as_ptr() as *const u16, c.example);
    assert_eq!(r2.not_unicode   .as_ref().unwrap().as_non_null().as_ptr() as *const u16, c.not_unicode);

    assert_eq!(r1.null          .is_null(), true);
    assert_eq!(r1.empty         .is_null(), false);
    assert_eq!(r1.example       .is_null(), false);
    assert_eq!(r1.not_unicode   .is_null(), false);

    assert_eq!(r2.null          .is_none(), true);
    assert_eq!(r2.empty         .is_none(), false);
    assert_eq!(r2.example       .is_none(), false);
    assert_eq!(r2.not_unicode   .is_none(), false);

    assert_eq!(r1.null          .is_empty(), true);
    assert_eq!(r1.empty         .is_empty(), true);
    assert_eq!(r1.example       .is_empty(), false);
    assert_eq!(r1.not_unicode   .is_empty(), false);

    assert_eq!(r2.null          .as_ref().map_or(true, |s| s.is_empty()), true);
    assert_eq!(r2.empty         .as_ref().map_or(true, |s| s.is_empty()), true);
    assert_eq!(r2.example       .as_ref().map_or(true, |s| s.is_empty()), false);
    assert_eq!(r2.not_unicode   .as_ref().map_or(true, |s| s.is_empty()), false);

    assert_eq!(r1.null          .to_units(), u_empty);
    assert_eq!(r1.empty         .to_units(), u_empty);
    assert_eq!(r1.example       .to_units(), u_example);
    assert_eq!(r1.not_unicode   .to_units(), u_not_unicode);

    assert_eq!(r2.null          .as_ref().map_or(&u_empty[..], |s| s.to_units()), &u_empty[..]);
    assert_eq!(r2.empty         .as_ref().map_or(&u_empty[..], |s| s.to_units()), &u_empty[..]);
    assert_eq!(r2.example       .as_ref().map_or(&u_empty[..], |s| s.to_units()), &u_example[..]);
    assert_eq!(r2.not_unicode   .as_ref().map_or(&u_empty[..], |s| s.to_units()), &u_not_unicode[..]);

    assert_eq!(r1.null          .to_units_with_nul(), u_empty0);
    assert_eq!(r1.empty         .to_units_with_nul(), u_empty0);
    assert_eq!(r1.example       .to_units_with_nul(), u_example0);
    assert_eq!(r1.not_unicode   .to_units_with_nul(), u_not_unicode0);

    assert_eq!(r2.null          .as_ref().map_or(&u_empty0[..], |s| s.to_units_with_nul()), &u_empty0[..]);
    assert_eq!(r2.empty         .as_ref().map_or(&u_empty0[..], |s| s.to_units_with_nul()), &u_empty0[..]);
    assert_eq!(r2.example       .as_ref().map_or(&u_empty0[..], |s| s.to_units_with_nul()), &u_example0[..]);
    assert_eq!(r2.not_unicode   .as_ref().map_or(&u_empty0[..], |s| s.to_units_with_nul()), &u_not_unicode0[..]);

    #[cfg(feature = "widestring-0-4")] {
        assert_eq!(r1.null          .to_u16cstr(), U16CStr::from_slice_with_nul(u_empty0).unwrap());
        assert_eq!(r1.empty         .to_u16cstr(), U16CStr::from_slice_with_nul(u_empty0).unwrap());
        assert_eq!(r1.example       .to_u16cstr(), U16CStr::from_slice_with_nul(u_example0).unwrap());
        assert_eq!(r1.not_unicode   .to_u16cstr(), U16CStr::from_slice_with_nul(u_not_unicode0).unwrap());

        let empty = U16CStr::from_slice_with_nul(u_empty0).unwrap();
        assert_eq!(r2.null          .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice_with_nul(u_empty0).unwrap());
        assert_eq!(r2.empty         .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice_with_nul(u_empty0).unwrap());
        assert_eq!(r2.example       .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice_with_nul(u_example0).unwrap());
        assert_eq!(r2.not_unicode   .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice_with_nul(u_not_unicode0).unwrap());

        assert_eq!(r1.null          .to_u16str().as_slice(), u_empty);
        assert_eq!(r1.empty         .to_u16str().as_slice(), u_empty);
        assert_eq!(r1.example       .to_u16str().as_slice(), u_example);
        assert_eq!(r1.not_unicode   .to_u16str().as_slice(), u_not_unicode);

        assert_eq!(r2.null          .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &[0xBAD][..]);
        assert_eq!(r2.empty         .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &u_empty[..]);
        assert_eq!(r2.example       .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &u_example[..]);
        assert_eq!(r2.not_unicode   .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &u_not_unicode[..]);
    }

    assert_eq!(r1.null          .to_string_lossy(), "");
    assert_eq!(r1.empty         .to_string_lossy(), "");
    assert_eq!(r1.example       .to_string_lossy(), "example");
    assert_eq!(r1.not_unicode   .to_string_lossy(), "\u{FFFD}\u{FFFD}");

    assert_eq!(r2.null          .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "");
    assert_eq!(r2.empty         .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "");
    assert_eq!(r2.example       .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "example");
    assert_eq!(r2.not_unicode   .as_ref().map_or(Cow::Borrowed(""), |s| s.to_string_lossy()), "\u{FFFD}\u{FFFD}");

    assert_eq!(format!("{:?}", r1.null          ), "\"\"" );
    assert_eq!(format!("{:?}", r1.empty         ), "\"\"" );
    assert_eq!(format!("{:?}", r1.example       ), "\"example\"" );
    assert_eq!(format!("{:?}", r1.not_unicode   ), "\"\\udc00\\udc00\"" );

    assert_eq!(format!("{:?}", r2.null          ), "None" );
    assert_eq!(format!("{:?}", r2.empty         ), "Some(\"\")" );
    assert_eq!(format!("{:?}", r2.example       ), "Some(\"example\")" );
    assert_eq!(format!("{:?}", r2.not_unicode   ), "Some(\"\\udc00\\udc00\")" );
}

mod cstrptr_lifetime_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrPtr) {}
    /// let local = *b"example\0";
    /// f(CStrPtr::from_bytes_with_nul(&local).unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrPtr<'static>) {}
    /// let local = *b"example\0";
    /// f(CStrPtr::from_bytes_with_nul(&local).unwrap());
    /// ```
    #[allow(dead_code)] struct FromBytesWithNul;

    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrPtr) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrPtr::from_bytes_with_nul_unchecked(&local) });
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrPtr<'static>) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrPtr::from_bytes_with_nul_unchecked(&local) });
    /// ```
    #[allow(dead_code)] struct FromBytesWithNulUnchecked;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static>) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    #[allow(dead_code)] struct ToBytesLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static>) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    #[allow(dead_code)] struct ToBytesWithNulLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static>) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    #[allow(dead_code)] struct ToCStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static>) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    #[allow(dead_code)] struct ToStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    #[allow(dead_code)] struct ToStringLossy;
}

mod cstrnonnull_lifetime_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrNonNull) {}
    /// let local = *b"example\0";
    /// f(CStrNonNull::from_bytes_with_nul(&local).unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrNonNull<'static>) {}
    /// let local = *b"example\0";
    /// f(CStrNonNull::from_bytes_with_nul(&local).unwrap());
    /// ```
    #[allow(dead_code)] struct FromBytesWithNul;

    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrNonNull) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrNonNull::from_bytes_with_nul_unchecked(&local) });
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrNonNull<'static>) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrNonNull::from_bytes_with_nul_unchecked(&local) });
    /// ```
    #[allow(dead_code)] struct FromBytesWithNulUnchecked;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static>) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    #[allow(dead_code)] struct ToBytesLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static>) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    #[allow(dead_code)] struct ToBytesWithNulLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static>) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    #[allow(dead_code)] struct ToCStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static>) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    #[allow(dead_code)] struct ToStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    #[allow(dead_code)] struct ToStringLossy;
}
