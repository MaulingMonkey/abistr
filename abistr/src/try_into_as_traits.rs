use crate::*;

use std::ffi::*;



/// Converts `self` ([str]/[String]/[CStr]/[CString]) into something that implements [AsCStr]
pub trait TryIntoAsCStr {
    /// The temporary type that can be treated as a C-string.
    type Target : AsCStr;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, NulError>;
}

impl<T: AsCStr> TryIntoAsCStr for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, NulError> { Ok(self) }
}

impl<'s> TryIntoAsCStr for &'s str {
    type Target = CString;
    fn try_into(self) -> Result<Self::Target, NulError> {
        CString::new(self)
    }
}

impl TryIntoAsCStr for String {
    type Target = CString;
    fn try_into(self) -> Result<Self::Target, NulError> {
        CString::new(self)
    }
}



/// Converts `self` ([str]/[String]/[CStr]/[CString]/\(\)) into something that implements [AsOptCStr]
pub trait TryIntoAsOptCStr {
    /// The temporary type that can be treated as an [Option]al C-string.
    type Target : AsOptCStr;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, NulError>;
}

impl<T: AsOptCStr> TryIntoAsOptCStr for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, NulError> { Ok(self) }
}

impl<'s> TryIntoAsOptCStr for &'s str {
    type Target = CString;
    fn try_into(self) -> Result<Self::Target, NulError> {
        CString::new(self)
    }
}

impl TryIntoAsOptCStr for String {
    type Target = CString;
    fn try_into(self) -> Result<Self::Target, NulError> {
        CString::new(self)
    }
}

impl<'s> TryIntoAsOptCStr for Option<&'s str> {
    type Target = Option<CString>;
    fn try_into(self) -> Result<Self::Target, NulError> {
        self.map_or(Ok(None), |s| CString::new(s).map(|s| Some(s)))
    }
}

impl TryIntoAsOptCStr for Option<String> {
    type Target = Option<CString>;
    fn try_into(self) -> Result<Self::Target, NulError> {
        self.map_or(Ok(None), |s| CString::new(s).map(|s| Some(s)))
    }
}



#[test] fn basic_usage() {
    fn f(_: impl TryIntoAsCStr) {}
    //f(()); // not legal
    f("test");
    f(cstr!("test"));
    f(CStrNonNull::from_bytes_with_nul(b"test\0").unwrap());
    f(String::from("test"));
    f(CString::new("test").unwrap());
    f(CString::new("test").unwrap().as_c_str());

    //f(CStrPtr::from_bytes_with_nul(b"test\0").unwrap()); // CStrPtr could be null, not passable
    //f(CStrPtr::NULL);


    fn o(_: impl TryIntoAsOptCStr) {}
    o(());
    o("test");
    o(cstr!("test"));
    o(CStrNonNull::from_bytes_with_nul(b"test\0").unwrap());
    o(String::from("test"));
    o(CString::new("test").unwrap());
    o(CString::new("test").unwrap().as_c_str());

    o(Some("test"));
    o(Some(cstr!("test")));
    o(Some(CStrNonNull::from_bytes_with_nul(b"test\0").unwrap()));
    o(Some(String::from("test")));
    o(Some(CString::new("test").unwrap()));
    o(Some(CString::new("test").unwrap().as_c_str()));

    o(CStrPtr::from_bytes_with_nul(b"test\0").unwrap());
    o(CStrPtr::NULL);
}
