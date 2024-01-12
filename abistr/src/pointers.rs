use crate::*;
use crate::unit::private::{Unit as _};

#[cfg(test)] use core::ffi::c_char;
use core::ffi::CStr;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::ptr::*;
use core::str::Utf8Error;



/// <code>[CStrPtr]&lt;[Encoding]&gt;</code> is ABI compatible with <code>*const [Encoding]::[Unit](Encoding::Unit)</code>.  <code>[null]\(\)</code> is treated as an empty string.
///
/// If you want to treat <code>[null]\(\)</code> as [`None`], use <code>[Option]<[CStrNonNull]></code> instead.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CStrPtr<'s, E: Encoding> {
    ptr:        *const E::Unit,
    phantom:    PhantomData<&'s [E::Unit]>,
}

unsafe impl<'s, E: Encoding> Send for CStrPtr<'s, E> {}
unsafe impl<'s, E: Encoding> Sync for CStrPtr<'s, E> {}

impl<'s, E: Encoding> CStrPtr<'s, E> {
    /// A <code>[null]\(\)</code> [CStrPtr].
    pub const NULL : Self = Self { ptr: null(), phantom: PhantomData };

    /// Convert a raw C-string into a [`CStrPtr`].  Note that the lifetime of the returned reference is unbounded!
    ///
    /// ### Safety
    /// `ptr` may be null.  If it is not:
    /// *   `ptr` must point to a `\0`-terminated, `E` [Encoding], C string.
    /// *   The underlying C-string cannot change for the duration of the lifetime `'s`.
    /// *   The lifetime `'s` is unbounded by this fn.  Very easy to accidentally dangle.  Be careful!
    pub const unsafe fn from_ptr_unchecked(ptr: *const E::Unit) -> Self { Self { ptr, phantom: PhantomData } }

    /// Convert a raw slice of units, presumably with [Encoding] `E`, into a [`CStrPtr`].
    ///
    /// ### Returns
    /// *   <code>[Err]\(...\)</code> if `units` does not end with `\0`.
    /// *   <code>[Err]\(...\)</code> if `units` otherwise contains interior `\0`s.
    /// *   <code>[Err]\(...\)</code> if `units` contains invalid sequences for [Encoding] `E` (e.g. invalid UTF-16 if <code>E = [encoding::Utf16]</code>.)
    pub fn from_units_with_nul<U: Unit>(units: &'s [U]) -> Result<Self, FromUnitsWithNulError> where E: FromUnits<U> {
        let units = E::from_units(units).map_err(|_| FromUnitsWithNulError(()))?;
        let (nul, interior) = units.split_last().ok_or(FromUnitsWithNulError(()))?;
        if *nul != E::Unit::NUL { return Err(FromUnitsWithNulError(())); }
        if interior.contains(&E::Unit::NUL) { return Err(FromUnitsWithNulError(())); }
        Ok(unsafe { Self::from_ptr_unchecked(units.as_ptr()) })
    }

    /// Convert a raw slice of units to a [`CStrPtr`].  The resulting string will be terminated at the first `\0` in `units`.
    ///
    /// ### Safety
    /// *   `units` must contain at least one `\0`.
    /// *   `units` must have [Encoding] `E`.
    pub unsafe fn from_units_with_nul_unchecked(units: &'s [E::Unit]) -> Self {
        debug_assert!(units.ends_with(E::Unit::EMPTY) || units.contains(&E::Unit::NUL), "Undefined Behavior: `units` contained no `\0`!");
        E::debug_check_valid(units);
        unsafe { Self::from_ptr_unchecked(units.as_ptr()) }
    }

    /// Treat `self` as a raw, possibly <code>[null]\(\)</code> C string.
    pub const fn as_ptr(&self) -> *const E::Unit { self.ptr }

    /// Checks if `self` is <code>[null]\(\)</code>.
    pub fn is_null(&self) -> bool { self.ptr.is_null() }

    /// Checks if `self` is empty (either null, or the first character is `\0`.)
    pub fn is_empty(&self) -> bool { self.ptr.is_null() || E::Unit::NUL == unsafe { *self.ptr } }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, **excluding** the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units(&self) -> &'s [E::Unit] {
        if self.ptr.is_null() { return &[]; }
        let start = self.ptr;
        unsafe { core::slice::from_raw_parts(start, strlen(start)) }
    }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, including the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units_with_nul(&self) -> &'s [E::Unit] {
        if self.ptr.is_null() { return E::Unit::EMPTY; }
        let start = self.ptr;
        unsafe { core::slice::from_raw_parts(start, strlen(start) + 1) }
    }

    /// Convert `self` to a <code>[alloc::borrow::Cow]\<[str]\></code>.
    ///
    /// `O(n)` to find the terminal `\0` and convert/validate.
    #[cfg(feature = "alloc")] pub fn to_string_lossy(&self) -> alloc::borrow::Cow<'s, str> where E: ToChars { E::to_string_lossy(self.to_units()) }
}

