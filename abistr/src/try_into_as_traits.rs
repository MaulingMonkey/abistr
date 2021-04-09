use crate::*;

use std::ffi::*;
use std::os::raw::c_char;



/// Converts `self` ([str]/[String]/[CStr]/[CString]) into something that implements [AsCStr]
pub trait TryIntoAsCStr<C = c_char> {
    /// The temporary type that can be treated as a C-string.
    type Target : AsCStr<C>;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, NulError>;
}

impl<C, T: AsCStr<C>> TryIntoAsCStr<C> for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, NulError> { Ok(self) }
}

impl TryIntoAsCStr<c_char> for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
impl TryIntoAsCStr<u8    > for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsCStr<u16   > for &'_ str { type Target = widestring_0_4::U16CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsCStr<u32   > for &'_ str { type Target = widestring_0_4::U32CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }

impl TryIntoAsCStr<c_char> for String { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
impl TryIntoAsCStr<u8    > for String { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsCStr<u16   > for String { type Target = widestring_0_4::U16CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsCStr<u32   > for String { type Target = widestring_0_4::U32CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }



/// Converts `self` ([str]/[String]/[CStr]/[CString]/\(\)) into something that implements [AsOptCStr]
pub trait TryIntoAsOptCStr<C = c_char> {
    /// The temporary type that can be treated as an [Option]al C-string.
    type Target : AsOptCStr<C>;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, NulError>;
}

impl<C, T: AsOptCStr<C>> TryIntoAsOptCStr<C> for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, NulError> { Ok(self) }
}

impl TryIntoAsOptCStr<c_char> for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
impl TryIntoAsOptCStr<u8    > for &'_ str { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u16> for &'_ str { type Target = widestring_0_4::U16CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u32> for &'_ str { type Target = widestring_0_4::U32CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }

impl TryIntoAsOptCStr<c_char> for String { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
impl TryIntoAsOptCStr<u8    > for String { type Target = CString; fn try_into(self) -> Result<Self::Target, NulError> { CString::new(self) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u16> for String { type Target = widestring_0_4::U16CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u32> for String { type Target = widestring_0_4::U32CString; fn try_into(self) -> Result<Self::Target, NulError> { Self::Target::from_str(self).map_err(|_| CString::new("\0").unwrap_err()) } }

impl TryIntoAsOptCStr<c_char> for Option<&'_ str> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| CString::new(s).map(|s| Some(s))) } }
impl TryIntoAsOptCStr<u8    > for Option<&'_ str> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| CString::new(s).map(|s| Some(s))) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u16> for Option<&'_ str> { type Target = Option<widestring_0_4::U16CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| widestring_0_4::U16CString::from_str(s).map(|s| Some(s)).map_err(|_| CString::new("\0").unwrap_err())) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u32> for Option<&'_ str> { type Target = Option<widestring_0_4::U32CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| widestring_0_4::U32CString::from_str(s).map(|s| Some(s)).map_err(|_| CString::new("\0").unwrap_err())) } }

impl TryIntoAsOptCStr<c_char> for Option<String> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| CString::new(s).map(|s| Some(s))) } }
impl TryIntoAsOptCStr<u8    > for Option<String> { type Target = Option<CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| CString::new(s).map(|s| Some(s))) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u16> for Option<String> { type Target = Option<widestring_0_4::U16CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| widestring_0_4::U16CString::from_str(s).map(|s| Some(s)).map_err(|_| CString::new("\0").unwrap_err())) } }
#[cfg(feature = "widestring-0-4")] impl TryIntoAsOptCStr<u32> for Option<String> { type Target = Option<widestring_0_4::U32CString>; fn try_into(self) -> Result<Self::Target, NulError> { self.map_or(Ok(None), |s| widestring_0_4::U32CString::from_str(s).map(|s| Some(s)).map_err(|_| CString::new("\0").unwrap_err())) } }



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
