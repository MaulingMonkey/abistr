use crate::*;

#[cfg(feature = "std")] #[cfg(unix   )] use std::os::unix::ffi::*;
#[cfg(feature = "std")] #[cfg(windows)] use std::os::windows::ffi::*;



/// Converts `self` ([str]/[String]/[CStr]/[CString]) into something that implements [AsCStr]
pub trait TryIntoAsCStr<E: Encoding> {
    /// The temporary type that can be treated as a C-string.
    type Target : AsCStr<E>;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, InteriorNulError>;
}

impl<E: Encoding, T: AsCStr<E>> TryIntoAsCStr<E> for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(self) }
}

#[cfg(feature = "alloc")] const _ : () = {
    use alloc::string::String;

    impl TryIntoAsCStr<Utf8     > for &'_ str { type Target = EString0<Utf8     >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsCStr<Utf8ish  > for &'_ str { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsCStr<Unknown8 > for &'_ str { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsCStr<Utf16    > for &'_ str { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Utf16ish > for &'_ str { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ str { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Utf32    > for &'_ str { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsCStr<Utf32ish > for &'_ str { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Unknown32> for &'_ str { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }

    impl TryIntoAsCStr<Utf8     > for &'_ String { type Target = EString0<Utf8     >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsCStr<Utf8ish  > for &'_ String { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsCStr<Unknown8 > for &'_ String { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsCStr<Utf16    > for &'_ String { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Utf16ish > for &'_ String { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ String { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Utf32    > for &'_ String { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsCStr<Utf32ish > for &'_ String { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Unknown32> for &'_ String { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }

    impl TryIntoAsCStr<Utf8     > for String { type Target = EString0<Utf8     >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_bytes()) } } }
    impl TryIntoAsCStr<Utf8ish  > for String { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_bytes()) } } }
    impl TryIntoAsCStr<Unknown8 > for String { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_bytes()) } } }
    impl TryIntoAsCStr<Utf16    > for String { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Utf16ish > for String { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Unknown16> for String { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsCStr<Utf32    > for String { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsCStr<Utf32ish > for String { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Unknown32> for String { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
};

#[cfg(all(feature = "std", unix))] const _ : () = {
    impl TryIntoAsCStr<Unknown8 > for &'_ std::ffi::OsStr    { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown8 > for &'_ std::ffi::OsString { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown8 > for     std::ffi::OsString { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Unknown8 > for &'_ std::path::Path    { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown8 > for &'_ std::path::PathBuf { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown8 > for     std::path::PathBuf { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_os_string().into_vec()) } } }
};

#[cfg(all(feature = "std", unix, feature = "assume-std-ffi-osstr-utf8ish-unix"))] const _ : () = {
    impl TryIntoAsCStr<Utf8ish  > for &'_ std::ffi::OsStr    { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Utf8ish  > for &'_ std::ffi::OsString { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Utf8ish  > for     std::ffi::OsString { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Utf8ish  > for &'_ std::path::Path    { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Utf8ish  > for &'_ std::path::PathBuf { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsCStr<Utf8ish  > for     std::path::PathBuf { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_os_string().into_vec()) } } }
};

#[cfg(all(feature = "std", windows))] const _ : () = {
    impl TryIntoAsCStr<Utf16ish > for &'_ std::ffi::OsStr    { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ std::ffi::OsStr    { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsCStr<Utf16ish > for &'_ std::ffi::OsString { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ std::ffi::OsString { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsCStr<Utf16ish > for     std::ffi::OsString { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsCStr<Unknown16> for     std::ffi::OsString { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }

    impl TryIntoAsCStr<Utf16ish > for &'_ std::path::Path    { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ std::path::Path    { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsCStr<Utf16ish > for &'_ std::path::PathBuf { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ std::path::PathBuf { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsCStr<Utf16ish > for     std::path::PathBuf { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsCStr<Unknown16> for     std::path::PathBuf { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
};

#[cfg(all(feature = "alloc", feature = "widestring"))] const _ : () = {
    // XXX: *probably* sound, assuming `widestring::Utf16*` demands valid Utf16 like `widestring::Utf32*` demands valid Utf32.
    impl TryIntoAsCStr<Utf16    > for &'_ widestring::Utf16Str      { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsCStr<Utf16    > for &'_ widestring::Utf16String   { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsCStr<Utf16    > for     widestring::Utf16String   { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Utf16ish > for &'_ widestring::Utf16Str      { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsCStr<Utf16ish > for &'_ widestring::Utf16String   { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsCStr<Utf16ish > for     widestring::Utf16String   { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Unknown16> for &'_ widestring::Utf16Str      { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ widestring::Utf16String   { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsCStr<Unknown16> for     widestring::Utf16String   { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Unknown16> for &'_ widestring::U16Str        { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown16> for &'_ widestring::U16String     { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown16> for     widestring::U16String     { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    // The impl of https://docs.rs/widestring/latest/widestring/utfstr/struct.Utf32Str.html#method.as_char_slice requires these types be valid Utf32
    impl TryIntoAsCStr<Utf32    > for &'_ widestring::Utf32Str      { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsCStr<Utf32    > for &'_ widestring::Utf32String   { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsCStr<Utf32    > for     widestring::Utf32String   { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } } // TODO: vec no-clone optimizations? (awkward char != u32 typing roadbump)

    impl TryIntoAsCStr<Utf32ish > for &'_ widestring::Utf32Str      { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Utf32ish > for &'_ widestring::Utf32String   { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Utf32ish > for     widestring::Utf32String   { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Unknown32> for &'_ widestring::Utf32Str      { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Unknown32> for &'_ widestring::Utf32String   { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsCStr<Unknown32> for     widestring::Utf32String   { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Unknown32> for &'_ widestring::U32Str        { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown32> for &'_ widestring::U32String     { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Unknown32> for     widestring::U32String     { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }
};

#[cfg(all(feature = "alloc", feature = "widestring", feature = "assume-widestring-utfish"))] const _ : () = {
    impl TryIntoAsCStr<Utf16ish > for &'_ widestring::U16Str        { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Utf16ish > for &'_ widestring::U16String     { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Utf16ish > for     widestring::U16String     { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsCStr<Utf32ish > for &'_ widestring::U32Str        { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Utf32ish > for &'_ widestring::U32String     { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsCStr<Utf32ish > for     widestring::U32String     { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }
};



/// Converts `self` ([str]/[String]/[CStr]/[CString]/\(\)) into something that implements [AsOptCStr]
pub trait TryIntoAsOptCStr<E: Encoding> {
    /// The temporary type that can be treated as an [Option]al C-string.
    type Target : AsOptCStr<E>;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, InteriorNulError>;
}

impl<E: Encoding, T: AsOptCStr<E>> TryIntoAsOptCStr<E> for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, InteriorNulError> { Ok(self) }
}



#[cfg(feature = "alloc")] const _ : () = {
    use alloc::string::String;

    impl TryIntoAsOptCStr<Utf8     > for &'_ str { type Target = EString0<Utf8     >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for &'_ str { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for &'_ str { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsOptCStr<Utf16    > for &'_ str { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for &'_ str { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ str { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Utf32    > for &'_ str { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsOptCStr<Utf32ish > for &'_ str { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Unknown32> for &'_ str { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }

    impl TryIntoAsOptCStr<Utf8     > for &'_ String { type Target = EString0<Utf8     >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for &'_ String { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for &'_ String { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.bytes()) } } }
    impl TryIntoAsOptCStr<Utf16    > for &'_ String { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for &'_ String { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ String { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Utf32    > for &'_ String { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsOptCStr<Utf32ish > for &'_ String { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Unknown32> for &'_ String { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }

    impl TryIntoAsOptCStr<Utf8     > for String { type Target = EString0<Utf8     >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_bytes()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for String { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_bytes()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for String { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_bytes()) } } }
    impl TryIntoAsOptCStr<Utf16    > for String { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for String { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Unknown16> for String { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_utf16()) } } }
    impl TryIntoAsOptCStr<Utf32    > for String { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsOptCStr<Utf32ish > for String { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Unknown32> for String { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
};

#[cfg(all(feature = "std", unix))] const _ : () = {
    impl TryIntoAsOptCStr<Unknown8 > for &'_ std::ffi::OsStr    { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for &'_ std::ffi::OsString { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for     std::ffi::OsString { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Unknown8 > for &'_ std::path::Path    { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for &'_ std::path::PathBuf { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown8 > for     std::path::PathBuf { type Target = EString0<Unknown8 >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_os_string().into_vec()) } } }
};

#[cfg(all(feature = "std", unix, feature = "assume-std-ffi-osstr-utf8ish-unix"))] const _ : () = {
    impl TryIntoAsOptCStr<Utf8ish  > for &'_ std::ffi::OsStr    { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for &'_ std::ffi::OsString { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for     std::ffi::OsString { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Utf8ish  > for &'_ std::path::Path    { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for &'_ std::path::PathBuf { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().as_bytes().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf8ish  > for     std::path::PathBuf { type Target = EString0<Utf8ish  >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_os_string().into_vec()) } } }
};

#[cfg(all(feature = "std", windows))] const _ : () = {
    impl TryIntoAsOptCStr<Utf16ish > for &'_ std::ffi::OsStr    { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ std::ffi::OsStr    { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for &'_ std::ffi::OsString { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ std::ffi::OsString { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for     std::ffi::OsString { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }
    impl TryIntoAsOptCStr<Unknown16> for     std::ffi::OsString { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.encode_wide()) } } }

    impl TryIntoAsOptCStr<Utf16ish > for &'_ std::path::Path    { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ std::path::Path    { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for &'_ std::path::PathBuf { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ std::path::PathBuf { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for     std::path::PathBuf { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
    impl TryIntoAsOptCStr<Unknown16> for     std::path::PathBuf { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_os_str().encode_wide()) } } }
};

#[cfg(all(feature = "alloc", feature = "widestring"))] const _ : () = {
    // XXX: *probably* sound, assuming `widestring::Utf16*` demands valid Utf16 like `widestring::Utf32*` demands valid Utf32.
    impl TryIntoAsOptCStr<Utf16    > for &'_ widestring::Utf16Str      { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsOptCStr<Utf16    > for &'_ widestring::Utf16String   { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsOptCStr<Utf16    > for     widestring::Utf16String   { type Target = EString0<Utf16    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Utf16ish > for &'_ widestring::Utf16Str      { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for &'_ widestring::Utf16String   { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for     widestring::Utf16String   { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Unknown16> for &'_ widestring::Utf16Str      { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ widestring::Utf16String   { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.code_units()) } } }
    impl TryIntoAsOptCStr<Unknown16> for     widestring::Utf16String   { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Unknown16> for &'_ widestring::U16Str        { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown16> for &'_ widestring::U16String     { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown16> for     widestring::U16String     { type Target = EString0<Unknown16>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    // The impl of https://docs.rs/widestring/latest/widestring/utfstr/struct.Utf32Str.html#method.as_char_slice requires these types be valid Utf32
    impl TryIntoAsOptCStr<Utf32    > for &'_ widestring::Utf32Str      { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsOptCStr<Utf32    > for &'_ widestring::Utf32String   { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } }
    impl TryIntoAsOptCStr<Utf32    > for     widestring::Utf32String   { type Target = EString0<Utf32    >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars()) } } } // TODO: vec no-clone optimizations? (awkward char != u32 typing roadbump)

    impl TryIntoAsOptCStr<Utf32ish > for &'_ widestring::Utf32Str      { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Utf32ish > for &'_ widestring::Utf32String   { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Utf32ish > for     widestring::Utf32String   { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Unknown32> for &'_ widestring::Utf32Str      { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Unknown32> for &'_ widestring::Utf32String   { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.chars().map(u32::from)) } } }
    impl TryIntoAsOptCStr<Unknown32> for     widestring::Utf32String   { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Unknown32> for &'_ widestring::U32Str        { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown32> for &'_ widestring::U32String     { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Unknown32> for     widestring::U32String     { type Target = EString0<Unknown32>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }
};

#[cfg(all(feature = "alloc", feature = "widestring", feature = "assume-widestring-utfish"))] const _ : () = {
    impl TryIntoAsOptCStr<Utf16ish > for &'_ widestring::U16Str        { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for &'_ widestring::U16String     { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf16ish > for     widestring::U16String     { type Target = EString0<Utf16ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }

    impl TryIntoAsOptCStr<Utf32ish > for &'_ widestring::U32Str        { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf32ish > for &'_ widestring::U32String     { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_iter(self.as_slice().iter().copied()) } } }
    impl TryIntoAsOptCStr<Utf32ish > for     widestring::U32String     { type Target = EString0<Utf32ish >; fn try_into(self) -> Result<Self::Target, InteriorNulError> { unsafe { EString0::from_vec_no_nul(self.into_vec()) } } }
};

#[cfg(feature = "alloc")] const _ : () = {
    use alloc::string::String;

    impl TryIntoAsOptCStr<Utf8     > for Option<&'_ str> { type Target = Option<EString0<Utf8     >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.bytes()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<&'_ str> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.bytes()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<&'_ str> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.bytes()) }?))) } }
    impl TryIntoAsOptCStr<Utf16    > for Option<&'_ str> { type Target = Option<EString0<Utf16    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ str> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ str> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Utf32    > for Option<&'_ str> { type Target = Option<EString0<Utf32    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars()) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<&'_ str> { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<&'_ str> { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }

    impl TryIntoAsOptCStr<Utf8     > for Option<&'_ String> { type Target = Option<EString0<Utf8     >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.bytes()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<&'_ String> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.bytes()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<&'_ String> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.bytes()) }?))) } }
    impl TryIntoAsOptCStr<Utf16    > for Option<&'_ String> { type Target = Option<EString0<Utf16    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ String> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ String> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Utf32    > for Option<&'_ String> { type Target = Option<EString0<Utf32    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars()) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<&'_ String> { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<&'_ String> { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }

    impl TryIntoAsOptCStr<Utf8     > for Option<String> { type Target = Option<EString0<Utf8     >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_bytes()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<String> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_bytes()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<String> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_bytes()) }?))) } }
    impl TryIntoAsOptCStr<Utf16    > for Option<String> { type Target = Option<EString0<Utf16    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<String> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<String> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_utf16()) }?))) } }
    impl TryIntoAsOptCStr<Utf32    > for Option<String> { type Target = Option<EString0<Utf32    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars()) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<String> { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<String> { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
};

#[cfg(all(feature = "std", unix))] const _ : () = {
    impl TryIntoAsOptCStr<Unknown8 > for Option<&'_ std::ffi::OsStr   > { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<&'_ std::ffi::OsString> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<    std::ffi::OsString> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Unknown8 > for Option<&'_ std::path::Path   > { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<&'_ std::path::PathBuf> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown8 > for Option<    std::path::PathBuf> { type Target = Option<EString0<Unknown8 >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_os_string().into_vec()) }?))) } }
};

#[cfg(all(feature = "std", unix, feature = "assume-std-ffi-osstr-utf8ish-unix"))] const _ : () = {
    impl TryIntoAsOptCStr<Utf8ish  > for Option<&'_ std::ffi::OsStr   > { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<&'_ std::ffi::OsString> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<    std::ffi::OsString> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Utf8ish  > for Option<&'_ std::path::Path   > { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<&'_ std::path::PathBuf> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().as_bytes().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf8ish  > for Option<    std::path::PathBuf> { type Target = Option<EString0<Utf8ish  >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_os_string().into_vec()) }?))) } }
};

