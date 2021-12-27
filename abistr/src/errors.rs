use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};



/// The buffer in question is too small to contain the string in question
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferTooSmallError(pub(crate) ());

impl Debug      for BufferTooSmallError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("BufferTooSmallError") } }
impl Display    for BufferTooSmallError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is too large for the buffer") } }
impl Error      for BufferTooSmallError { fn description(&self) -> &str { "data provided is too large for the buffer" } }



/// The string in question contains no terminal `\0`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotNulTerminatedError(pub(crate) ());

impl Debug      for NotNulTerminatedError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("NotNulTerminatedError") } }
impl Display    for NotNulTerminatedError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is not nul terminated") } }
impl Error      for NotNulTerminatedError { fn description(&self) -> &str { "data provided is not nul terminated" } }

impl From<NotNulTerminatedError> for std::ffi::FromBytesWithNulError {
    fn from(_: NotNulTerminatedError) -> std::ffi::FromBytesWithNulError {
        std::ffi::CStr::from_bytes_with_nul(&[]).unwrap_err()
    }
}

impl From<NotNulTerminatedError> for FromUnitsWithNulError {
    fn from(_: NotNulTerminatedError) -> FromUnitsWithNulError {
        FromUnitsWithNulError(())
    }
}



/// The string in question contains no terminal `\0`, or contains an interior `\0`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FromUnitsWithNulError(pub(crate) ());
impl Debug      for FromUnitsWithNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("FromUnitsWithNulError") } }
impl Display    for FromUnitsWithNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is not nul terminated, or contains interior nuls") } }
impl Error      for FromUnitsWithNulError { fn description(&self) -> &str { "data provided is not nul terminated, or contains interior nuls" } }



/// The string in question contains an interior `\0`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InteriorNulError(pub(crate) ());
impl Debug      for InteriorNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("InteriorNulError") } }
impl Display    for InteriorNulError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided contains interior nuls") } }
impl Error      for InteriorNulError { fn description(&self) -> &str { "data provided contains interior nuls" } }
impl From<std::ffi::NulError> for InteriorNulError { fn from(_: std::ffi::NulError) -> Self { Self(()) } }
