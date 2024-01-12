//! [`CP437`], [`Unknown8`], [`Unknown16`], [`Unknown32`],
//! [`Utf8`], [`Utf8ish`], [`Utf16`], [`Utf16ish`], [`Utf32`], [`Utf32ish`],
//! [`WindowsCurrentAnsiCodePage`]

use bytemuck::{CheckedBitPattern, NoUninit};

use crate::*;

#[cfg(feature = "alloc")] use core::iter::FromIterator;

use core::convert::TryFrom;
use core::fmt::{self, Formatter};



/// An encoding scheme, mapping unsigned integer values to characters.
///
/// May assume each value can be converted into a Unicode value or placeholder.
pub trait Encoding : Copy + 'static {
    /// The minimum unit of information.
    type Unit : Unit;

    /// For use in [`core::fmt::Debug`] implementations.
    fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result;

    /// In debug builds, check if `units` is valid for this [`Encoding`].
    ///
    /// This is used to help detect and diagnose undefined behavior in debug builds.
    /// Do **not** rely on it to *avoid* undefined behavior at runtime: it is always valid for this to do nothing.
    fn debug_check_valid(_units: &[Self::Unit]) {}

    /// In debug builds, check if `units` is valid for this [`Encoding`].
    ///
    /// This is used to help detect and diagnose undefined behavior in debug builds.
    /// Do **not** rely on it to *avoid* undefined behavior at runtime: it is always valid for this to do nothing.
    ///
    /// ### Safety
    /// *   If `units` is non-null, it must point to a valid `\0`-terminated string.
    unsafe fn debug_check_valid_ptr(_units: *const Self::Unit) {}
}



/// Create <code>[Encoding]::[Unit](Encoding::Unit)</code>s from [`Unit`]s infalliably.
///
/// ### Safety
/// A lot of code relies this trait being correctly implemented for soundness.
/// E.g. [`Utf8`]-encoded strings are expected to be convertable to <code>&amp;[str]</code> without extra checks, and thus cannot implement this trait.
pub unsafe trait FromUnitsInfalliable<U: crate::Unit> : Encoding {
    /// Cast `units` to this [Encoding]'s units.
    fn from_units_infalliable(units: &[U]) -> &[Self::Unit];

    /// Cast `units` to this [Encoding]'s units.
    ///
    /// ### Safety
    /// *   `units` must be a valid `\0`-terminated C-string
    /// *   `units` cannot be <code>[null]\(\)</code>
    /// *   `units` must outlive the returned slice
    unsafe fn from_units_infalliable_ptr<'a>(units: *const U, include_nul: bool) -> &'a [Self::Unit] {
        Self::from_units_infalliable(unsafe { core::slice::from_raw_parts(units, strlen(units).wrapping_add(include_nul.into())) })
    }
}



/// Create <code>[Encoding]::[Unit](Encoding::Unit)</code>s from [`Unit`]s.
///
/// ### Safety
/// A lot of code relies this trait being correctly implemented for soundness.
/// E.g. [`Utf8`]-encoded strings are expected to be convertable to <code>&amp;[str]</code> without extra checks.
pub unsafe trait FromUnits<U: crate::Unit> : Encoding {
    /// Attempt to verify `units` are valid for this [Encoding].
    ///
    /// ### Returns
    /// *   `Ok(...)`               &mdash; the entire string was valid for this [Encoding].
    /// *   `Err((valid, invalid))` &mdash; the string was partially valid for this [Encoding].  The returned slice is split between the valid and invalid portions.
    fn from_units(units: &[U]) -> Result<&[Self::Unit], (&[Self::Unit], &[U])>;

