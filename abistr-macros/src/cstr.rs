use proc_macro::{TokenStream, TokenTree, Delimiter, Group, Ident, Literal, Punct, Spacing, Span};

use std::convert::TryFrom;
use std::iter::FromIterator;


pub(super) trait Unit : From<u8> {
    fn name() -> &'static str;
    fn into_ts(units: &[Self], s: Span) -> TokenStream;
    fn extend(units: &mut Vec<Self>, ch: char);
}

impl Unit for u8 {
    fn name() -> &'static str { "u8" }

    fn into_ts(units: &[Self], s: Span) -> TokenStream {
        let mut literal = Literal::byte_string(units);
        literal.set_span(s);
        TokenStream::from(TokenTree::from(literal))
    }

    fn extend(units: &mut Vec<Self>, ch: char) {
        let mut buf = [0, 0, 0, 0, 0, 0];
        units.extend(ch.encode_utf8(&mut buf).bytes());
    }
}

impl Unit for u16 {
    fn name() -> &'static str { "u16" }

    fn into_ts(units: &[Self], s: Span) -> TokenStream {
        let mut elements = TokenStream::new();
        for u in units.iter().copied() {
            elements.extend(Some(TokenTree::from(Literal::u16_suffixed(u))));
            elements.extend(Some(ttp(',', Spacing::Alone, s)));
        }

        let mut array = Group::new(Delimiter::Bracket, elements);
        array.set_span(s);

        let mut o = TokenStream::new();
        o.extend(Some(ttp('&', Spacing::Alone, s)));
        o.extend(Some(TokenTree::from(array)));
        o
    }

    fn extend(units: &mut Vec<Self>, ch: char) {
        let mut buf = [0, 0];
        units.extend(ch.encode_utf16(&mut buf).iter().copied());
    }
}

impl Unit for u32 {
    fn name() -> &'static str { "u32" }

    fn into_ts(units: &[Self], s: Span) -> TokenStream {
        let mut elements = TokenStream::new();
        for u in units.iter().copied() {
            elements.extend(Some(TokenTree::from(Literal::u32_suffixed(u))));
            elements.extend(Some(ttp(',', Spacing::Alone, s)));
        }

        let mut array = Group::new(Delimiter::Bracket, elements);
        array.set_span(s);

        let mut o = TokenStream::new();
        o.extend(Some(ttp('&', Spacing::Alone, s)));
        o.extend(Some(TokenTree::from(array)));
        o
    }

    fn extend(units: &mut Vec<Self>, ch: char) {
        units.push(ch as u32);
    }
}