impl<'s, E: Encoding<Unit = u8>> CStrPtr<'s, E> {
    #[doc(hidden)] pub fn from_bytes_with_nul(bytes: &'s [u8]) -> Result<Self, FromUnitsWithNulError> where E: FromUnits<u8> { Self::from_units_with_nul(bytes) }
    #[doc(hidden)] pub unsafe fn from_bytes_with_nul_unchecked(bytes: &'s [u8]) -> Self { unsafe { Self::from_units_with_nul_unchecked(bytes) } }
    #[doc(hidden)] pub fn to_bytes(&self) -> &'s [u8] { self.to_units() }
    #[doc(hidden)] pub fn to_bytes_with_nul(&self) -> &'s [u8] { self.to_units_with_nul() }

    /// Convert `self` to a [`core::ffi::CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_cstr(&self) -> &'s CStr {
        if self.ptr.is_null() {
            unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") }
        } else {
            unsafe { CStr::from_ptr(self.ptr.cast()) }
        }
    }
}

impl<'s> CStrPtr<'s, Utf8ish> {
    /// Convert `self` to a <code>&[str]</code>.
    ///
    /// `O(n)` to find the terminal `\0` and validate UTF-8.
    pub fn to_str(&self) -> Result<&'s str, Utf8Error> { core::str::from_utf8(self.to_units()) }
}

impl<'s> CStrPtr<'s, Utf8> {
    /// Convert `self` to a <code>&[str]</code>.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_str(&self) -> &'s str { unsafe { core::str::from_utf8_unchecked(self.to_units()) } }
}

#[cfg(feature = "widestring")] const _ : () = {
    use widestring::*;

    impl<'s, E: Encoding<Unit = u16>> CStrPtr<'s, E> {
        /// Convert `self` to a [`U16CStr`].
        ///
        /// `O(n)` to find the terminal `\0`.
        pub fn to_u16cstr(&self) -> &'s U16CStr {
            if self.ptr.is_null() {
                Default::default()
            } else {
                unsafe { U16CStr::from_ptr_str(self.ptr) }
            }
        }

        /// Convert `self` to a [`U16Str`].
        ///
        /// `O(n)` to find the terminal `\0`
        pub fn to_u16str(&self) -> &'s U16Str { U16Str::from_slice(self.to_units()) }
    }

    impl<'s, E: Encoding<Unit = u32>> CStrPtr<'s, E> {
        /// Convert `self` to a [`U32CStr`].
        ///
        /// `O(n)` to find the terminal `\0`.
        pub fn to_u32cstr(&self) -> &'s U32CStr {
            if self.ptr.is_null() {
                Default::default()
            } else {
                unsafe { U32CStr::from_ptr_str(self.ptr) }
            }
        }

        /// Convert `self` to a [`U32Str`].
        ///
        /// `O(n)` to find the terminal `\0`
        pub fn to_u32str(&self) -> &'s U32Str { U32Str::from_slice(self.to_units()) }
    }

    #[cfg(todo)] const _ : () = { /* ...conversions to Utf{16,32}Str, U{16,32}CStr ? */ };
};

impl<E: Encoding> Debug for CStrPtr<'_, E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { E::debug_fmt(self.to_units(), f) }
}

impl<E: Encoding> Default for CStrPtr<'_, E> {
    fn default() -> Self { Self { ptr: E::Unit::EMPTY.as_ptr(), phantom: PhantomData } }
}

impl<'s, E: Encoding<Unit = u8>> From<CStrPtr<'s, E>> for &'s CStr {
    fn from(s: CStrPtr<'s, E>) -> Self { s.to_cstr() }
}

impl<'s> From<&'s CStr> for CStrPtr<'s, Unknown8> {
    fn from(s: &'s CStr) -> Self { unsafe { CStrPtr::from_ptr_unchecked(s.as_ptr().cast()) } }
}



