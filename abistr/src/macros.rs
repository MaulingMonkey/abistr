#[cfg(any(doc, test))] use crate::*;


#[cfg(test)] macro_rules! assert_abi_compatible {
    ( $left:ty, $right:ty ) => {{
        assert!(
            core::mem::size_of::<$left>() == core::mem::size_of::<$right>(),
            "ABIs not compatible: size_of::<{}>() == {} != {} == size_of::<{}>()",
            stringify!($left), core::mem::size_of::<$left>(), core::mem::size_of::<$right>(), stringify!($right)
        );
        assert!(
            core::mem::align_of::<$left>() == core::mem::align_of::<$right>(),
            "ABIs not compatible: align_of::<{}>() == {} != {} == align_of::<{}>()",
            stringify!($left), core::mem::align_of::<$left>(), core::mem::align_of::<$right>(), stringify!($right)
        );
    }};
}

macro_rules! for_each {
    ( use {$(($path1:path, $path2:path, $path3:path)),+ $(,)?} as ($ident1:ident, $ident2:ident, $ident3:ident) $block:block ) => { $( const _ : () = { use $path1 as $ident1; use $path2 as $ident2; use $path3 as $ident3; $block }; )+ };
    ( use {$(($path1:path, $path2:path, $path3:path)),+ $(,)?} as ($ident1:ident, $ident2:ident, $ident3:ident); $($tt:tt)* ) => { for_each! { use {$(($path1, $path2, $path3)),+} as ($ident1, $ident2, $ident3) { for_each! { $($tt)* } } } };

    ( use {$(($path1:path, $path2:path)),+ $(,)?} as ($ident1:ident, $ident2:ident) $block:block ) => { $( const _ : () = { use $path1 as $ident1; use $path2 as $ident2; $block }; )+ };
    ( use {$(($path1:path, $path2:path)),+ $(,)?} as ($ident1:ident, $ident2:ident); $($tt:tt)* ) => { for_each! { use {$(($path1, $path2)),+} as ($ident1, $ident2) { for_each! { $($tt)* } } } };

    ( use {$($path:path),+ $(,)?} as $ident:ident $block:block ) => { $( const _ : () = { use $path as $ident; $block }; )+ };
    ( use {$($path:path),+ $(,)?} as $ident:ident; $($tt:tt)* ) => { for_each! { use {$($path),+} as $ident { for_each! { $($tt)* } } } };

    ( $($tt:tt)* ) => { $($tt)* };
}

// The wildcard versions of these macros generate better error messages.

#[doc = "Create a <code>&[CStrNonNull]<[Unknown8] ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! unknown8  { ( $string:literal ) => { $crate::abistr_macros::unknown8!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Unknown8] ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! unknown8  { ( $($tt:tt)+      ) => { $crate::abistr_macros::unknown8!(($crate) $($tt)+) } }
#[doc = "Create a <code>&[CStrNonNull]<[Unknown16]></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! unknown16 { ( $string:literal ) => { $crate::abistr_macros::unknown16!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Unknown16]></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! unknown16 { ( $($tt:tt)+      ) => { $crate::abistr_macros::unknown16!(($crate) $($tt)+) } }
#[doc = "Create a <code>&[CStrNonNull]<[Unknown32]></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! unknown32 { ( $string:literal ) => { $crate::abistr_macros::unknown32!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Unknown32]></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! unknown32 { ( $($tt:tt)+      ) => { $crate::abistr_macros::unknown32!(($crate) $($tt)+) } }

#[doc = "Create a <code>&[CStrNonNull]<[Utf8]     ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! utf8      { ( $string:literal ) => { $crate::abistr_macros::utf8!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf8]     ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! utf8      { ( $($tt:tt)+      ) => { $crate::abistr_macros::utf8!(($crate) $($tt)+) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf16]    ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! utf16     { ( $string:literal ) => { $crate::abistr_macros::utf16!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf16]    ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! utf16     { ( $($tt:tt)+      ) => { $crate::abistr_macros::utf16!(($crate) $($tt)+) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf32]    ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! utf32     { ( $string:literal ) => { $crate::abistr_macros::utf32!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf32]    ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! utf32     { ( $($tt:tt)+      ) => { $crate::abistr_macros::utf32!(($crate) $($tt)+) } }

