use crate::*;

#[cfg(feature = "std")] use std::error::Error;
#[cfg(feature = "std")] use std::ffi;

use core::fmt::{self, Debug, Display, Formatter};

#[cfg_attr(not(feature = "std"), allow(dead_code))]
macro_rules! convert {
    ( $src:ty => $dst:ty ) => {
        impl From<$src> for $dst {
            fn from(_: $src) -> Self { Self(()) }
        }
    };
}



/// The buffer in question is too small to contain the string in question
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferTooSmallError(pub(crate) ());

impl Debug      for BufferTooSmallError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("BufferTooSmallError") } }
impl Display    for BufferTooSmallError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is too large for the buffer") } }
#[cfg(feature = "std")]
impl Error      for BufferTooSmallError { fn description(&self) -> &str { "data provided is too large for the buffer" } }



/// The string in question contains no terminal `\0`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotNulTerminatedError(pub(crate) ());

impl Debug      for NotNulTerminatedError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("NotNulTerminatedError") } }
impl Display    for NotNulTerminatedError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is not nul terminated") } }
#[cfg(feature = "std")]
impl Error      for NotNulTerminatedError { fn description(&self) -> &str { "data provided is not nul terminated" } }



/// The string in question contains no terminal `\0`, contains an interior `\0`, or contains an invalid sequence for the [`Encoding`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FromUnitsWithNulError(pub(crate) ());
impl Debug      for FromUnitsWithNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("FromUnitsWithNulError") } }
impl Display    for FromUnitsWithNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is not nul terminated, contains interior nuls, or contains invalid sequences for the Encoding") } }
#[cfg(feature = "std")]
impl Error      for FromUnitsWithNulError { fn description(&self) -> &str { "data provided is not nul terminated, contains interior nuls, or contains invalid sequences for the Encoding" } }
#[cfg(feature = "std")] convert!(ffi::FromBytesWithNulError => FromUnitsWithNulError);
// convert!(ffi::FromVecWithNulError => FromUnitsWithNulError); // not yet stable
// convert!(NotNulTerminatedError => FromUnitsWithNulError);    // ...is this lossy?
// convert!(InteriorNulError => FromUnitsWithNulError);         // ...is this lossy?



/// The string in question contains an interior `\0`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InteriorNulError(pub(crate) ());
impl Debug      for InteriorNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("InteriorNulError") } }
impl Display    for InteriorNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided contains interior nuls") } }
#[cfg(feature = "std")]
impl Error      for InteriorNulError { fn description(&self) -> &str { "data provided contains interior nuls" } }
#[cfg(feature = "std")] convert!(ffi::NulError => InteriorNulError);

impl InteriorNulError {
    pub(crate) fn check<U: Unit>(str: &[U]) -> Result<(), InteriorNulError> {
        if str.iter().copied().any(|u| u == U::NUL) {
            Err(InteriorNulError(()))
        } else {
            Ok(())
        }
    }
}