    /// Attempt to verify `units` are valid for this [Encoding].
    ///
    /// ### Returns
    /// *   `Ok(...)`               &mdash; the entire string was valid for this [Encoding].
    /// *   `Err((valid, invalid))` &mdash; the string was partially valid for this [Encoding].  The returned slice is split between the valid and invalid portions.
    ///
    /// ### Safety
    /// *   `units` must be a valid `\0`-terminated C-string
    /// *   `units` cannot be <code>[null]\(\)</code>
    /// *   `units` must outlive the returned slice
    unsafe fn from_units_ptr<'a>(units: *const U, include_nul: bool) -> Result<&'a [Self::Unit], (&'a [Self::Unit], &'a [U])> {
        Self::from_units(unsafe { core::slice::from_raw_parts(units, strlen(units).wrapping_add(include_nul.into())) })
    }
}

unsafe impl<U: crate::Unit, T: FromUnitsInfalliable<U>> FromUnits<U> for T {
    fn from_units(units: &[U]) -> Result<&[Self::Unit], (&[Self::Unit], &[U])> { Ok(Self::from_units_infalliable(units)) }
}



/// Convert a <code>[Encoding]::[Unit](Encoding::Unit)</code> to [`char`] in a `1:1` infalliable fashion.
pub trait FixedToCharInfalliable : Encoding {
    /// Convert a <code>[Encoding]::[Unit](Encoding::Unit)</code> to [`char`] in a `1:1` infalliable fashion.
    fn to_char(unit: Self::Unit) -> char;
}

/// Convert a slice of <code>[Encoding]::[Unit](Encoding::Unit)</code> to [`char`]s.
pub trait ToChars : Encoding {
    /// Trim the first [`char`] off of `units`.
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()>;

    /// Format `units` as a string.
    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> {
        let original = units;
        if core::any::TypeId::of::<Self::Unit>() == core::any::TypeId::of::<u8>() {
            if let Ok(s) = core::str::from_utf8(unsafe { core::slice::from_raw_parts(original.as_ptr().cast(), original.len()) }) {
                let mut chars = s.chars();
                let mut units = original;
                loop {
                    match (chars.next(), (!units.is_empty()).then(|| Self::next_char(&mut units))) {
                        (Some(ch1), Some(Ok(ch2))   ) => { if ch1 != ch2 { break } },
                        (None,      None            ) => return s.into(), // reached end of string, it's valid!
                        (_,         _               ) => break, // mismatch, decode error, etc.
                    }
                }
            }
        }

        let mut s = alloc::string::String::new();
        s.reserve(original.len());
        let mut units = original;
        while !units.is_empty() {
            let _prev_len = units.len();
            match Self::next_char(&mut units) {
                Ok(ch) => s.push(ch),
                Err(()) => s.push(char::REPLACEMENT_CHARACTER),
            }
            debug_assert!(units.len() < _prev_len, "Self::next_char failed to advance");
        }
        s.into()
    }
}

impl<T: FixedToCharInfalliable> ToChars for T {
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()> {
        let (first, rest) = units.split_first().ok_or(())?;
        *units = rest;
        Ok(Self::to_char(*first))
    }
}



/// An unknown 8-bit encoding.
#[derive(Clone, Copy)] pub struct Unknown8;
impl Encoding for Unknown8 {
    type Unit = u8;
    fn debug_fmt(units: &[u8], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) }
}
unsafe impl FromUnitsInfalliable<i8> for Unknown8 { fn from_units_infalliable(units: &[i8]) -> &[u8] { bytemuck::must_cast_slice(units) } }
unsafe impl FromUnitsInfalliable<u8> for Unknown8 { fn from_units_infalliable(units: &[u8]) -> &[u8] { units } }

/// An unknown 16-bit encoding.
#[derive(Clone, Copy)] pub struct Unknown16;
impl Encoding for Unknown16 {
    type Unit = u16;
    fn debug_fmt(units: &[u16], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c16_units(units, fmt) }
}
unsafe impl FromUnitsInfalliable<u16> for Unknown16 { fn from_units_infalliable(units: &[u16]) -> &[u16] { units } }

