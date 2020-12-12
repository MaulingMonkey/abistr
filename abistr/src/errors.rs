use std::error::Error;
use std::ffi::{CStr, FromBytesWithNulError};
use std::fmt::{self, Debug, Display, Formatter};



/// The string in question contains no terminal `\0`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotNulTerminatedError(pub(crate) ());

impl Debug      for NotNulTerminatedError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("NotNulTerminatedError") } }
impl Display    for NotNulTerminatedError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("data provided is not nul terminated") } }
impl Error      for NotNulTerminatedError { fn description(&self) -> &str { "data provided is not nul terminated" } }

impl From<NotNulTerminatedError> for FromBytesWithNulError {
    fn from(_: NotNulTerminatedError) -> FromBytesWithNulError {
        CStr::from_bytes_with_nul(&[]).unwrap_err()
    }
}
