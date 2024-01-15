//! Windows [`Encoding`]s:
//! [`System`], [`CurrentThread`], [`ConsoleInput`], [`ConsoleOutput`]
//!
//! Microsoft [recommends](https://learn.microsoft.com/en-us/windows/win32/intl/unicode) using the codepage-agnostic
//! [UTF-16]ish `wchar_t`-based `*W` APIs that work directly with Windows&nbsp;NT's native internal encoding.
//! I do too:  Not even [ASCII] is safe from [Mojibake].
//! This module, arguably, mainly exists to discourage it's own use.
//!
//!
//!
//! # There are too many simultanious [`Encoding`]s.
//!
//! A windows process typically has **multiple, different** codepages active within the context of a single thread at once.<br>
//! Running `examples/hello-world` on my `en-US` machine, I get:
//!
//! | [`Encoding`]      | <code>[CodePage]::[from](CodePage::from)(...)</code> <br> [Code Page Identifiers](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)    |
//! | ------------------| --------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [`System`]        | <code>[1251]</code>
//! | [`CurrentThread`] | <code>[1251]</code>
//! | [`ConsoleInput`]  | <code>[437]</code>
//! | [`ConsoleOutput`] | <code>[437]</code> (when run from a console or powershell) <br> <code>65001 = [UTF-8]</code> (when run under [Visual Studio Code].)
//!
//!
//!
//! # `Encoding`s need not be based on ASCII.
//!
//! For example, if you invoke <code>[chcp] [037]</code> to change your console codepage to an [EBCDIC] derivative,
//! `0x41` will encode `NBSP` instead of `'A'`, and `0xC1` will encode `'A'` instead of `┴`.
//! While such extremes are uncommon, [Shift JIS] is more common &mdash; which while [ASCII]-*based*,
//! replaces `|` with `¥` for `0x5C`, and `~` with `‾` for `0x7E`.
//!
//! It is little wonder, then, that Microsoft [recommends](https://learn.microsoft.com/en-us/windows/win32/intl/unicode) using the codepage-agnostic [UTF-16]ish `wchar_t`-based `*W` APIs that work directly with Windows&nbsp;NT's native internal encoding.
//! I do too:  Not even [ASCII] is safe from [Mojibake].
//! The types in this module arguably exist mostly to discourage you from using them.
//!
//! ```text
//! C:\local\hello-world\with\WriteConsoleA>cargo run --quiet
//! Hello, world!
//!
//! C:\local\hello-world\with\WriteConsoleA>chcp 037
//! Active code page: 37
//!
//! C:\local\hello-world\with\WriteConsoleA>cargo run --quiet
//! çÁ%%?Ï?Ê%À
//!
//! C:\local\hello-world\with\WriteConsoleA>chcp 437
//! Active code page: 437
//! ```
//!
//!
//!
//! # Windows codepages *generally* aren't wide, but...
//!
//! There *are* a few wide code pages for special use cases:
//!
//! | Identifier | .NET Name | Additional information ([Source](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers))    |
//! | -----------| ----------| -----------------------------------------------------------------------------------------------------------------|
//! | 1200  | utf-16        | Unicode UTF-16, little endian byte order (BMP of ISO 10646); available only to managed applications
//! | 1201  | unicodeFFFE   | Unicode UTF-16, big endian byte order; available only to managed applications
//! | 12000 | utf-32        | Unicode UTF-32, little endian byte order; available only to managed applications
//! | 12001 | utf-32BE      | Unicode UTF-32, big endian byte order; available only to managed applications
//!
//! These are at least rejected by <code>[chcp]</code>:
//!
//! ```text
//! C:\>chcp 1200
//! Invalid code page
//!
//! C:\>chcp 1201
//! Invalid code page
//!
//! C:\>chcp 12000
//! Invalid code page
//!
//! C:\>chcp 12001
//! Invalid code page
//! ```
//!
//!
//! # Application manifests can *request* UTF-8, but...
//!
//! Applications can [opt-in to UTF-8 via manifest](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/use-utf8-code-page#set-a-process-code-page-to-utf-8) (changing the observed [`System`] code page?)<br>
//! While this might *sound* nice to an author of UTF-8 laden rust code:
//! *   Libraries cannot depend on this, as it is an application-wide setting.
//! *   *Applications* cannot depend on this either, as older windows might ignore the manifest settings.
//! *   It risks [Mojibake](https://en.wikipedia.org/wiki/Mojibake) &mdash; Microsoft themselves point out this manifest setting [isn't supported by GDI](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/use-utf8-code-page#set-a-process-code-page-to-utf-8).
//! *   Windows NT is ≈UTF-16 internally anyways &mdash; so this won't let you avoid conversion, merely shift where conversion occurs.
//!
//! [037]:                  https://www.compart.com/en/unicode/charsets/IBM037
//! [437]:                  https://www.compart.com/en/unicode/charsets/IBM437
//! [1251]:                 https://en.wikipedia.org/wiki/Windows-1251
//! [ASCII]:                https://en.wikipedia.org/wiki/ASCII
//! [chcp]:                 https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/chcp
//! [EBCDIC]:               https://en.wikipedia.org/wiki/EBCDIC
//! [Mojibake]:             https://en.wikipedia.org/wiki/Mojibake
//! [Shift JIS]:            https://en.wikipedia.org/wiki/Shift_JIS
//! [UTF-8]:                https://en.wikipedia.org/wiki/UTF-8
//! [UTF-16]:               https://en.wikipedia.org/wiki/UTF-16
//! [Visual Studio Code]:   https://code.visualstudio.com/

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use bytemuck::*;
use core::ffi::*;
use core::fmt::{self, Debug, Formatter, Write};



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-getacp)\]
/// `CP_ACP` / `GetACP()`
/// <br>
/// The system codepage, used for GDI etc?
#[derive(Clone, Copy)] pub struct System;

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-getcpinfoexa)\]
/// `CP_THREAD_ACP`
/// <br>
/// The current thread's codepage.
/// Changes if you <code>[SetThreadLocale]\(...\)</code>.
///
/// [SetThreadLocale]:  https://learn.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-setthreadlocale
#[derive(Clone, Copy)] pub struct CurrentThread;

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolecp)\]
/// `GetConsoleCP()`
/// <br>
/// Typically the console's <code>[chcp]</code>.
/// Can differ from [`ConsoleOutput`] in a pseudo console ([Visual Studio Code].)
///
/// [chcp]:                 https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/chcp
/// [Visual Studio Code]:   https://code.visualstudio.com/
#[derive(Clone, Copy)] pub struct ConsoleInput;

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsoleoutputcp)\]
/// `GetConsoleOutputCP()`
/// <br>
/// Typically the console's <code>[chcp]</code>.
/// Can differ from [`ConsoleInput`] in a pseudo console ([Visual Studio Code].)
///
/// [chcp]:                 https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/chcp
/// [Visual Studio Code]:   https://code.visualstudio.com/
#[derive(Clone, Copy)] pub struct ConsoleOutput;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)\]
/// Code Page Identifier
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::Pod, bytemuck::Zeroable)] #[repr(transparent)] pub struct CodePage(u32);
impl CodePage {
    /// A code page value.  This *should* be a real codepage such as 65001 (UTF-8), not a psuedo-codepage such as 3 (`CP_THREAD_ACP`.)
    pub const fn new_unchecked(value: u32) -> Self { Self(value) }
}