/// An unknown 32-bit encoding.
#[derive(Clone, Copy)] pub struct Unknown32;
impl Encoding for Unknown32 {
    type Unit = u32;
    fn debug_fmt(units: &[u32], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c32_units(units, fmt) }
}
unsafe impl FromUnitsInfalliable<u32 > for Unknown32 { fn from_units_infalliable(units: &[u32 ]) -> &[u32] { units } }
unsafe impl FromUnitsInfalliable<char> for Unknown32 { fn from_units_infalliable(units: &[char]) -> &[u32] { bytemuck::must_cast_slice(units) } }



/// [Code page 437](https://en.wikipedia.org/wiki/Code_page_437), a fixed-length encoding with symbols for `0x00..0x20` instead of control codes.
#[derive(Clone, Copy)] pub struct CP437;
impl Encoding for CP437 {
    type Unit = u8;
    fn debug_fmt(units: &[u8], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) }
}
unsafe impl FromUnitsInfalliable<i8> for CP437 { fn from_units_infalliable(units: &[i8]) -> &[u8] { bytemuck::must_cast_slice(units) } }
unsafe impl FromUnitsInfalliable<u8> for CP437 { fn from_units_infalliable(units: &[u8]) -> &[u8] { units } }
impl FixedToCharInfalliable for CP437 {
    fn to_char(unit: u8) -> char {
        // https://en.wikipedia.org/wiki/Code_page_437#Character_set
        match unit {
            0x00 => '\u{0000}', 0x01 => '\u{263A}', 0x02 => '\u{263B}', 0x03 => '\u{2665}', 0x04 => '\u{2666}', 0x05 => '\u{2663}', 0x06 => '\u{2660}', 0x07 => '\u{2022}',
            0x08 => '\u{25D8}', 0x09 => '\u{25CB}', 0x0A => '\u{25D9}', 0x0B => '\u{2642}', 0x0C => '\u{2640}', 0x0D => '\u{266A}', 0x0E => '\u{266B}', 0x0F => '\u{263C}',
            0x10 => '\u{25BA}', 0x11 => '\u{25C4}', 0x12 => '\u{2195}', 0x13 => '\u{203C}', 0x14 => '\u{00B6}', 0x15 => '\u{00A7}', 0x16 => '\u{25AC}', 0x17 => '\u{21A8}',
            0x18 => '\u{2191}', 0x19 => '\u{2193}', 0x1A => '\u{2192}', 0x1B => '\u{2190}', 0x1C => '\u{221F}', 0x1D => '\u{2194}', 0x1E => '\u{25B2}', 0x1F => '\u{25BC}',
            0x20 ..= 0x7E => char::from(unit),                                                                                                          0x7F => '\u{2302}', // house/delta symbol
            0x80 => '\u{00C7}', 0x81 => '\u{00FC}', 0x82 => '\u{00E9}', 0x83 => '\u{00E2}', 0x84 => '\u{00E4}', 0x85 => '\u{00E0}', 0x86 => '\u{00E5}', 0x87 => '\u{00E7}',
            0x88 => '\u{00EA}', 0x89 => '\u{00EB}', 0x8A => '\u{00E8}', 0x8B => '\u{00EF}', 0x8C => '\u{00EE}', 0x8D => '\u{00EC}', 0x8E => '\u{00C4}', 0x8F => '\u{00C5}',
            0x90 => '\u{00C9}', 0x91 => '\u{00E6}', 0x92 => '\u{00C6}', 0x93 => '\u{00F4}', 0x94 => '\u{00F6}', 0x95 => '\u{00F2}', 0x96 => '\u{00FB}', 0x97 => '\u{00F9}',
            0x98 => '\u{00FF}', 0x99 => '\u{00D6}', 0x9A => '\u{00DC}', 0x9B => '\u{00A2}', 0x9C => '\u{00A3}', 0x9D => '\u{00A5}', 0x9E => '\u{20A7}', 0x9F => '\u{0192}',
            0xA0 => '\u{00E1}', 0xA1 => '\u{00ED}', 0xA2 => '\u{00F3}', 0xA3 => '\u{00FA}', 0xA4 => '\u{00F1}', 0xA5 => '\u{00D1}', 0xA6 => '\u{00AA}', 0xA7 => '\u{00BA}',
            0xA8 => '\u{00BF}', 0xA9 => '\u{2310}', 0xAA => '\u{00AC}', 0xAB => '\u{00BD}', 0xAC => '\u{00BC}', 0xAD => '\u{00A1}', 0xAE => '\u{00AB}', 0xAF => '\u{00BB}',
            0xB0 => '\u{2591}', 0xB1 => '\u{2592}', 0xB2 => '\u{2593}', 0xB3 => '\u{2502}', 0xB4 => '\u{2524}', 0xB5 => '\u{2561}', 0xB6 => '\u{2562}', 0xB7 => '\u{2556}',
            0xB8 => '\u{2555}', 0xB9 => '\u{2563}', 0xBA => '\u{2551}', 0xBB => '\u{2557}', 0xBC => '\u{255D}', 0xBD => '\u{255C}', 0xBE => '\u{255B}', 0xBF => '\u{2510}',
            0xC0 => '\u{2514}', 0xC1 => '\u{2534}', 0xC2 => '\u{252C}', 0xC3 => '\u{251C}', 0xC4 => '\u{2500}', 0xC5 => '\u{253C}', 0xC6 => '\u{255E}', 0xC7 => '\u{255F}',
            0xC8 => '\u{255A}', 0xC9 => '\u{2554}', 0xCA => '\u{2569}', 0xCB => '\u{2566}', 0xCC => '\u{2560}', 0xCD => '\u{2550}', 0xCE => '\u{256C}', 0xCF => '\u{2567}',
            0xD0 => '\u{2568}', 0xD1 => '\u{2564}', 0xD2 => '\u{2565}', 0xD3 => '\u{2559}', 0xD4 => '\u{2558}', 0xD5 => '\u{2552}', 0xD6 => '\u{2553}', 0xD7 => '\u{256B}',
            0xD8 => '\u{256A}', 0xD9 => '\u{2518}', 0xDA => '\u{250C}', 0xDB => '\u{2588}', 0xDC => '\u{2584}', 0xDD => '\u{258C}', 0xDE => '\u{2590}', 0xDF => '\u{2580}',
            0xE0 => '\u{03B1}', 0xE1 => '\u{00DF}', 0xE2 => '\u{0393}', 0xE3 => '\u{03C0}', 0xE4 => '\u{03A3}', 0xE5 => '\u{03C3}', 0xE6 => '\u{00B5}', 0xE7 => '\u{03C4}',
            0xE8 => '\u{03A6}', 0xE9 => '\u{0398}', 0xEA => '\u{03A9}', 0xEB => '\u{03B4}', 0xEC => '\u{221E}', 0xED => '\u{03C6}', 0xEE => '\u{03B5}', 0xEF => '\u{2229}',
            0xF0 => '\u{2261}', 0xF1 => '\u{00B1}', 0xF2 => '\u{2265}', 0xF3 => '\u{2264}', 0xF4 => '\u{2320}', 0xF5 => '\u{2321}', 0xF6 => '\u{00F7}', 0xF7 => '\u{2248}',
            0xF8 => '\u{00B0}', 0xF9 => '\u{2219}', 0xFA => '\u{00B7}', 0xFB => '\u{221A}', 0xFC => '\u{207F}', 0xFD => '\u{00B2}', 0xFE => '\u{25A0}', 0xFF => '\u{00A0}',
        }
    }
}