/// <code>[Option]&lt;[CStrNonNull]&lt;[Encoding]&gt;&gt;</code> is ABI compatible with <code>*const [Encoding]::[Unit](Encoding::Unit)</code>.
///
/// If you want to treat <code>[null]\(\)</code> as `""`, use [`CStrPtr`] instead.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CStrNonNull<'s, E: Encoding> {
    ptr:        NonNull<E::Unit>,
    phantom:    PhantomData<&'s [E::Unit]>,
}

unsafe impl<'s, E: Encoding> Send for CStrNonNull<'s, E> {}
unsafe impl<'s, E: Encoding> Sync for CStrNonNull<'s, E> {}

impl<'s, E: Encoding> From<       CStrNonNull<'s, E> > for CStrPtr<'s, E> { fn from(p:        CStrNonNull<'s, E> ) -> Self { unsafe { core::mem::transmute(p) } } }
impl<'s, E: Encoding> From<Option<CStrNonNull<'s, E>>> for CStrPtr<'s, E> { fn from(p: Option<CStrNonNull<'s, E>>) -> Self { unsafe { core::mem::transmute(p) } } }

impl<'s, E: Encoding> CStrNonNull<'s, E> {
    /// Convert a raw C-string into a [`CStrPtr`].  Note that the lifetime of the returned reference is unbounded!
    ///
    /// ### Safety
    /// *   `ptr` cannot be null.
    /// *   `ptr` must point to a `\0`-terminated, `E` [Encoding], C string
    /// *   The underlying C-string cannot change for the duration of the lifetime `'s`.
    /// *   The lifetime `'s` is unbounded by this fn.  Very easy to accidentally dangle.  Be careful!
    pub const unsafe fn from_ptr_unchecked(ptr: *const E::Unit) -> Self { Self { ptr: unsafe { NonNull::new_unchecked(ptr as *mut _) }, phantom: PhantomData } }

    /// Convert a raw slice of units, presumably with [Encoding] `E`, into a [`CStrNonNull`].
    ///
    /// ### Returns
    /// *   <code>[Err]\(...\)</code> if `units` does not end with `\0`.
    /// *   <code>[Err]\(...\)</code> if `units` otherwise contains interior `\0`s.
    /// *   <code>[Err]\(...\)</code> if `units` contains invalid sequences for [Encoding] `E` (e.g. invalid UTF-16 if <code>E = [encoding::Utf16]</code>.)
    pub fn from_units_with_nul<U: Unit>(units: &'s [U]) -> Result<Self, FromUnitsWithNulError> where E: FromUnits<U> {
        let units = E::from_units(units).map_err(|_| FromUnitsWithNulError(()))?;
        let (nul, interior) = units.split_last().ok_or(FromUnitsWithNulError(()))?;
        if *nul != E::Unit::NUL { return Err(FromUnitsWithNulError(())); }
        if interior.contains(&E::Unit::NUL) { return Err(FromUnitsWithNulError(())); }
        Ok(unsafe { Self::from_ptr_unchecked(units.as_ptr().cast()) })
    }

    /// Convert a raw slice of units to a [`CStrNonNull`].  The resulting string will be terminated at the first `\0` in `units`.
    ///
    /// ### Safety
    /// *   `units` must contain at least one `\0`.
    /// *   `units` must have [Encoding] `E`.
    pub unsafe fn from_units_with_nul_unchecked(units: &'s [E::Unit]) -> Self {
        debug_assert!(units.ends_with(E::Unit::EMPTY) || units.contains(&E::Unit::NUL), "Undefined Behavior: `units` contained no `\0`!");
        E::debug_check_valid(units);
        unsafe { Self::from_ptr_unchecked(units.as_ptr()) }
    }

    /// Use [`from_units_with_nul_unchecked`](Self::from_units_with_nul_unchecked) or [`cstr!`] instead!
    #[doc(hidden)] // This fn only exists to allow the use of the totally safe `cstr!` macro in `#![forbid(unsafe_code)]` codebases.
    pub const fn zzz_unsound_do_not_call_this_directly_from_macro_units_with_nul(units: &'s [E::Unit]) -> Self {
        unsafe { Self::from_ptr_unchecked(units.as_ptr()) }
    }

    /// Treat `self` as a raw C string.
    pub const fn as_ptr(&self) -> *const E::Unit { self.ptr.as_ptr().cast() }

    /// Treat `self` as a [`NonNull`] C string.
    pub const fn as_non_null(&self) -> NonNull<E::Unit> { self.ptr }

