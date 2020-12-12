use std::fmt::{self, Formatter};

pub(crate) fn cstr_bytes(bytes: &[u8], f: &mut Formatter) -> fmt::Result {
    write!(f, "\"")?;
    for b in bytes.iter().copied() {
        match b {
            b'\t'           => write!(f, "\\t")?,
            b'\r'           => write!(f, "\\r")?,
            b'\n'           => write!(f, "\\n")?,
            b'\''           => write!(f, "\\'")?,
            b'\"'           => write!(f, "\\\"")?,
            b'\\'           => write!(f, "\\\\")?,
            0x20 ..= 0x7E   => write!(f, "{}", b as char)?,
            esc             => write!(f, "\\x{:02x}", esc)?,
        }
    }
    write!(f, "\"")?;
    Ok(())
}