#[cfg(all(feature = "std", windows))] const _ : () = {
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ std::ffi::OsStr   > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ std::ffi::OsStr   > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ std::ffi::OsString> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ std::ffi::OsString> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<    std::ffi::OsString> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<    std::ffi::OsString> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.encode_wide()) }?))) } }

    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ std::path::Path   > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ std::path::Path   > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ std::path::PathBuf> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ std::path::PathBuf> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<    std::path::PathBuf> { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().encode_wide()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<    std::path::PathBuf> { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_os_str().encode_wide()) }?))) } }
};

#[cfg(all(feature = "alloc", feature = "widestring"))] const _ : () = {
    // XXX: *probably* sound, assuming `widestring::Utf16*` demands valid Utf16 like `widestring::Utf32*` demands valid Utf32.
    impl TryIntoAsOptCStr<Utf16    > for Option<&'_ widestring::Utf16Str    > { type Target = Option<EString0<Utf16    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.code_units()) }?))) } }
    impl TryIntoAsOptCStr<Utf16    > for Option<&'_ widestring::Utf16String > { type Target = Option<EString0<Utf16    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.code_units()) }?))) } }
    impl TryIntoAsOptCStr<Utf16    > for Option<    widestring::Utf16String > { type Target = Option<EString0<Utf16    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ widestring::Utf16Str    > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.code_units()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ widestring::Utf16String > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.code_units()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<    widestring::Utf16String > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ widestring::Utf16Str    > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.code_units()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ widestring::Utf16String > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.code_units()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<    widestring::Utf16String > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ widestring::U16Str      > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<&'_ widestring::U16String   > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown16> for Option<    widestring::U16String   > { type Target = Option<EString0<Unknown16>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    // The impl of https://docs.rs/widestring/latest/widestring/utfstr/struct.Utf32Str.html#method.as_char_slice requires these types be valid Utf32
    impl TryIntoAsOptCStr<Utf32    > for Option<&'_ widestring::Utf32Str    > { type Target = Option<EString0<Utf32    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars()) }?))) } }
    impl TryIntoAsOptCStr<Utf32    > for Option<&'_ widestring::Utf32String > { type Target = Option<EString0<Utf32    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars()) }?))) } }
    impl TryIntoAsOptCStr<Utf32    > for Option<    widestring::Utf32String > { type Target = Option<EString0<Utf32    >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars()) }?))) } } // TODO: vec no-clone optimizations? (awkward char != u32 typing roadbump)

    impl TryIntoAsOptCStr<Utf32ish > for Option<&'_ widestring::Utf32Str    > { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<&'_ widestring::Utf32String > { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<    widestring::Utf32String > { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Unknown32> for Option<&'_ widestring::Utf32Str    > { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<&'_ widestring::Utf32String > { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.chars().map(u32::from)) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<    widestring::Utf32String > { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Unknown32> for Option<&'_ widestring::U32Str      > { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<&'_ widestring::U32String   > { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Unknown32> for Option<    widestring::U32String   > { type Target = Option<EString0<Unknown32>>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }
};


#[cfg(all(feature = "alloc", feature = "widestring", feature = "assume-widestring-utfish"))] const _ : () = {
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ widestring::U16Str      > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<&'_ widestring::U16String   > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf16ish > for Option<    widestring::U16String   > { type Target = Option<EString0<Utf16ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }

    impl TryIntoAsOptCStr<Utf32ish > for Option<&'_ widestring::U32Str      > { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<&'_ widestring::U32String   > { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_iter(s.as_slice().iter().copied()) }?))) } }
    impl TryIntoAsOptCStr<Utf32ish > for Option<    widestring::U32String   > { type Target = Option<EString0<Utf32ish >>; fn try_into(self) -> Result<Self::Target, InteriorNulError> { self.map_or(Ok(None), |s| Ok(Some(unsafe { EString0::from_vec_no_nul(s.into_vec()) }?))) } }
};