/// Valid [UTF-8](https://en.wikipedia.org/wiki/UTF-8).  This is the encoding of most Rust strings.
#[derive(Clone, Copy)] pub struct Utf8;
impl Encoding for Utf8 {
    type Unit = u8;
    fn debug_fmt(units: &[u8], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) }
    fn debug_check_valid(units: &[u8]) { if cfg!(debug_assertions) { core::str::from_utf8(units).expect("`units` is not a valid UTF-8 string"); } }
    unsafe fn debug_check_valid_ptr(units: *const u8) { if cfg!(debug_assertions) { unsafe { core::ffi::CStr::from_ptr(units.cast()).to_str().expect("`units` is not a valid UTF-8 string"); } } }
}
impl ToChars for Utf8 {
    fn next_char(units: &mut &[u8]) -> Result<char, ()> {
        // https://en.wikipedia.org/wiki/UTF-8#Encoding
        let (lead, after) = units.split_first().ok_or(())?;
        let lead = *lead;
        let size = match lead {
            0b_0_0000000 ..= 0b_0_1111111 => 1,
            0b_1_0000000 ..= 0b_101_11111 => { *units = after; return Err(()) },
            0b_110_00000 ..= 0b_110_11111 => 2,
            0b_1110_0000 ..= 0b_1110_1111 => 3,
            0b_11110_000 ..= 0b_11110_111 => 4,
            0b_111110_00 ..= 0b_111111_11 => { *units = after; return Err(()) },
        };
        if size > units.len() { *units = &[]; return Err(()); }
        let (ch, after) = units.split_at(size);
        *units = after;
        let mut s = core::str::from_utf8(ch).map_err(|_| ())?.chars();
        s.next().ok_or(())
    }

    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> { alloc::string::String::from_utf8_lossy(units) }
}

