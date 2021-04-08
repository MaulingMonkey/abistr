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
    use std::fmt::{self, Formatter};

    /// ### Safety
    ///
    /// *   It must be safe to initialize an array of these via `unsafe { std::mem::zeroed() }`
    pub unsafe trait Unit : Default + Copy + PartialEq {
        const NUL : Self;
        fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result;
    }

    unsafe impl Unit for u8  { const NUL : Self = 0; fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(buf, fmt) } }
    unsafe impl Unit for u16 { const NUL : Self = 0; fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c16_units(buf, fmt) } }
    unsafe impl Unit for u32 { const NUL : Self = 0; fn debug(buf: &[Self], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c32_units(buf, fmt) } }
}