impl From<CodePage>         for u32      { fn from(_src: CodePage       ) -> Self { _src.0 } }
impl From<System>           for CodePage { fn from(_src: System         ) -> CodePage { CodePage(unsafe { GetACP() }) } }
//impl From<OEM>            for CodePage { fn from(_src: OEM            ) -> CodePage { CodePage::from(PsuedoCodePage::from(_src)) } }
//impl From<MAC>            for CodePage { fn from(_src: MAC            ) -> CodePage { CodePage::from(PsuedoCodePage::from(_src)) } }
impl From<CurrentThread>    for CodePage { fn from(_src: CurrentThread  ) -> CodePage { CodePage::from(PsuedoCodePage::from(_src)) } }
//impl From<Symbols>        for CodePage { fn from(_src: Symbols        ) -> CodePage { CodePage::from(PsuedoCodePage::from(_src)) } }
//impl From<Utf7>           for CodePage { fn from(_src: Utf7           ) -> CodePage { CodePage::new_unchecked(65000) } } // CP_UTF7
impl From<Utf8>             for CodePage { fn from(_src: Utf8           ) -> CodePage { CodePage::new_unchecked(65001) } } // CP_UTF8
impl From<Utf8ish>          for CodePage { fn from(_src: Utf8ish        ) -> CodePage { CodePage::new_unchecked(65001) } } // CP_UTF8
impl From<ConsoleInput>     for CodePage { fn from(_src: ConsoleInput   ) -> CodePage { CodePage(unsafe { GetConsoleCP() }) } }
impl From<ConsoleOutput>    for CodePage { fn from(_src: ConsoleOutput  ) -> CodePage { CodePage(unsafe { GetConsoleOutputCP() }) } }
impl From<PsuedoCodePage>   for CodePage {
    fn from(value: PsuedoCodePage) -> CodePage {
        let mut info = CPINFOEXA::zeroed();
        let r = unsafe { GetCPInfoExA(value.0, 0, &mut info) };
        debug_assert!(r != 0);
        CodePage(info.code_page)
    }
}



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)\]
/// Code Page Identifier
/// or psuedo-codepage such as `CP_ACP` (system active codepage), `CP_THREAD_ACP` (current thread active codepage), etc.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::Pod, bytemuck::Zeroable)] #[repr(transparent)] pub struct PsuedoCodePage(u32);
impl PsuedoCodePage {
    /// A code page value.  This *should* be a real codepage such as 65001 (UTF-8), or a psuedo-codepage such as 3 (`CP_THREAD_ACP`.)
    pub const fn new_unchecked(value: u32) -> Self { Self(value) }
}