/// [UTF-8](https://en.wikipedia.org/wiki/UTF-8).  Might contain invalid sequences, invalid codepoints, etc.  Common encoding on Linux.
#[derive(Clone, Copy)] pub struct Utf8ish;
impl Encoding for Utf8ish {
    type Unit = u8;
    fn debug_fmt(units: &[u8], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) }
}
impl ToChars for Utf8ish {
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()> { Ok(Utf8::next_char(units).unwrap_or(char::REPLACEMENT_CHARACTER)) }
    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> { alloc::string::String::from_utf8_lossy(units) }
}
unsafe impl FromUnitsInfalliable<i8> for Utf8ish { fn from_units_infalliable(units: &[i8]) -> &[u8] { bytemuck::must_cast_slice(units) } }
unsafe impl FromUnitsInfalliable<u8> for Utf8ish { fn from_units_infalliable(units: &[u8]) -> &[u8] { units } }

impl From<Utf8 > for Utf8ish  { fn from(_: Utf8 ) -> Self { Self } }

/// Valid [UTF-16](https://en.wikipedia.org/wiki/UTF-16).
#[derive(Clone, Copy)] pub struct Utf16;
impl Encoding for Utf16 {
    type Unit = u16;
    fn debug_fmt(units: &[u16], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c16_units(units, fmt) }
}
impl ToChars for Utf16 {
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()> {
        let first = *take_first(units).ok_or(())?;

        const SURROGATE_SIGNAL_MASK : u16 = 0b11111100_00000000;
        const SURROGATE_VALUE_MASK  : u16 = 0b00000011_00000000;
        const SURROGATE_HIGH        : u16 = 0b110110_0000000000;
        const SURROGATE_LOW         : u16 = 0b110111_0000000000;

        let hi;
        let lo;
        let before_take = *units;
        match first & SURROGATE_SIGNAL_MASK {
            SURROGATE_LOW => {
                lo = first; hi = *take_first(units).ok_or(())?;
                if hi & SURROGATE_SIGNAL_MASK != SURROGATE_HIGH { *units = before_take; return Err(()) }
            },
            SURROGATE_HIGH => {
                hi = first; lo = *take_first(units).ok_or(())?;
                if lo & SURROGATE_SIGNAL_MASK != SURROGATE_LOW { *units = before_take; return Err(()) }
            },
            _not_a_surrogate => {
                return char::from_u32(first.into()).ok_or(());
            }
        };

        char::from_u32(((hi & SURROGATE_VALUE_MASK) << 10 | (lo & SURROGATE_VALUE_MASK)).into()).ok_or(())
    }

    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> { alloc::string::String::from_utf16_lossy(units).into() }
}