    /// Checks if `self` is empty (either <code>[null]\(\)</code>, or the first character is `\0`.)
    pub fn is_empty(&self) -> bool { E::Unit::NUL == unsafe { *self.ptr.as_ptr().cast() } }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, **excluding** the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units(&self) -> &'s [E::Unit] {
        let start = self.ptr.as_ptr().cast();
        unsafe { core::slice::from_raw_parts(start, strlen(start) + 0) }
    }

    /// Convert `self` to a <code>&\[[Unit]\]</code> slice, including the terminal `\0`.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_units_with_nul(&self) -> &'s [E::Unit] {
        let start = self.ptr.as_ptr().cast();
        unsafe { core::slice::from_raw_parts(start, strlen(start) + 1) }
    }

    /// Convert `self` to a <code>[alloc::borrow::Cow]\<[str]\></code>.
    ///
    /// `O(n)` to find the terminal `\0` and convert/validate.
    #[cfg(feature = "alloc")] pub fn to_string_lossy(&self) -> alloc::borrow::Cow<'s, str> where E: ToChars { E::to_string_lossy(self.to_units()) }
}

impl<'s, E: Encoding<Unit = u8>> CStrNonNull<'s, E> {
    #[doc(hidden)] pub fn from_bytes_with_nul(bytes: &'s [u8]) -> Result<Self, FromUnitsWithNulError> where E: FromUnits<u8> { Self::from_units_with_nul(bytes) }
    #[doc(hidden)] pub unsafe fn from_bytes_with_nul_unchecked(bytes: &'s [u8]) -> Self { unsafe { Self::from_units_with_nul_unchecked(bytes) } }
    #[doc(hidden)] pub fn to_bytes(&self) -> &'s [u8] { self.to_units() }
    #[doc(hidden)] pub fn to_bytes_with_nul(&self) -> &'s [u8] { self.to_units_with_nul() }

    /// Convert `self` to a [`core::ffi::CStr`].
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_cstr(&self) -> &'s CStr { unsafe { CStr::from_ptr(self.as_ptr().cast()) } }
}

impl<'s> CStrNonNull<'s, Utf8ish> {
    /// Convert `self` to a <code>&[str]</code>.
    ///
    /// `O(n)` to find the terminal `\0` and validate UTF-8.
    pub fn to_str(&self) -> Result<&'s str, Utf8Error> { core::str::from_utf8(self.to_units()) }
}

impl<'s> CStrNonNull<'s, Utf8> {
    /// Convert `self` to a <code>&[str]</code>.
    ///
    /// `O(n)` to find the terminal `\0`.
    pub fn to_str(&self) -> &'s str { unsafe { core::str::from_utf8_unchecked(self.to_units()) } }
}

#[cfg(feature = "widestring")] const _ : () = {
    use widestring::*;

    impl<'s, E: Encoding<Unit = u16>> CStrNonNull<'s, E> {
        /// Convert `self` to a [`U16CStr`].
        ///
        /// `O(n)` to find the terminal `\0`.
        pub fn to_u16cstr(&self) -> &'s U16CStr { unsafe { U16CStr::from_ptr_str(self.as_ptr()) } }

        /// Convert `self` to a [`U16Str`].
        ///
        /// `O(n)` to find the terminal `\0`.
        pub fn to_u16str(&self) -> &'s U16Str { U16Str::from_slice(self.to_units()) }
    }

    impl<'s, E: Encoding<Unit = u32>> CStrNonNull<'s, E> {
        /// Convert `self` to a [`U32CStr`].
        ///
        /// `O(n)` to find the terminal `\0`.
        pub fn to_u32cstr(&self) -> &'s U32CStr { unsafe { U32CStr::from_ptr_str(self.as_ptr()) } }

        /// Convert `self` to a [`U32Str`].
        ///
        /// `O(n)` to find the terminal `\0`.
        pub fn to_u32str(&self) -> &'s U32Str { U32Str::from_slice(self.to_units()) }
    }
};

impl<E: Encoding> Debug for CStrNonNull<'_, E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { E::debug_fmt(self.to_units(), f) }
}

impl<E: Encoding> Default for CStrNonNull<'_, E> {
    fn default() -> Self { Self { ptr: unsafe { NonNull::new_unchecked(E::Unit::EMPTY.as_ptr() as *mut _) }, phantom: PhantomData } }
}

impl<'s, E: Encoding<Unit = u8>> From<CStrNonNull<'s, E>> for &'s CStr {
    fn from(s: CStrNonNull<'s, E>) -> Self { s.to_cstr() }
}

