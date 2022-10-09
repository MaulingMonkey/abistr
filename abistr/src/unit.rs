/// [`u8`]/[`u16`]/[`u32`], a rough analog to a [Unicode Code Unit](https://unicode.org/glossary/#code_unit).
///
/// | Encoding  | Rust      | C++       | Windows   | Other     |
/// | --------- | --------- | --------- | --------- | --------- |
/// | ASCII     | ~~c_char~~| char                                    |           |                   |
/// | UTF8      | [u8]      | [char8_t] (C++20), char, unsigned char  |           |                   |
/// | UTF16     | [u16]     | [char16_t] (C++11), uint16_t            | wchar_t   | [unichar], [jchar]|
/// | UTF32     | [u32]     | [char32_t] (C++11), uint32_t            |           | wchar_t           |
///
/// [char8_t]:              https://en.cppreference.com/w/cpp/language/types#char8_t
/// [char16_t]:             https://en.cppreference.com/w/cpp/language/types#char16_t
/// [char32_t]:             https://en.cppreference.com/w/cpp/language/types#char32_t
/// [jchar]:                https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html
/// [unichar]:              https://developer.apple.com/documentation/foundation/unichar
pub trait Unit : private::Unit {}
impl Unit for u8  {}
impl Unit for u16 {}
impl Unit for u32 {}

pub(crate) mod private {
    use crate::*;
    #[cfg(feature = "std")] use std::borrow::Cow;
    use core::char::REPLACEMENT_CHARACTER;
    use core::fmt::{self, Formatter};

    pub trait Unit : Default + Copy + PartialEq + 'static {
        type CChar : Copy + 'static; // XXX: eliminate?
        const NUL : Self;
        const EMPTY : &'static [Self; 1];
        fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result;
        #[cfg(feature = "std")] fn to_string_lossy(buf: &[Self]) -> Cow<str>;
        fn zeroed<const N: usize>() -> [Self; N];
    }

    impl Unit for u8 {
        type CChar = c_char;
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(buf, fmt) }
        #[cfg(feature = "std")] fn to_string_lossy(buf: &[Self]) -> Cow<str> { String::from_utf8_lossy(buf) }
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }

    impl Unit for u16 {
        type CChar = Self;
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c16_units(buf, fmt) }
        #[cfg(feature = "std")] fn to_string_lossy(buf: &[Self]) -> Cow<str> { Cow::Owned(String::from_utf16_lossy(buf)) }
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }

    impl Unit for u32 {
        type CChar = Self;
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c32_units(buf, fmt) }
        #[cfg(feature = "std")] fn to_string_lossy(buf: &[Self]) -> Cow<str> { Cow::Owned(buf.iter().copied().map(|ch| core::char::from_u32(ch).unwrap_or(REPLACEMENT_CHARACTER)).collect::<String>()) }
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }
}

pub(crate) unsafe fn strlen<U: Unit>(mut str: *const U) -> usize {
    let mut n = 0;
    loop {
        if *str == U::NUL { return n; }
        n += 1;
        str = str.offset(1);
    }
}