/// [UTF-16](https://en.wikipedia.org/wiki/UTF-16).  Might contain invalid sequences (orphaned/bad surrogate pairs), invalid codepoints, etc.
#[derive(Clone, Copy)] pub struct Utf16ish;
impl Encoding for Utf16ish {
    type Unit = u16;
    fn debug_fmt(units: &[u16], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c16_units(units, fmt) }
}
impl ToChars for Utf16ish {
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()> { Ok(Utf16::next_char(units).unwrap_or(char::REPLACEMENT_CHARACTER)) }
    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> { alloc::string::String::from_utf16_lossy(units).into() }
}
unsafe impl FromUnitsInfalliable<u16> for Utf16ish { fn from_units_infalliable(units: &[u16]) -> &[u16] { units } }
impl From<Utf16> for Utf16ish { fn from(_: Utf16) -> Self { Self } }

/// Valid [UTF-32](https://en.wikipedia.org/wiki/UTF-32).
#[derive(Clone, Copy)] pub struct Utf32;
impl Encoding for Utf32 {
    type Unit = char;
    fn debug_fmt(units: &[char], fmt: &mut Formatter) -> fmt::Result { crate::fmt::char_units(units, fmt) }
}
impl ToChars for Utf32 {
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()> { take_first(units).ok_or(()).copied() }
    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> { alloc::string::String::from_iter(units.iter().copied()).into() }
}
unsafe impl FromUnitsInfalliable<char> for Utf32 { fn from_units_infalliable(units: &[char]) -> &[char] { units } }
unsafe impl FromUnits<u32> for Utf32 {
    fn from_units(units: &[u32]) -> Result<&[Self::Unit], (&[Self::Unit], &[u32])> {
        let (ok, err) = must_cast_slice_checked_partial(units);
        if err.is_empty() { Ok(ok) } else { Err((ok, err)) }
    }
}

/// [UTF-32](https://en.wikipedia.org/wiki/UTF-32).  May contain invalid codepoints.
#[derive(Clone, Copy)] pub struct Utf32ish;
impl Encoding for Utf32ish {
    type Unit = u32;
    fn debug_fmt(units: &[u32], fmt: &mut Formatter) -> fmt::Result { crate::fmt::c32_units(units, fmt) }
}
impl ToChars for Utf32ish {
    fn next_char(units: &mut &[Self::Unit]) -> Result<char, ()> { char::try_from(*take_first(units).ok_or(())?).map_err(|_| {}) }
    #[cfg(feature = "alloc")] fn to_string_lossy(units: &[Self::Unit]) -> alloc::borrow::Cow<str> { alloc::string::String::from_iter(units.iter().copied().map(|u| char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))).into() }
}
unsafe impl FromUnitsInfalliable<u32 > for Utf32ish { fn from_units_infalliable(units: &[u32 ]) -> &[u32] { units } }
unsafe impl FromUnitsInfalliable<char> for Utf32ish { fn from_units_infalliable(units: &[char]) -> &[u32] { bytemuck::must_cast_slice(units) } }
impl From<Utf32> for Utf32ish { fn from(_: Utf32) -> Self { Self } }