impl<'s> From<&'s CStr> for CStrNonNull<'s, Unknown8> {
    fn from(s: &'s CStr) -> Self { unsafe { CStrNonNull::from_ptr_unchecked(s.as_ptr().cast()) } }
}

for_each! {
    use {CStrNonNull, CStrPtr} as CStrX;

    impl<'s> CStrX<'s, Unknown8> {
        /// Assume `self` is (more-or-less) UTF-8 encoded.
        ///
        /// ### Safety
        /// 100% safe, although this might result in [Mojibake](https://en.wikipedia.org/wiki/Mojibake).
        /// [Utf8ish] allows invalid [`char`]s and sequences.<br>
        /// [`Self::assume_utf8`] is an `unsafe` alternative that does *not* allow for invalid sequences.
        pub fn assume_utf8ish(self) -> CStrX<'s, Utf8ish> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }

        /// Assume `self` is 100% valid UTF-8 (valid [`char`]s and sequences.)
        ///
        /// ### Safety
        /// Invalid [`char`]s or sequences (surrogate code points, code points above 0x10FFFF, overlong encodings) are undefined behavior.<br>
        /// [Mojibake](https://en.wikipedia.org/wiki/Mojibake) is also possible.<br>
        /// Consider [`assume_utf8ish`](Self::assume_utf8ish) for a safe alternative.
        pub unsafe fn assume_utf8(self) -> CStrX<'s, Utf8> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }
    }

    impl<'s> CStrX<'s, Utf8ish> {
        /// Assume `self` is 100% valid UTF-8 (valid [`char`]s and sequences.)
        ///
        /// ### Safety
        /// Invalid [`char`]s or sequences (surrogate code points, code points above 0x10FFFF, overlong encodings) are undefined behavior.<br>
        /// [Mojibake](https://en.wikipedia.org/wiki/Mojibake) is also possible.<br>
        pub unsafe fn assume_utf8(self) -> CStrX<'s, Utf8> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }
    }



    impl<'s> CStrX<'s, Unknown16> {
        /// Assume `self` is (more-or-less) UTF-16 encoded.
        ///
        /// ### Safety
        /// 100% safe, although this might result in [Mojibake](https://en.wikipedia.org/wiki/Mojibake).
        /// [Utf16ish] allows invalid [`char`]s and sequences.<br>
        /// [`Self::assume_utf16`] is an `unsafe` alternative that does *not* allow for invalid sequences.
        pub fn assume_utf16ish(self) -> CStrX<'s, Utf16ish> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }

        /// Assume `self` is 100% valid UTF-16 (valid [`char`]s and sequences.)
        ///
        /// ### Safety
        /// Invalid [`char`]s or sequences (unpaired surrogate code points, values above 0x10FFFF) are undefined behavior.<br>
        /// [Mojibake](https://en.wikipedia.org/wiki/Mojibake) is also possible.<br>
        /// Consider [`assume_utf16ish`](Self::assume_utf16ish) for a safe alternative.
        pub unsafe fn assume_utf16(self) -> CStrX<'s, Utf16> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }
    }

    impl<'s> CStrX<'s, Utf16ish> {
        /// Assume `self` is 100% valid UTF-16 (valid [`char`]s and sequences.)
        ///
        /// ### Safety
        /// Invalid [`char`]s or sequences (unpaired surrogate code points, values above 0x10FFFF) are undefined behavior.<br>
        /// [Mojibake](https://en.wikipedia.org/wiki/Mojibake) is also possible.<br>
        pub unsafe fn assume_utf16(self) -> CStrX<'s, Utf16> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }
    }



    impl<'s> CStrX<'s, Unknown32> {
        /// Assume `self` is (more-or-less) UTF-32 encoded.
        ///
        /// ### Safety
        /// 100% safe, although this might result in [Mojibake](https://en.wikipedia.org/wiki/Mojibake).
        /// [Utf32ish] allows invalid [`char`]s.<br>
        /// [`Self::assume_utf32`] is an `unsafe` alternative that does *not* allow for invalid sequences.
        pub fn assume_utf32ish(self) -> CStrX<'s, Utf32ish> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }

        /// Assume `self` is 100% valid UTF-32 (valid [`char`]s.)
        ///
        /// ### Safety
        /// Invalid [`char`]s (surrogate code points, values above 0x10FFFF) are undefined behavior.
        /// [Mojibake](https://en.wikipedia.org/wiki/Mojibake) is also possible.<br>
        /// Consider [`assume_utf32ish`](Self::assume_utf32ish) for a safe alternative.
        pub unsafe fn assume_utf32(self) -> CStrX<'s, Utf32> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }
    }

    impl<'s> CStrX<'s, Utf32ish> {
        /// Assume `self` is 100% valid UTF-32 (valid [`char`]s.)
        ///
        /// ### Safety
        /// Invalid [`char`]s (surrogate code points, values above 0x10FFFF) are undefined behavior.
        /// [Mojibake](https://en.wikipedia.org/wiki/Mojibake) is also possible.<br>
        pub unsafe fn assume_utf32(self) -> CStrX<'s, Utf32> { unsafe { CStrX::from_ptr_unchecked(self.as_ptr().cast()) } }
    }
}



