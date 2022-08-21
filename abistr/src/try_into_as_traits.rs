use crate::{c_char, *};

#[cfg(feature = "std")] use std::ffi::*;



/// Converts `self` ([str]/[String]/[CStr]/[CString]) into something that implements [AsCStr]
pub trait TryIntoAsCStr<C = c_char> {
    /// The temporary type that can be treated as a C-string.
    type Target : AsCStr<C>;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, InteriorNulError>;
}

impl<C, T: AsCStr<C>> TryIntoAsCStr<C> for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(self) }
}

#[cfg(feature = "std")] impl TryIntoAsCStr<c_char> for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "std")] impl TryIntoAsCStr<u8    > for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "widestring")] impl TryIntoAsCStr<u16   > for &'_ str { type Target = widestring::U16CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }
#[cfg(feature = "widestring")] impl TryIntoAsCStr<u32   > for &'_ str { type Target = widestring::U32CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }

#[cfg(feature = "std")] impl TryIntoAsCStr<c_char> for String { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "std")] impl TryIntoAsCStr<u8    > for String { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "widestring")] impl TryIntoAsCStr<u16   > for String { type Target = widestring::U16CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }
#[cfg(feature = "widestring")] impl TryIntoAsCStr<u32   > for String { type Target = widestring::U32CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }



/// Converts `self` ([str]/[String]/[CStr]/[CString]/\(\)) into something that implements [AsOptCStr]
pub trait TryIntoAsOptCStr<C = c_char> {
    /// The temporary type that can be treated as an [Option]al C-string.
    type Target : AsOptCStr<C>;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, InteriorNulError>;
}

impl<C, T: AsOptCStr<C>> TryIntoAsOptCStr<C> for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(self) }
}

#[cfg(feature = "std")] impl TryIntoAsOptCStr<c_char> for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "std")] impl TryIntoAsOptCStr<u8    > for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u16> for &'_ str { type Target = widestring::U16CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u32> for &'_ str { type Target = widestring::U32CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }

#[cfg(feature = "std")] impl TryIntoAsOptCStr<c_char> for String { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "std")] impl TryIntoAsOptCStr<u8    > for String { type Target = CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(CString::new(self)?) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u16> for String { type Target = widestring::U16CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u32> for String { type Target = widestring::U32CString; fn try_into(self) -> Result<Self::Target, InteriorNulError> { Self::Target::from_str(self).map_err(|_| InteriorNulError(())) } }

#[cfg(feature = "std")] impl TryIntoAsOptCStr<c_char> for Option<&'_ str> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(CString::new(s)?))) } }
#[cfg(feature = "std")] impl TryIntoAsOptCStr<u8    > for Option<&'_ str> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(CString::new(s)?))) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u16> for Option<&'_ str> { type Target = Option<widestring::U16CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| widestring::U16CString::from_str(s).map(|s| Some(s)).map_err(|_| InteriorNulError(()))) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u32> for Option<&'_ str> { type Target = Option<widestring::U32CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| widestring::U32CString::from_str(s).map(|s| Some(s)).map_err(|_| InteriorNulError(()))) } }

#[cfg(feature = "std")] impl TryIntoAsOptCStr<c_char> for Option<String> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(CString::new(s)?))) } }
#[cfg(feature = "std")] impl TryIntoAsOptCStr<u8    > for Option<String> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(CString::new(s)?))) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u16> for Option<String> { type Target = Option<widestring::U16CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| widestring::U16CString::from_str(s).map(|s| Some(s)).map_err(|_| InteriorNulError(()))) } }
#[cfg(feature = "widestring")] impl TryIntoAsOptCStr<u32> for Option<String> { type Target = Option<widestring::U32CString>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| widestring::U32CString::from_str(s).map(|s| Some(s)).map_err(|_| InteriorNulError(()))) } }



#[test] fn basic_usage() {
    fn f(_: impl TryIntoAsCStr) {}
    #[cfg(feature = "std")] f("test");
    f(cstr!("test"));
    #[cfg(feature = "std")] f(CStrNonNull::from_bytes_with_nul(b"test\0").unwrap());
    #[cfg(feature = "std")] f(String::from("test"));
    #[cfg(feature = "std")] f(CString::new("test").unwrap());
    #[cfg(feature = "std")] f(CString::new("test").unwrap().as_c_str());



    fn o(_: impl TryIntoAsOptCStr) {}
    o(());
    #[cfg(feature = "std")] o("test");
    o(cstr!("test"));
    #[cfg(feature = "std")] o(CStrNonNull::from_bytes_with_nul(b"test\0").unwrap());
    #[cfg(feature = "std")] o(String::from("test"));
    #[cfg(feature = "std")] o(CString::new("test").unwrap());
    #[cfg(feature = "std")] o(CString::new("test").unwrap().as_c_str());

    #[cfg(feature = "std")] o(Some("test"));
    o(Some(cstr!("test")));
    #[cfg(feature = "std")] o(Some(CStrNonNull::from_bytes_with_nul(b"test\0").unwrap()));
    #[cfg(feature = "std")] o(Some(String::from("test")));
    #[cfg(feature = "std")] o(Some(CString::new("test").unwrap()));
    #[cfg(feature = "std")] o(Some(CString::new("test").unwrap().as_c_str()));

    #[cfg(feature = "std")] o(CStrPtr::from_bytes_with_nul(b"test\0").unwrap());
    o(CStrPtr::NULL);
}

#[cfg(feature = "std")] #[allow(dead_code)] mod compile_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn o(_: impl TryIntoAsOptCStr) {}
    /// o(());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: impl TryIntoAsCStr) {}
    /// f(());
    /// ```
    struct Unit;

    /// ```no_run
    /// use abistr::*;
    /// fn o(_: impl TryIntoAsOptCStr) {}
    /// o(CStrPtr::from_bytes_with_nul(b"test\0").unwrap());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: impl TryIntoAsCStr) {}
    /// f(CStrPtr::from_bytes_with_nul(b"test\0").unwrap());
    /// ```
    struct CStrPtrTest;

    /// ```no_run
    /// use abistr::*;
    /// fn o(_: impl TryIntoAsOptCStr) {}
    /// o(CStrPtr::NULL);
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: impl TryIntoAsCStr) {}
    /// f(CStrPtr::NULL);
    /// ```
    struct CStrPtrNull;
}