pub(super) fn cstr_impl<U: Unit>(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let crate_ = match input.next() {
        Some(TokenTree::Group(g)) => match g.delimiter() {
            Delimiter::Brace        => return compile_error("expected `($crate)` as first token, got `{ ... }`", g.span()).into(),
            Delimiter::Bracket      => return compile_error("expected `($crate)` as first token, got `[ ... ]`", g.span()).into(),
            Delimiter::None         => return compile_error("expected `($crate)` as first token, got `Ø ... Ø`", g.span()).into(),
            Delimiter::Parenthesis  => g.stream(),
        },
        Some(tt)    => return compile_error(format!("expected `($crate)` as first token, got `{}`", tt), tt.span()).into(),
        None        => return compile_error("expected `($crate)` as first token, got nothing", Span::call_site()).into(),
    };

    let literal = match input.next() {
        Some(TokenTree::Literal(lit)) => {
            if let Some(unexpected) = input.next() {
                return compile_error(format!("cstr!(...) expects a single string argument, unexpected `{}` token after said argument", unexpected), unexpected.span()).into();
            }
            lit
        },
        Some(TokenTree::Group(g)) => match g.delimiter() {
            Delimiter::Brace        => return compile_error("expected `\"string\"` as second token, got `{ ... }`", g.span()).into(),
            Delimiter::Bracket      => return compile_error("expected `\"string\"` as second token, got `[ ... ]`", g.span()).into(),
            Delimiter::None         => return compile_error("expected `\"string\"` as second token, got `Ø ... Ø`", g.span()).into(),
            Delimiter::Parenthesis  => return compile_error("expected `\"string\"` as second token, got `( ... )`", g.span()).into(),
        },
        Some(tt)    => return compile_error(format!("expected `\"string\"` as second token, got `{}`", tt), tt.span()).into(),
        None        => return compile_error("expected string argument to cstr!() macro", Span::call_site()).into(),
    };

    let parsed_literal = match parse_str::<U>(&literal) {
        Ok(r) => r,
        Err(err) => return err,
    };

    let s = literal.span();
    let mut o = TokenStream::new();
    o.extend(crate_);
    o.extend(vec![
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttid("CStrNonNull", s),
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttp('<', Spacing::Joint, s),
        ttid(U::name(), s),
        ttp('>', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttid("zzz_unsound_do_not_call_this_directly_from_macro_units_with_nul", s),
        ttg(Delimiter::Parenthesis, s, parsed_literal)
    ].into_iter());

    o
}

fn parse_str<U: Unit>(literal: &Literal) -> Result<TokenStream, TokenStream> {
    let s = literal.span();

    let literal = literal.to_string();
    let (byte, raw, mut literal) = if literal.starts_with("rb") || literal.starts_with("br") {
        (true, true, &literal[2..])
    } else if literal.starts_with("r") {
        (false, true, &literal[1..])
    } else if literal.starts_with("b") {
        (true, false, &literal[1..])
    } else {
        (false, false, &literal[..])
    };

    while let Some(l) = literal.strip_prefix("#") {
        literal = l.strip_suffix("#").ok_or_else(|| compile_error("expected string literal to havea balanced number of starting and ending `#`s", s))?;
    }

    let literal = literal
        .strip_prefix("\"").ok_or_else(|| compile_error("expected string literal to start with `\"`", s))?
        .strip_suffix("\"").ok_or_else(|| compile_error("expected string literal to end with `\"`", s))?;

    let mut units = Vec::<U>::new();
    let mut chars = literal.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' if !raw => {
                match chars.next() {
                    Some('0')  => Err(compile_error("interior `\0` not permitted in C string", s))?,
                    Some('t')  => units.push(U::from(b'\t')),
                    Some('n')  => units.push(U::from(b'\n')),
                    Some('r')  => units.push(U::from(b'\r')),
                    Some('\\') => units.push(U::from(b'\\')),
                    Some('\'') => units.push(U::from(b'\'')),
                    Some('\"') => units.push(U::from(b'\"')),
                    Some('x') => {
                        let mut v = 0u8;
                        for _ in 0..2 {
                            let ch = chars.next().ok_or_else(|| compile_error("expected two hexidecimal characters after `\\x` escape sequence", s))?;
                            v = v * 16 + match ch {
                                ch @ '0' ..= '9'    => ch as u8 - b'0',
                                ch @ 'a' ..= 'f'    => ch as u8 - b'a' + 10,
                                ch @ 'A' ..= 'F'    => ch as u8 - b'A' + 10,
                                _                   => Err(compile_error("expected two hexidecimal characters after `\\x` escape sequence", s))?,
                            };
                        }
                        if v == 0 {
                            Err(compile_error("interior `\0` not permitted in C string", s))?
                        } else if std::mem::size_of::<U>() != 1 {
                            Err(compile_error("`\\x` escape sequences are ambiguous - and thus forbidden - inside unicode strings (should it be 1 byte? 1 code unit? 2 hex values? 4 hex values?)", s))?
                        } else if !byte && v > 0x7F {
                            Err(compile_error("this form of character escape may only be used with characters in the range [\\x00-\\x7f]", s))?
                        }
                        units.push(U::from(v));
                    },
                    Some('u') if byte => Err(compile_error("unicode escape sequences cannot be used as a byte or in a byte string", s))?,
                    Some('u') => {
                        let mut v = 0u32;
                        if chars.next() != Some('{') { Err(compile_error("expected `{` after `\\u` escape sequence", s))? }
                        for i in 0..7 {
                            let ch = chars.next().ok_or_else(|| compile_error("expected 1-6 hexidecimal characters in `\\u{...}` escape sequence", s))?;
                            v = v * 16 + match ch {
                                ch @ '0' ..= '9' if i != 6  => ch as u32 - b'0' as u32,
                                ch @ 'a' ..= 'f' if i != 6  => ch as u32 - b'a' as u32 + 10,
                                ch @ 'A' ..= 'F' if i != 6  => ch as u32 - b'A' as u32 + 10,
                                '}'              if i != 0  => break,
                                _                           => Err(compile_error("expected 1-6 hexidecimal characters in `\\u{...}` escape sequence", s))?,
                            };
                        }
                        if v == 0 { Err(compile_error("interior `\0` not permitted in C string", s))? }
                        let ch = char::try_from(v).map_err(|_| compile_error(format!("invalid unicode codepoint U+{:04X} in `\\u{{...}}` escape sequence", v), s))?;
                        U::extend(&mut units, ch);
                    },
                    Some(ch) => U::extend(&mut units, ch),
                    None => return Err(compile_error("expected character after `\\` in string", s).into()),
                }
            },
            ch => {
                if ch == '\0' { Err(compile_error("interior `\0` not permitted in C string", s))? }
                U::extend(&mut units, ch);
            },
        }
    }
    units.push(U::from(0));
    Ok(U::into_ts(&units, s))
}

fn ttid(string: &str, span: Span) -> TokenTree {
    Ident::new(string, span).into()
}

fn ttp(ch: char, spacing: Spacing, span: Span) -> TokenTree {
    let mut o = Punct::new(ch, spacing);
    o.set_span(span);
    o.into()
}

fn ttg(delimiter: Delimiter, span: Span, tts: impl IntoIterator<Item = TokenTree>) -> TokenTree {
    let mut o = Group::new(delimiter, TokenStream::from_iter(tts.into_iter()));
    o.set_span(span);
    o.into()
}

fn tts(str: impl AsRef<str>, span: Span) -> TokenTree {
    let mut o = Literal::string(str.as_ref());
    o.set_span(span);
    o.into()
}

fn compile_error(error: impl AsRef<str>, s: Span) -> TokenTree {
    ttg(Delimiter::None, s, vec![
        ttid("core", s),
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttid("compile_error", s),
        ttp('!', Spacing::Joint, s),
        ttg(Delimiter::Parenthesis, s, vec![
            tts(error.as_ref(), s),
        ]),
    ])
}
