#[cfg(any(doc, test))] use crate::*;


#[cfg(test)] macro_rules! assert_abi_compatible {
    ( $left:ty, $right:ty ) => {{
        assert!(
            std::mem::size_of::<$left>() == std::mem::size_of::<$right>(),
            "ABIs not compatible: size_of::<{}>() == {} != {} == size_of::<{}>()",
            stringify!($left), std::mem::size_of::<$left>(), std::mem::size_of::<$right>(), stringify!($right)
        );
        assert!(
            std::mem::align_of::<$left>() == std::mem::align_of::<$right>(),
            "ABIs not compatible: align_of::<{}>() == {} != {} == align_of::<{}>()",
            stringify!($left), std::mem::align_of::<$left>(), std::mem::align_of::<$right>(), stringify!($right)
        );
    }};
}

/// Create a <code>&[CStrNonNull]</code> literal at compile time
#[cfg(doc)]
#[macro_export]
macro_rules! cstr {
    ( $string:literal ) => {
        $crate::abistr_macros::cstr_impl!(($crate) $string)
    };
}

/// Create a <code>&[CStrNonNull]</code> literal at compile time
#[cfg(not(doc))] // use wildcards for better error messages from proc macro
#[macro_export]
macro_rules! cstr {
    ( $($tt:tt)+ ) => {
        $crate::abistr_macros::cstr_impl!(($crate) $($tt)+)
    };
}



#[test] fn basics() {
    fn a(_: CStrNonNull<'static>) {}
    fn b(_: CStrNonNull) {}

    let empty       = cstr!("");
    let example     = cstr!("example");
    let not_unicode = cstr!(b"\xFF\xFF");

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