/// "Narrow" encoding of some sort &mdash; *N.B. this might not be a strict superset of 7-bit US-ASCII.*
///
/// For example, this could be:
/// *   [Code page 437](https://en.wikipedia.org/wiki/Code_page_437), a fixed-length encoding with symbols for `0x00..0x20` instead of control codes.<br>
///     Default for U.S. English / North America?
/// *   [Shift JIS](https://en.wikipedia.org/wiki/Shift_JIS), a variable-length encoding that replaces `|` with `¥` for `0x5C` and `~` with `‾` for `0x7E`.<br>
///     Also uses the low-ASCII range in continuation bytes.
/// *   [Windows-1251](https://en.wikipedia.org/wiki/Windows-1251), a common default in Windows for Cyrillic languages such as Russian.
/// *   [Windows-1252](https://en.wikipedia.org/wiki/Windows-1252), a common default in Windows for English and other Romance/Germanic languages.
/// *   [US-ASCII](https://en.wikipedia.org/wiki/ASCII), a 7-bit fixed-length encoding.
/// *   [UTF-8](https://en.wikipedia.org/wiki/UTF-8), a variable-length unicode encoding.
/// *   [UTF-7](https://en.wikipedia.org/wiki/UTF-7), a non-standard variable-length email-safe unicode encoding.
/// *   One of [hundreds of other codepages](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)
///
/// This is *usually* a system-wide setting - however, applications can now [opt-in to process-specific UTF-8 via manifest](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/use-utf8-code-page#set-a-process-code-page-to-utf-8).<br>
/// While this might *sound* nice to an author of UTF-8 laden rust code:
/// *   Libraries cannot depend on this, as it is an application-wide setting.
/// *   *Applications* cannot depend on this either, as older windows might ignore the manifest settings.
/// *   It risks [Mojibake](https://en.wikipedia.org/wiki/Mojibake) &mdash; Microsoft themselves point out this manifest setting [isn't supported by GDI](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/use-utf8-code-page#set-a-process-code-page-to-utf-8).
/// *   Windows NT is ≈UTF-16 internally anyways &mdash; so this won't let you avoid conversion, merely shift where conversion occurs.
///
/// There are a few **wide** "narrow" encodings that I don't quite know what to make of &mdash; hopefully these are never process locales:
///
/// | Identifier | .NET Name | Additional information |
/// | -----------| ----------| -----------------------|
/// | ...   | ...           | ...
/// | 1200  | utf-16        | Unicode UTF-16, little endian byte order (BMP of ISO 10646); available only to managed applications
/// | 1201  | unicodeFFFE   | Unicode UTF-16, big endian byte order; available only to managed applications
/// | ...   | ...           | ...
/// | 12000 | utf-32        | Unicode UTF-32, little endian byte order; available only to managed applications
/// | 12001 | utf-32BE      | Unicode UTF-32, big endian byte order; available only to managed applications
/// | ...   | ...           | ...
///
/// (Source: [Code Page Identifiers](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers))
///
#[cfg(windows)] #[derive(Clone, Copy)] pub struct WindowsCurrentAnsiCodePage;
#[cfg(windows)] impl Encoding for WindowsCurrentAnsiCodePage {
    type Unit = u8;
    fn debug_fmt(units: &[u8], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) }
}



fn take_first<'s, T>(slice: &mut &'s [T]) -> Option<&'s T> {
    let (first, after) = slice.split_first()?;
    *slice = after;
    Some(first)
}

/// A hybrid of [`bytemuck::must_cast_slice`] from `S` → `D::Bits` and [`bytemuck::checked::cast_slice`] from `D::Bits` → `D` (splitting the slice where it fails instead of panicing.)
fn must_cast_slice_checked_partial<S: NoUninit, D: CheckedBitPattern>(s: &[S]) -> (&[D], &[D::Bits]) {
    let bits : &[D::Bits] = bytemuck::must_cast_slice(s);
    let i = bits.iter().position(|b| !D::is_valid_bit_pattern(b)).unwrap_or(bits.len());
    let (ok, invalid) = bits.split_at(i);
    let ok = unsafe { core::slice::from_raw_parts(ok.as_ptr().cast(), ok.len()) };
    (ok, invalid)
}