#[test] fn basic_usage() {
    fn f(_: impl TryIntoAsCStr<Unknown8>) {}
    #[cfg(feature = "alloc")] f("test");
    f(unknown8!("test"));
    #[cfg(feature = "alloc")] f(alloc::string::String::from("test"));
    #[cfg(feature = "alloc")] f(alloc::ffi::CString::new("test").unwrap());
    f(core::ffi::CStr::from_bytes_with_nul(b"test\0").unwrap());



    fn o(_: impl TryIntoAsOptCStr<Unknown8>) {}
    o(());
    #[cfg(feature = "alloc")] o("test");
    o(unknown8!("test"));
    o(CStrPtr::from(unknown8!("test")));
    o(core::ffi::CStr::from_bytes_with_nul(b"test\0").unwrap());
    #[cfg(feature = "alloc")] o(alloc::string::String::from("test"));
    #[cfg(feature = "alloc")] o(alloc::ffi::CString::new("test").unwrap());
    f(core::ffi::CStr::from_bytes_with_nul(b"test\0").unwrap());

    #[cfg(feature = "alloc")] o(Some("test"));
    o(Some(unknown8!("test")));
    #[cfg(feature = "alloc")] o(Some(alloc::string::String::from("test")));
    #[cfg(feature = "alloc")] o(Some(alloc::ffi::CString::new("test").unwrap()));
    o(Some(core::ffi::CStr::from_bytes_with_nul(b"test\0").unwrap()));

    o(CStrPtr::from(unknown8!("test")));
    o(CStrPtr::<Unknown8>::NULL);
    o(());
}

#[cfg(feature = "std")] #[allow(dead_code)] mod compile_tests {
    /// ```no_run
    /// use abistr::*;
    /// fn o(_: impl TryIntoAsOptCStr<encoding::Unknown8>) {}
    /// o(());
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: impl TryIntoAsCStr<encoding::Unknown8>) {}
    /// f(());
    /// ```
    struct Unit;

    /// ```no_run
    /// use abistr::*;
    /// fn o(_: impl TryIntoAsOptCStr<encoding::Unknown8>) {}
    /// o(CStrPtr::from(unknown8!("test")));
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: impl TryIntoAsCStr<encoding::Unknown8>) {}
    /// f(CStrPtr::from(unknown8!("test")));
    /// ```
    struct CStrPtrTest;

    /// ```no_run
    /// use abistr::*;
    /// fn o(_: impl TryIntoAsOptCStr<encoding::Unknown8>) {}
    /// o(CStrPtr::<encoding::Unknown8>::NULL);
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// fn f(_: impl TryIntoAsCStr<encoding::Unknown8>) {}
    /// f(CStrPtr::<encoding::Unknown8>::NULL);
    /// ```
    struct CStrPtrNull;
}