impl From<PsuedoCodePage>   for u32            { fn from(pcp: PsuedoCodePage) -> Self { pcp.0 } }
impl From<System>           for PsuedoCodePage { fn from(_src: System       ) -> PsuedoCodePage { PsuedoCodePage(    0) } } // CP_ACP
//impl From<OEM>            for PsuedoCodePage { fn from(_src: OEM          ) -> PsuedoCodePage { PsuedoCodePage(    1) } } // CP_OEMCP
//impl From<MAC>            for PsuedoCodePage { fn from(_src: MAC          ) -> PsuedoCodePage { PsuedoCodePage(    2) } } // CP_MACCP
impl From<CurrentThread>    for PsuedoCodePage { fn from(_src: CurrentThread) -> PsuedoCodePage { PsuedoCodePage(    3) } } // CP_THREAD_ACP
//impl From<Symbols>        for PsuedoCodePage { fn from(_src: Symbols      ) -> PsuedoCodePage { PsuedoCodePage(   42) } } // CP_SYMBOL
//impl From<Utf7>           for PsuedoCodePage { fn from(_src: Utf7         ) -> PsuedoCodePage { PsuedoCodePage::from(CodePage::from(_src)) } }
impl From<Utf8>             for PsuedoCodePage { fn from(_src: Utf8         ) -> PsuedoCodePage { PsuedoCodePage::from(CodePage::from(_src)) } }
impl From<Utf8ish>          for PsuedoCodePage { fn from(_src: Utf8ish      ) -> PsuedoCodePage { PsuedoCodePage::from(CodePage::from(_src)) } }
impl From<ConsoleInput>     for PsuedoCodePage { fn from(_src: ConsoleInput ) -> PsuedoCodePage { PsuedoCodePage::from(CodePage::from(_src)) } }
impl From<ConsoleOutput>    for PsuedoCodePage { fn from(_src: ConsoleOutput) -> PsuedoCodePage { PsuedoCodePage::from(CodePage::from(_src)) } }
impl From<CodePage>         for PsuedoCodePage { fn from(_src: CodePage     ) -> PsuedoCodePage { PsuedoCodePage(_src.0) } } // all code-pages count as PsuedoCodePage s



impl Encoding for System        { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }
impl Encoding for CurrentThread { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }
impl Encoding for ConsoleInput  { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }
impl Encoding for ConsoleOutput { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }

// TODO: Implement these in terms of MultiByteToWideChar ?
//impl ToChars for System           { ... }
//impl ToChars for CurrentThread    { ... }
//impl ToChars for ConsoleInput     { ... }
//impl ToChars for ConsoleOutput    { ... }

impl Debug for       CodePage { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { debug("CodePage",       self.0, f) } }
impl Debug for PsuedoCodePage { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { debug("PsuedoCodePage", self.0, f) } }

fn debug(ty: &'static str, codepage: u32, fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "{ty}({codepage} {psuedo})", psuedo = match codepage {
        0  => "CP_ACP",
        1  => "CP_OEMCP",
        2  => "CP_MACCP",
        3  => "CP_THREAD_CP",
        42 => "CP_SYMBOL",
        _ => {
            let mut info = CPINFOEXW::zeroed();
            if 0 == unsafe { GetCPInfoExW(codepage, 0, &mut info) } {
                write!(fmt, "{ty}({codepage})")?;
            } else {
                write!(fmt, "{ty}(")?;
                let mut units = info.code_page_name.to_units(); // N.B. this contains the integer value *and* description, e.g. "1252  (ANSI - Latin I)", "437   (OEM - United States)", "65001 (UTF-8)"
                while !units.is_empty() {
                    let ch = Utf16ish::next_char(&mut units).unwrap_or('?');
                    if ch == '\"' { fmt.write_char('\\')?; }
                    fmt.write_char(ch)?;
                }
                write!(fmt, ")")?;
            }
            return Ok(())
        },
    })
}



#[link(name = "user32")] extern "system" {
    fn GetACP() -> c_uint;
    fn GetConsoleCP() -> c_uint;
    fn GetConsoleOutputCP() -> c_uint;
    fn GetCPInfoExA(code_page: c_uint, dw_flags: c_uint, cp_info_ex: &mut CPINFOEXA) -> c_uint;
    fn GetCPInfoExW(code_page: c_uint, dw_flags: c_uint, cp_info_ex: &mut CPINFOEXW) -> c_uint;
}

#[repr(C)] #[derive(Clone, Zeroable)] struct CPINFOEXA {
    pub max_char_size:          c_uint,
    pub default_char:           [u8;  2],
    pub lead_byte:              [u8; 12],
    pub unicode_default_char:   u16,
    pub code_page:              c_uint,
    pub code_page_name:         CStrBuf<Unknown8, 260>,
}

#[repr(C)] #[derive(Clone, Zeroable)] struct CPINFOEXW {
    pub max_char_size:          c_uint,
    pub default_char:           [u8;  2],
    pub lead_byte:              [u8; 12],
    pub unicode_default_char:   u16,
    pub code_page:              c_uint,
    pub code_page_name:         CStrBuf<Utf16ish, 260>,
}
