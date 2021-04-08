use crate::*;

use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::ffi::*;
use std::str::*;
#[cfg(doc)] use std::os::raw::c_char;



/// <code>[CStrBuf]<\[[Unit]; 128\]></code> is ABI compatible with <code>\[[Unit]; 128\]</code>.
///
/// ### Safety
///
/// There is no guarantee the underlying buffer is `\0` terminated, and no reasonable way to create such a guarantee
/// when the type is used in raw C structure FFI - the underlying C code might not `\0`-terminate the buffer, and you
/// could immediately pass the structure to another C fn without calling a single sanitization function.
///
/// If wrapping a C fn that takes a buffer-laden structure as input, you are **strongly** encouraged to call either
/// [`CStrBuf::nul_truncate`], or [`CStrBuf::validate`] (and error out on any [`NotNulTerminatedError`]s) in your safe
/// fn before passing it to C.  Not doing so is almost certain to lead to undefined behavior, although there are some
/// exceptions where the buffer is not expected to be nul terminated (e.g. some magic marker strings in file headers, as
/// an example.)
///
/// You could also write some gnarly malicious [`AsRef`]/[`AsMut`] impls for `B` that e.g. return different buffers when
/// called multiple times.  While I believe I've guarded against unsoundness, such types would likely break guarantees
/// that you might otherwise rely on for FFI.  So... don't.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CStrBuf<B: Array> {
    buffer: B,
}