#[test] fn abi_layout() {
    assert_abi_compatible!(CStrPtr<Unknown8>,               *const c_char);
    assert_abi_compatible!(Option<CStrNonNull<Unknown8>>,   *const c_char);
    assert_abi_compatible!(CStrNonNull<Unknown8>,           NonNull<c_char>);

    assert_abi_compatible!(CStrPtr<Unknown16>,              *const u16);
    assert_abi_compatible!(Option<CStrNonNull<Unknown16>>,  *const u16);
    assert_abi_compatible!(CStrNonNull<Unknown16>,          NonNull<u16>);
}



#[test] fn struct_interop_narrow() {
    use crate::*;
    use core::mem::*;

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
        null:           CStrPtr<'static, Utf8ish>,
        empty:          CStrPtr<'static, Utf8ish>,
        example:        CStrPtr<'static, Utf8ish>,
        not_unicode:    CStrPtr<'static, Utf8ish>,
    }
    let r1 : &R1 = unsafe { transmute(&c) };

    assert_abi_compatible!(R2, C);
    #[repr(C)] struct R2 {
        null:           Option<CStrNonNull<'static, Utf8ish>>,
        empty:          Option<CStrNonNull<'static, Utf8ish>>,
        example:        Option<CStrNonNull<'static, Utf8ish>>,
        not_unicode:    Option<CStrNonNull<'static, Utf8ish>>,
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

    #[cfg(feature = "std")] {
        assert_eq!(r1.null          .to_cstr(), CStr::from_bytes_with_nul(b"\0").unwrap());
        assert_eq!(r1.empty         .to_cstr(), CStr::from_bytes_with_nul(b"\0").unwrap());
        assert_eq!(r1.example       .to_cstr(), CStr::from_bytes_with_nul(b"example\0").unwrap());
        assert_eq!(r1.not_unicode   .to_cstr(), CStr::from_bytes_with_nul(b"\xFF\xFF\0").unwrap());

        let empty = CStr::from_bytes_with_nul(b"\0").unwrap();
        assert_eq!(r2.null          .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"\0").unwrap());
        assert_eq!(r2.empty         .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"\0").unwrap());
        assert_eq!(r2.example       .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"example\0").unwrap());
        assert_eq!(r2.not_unicode   .as_ref().map_or(empty, |s| s.to_cstr()), CStr::from_bytes_with_nul(b"\xFF\xFF\0").unwrap());
    }

    assert_eq!(r1.null          .to_str(), Ok(""));
    assert_eq!(r1.empty         .to_str(), Ok(""));
    assert_eq!(r1.example       .to_str(), Ok("example"));
    assert_eq!(r1.not_unicode   .to_str().is_err(), true);

    assert_eq!(r2.null          .as_ref().map_or(Ok(""),    |s| s.to_str()), Ok(""));
    assert_eq!(r2.empty         .as_ref().map_or(Ok(""),    |s| s.to_str()), Ok(""));
    assert_eq!(r2.example       .as_ref().map_or(Ok(""),    |s| s.to_str()), Ok("example"));
    assert_eq!(r2.not_unicode   .as_ref().map_or(false,     |s| s.to_str().is_err()), true);

    #[cfg(feature = "std")] {
        use std::borrow::Cow;
        use std::format;

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
}

