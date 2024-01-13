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
//! | [`Encoding`]      | [`ToAnsiCodePage::to_ansi_code_page`] <br> [Code Page Identifiers](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)    |
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
use core::fmt::{self, Formatter};



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



/// Get the appropriate [Code Page Identifier](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers) for this [Encoding].
pub trait ToAnsiCodePage : Encoding {
    /// Get the appropriate [Code Page Identifier](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers) for this [Encoding].
    fn to_ansi_code_page(self) -> u32;
}

impl ToAnsiCodePage for System          { fn to_ansi_code_page(self) -> u32 { unsafe { GetACP()             } } }
impl ToAnsiCodePage for ConsoleInput    { fn to_ansi_code_page(self) -> u32 { unsafe { GetConsoleCP()       } } }
impl ToAnsiCodePage for ConsoleOutput   { fn to_ansi_code_page(self) -> u32 { unsafe { GetConsoleOutputCP() } } }
impl ToAnsiCodePage for CurrentThread   {
    fn to_ansi_code_page(self) -> u32 {
        let mut info = CPINFOEXA::zeroed();
        let r = unsafe { GetCPInfoExA(CP_THREAD_ACP, 0, &mut info) };
        debug_assert!(r != 0);
        info.code_page
    }
}

impl Encoding for System        { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }
impl Encoding for CurrentThread { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }
impl Encoding for ConsoleInput  { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }
impl Encoding for ConsoleOutput { type Unit = u8; fn debug_fmt(units: &[Self::Unit], fmt: &mut Formatter) -> fmt::Result { crate::fmt::cstr_bytes(units, fmt) } }

// TODO: Implement these in terms of MultiByteToWideChar ?
//impl ToChars for System           { ... }
//impl ToChars for CurrentThread    { ... }
//impl ToChars for ConsoleInput     { ... }
//impl ToChars for ConsoleOutput    { ... }



#[link(name = "user32")] extern "system" {
    fn GetACP() -> c_uint;
    fn GetConsoleCP() -> c_uint;
    fn GetConsoleOutputCP() -> c_uint;
    fn GetCPInfoExA(code_page: c_uint, dw_flags: c_uint, cp_info_ex: &mut CPINFOEXA) -> c_uint;
}

#[repr(C)] #[derive(Clone, Zeroable)] struct CPINFOEXA {
    pub max_char_size:          c_uint,
    pub default_char:           [u8;  2],
    pub lead_byte:              [u8; 12],
    pub unicode_default_char:   u16,
    pub code_page:              c_uint,
    pub code_page_name:         CStrBuf<Unknown8, 260>,
}

const CP_OEMCP      : u32 = 1;
const CP_MACCP      : u32 = 2;
const CP_THREAD_ACP : u32 = 3;
const CP_UTF7       : u32 = 65000;
const CP_UTF8       : u32 = 65001;