impl<B: Array> CStrBuf<B> {
    /// Create a [`CStrBuf`] from `data` + `\0`.  Will be truncated (with the `\0`) to fit if `data` is too long.
    ///
    /// ### Panics
    ///
    /// If `self.buffer.is_empty()` (...did you create a `CStrBuf<[u8; 0]>` or something?  Weirdo.)
    pub fn from_truncate(data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Self {
        let mut s = Self::default();
        let _ = s.set_truncate(data);
        s
    }

    /// Create a [`CStrBuf`] from `data` + `\0`.  Will be truncated to fit if `data` is too long.  **Not** guaranteed to be `\0`-terminated!
    pub unsafe fn from_truncate_without_nul(data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Self {
        let mut s = Self::default();
        let _ = s.set_truncate_without_nul(data);
        s
    }

    /// Create a [`CStrBuf`] from `data` + `\0`.
    pub fn try_from(data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Result<Self, BufferTooSmallError> {
        let mut s = Self::default();
        s.try_set(data)?;
        Ok(s)
    }

    /// Create a [`CStrBuf`] from `data` + `\0`.  Will succeed even if the `\0` doesn't fit.
    pub unsafe fn try_from_without_nul(data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Result<Self, BufferTooSmallError> {
        let mut s = Self::default();
        s.try_set_without_nul(data)?;
        Ok(s)
    }

    /// Access the underlying byte buffer of `self`
    pub fn buffer(&self) -> &[B::Unit] { self.buffer.as_slice() }

    /// Checks if `self` is empty (e.g. the first character is `\0`.)
    pub fn is_empty(&self) -> bool { self.buffer.as_slice().first().copied() == Some(private::Unit::NUL) }

    /// Get the code units of the string portion of the buffer.  This will not contain any `\0` characters, and is not guaranteed to have a `\0` after the slice!
    ///
    /// `O(n)` to locate the terminal `\0`.
    pub fn to_units(&self) -> &[B::Unit] {
        let buffer = self.buffer.as_slice();
        match buffer.iter().copied().position(|ch| ch == private::Unit::NUL) {
            Some(nul)   => &buffer[..nul],
            None        => buffer,
        }
    }

    /// Get the code units of the string portion of the buffer, including the terminal `\0`.
    /// Since the buffer might not *contain* a terminal `\0`, this may fail.
    /// You might prefer [`to_bytes`](Self::to_bytes), which cannot fail.
    ///
    /// `O(n)` to locate the terminal `\0`.
    pub fn to_units_with_nul(&self) -> Result<&[B::Unit], NotNulTerminatedError> {
        let buffer = self.buffer.as_slice();
        match buffer.iter().copied().position(|ch| ch == private::Unit::NUL) {
            Some(nul)   => Ok(&buffer[..=nul]),
            None        => Err(NotNulTerminatedError(())),
        }
    }

    /// Ensure the buffer is `\0`-terminated, returning <code>[Err]\([NotNulTerminatedError]\)</code> otherwise.
    ///
    /// `O(n)` to locate the terminal `\0`.
    pub fn validate(&self) -> Result<(), NotNulTerminatedError> { self.to_units_with_nul().map(|_| ()) }

    /// Access the underlying byte buffer of `self`.
    ///
    /// ### Safety
    ///
    /// Many C APIs assume the underlying buffer is `\0`-terminated, and this method would let you change that.
    /// However, it's worth noting that [`CStrBuf`] technically makes no such guarantee!
    pub unsafe fn buffer_mut(&mut self) -> &mut [B::Unit] { self.buffer.as_slice_mut() }

    /// Ensure the buffer is `\0`-terminated by setting the last character to be `\0`.
    ///
    /// ### Panics
    ///
    /// If `self.buffer.is_empty()` (...did you create a `CStrBuf<[u8; 0]>` or something?  Weirdo.)
    pub fn nul_truncate(&mut self) -> CStrNonNull {
        let buffer = self.buffer.as_slice_mut();
        *buffer.last_mut().unwrap() = private::Unit::NUL;
        unsafe { CStrNonNull::from_ptr_unchecked_unbounded(buffer.as_ptr().cast()) }
    }

    /// Modifies the buffer to contain `data` + `\0`.
    /// If `data` will not fit, it will be truncated with a final `\0` before returning <code>[Err]\([BufferTooSmallError]\)</code>.
    ///
    /// ### Panics
    ///
    /// If `self.buffer.as_slice_mut().is_empty()` (...did you create a `CStrBuf<[u8; 0]>` or something?  Weirdo.)
    pub fn set_truncate(&mut self, data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Result<(), BufferTooSmallError> {
        let src = data.as_ref();
        let dst = self.buffer.as_slice_mut();
        let n = (dst.len()-1).min(src.len());
        dst[..n].copy_from_slice(&src[..n]);
        dst[n] = private::Unit::NUL;
        if src.len() >= dst.len() { Err(BufferTooSmallError(()))? }
        Ok(())
    }

    /// Modifies the buffer to contain `data` + `\0`.
    /// If `data` will not fit, it will be truncated - *without* a final `\0` - before returning <code>[Err]\([BufferTooSmallError]\)</code>.
    pub unsafe fn set_truncate_without_nul(&mut self, data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Result<(), BufferTooSmallError> {
        let src = data.as_ref();
        let dst = self.buffer.as_slice_mut();
        let n = dst.len().min(src.len());
        dst[..n].copy_from_slice(&src[..n]);
        if let Some(dst) = dst.get_mut(n) { *dst = private::Unit::NUL; }
        if src.len() > dst.len() { Err(BufferTooSmallError(()))? }
        Ok(())
    }

    /// Modifies the buffer to contain `data` + `\0`.
    /// If `data` + '\0' will not fit, <code>[Err]\([BufferTooSmallError]\)</code> will be returned without modifying the underlying buffer.
    pub fn try_set(&mut self, data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Result<(), BufferTooSmallError> {
        let src = data.as_ref();
        let dst = self.buffer.as_slice_mut();
        if src.len() >= dst.len() { Err(BufferTooSmallError(()))? }
        dst[..src.len()].copy_from_slice(src);
        dst[src.len()] = private::Unit::NUL;
        Ok(())
    }

    /// Modifies the buffer to contain `data` (and a `\0` - but only if it will fit!)
    /// If `data` will not fit, <code>[Err]\([BufferTooSmallError]\)</code> will be returned without modifying the underlying buffer.
    pub unsafe fn try_set_without_nul(&mut self, data: &(impl AsRef<[B::Unit]> + ?Sized)) -> Result<(), BufferTooSmallError> {
        let src = data.as_ref();
        let dst = self.buffer.as_slice_mut();
        if src.len() > dst.len() { Err(BufferTooSmallError(()))? }
        dst[..src.len()].copy_from_slice(src);
        if let Some(dst) = dst.get_mut(src.len()) { *dst = private::Unit::NUL; }
        Ok(())
    }
}

impl<B: Array<Unit = u8>> CStrBuf<B> {
    #[doc(hidden)] pub fn to_bytes(&self) -> &[u8] { self.to_units() } // legacy alias for 0.1.1
    #[doc(hidden)] pub fn to_bytes_with_nul(&self) -> Result<&[u8], NotNulTerminatedError> { self.to_units_with_nul() } // legacy alias for 0.1.1

    /// Attempt to convert the buffer to a [`CStr`], returning <code>[Err]\([NotNulTerminatedError]\)</code> instead if the underlying buffer isn't `\0`-terminated.
    /// You might prefer [`to_string_lossy`](Self::to_string_lossy), which cannot fail, or [`to_str`](Self::to_str), which can fail due to invalid UTF8, but not due to missing `\0`s.
    ///
    /// `O(n)` to locate the terminal `\0`.
    pub fn to_cstr(&self) -> Result<&CStr, NotNulTerminatedError> { self.to_bytes_with_nul().map(|bytes| unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }) }

    /// Attempt to convert the buffer to a <code>&[str]</code>, returning <code>[Err]\([Utf8Error]\)</code> instead if the underlying buffer wasn't valid UTF8.
    ///
    /// `O(n)` to locate the terminal `\0`.
    pub fn to_str(&self) -> Result<&str, Utf8Error> { from_utf8(self.to_bytes()) }

    /// Convert the buffer to a <code>&[str]</code>, allocating and replacing invalid UTF8 with [`U+FFFD REPLACEMENT CHARACTER`][std::char::REPLACEMENT_CHARACTER] if necessary.
    ///
    /// `O(n)` to locate the terminal `\0`.
    pub fn to_string_lossy(&self) -> Cow<'_, str> { String::from_utf8_lossy(self.to_bytes()) }
}

impl<B: Array> Default for CStrBuf<B> {
    fn default() -> Self { Self { buffer: private::Array::zeroed() } }
}

impl<B: Array> Debug for CStrBuf<B> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { private::Unit::debug(self.to_units(), f) }
}



#[cfg(feature = "bytemuck")] mod _bytemuck {
    use super::*;
    unsafe impl<B: bytemuck::Pod        > bytemuck::Pod         for CStrBuf<B> {}
    unsafe impl<B: bytemuck::Zeroable   > bytemuck::Zeroable    for CStrBuf<B> {}
}



#[test] fn abi_layout() {
    use std::os::raw::c_char;
    assert_abi_compatible!([c_char;  1], CStrBuf<[u8;  1]>);
    assert_abi_compatible!([c_char;  2], CStrBuf<[u8;  2]>);
    assert_abi_compatible!([c_char;  3], CStrBuf<[u8;  3]>);
    assert_abi_compatible!([c_char;  4], CStrBuf<[u8;  4]>);
    assert_abi_compatible!([c_char;  6], CStrBuf<[u8;  6]>);
    assert_abi_compatible!([c_char;  8], CStrBuf<[u8;  8]>);
    assert_abi_compatible!([c_char; 12], CStrBuf<[u8; 12]>);
    assert_abi_compatible!([c_char; 16], CStrBuf<[u8; 16]>);
    assert_abi_compatible!([c_char; 24], CStrBuf<[u8; 24]>);
    assert_abi_compatible!([c_char; 99], CStrBuf<[u8; 99]>);

    #[allow(non_camel_case_types)] type char16_t = u16; // could also be wchar_t on windows, unichar on iOS, etc.
    assert_abi_compatible!([char16_t;  1], CStrBuf<[char16_t;  1]>);
    assert_abi_compatible!([char16_t;  2], CStrBuf<[char16_t;  2]>);
    assert_abi_compatible!([char16_t;  3], CStrBuf<[char16_t;  3]>);
    assert_abi_compatible!([char16_t;  4], CStrBuf<[char16_t;  4]>);
    assert_abi_compatible!([char16_t;  6], CStrBuf<[char16_t;  6]>);
    assert_abi_compatible!([char16_t;  8], CStrBuf<[char16_t;  8]>);
    assert_abi_compatible!([char16_t; 12], CStrBuf<[char16_t; 12]>);
    assert_abi_compatible!([char16_t; 16], CStrBuf<[char16_t; 16]>);
    assert_abi_compatible!([char16_t; 24], CStrBuf<[char16_t; 24]>);
    assert_abi_compatible!([char16_t; 99], CStrBuf<[char16_t; 99]>);

    #[allow(non_camel_case_types)] type char32_t = u32; // could also be wchar_t on *nix
    assert_abi_compatible!([char32_t;  1], CStrBuf<[char32_t;  1]>);
    assert_abi_compatible!([char32_t;  2], CStrBuf<[char32_t;  2]>);
    assert_abi_compatible!([char32_t;  3], CStrBuf<[char32_t;  3]>);
    assert_abi_compatible!([char32_t;  4], CStrBuf<[char32_t;  4]>);
    assert_abi_compatible!([char32_t;  6], CStrBuf<[char32_t;  6]>);
    assert_abi_compatible!([char32_t;  8], CStrBuf<[char32_t;  8]>);
    assert_abi_compatible!([char32_t; 12], CStrBuf<[char32_t; 12]>);
    assert_abi_compatible!([char32_t; 16], CStrBuf<[char32_t; 16]>);
    assert_abi_compatible!([char32_t; 24], CStrBuf<[char32_t; 24]>);
    assert_abi_compatible!([char32_t; 99], CStrBuf<[char32_t; 99]>);
}



#[test] fn from() {
    type CB8 = CStrBuf<[u8; 8]>;
    {
        assert_eq!(CB8::from_truncate(b"1234567890").to_bytes(), b"1234567");
    }
    unsafe {
        assert_eq!(CB8::from_truncate_without_nul(b"1234567890").to_bytes(), b"12345678");
    }
    {
        assert_eq!(CB8::try_from(b"1234567890").is_err(), true);
        assert_eq!(CB8::try_from(b"12345678"  ).is_err(), true);
        assert_eq!(CB8::try_from(b"1234567"   ).unwrap().to_bytes(), b"1234567");
    }
    unsafe {
        assert_eq!(CB8::try_from_without_nul(b"1234567890").is_err(), true);
        assert_eq!(CB8::try_from_without_nul(b"12345678"  ).unwrap().to_bytes(), b"12345678");
        assert_eq!(CB8::try_from_without_nul(b"1234567"   ).unwrap().to_bytes(), b"1234567");
    }
}



#[test] fn set() {
    type CB8 = CStrBuf<[u8; 8]>;
    let reference = CB8::from_truncate(b"ref");
    {
        let mut cb = reference;
        assert_eq!(cb.set_truncate(b"1234567890").is_err(), true);
        assert_eq!(cb.to_bytes(), b"1234567");
        assert_eq!(cb.set_truncate(b"1234").is_err(), false);
        assert_eq!(cb.to_bytes(), b"1234");
    }
    unsafe {
        let mut cb = reference;
        assert_eq!(cb.set_truncate_without_nul(b"1234567890").is_err(), true);
        assert_eq!(cb.to_bytes(), b"12345678");
        assert_eq!(cb.set_truncate_without_nul(b"1234").is_err(), false);
        assert_eq!(cb.to_bytes(), b"1234");
    }
    {
        let mut cb = reference;
        assert_eq!(cb.try_set(b"1234567890").is_err(), true);
        assert_eq!(cb.to_bytes(), b"ref");
        assert_eq!(cb.try_set(b"12345678").is_err(), true);
        assert_eq!(cb.to_bytes(), b"ref");
        assert_eq!(cb.try_set(b"1234").is_err(), false);
        assert_eq!(cb.to_bytes(), b"1234");
    }
    unsafe {
        let mut cb = reference;
        assert_eq!(cb.try_set_without_nul(b"1234567890").is_err(), true);
        assert_eq!(cb.to_bytes(), b"ref");
        assert_eq!(cb.try_set_without_nul(b"12345678").is_err(), false);
        assert_eq!(cb.to_bytes(), b"12345678");
        assert_eq!(cb.try_set_without_nul(b"1234").is_err(), false);
        assert_eq!(cb.to_bytes(), b"1234");
    }
}



#[allow(overflowing_literals)]
#[test] fn struct_interop() {
    use std::mem::*;
    use std::os::raw::c_char;