#[test] fn struct_interop_wide() {
    use core::mem::*;

    let u_empty : &[u16; 0] = &[];
    let u_empty0 = &[0u16];

    let u_example  = &[b'e' as u16, b'x' as u16, b'a' as u16, b'm' as u16, b'p' as u16, b'l' as u16, b'e' as u16];
    let u_example0 = &[b'e' as u16, b'x' as u16, b'a' as u16, b'm' as u16, b'p' as u16, b'l' as u16, b'e' as u16, 0u16];

    // UTF-16 encodes surrogates with: `[high, low]`
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
        null:           CStrPtr<'static, Utf16ish>,
        empty:          CStrPtr<'static, Utf16ish>,
        example:        CStrPtr<'static, Utf16ish>,
        not_unicode:    CStrPtr<'static, Utf16ish>,
    }
    let r1 : &R1 = unsafe { transmute(&c) };

    assert_abi_compatible!(R2, C);
    #[repr(C)] struct R2 {
        null:           Option<CStrNonNull<'static, Utf16ish>>,
        empty:          Option<CStrNonNull<'static, Utf16ish>>,
        example:        Option<CStrNonNull<'static, Utf16ish>>,
        not_unicode:    Option<CStrNonNull<'static, Utf16ish>>,
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

    #[cfg(feature = "widestring")] {
        use widestring::*;

        assert_eq!(r1.null          .to_u16cstr(), U16CStr::from_slice(u_empty0).unwrap());
        assert_eq!(r1.empty         .to_u16cstr(), U16CStr::from_slice(u_empty0).unwrap());
        assert_eq!(r1.example       .to_u16cstr(), U16CStr::from_slice(u_example0).unwrap());
        assert_eq!(r1.not_unicode   .to_u16cstr(), U16CStr::from_slice(u_not_unicode0).unwrap());

        let empty = U16CStr::from_slice(u_empty0).unwrap();
        assert_eq!(r2.null          .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice(u_empty0).unwrap());
        assert_eq!(r2.empty         .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice(u_empty0).unwrap());
        assert_eq!(r2.example       .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice(u_example0).unwrap());
        assert_eq!(r2.not_unicode   .as_ref().map_or(empty, |s| s.to_u16cstr()), U16CStr::from_slice(u_not_unicode0).unwrap());

        assert_eq!(r1.null          .to_u16str().as_slice(), u_empty);
        assert_eq!(r1.empty         .to_u16str().as_slice(), u_empty);
        assert_eq!(r1.example       .to_u16str().as_slice(), u_example);
        assert_eq!(r1.not_unicode   .to_u16str().as_slice(), u_not_unicode);

        assert_eq!(r2.null          .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &[0xBAD][..]);
        assert_eq!(r2.empty         .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &u_empty[..]);
        assert_eq!(r2.example       .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &u_example[..]);
        assert_eq!(r2.not_unicode   .as_ref().map_or(&[0xBAD][..], |s| s.to_u16str().as_slice()), &u_not_unicode[..]);
    }

    #[cfg(feature = "std")] {
        use std::borrow::Cow;
        use std::format;

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
}

#[cfg(feature = "std")] #[allow(dead_code)] mod cstrptr_lifetime_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrPtr<'_, encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(CStrPtr::from_bytes_with_nul(&local).unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrPtr<'static, encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(CStrPtr::from_bytes_with_nul(&local).unwrap());
    /// ```
    struct FromBytesWithNul;

    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrPtr<'_, encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrPtr::from_bytes_with_nul_unchecked(&local) });
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrPtr<'static, encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrPtr::from_bytes_with_nul_unchecked(&local) });
    /// ```
    struct FromBytesWithNulUnchecked;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    struct ToBytesLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    struct ToBytesWithNulLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown8>) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown8>) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    struct ToCStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Utf8ish>) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Utf8ish>) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    struct ToStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Utf8ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Utf8ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    struct ToStringLossy;
}

#[cfg(feature = "std")] #[allow(dead_code)] mod cstrptr16_lifetime_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrPtr<encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(CStrPtr::from_units_with_nul(&local).unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrPtr<'static, encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(CStrPtr::from_units_with_nul(&local).unwrap());
    /// ```
    struct FromUnitsWithNul;

    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrPtr<encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(unsafe { CStrPtr::from_units_with_nul_unchecked(&local) });
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrPtr<'static, encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(unsafe { CStrPtr::from_units_with_nul_unchecked(&local) });
    /// ```
    struct FromUnitsWithNulUnchecked;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown16>) -> &'static [u16] { ptr.to_units() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown16>) -> &'static [u16] { ptr.to_units() }
    /// ```
    struct ToUnitsLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown16>) -> &'static [u16] { ptr.to_units_with_nul() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown16>) -> &'static [u16] { ptr.to_units_with_nul() }
    /// ```
    struct ToUnitsWithNulLifetime;

    #[cfg(feature = "widestring")]
    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown16>) -> &'static widestring::U16CStr { ptr.to_u16cstr() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown16>) -> &'static widestring::U16CStr { ptr.to_u16cstr() }
    /// ```
    struct ToCStr;

    #[cfg(feature = "widestring")]
    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Unknown16>) -> &'static widestring::U16Str { ptr.to_u16str() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Unknown16>) -> &'static widestring::U16Str { ptr.to_u16str() }
    /// ```
    struct ToStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<'static, encoding::Utf16ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrPtr<encoding::Utf16ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    struct ToStringLossy;
}

