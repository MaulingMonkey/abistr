use core::fmt::{self, Formatter};

pub(crate) fn cstr_bytes(bytes: &[u8], f: &mut Formatter) -> fmt::Result {
    write!(f, "\"")?;
    for b in bytes.iter().copied() {
        match b {
            b'\0'           => write!(f, "\\0")?,
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

pub(crate) fn c16_units(units: &[u16], f: &mut Formatter) -> fmt::Result {
    write!(f, "\"")?;
    for u in units.iter().copied() {
        match u {
            0x0000          => write!(f, "\\0")?,
            0x0009          => write!(f, "\\t")?,
            0x000D          => write!(f, "\\r")?,
            0x000A          => write!(f, "\\n")?,
            0x0027          => write!(f, "\\'")?,
            0x0022          => write!(f, "\\\"")?,
            0x005C          => write!(f, "\\\\")?,
            0x20 ..= 0x7E   => write!(f, "{}", u as u8 as char)?,

            // Rust doesn't have a UTF16 code unit escape.  Use a C++ style "\u1234" instead of a Rust style "\u{1234}"
            // to underscore this fact, and discourage using this in text which might need to round trip, which would
            // fail on surrogate pairs.
            esc             => write!(f, "\\u{:04x}", esc)?,
        }
    }
    write!(f, "\"")?;
    Ok(())
}

pub(crate) fn c32_units(units: &[u32], f: &mut Formatter) -> fmt::Result {
    write!(f, "\"")?;
    for u in units.iter().copied() {
        match u {
            0x00000000      => write!(f, "\\0")?,
            0x00000009      => write!(f, "\\t")?,
            0x0000000D      => write!(f, "\\r")?,
            0x0000000A      => write!(f, "\\n")?,
            0x00000027      => write!(f, "\\'")?,
            0x00000022      => write!(f, "\\\"")?,
            0x0000005C      => write!(f, "\\\\")?,
            0x20 ..= 0x7E   => write!(f, "{}", u as u8 as char)?,
            esc             => write!(f, "\\u{{{:x}}}", esc)?,
        }
    }
    write!(f, "\"")?;
    Ok(())
}