#[doc = "Create a <code>&[CStrNonNull]<[Utf8ish]  ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! utf8ish   { ( $string:literal ) => { $crate::abistr_macros::utf8ish!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf8ish]  ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! utf8ish   { ( $($tt:tt)+      ) => { $crate::abistr_macros::utf8ish!(($crate) $($tt)+) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf16ish] ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! utf16ish  { ( $string:literal ) => { $crate::abistr_macros::utf16ish!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf16ish] ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! utf16ish  { ( $($tt:tt)+      ) => { $crate::abistr_macros::utf16ish!(($crate) $($tt)+) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf32ish] ></code> literal at compile time"] #[cfg(    doc )] #[macro_export] macro_rules! utf32ish  { ( $string:literal ) => { $crate::abistr_macros::utf32ish!(($crate) $string) } }
#[doc = "Create a <code>&[CStrNonNull]<[Utf32ish] ></code> literal at compile time"] #[cfg(not(doc))] #[macro_export] macro_rules! utf32ish  { ( $($tt:tt)+      ) => { $crate::abistr_macros::utf32ish!(($crate) $($tt)+) } }



#[test] fn basics() {
    fn a(_: CStrNonNull<'static, encoding::Utf8ish>) {}
    fn b(_: CStrNonNull<encoding::Utf8ish>) {}

    const _C : CStrNonNull<'static, encoding::Utf8> = utf8!("C");

    let empty       = utf8ish!("");
    let example     = utf8ish!("example");
    let not_unicode = utf8ish!(b"\xFF\xFF");

    assert_eq!(empty        .to_bytes(), b"");
    assert_eq!(example      .to_bytes(), b"example");
    assert_eq!(not_unicode  .to_bytes(), b"\xFF\xFF");

    a(empty);
    b(empty);
    a(example);
    b(example);
    a(not_unicode);
    b(not_unicode);
}

#[test] fn basics8() {
    fn a(_: CStrNonNull<'static, encoding::Utf8ish>) {}
    fn b(_: CStrNonNull<encoding::Utf8ish>) {}

    const _C : CStrNonNull<'static, encoding::Utf8> = utf8!("C");

    let empty       = utf8ish!("");
    let example     = utf8ish!("example");
    let not_unicode = utf8ish!(b"\xFF\xFF");

    assert_eq!(empty        .to_units(), b"");
    assert_eq!(example      .to_units(), b"example");
    assert_eq!(not_unicode  .to_units(), b"\xFF\xFF");

    a(empty);
    b(empty);
    a(example);
    b(example);
    a(not_unicode);
    b(not_unicode);
}

#[test] fn basics16() {
    fn a(_: CStrNonNull<'static, encoding::Utf16>) {}
    fn b(_: CStrNonNull<encoding::Utf16>) {}

    const _C : CStrNonNull<'static, encoding::Utf16> = utf16!("C");

    let empty       = utf16!("");
    let example     = utf16!("example");

    assert_eq!(empty        .to_units(), []);
    assert_eq!(example      .to_units(), [b'e' as u16, b'x' as u16, b'a' as u16, b'm' as u16, b'p' as u16, b'l' as u16, b'e' as u16]);

    a(empty);
    b(empty);
    a(example);
    b(example);
}

#[test] fn basics32() {
    fn a(_: CStrNonNull<'static, encoding::Utf32>) {}
    fn b(_: CStrNonNull<encoding::Utf32>) {}

    const _C : CStrNonNull<'static, encoding::Utf32> = utf32!("C");

    let empty       = utf32!("");
    let example     = utf32!("example");

    assert_eq!(empty        .to_units(), []);
    assert_eq!(example      .to_units(), "example".chars().collect::<alloc::vec::Vec<_>>().as_slice());

    a(empty);
    b(empty);
    a(example);
    b(example);
}

mod compile_tests {
    /// ```no_run
    /// use abistr::*;
    /// let _ = unknown8!(b"\xFF");
    /// let _ = utf8ish!(b"\xFF");
    /// ```
    ///
    /// ```compile_fail
    /// let _ = utf8!(b"\xFF"); // not valid UTF-8
    /// ```
    #[allow(dead_code)] struct HexByteFF;

    /// ```no_run
    /// use abistr::*;
    /// let _ = unknown8!("\x7F");
    /// let _ = utf8ish!("\x7F");
    /// let _ = utf8!("\x7F");
    /// ```
    #[allow(dead_code)] struct HexByte7F;

    /// ```compile_fail
    /// use abistr::*;
    /// let _ = unknown8!("\xFF"); // no b prefix means max is 7F
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// let _ = utf8ish!("\xFF"); // no b prefix means max is 7F
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// let _ = utf8!("\xFF"); // no b prefix means max is 7F, also not allowed by UTF-8
    /// ```
    #[allow(dead_code)] struct HexOutOfRange;

    /// ```compile_fail
    /// use abistr::*;
    /// let _ = utf16ish!("\xFF");
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// let _ = utf32ish!("\xFF");
    /// ```
    #[allow(dead_code)] struct HexAmbiguous;
}