#[cfg(feature = "std")] #[allow(dead_code)] mod cstrnonnull_lifetime_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrNonNull<encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(CStrNonNull::from_bytes_with_nul(&local).unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrNonNull<'static, encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(CStrNonNull::from_bytes_with_nul(&local).unwrap());
    /// ```
    struct FromBytesWithNul;

    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrNonNull<encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrNonNull::from_bytes_with_nul_unchecked(&local) });
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrNonNull<'static, encoding::Unknown8>) {}
    /// let local = *b"example\0";
    /// f(unsafe { CStrNonNull::from_bytes_with_nul_unchecked(&local) });
    /// ```
    struct FromBytesWithNulUnchecked;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes() }
    /// ```
    struct ToBytesLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown8>) -> &'static [u8] { ptr.to_bytes_with_nul() }
    /// ```
    struct ToBytesWithNulLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown8>) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown8>) -> &'static std::ffi::CStr { ptr.to_cstr() }
    /// ```
    struct ToCStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Utf8ish>) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Utf8ish>) -> &'static str { ptr.to_str().unwrap() }
    /// ```
    struct ToStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Utf8ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Utf8ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    struct ToStringLossy;
}

#[cfg(feature = "std")] #[allow(dead_code)] mod cstrnonnull16_lifetime_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrNonNull<encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(CStrNonNull::from_units_with_nul(&local).unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrNonNull<'static, encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(CStrNonNull::from_units_with_nul(&local).unwrap());
    /// ```
    struct FromBytesWithNul;

    /// ```no_run
    /// use abistr::*;
    /// fn f(_: CStrNonNull<encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(unsafe { CStrNonNull::from_units_with_nul_unchecked(&local) });
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: CStrNonNull<'static, encoding::Unknown16>) {}
    /// let local = [b'e' as u16, b'x' as u16, 0];
    /// f(unsafe { CStrNonNull::from_units_with_nul_unchecked(&local) });
    /// ```
    struct FromBytesWithNulUnchecked;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown16>) -> &'static [u16] { ptr.to_units() }
    /// fn g(ptr: CStrNonNull<         encoding::Unknown16>) -> &        [u16] { ptr.to_units() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown16>) -> &'static [u16] { ptr.to_units() }
    /// ```
    struct ToBytesLifetime;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown16>) -> &'static [u16] { ptr.to_units_with_nul() }
    /// fn g(ptr: CStrNonNull<         encoding::Unknown16>) -> &        [u16] { ptr.to_units_with_nul() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown16>) -> &'static [u16] { ptr.to_units_with_nul() }
    /// ```
    struct ToBytesWithNulLifetime;

    #[cfg(feature = "widestring")]
    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown16>) -> &'static widestring::U16CStr { ptr.to_u16cstr() }
    /// fn g(ptr: CStrNonNull<         encoding::Unknown16>) -> &        widestring::U16CStr { ptr.to_u16cstr() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown16>) -> &'static widestring::U16CStr { ptr.to_u16cstr() }
    /// ```
    struct ToCStr;

    #[cfg(feature = "widestring")]
    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Unknown16>) -> &'static widestring::U16Str { ptr.to_u16str() }
    /// fn g(ptr: CStrNonNull<         encoding::Unknown16>) -> &        widestring::U16Str { ptr.to_u16str() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Unknown16>) -> &'static widestring::U16Str { ptr.to_u16str() }
    /// ```
    struct ToStr;

    /// ```no_run
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<'static, encoding::Utf16ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// fn g(ptr: CStrNonNull<         encoding::Utf16ish>) -> std::borrow::Cow<         str> { ptr.to_string_lossy() }
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(ptr: CStrNonNull<encoding::Utf16ish>) -> std::borrow::Cow<'static, str> { ptr.to_string_lossy() }
    /// ```
    struct ToStringLossy;
}