    #[repr(C)] struct C {
        empty:          [c_char; 16],
        empty2:         [c_char; 16],
        empty3:         [c_char; 16],
        example:        [c_char; 16],
        full:           [c_char; 16],
        not_unicode:    [c_char; 16],
    }
    let mut c = C {
        empty:          [0; 16],
        empty2:         [b'f' as _; 16],
        empty3:         [b'f' as _; 16],
        example:        [0; 16],
        full:           [b'f' as _; 16],
        not_unicode:    [0; 16],
    };
    c.empty2[0] = 0;
    c.not_unicode[0] = 0xFF as c_char;
    c.not_unicode[1] = 0xFF as c_char;

    assert_abi_compatible!(R, C);
    #[repr(C)] struct R {
        empty:          CStrBuf<[u8; 16]>,
        empty2:         CStrBuf<[u8; 16]>,
        empty3:         CStrBuf<[u8; 16]>,
        example:        CStrBuf<[u8; 16]>,
        full:           CStrBuf<[u8; 16]>,
        not_unicode:    CStrBuf<[u8; 16]>,
    }
    let r : &mut R = unsafe { transmute(&mut c) };
    r.example.try_set(b"example").unwrap();
    r.empty3 = Default::default(); // !!! MUTATION !!!

    assert_eq!(r.empty          .is_empty(), true);
    assert_eq!(r.empty2         .is_empty(), true);
    assert_eq!(r.empty3         .is_empty(), true);
    assert_eq!(r.example        .is_empty(), false);
    assert_eq!(r.full           .is_empty(), false);
    assert_eq!(r.not_unicode    .is_empty(), false);

    assert_eq!(r.empty          .buffer(), b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert_eq!(r.empty2         .buffer(), b"\0fffffffffffffff");
    assert_eq!(r.empty3         .buffer(), b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert_eq!(r.example        .buffer(), b"example\0\0\0\0\0\0\0\0\0");
    assert_eq!(r.full           .buffer(), b"ffffffffffffffff");
    assert_eq!(r.not_unicode    .buffer(), b"\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

    assert_eq!(r.empty          .to_bytes(), b"");
    assert_eq!(r.empty2         .to_bytes(), b"");
    assert_eq!(r.empty3         .to_bytes(), b"");
    assert_eq!(r.example        .to_bytes(), b"example");
    assert_eq!(r.full           .to_bytes(), b"ffffffffffffffff");
    assert_eq!(r.not_unicode    .to_bytes(), b"\xFF\xFF");

    assert_eq!(r.empty          .to_bytes_with_nul(), Ok(&b"\0"[..]));
    assert_eq!(r.empty2         .to_bytes_with_nul(), Ok(&b"\0"[..]));
    assert_eq!(r.empty3         .to_bytes_with_nul(), Ok(&b"\0"[..]));
    assert_eq!(r.example        .to_bytes_with_nul(), Ok(&b"example\0"[..]));
    assert_eq!(r.full           .to_bytes_with_nul(), Err(NotNulTerminatedError(())));
    assert_eq!(r.not_unicode    .to_bytes_with_nul(), Ok(&b"\xFF\xFF\0"[..]));

    assert_eq!(r.empty          .to_cstr(), Ok(CStr::from_bytes_with_nul(b"\0").unwrap()));
    assert_eq!(r.empty2         .to_cstr(), Ok(CStr::from_bytes_with_nul(b"\0").unwrap()));
    assert_eq!(r.empty3         .to_cstr(), Ok(CStr::from_bytes_with_nul(b"\0").unwrap()));
    assert_eq!(r.example        .to_cstr(), Ok(CStr::from_bytes_with_nul(b"example\0").unwrap()));
    assert_eq!(r.full           .to_cstr(), Err(NotNulTerminatedError(())));
    assert_eq!(r.not_unicode    .to_cstr(), Ok(CStr::from_bytes_with_nul(b"\xFF\xFF\0").unwrap()));

    assert_eq!(r.empty          .to_str(), Ok(""));
    assert_eq!(r.empty2         .to_str(), Ok(""));
    assert_eq!(r.empty3         .to_str(), Ok(""));
    assert_eq!(r.example        .to_str(), Ok("example"));
    assert_eq!(r.full           .to_str(), Ok("ffffffffffffffff"));
    assert_eq!(r.not_unicode    .to_str().is_err(), true);

    assert_eq!(r.empty          .to_string_lossy(), "");
    assert_eq!(r.empty2         .to_string_lossy(), "");
    assert_eq!(r.empty3         .to_string_lossy(), "");
    assert_eq!(r.example        .to_string_lossy(), "example");
    assert_eq!(r.full           .to_string_lossy(), "ffffffffffffffff");
    assert_eq!(r.not_unicode    .to_string_lossy(), "\u{FFFD}\u{FFFD}");

    assert_eq!(r.empty          .validate().is_err(), false);
    assert_eq!(r.empty2         .validate().is_err(), false);
    assert_eq!(r.empty3         .validate().is_err(), false);
    assert_eq!(r.example        .validate().is_err(), false);
    assert_eq!(r.full           .validate().is_err(), true);
    assert_eq!(r.not_unicode    .validate().is_err(), false);

    assert_eq!(format!("{:?}", r.empty          ), "\"\"" );
    assert_eq!(format!("{:?}", r.empty2         ), "\"\"" );
    assert_eq!(format!("{:?}", r.empty3         ), "\"\"" );
    assert_eq!(format!("{:?}", r.example        ), "\"example\"" );
    assert_eq!(format!("{:?}", r.full           ), "\"ffffffffffffffff\"" );
    assert_eq!(format!("{:?}", r.not_unicode    ), "\"\\xff\\xff\"" );

    unsafe {
        assert_eq!(r.empty          .buffer_mut(), b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(r.empty2         .buffer_mut(), b"\0fffffffffffffff");
        assert_eq!(r.empty3         .buffer_mut(), b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(r.example        .buffer_mut(), b"example\0\0\0\0\0\0\0\0\0");
        assert_eq!(r.full           .buffer_mut(), b"ffffffffffffffff");
        assert_eq!(r.not_unicode    .buffer_mut(), b"\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    }

    // !!! MUTATION !!!
    assert_eq!(r.empty          .nul_truncate().to_bytes(), b"");
    assert_eq!(r.empty2         .nul_truncate().to_bytes(), b"");
    assert_eq!(r.empty3         .nul_truncate().to_bytes(), b"");
    assert_eq!(r.example        .nul_truncate().to_bytes(), b"example");
    assert_eq!(r.full           .nul_truncate().to_bytes(), b"fffffffffffffff");
    assert_eq!(r.not_unicode    .nul_truncate().to_bytes(), b"\xFF\xFF");
    // !!! MUTATED !!!

    assert_eq!(r.empty          .buffer(), b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert_eq!(r.empty2         .buffer(), b"\0ffffffffffffff\0"); // modified with terminal \0
    assert_eq!(r.empty3         .buffer(), b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert_eq!(r.example        .buffer(), b"example\0\0\0\0\0\0\0\0\0");
    assert_eq!(r.full           .buffer(), b"fffffffffffffff\0"); // modified with terminal \0
    assert_eq!(r.not_unicode    .buffer(), b"\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

    assert_eq!(r.empty          .validate().is_err(), false);
    assert_eq!(r.empty2         .validate().is_err(), false);
    assert_eq!(r.empty3         .validate().is_err(), false);
    assert_eq!(r.example        .validate().is_err(), false);
    assert_eq!(r.full           .validate().is_err(), false); // now valid
    assert_eq!(r.not_unicode    .validate().is_err(), false);
}
