/// [`u8`]/[`u16`]/[`u32`], a rough analog to a [Unicode Code Unit](https://unicode.org/glossary/#code_unit).
///
/// | Encoding  | Rust      | C++       | Windows   | Other     |
/// | --------- | --------- | --------- | --------- | --------- |
/// | ASCII     | ~~c_char~~| char                                    |           |                   |
/// | UTF-8     | [u8]      | [char8_t] (C++20), char, unsigned char  |           |                   |
/// | UTF-16    | [u16]     | [char16_t] (C++11), uint16_t            | wchar_t   | [unichar], [jchar]|
/// | UTF-32    | [u32]     | [char32_t] (C++11), uint32_t            |           | wchar_t           |
///
/// [char8_t]:              https://en.cppreference.com/w/cpp/language/types#char8_t
/// [char16_t]:             https://en.cppreference.com/w/cpp/language/types#char16_t
/// [char32_t]:             https://en.cppreference.com/w/cpp/language/types#char32_t
/// [jchar]:                https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html
/// [unichar]:              https://developer.apple.com/documentation/foundation/unichar
pub trait Unit : private::Unit {}
impl Unit for i8    {}
impl Unit for u8    {}
impl Unit for u16   {}
impl Unit for u32   {}
impl Unit for char  {}

pub(crate) mod private {
    use core::fmt::Debug;

    pub trait Unit : Default + Copy + PartialEq + Debug + 'static {
        const NUL : Self;
        const EMPTY : &'static [Self; 1];
        fn zeroed<const N: usize>() -> [Self; N];
    }

    impl Unit for i8 {
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }

    impl Unit for u8 {
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }

    impl Unit for u16 {
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }

    impl Unit for u32 {
        const NUL : Self = 0;
        const EMPTY : &'static [Self; 1] = &[0];
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }

    impl Unit for char {
        const NUL : Self = '\0';
        const EMPTY : &'static [Self; 1] = &['\0'];
        fn zeroed<const N: usize>() -> [Self; N] { unsafe { core::mem::zeroed() } }
    }
}

pub(crate) unsafe fn strlen<U: Unit>(mut str: *const U) -> usize {
    let mut n = 0;
    loop {
        if unsafe { *str } == U::NUL { return n; }
        n += 1;
        str = unsafe { str.offset(1) };
    }
}
