//! [`LC_CTYPE`]

#![allow(non_camel_case_types)]

use crate::*;

/// \[[en.cppreference.com](https://en.cppreference.com/w/cpp/locale/LC_categories)\]
/// `LC_CTYPE`
///
/// Read as: "Local Category: Character TYPE".
///
/// This controls the behavior of [null-terminated multibyte string functions](https://en.cppreference.com/w/c/string/multibyte) such as:
/// *   [`mbstowcs`](https://en.cppreference.com/w/c/string/multibyte/mbstowcs) &mdash; convert a narrow multibyte string to "its wide character representation." (typically UTF-16(?) or UTF-32) (C89)
/// *   [`mbrtowc`](https://en.cppreference.com/w/c/string/multibyte/mbrtowc)   &mdash; convert a narrow multibyte character to "its wide character representation." (typically **UCS2**(?) or UTF-32) (C95)
/// *   [`mbrtoc8`](https://en.cppreference.com/w/c/string/multibyte/mbrtoc8)   &mdash; convert a narrow multibyte character to UTF-8 (C23)
///
/// Some functions have contradictory documentation.
/// E.g. the following are documented by [en.cppreference.com](https://en.cppreference.com/) to convert from "the currently active C locale".
/// Microsoft, on the other hand, documents them as ["locale independent"](https://learn.microsoft.com/en-us/cpp/c-runtime-library/interpretation-of-multibyte-character-sequences#locale-independent-multibyte-routines) functions that convert from "UTF-8":
///
/// *   [`mbrtoc16`](https://en.cppreference.com/w/c/string/multibyte/mbrtoc16) &mdash; convert <u>???</u> to "its variable-length 16-bit wide character representation" (typically UTF-16) (C11)
/// *   [`mbrtoc32`](https://en.cppreference.com/w/c/string/multibyte/mbrtoc32) &mdash; convert <u>???</u> to "its variable-length 32-bit wide character representation" (typically UTF-32) (C11)
///
/// N.B. this is also a horrible mix of platform specific global and thread local state:
/// *   [`setlocale`](https://en.cppreference.com/w/c/locale/setlocale) (C standard)
///     &mdash; set the global locale (not thread safe!)
/// *   [`uselocale`](https://man7.org/linux/man-pages/man3/uselocale.3.html) (Linux)
///     &mdash; set/get *current thread* locale
/// *   [`_configthreadlocale(_ENABLE_PER_THREAD_LOCALE)`](https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/configthreadlocale) (Windows)
///     &mdash; enable thread-local locales for the current thread
///
///
///
/// # Linux/POSIX/Unix/??? Environment
///
/// "\*nix" style systems likely follow the pattern [gnu's libc documents](https://www.gnu.org/software/libc/manual/html_node/Standard-Environment.html):
///
/// *   `${LC_ALL}`     overrides all
/// *   `${LC_CTYPE}`   is used if `${LC_ALL}` is not set
/// *   `${LANG}`       is used if neither `${LC_ALL}` nor `${LC_CTYPE}` were set
///
///
///
/// # Windows Environment
///
/// Windows systems are more likely to eschew the C encoding/locale in favor of [`encoding::windows`] encoding/locales.
/// What interplay of defaults there might be between those locale's and C's locales gets... messy.
/// Recommended reading:
///
/// *   [Global state in the CRT](https://learn.microsoft.com/en-us/cpp/c-runtime-library/global-state)
/// *   [Locale](https://learn.microsoft.com/en-us/cpp/c-runtime-library/locale)
///     *   [Locale-dependent routines](https://learn.microsoft.com/en-us/cpp/c-runtime-library/locale#locale-dependent-routines)
/// *   [Code pages](https://learn.microsoft.com/en-us/cpp/c-runtime-library/code-pages)
/// *   [Interpretation of multibyte-character sequences](https://learn.microsoft.com/en-us/cpp/c-runtime-library/interpretation-of-multibyte-character-sequences)
///
///
/// # Relevant Headers
///
/// *   `<stdlib.h>`
/// *   `<wchar.h>` (C99 § 7.24)
/// *   `<wctype.h>` (C99 § 7.25)
///
///
///
/// # Recommended Reading
///
/// en.cppreference.com:
///
/// *   [C / Localization support](https://en.cppreference.com/w/c/locale)
/// *   [C / Strings library / Null-terminated multibyte strings](https://en.cppreference.com/w/c/string/multibyte)
/// *   [C / Strings library / Null-terminated wide strings](https://en.cppreference.com/w/c/string/wide)
/// *   [C++ / Localizations library](https://en.cppreference.com/w/cpp/locale)
#[derive(Clone, Copy)] pub struct LC_CTYPE;

impl Encoding for LC_CTYPE {
    type Unit = u8;
    fn debug_fmt(units: &[Self::Unit], fmt: &mut core::fmt::Formatter) -> core::fmt::Result { crate::fmt::cstr_bytes(units, fmt) }
}
