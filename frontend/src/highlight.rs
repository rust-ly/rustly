//! A small Rust syntax highlighter for snippets. It only needs to colour
//! keywords, strings, comments, numbers, macros and type names; the editor
//! (Monaco, in M5) does the real thing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Plain,
    Keyword,
    Str,
    Comment,
    Number,
    Macro,
    Type,
}

impl Token {
    pub fn class(self) -> &'static str {
        match self {
            Token::Plain => "",
            Token::Keyword => "tok-kw",
            Token::Str => "tok-str",
            Token::Comment => "tok-com",
            Token::Number => "tok-num",
            Token::Macro => "tok-mac",
            Token::Type => "tok-ty",
        }
    }
}

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Splits one line into coloured pieces. Block comments and strings that span
/// lines are rare in snippets and are coloured as plain text.
pub fn line(text: &str) -> Vec<(Token, &str)> {
    let mut out: Vec<(Token, &str)> = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        let c = bytes[i];
        let token = if text[i..].starts_with("//") {
            i = bytes.len();
            Token::Comment
        } else if c == b'"' {
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                i += if bytes[i] == b'\\' { 2 } else { 1 };
            }
            i = (i + 1).min(bytes.len());
            Token::Str
        } else if c == b'\'' && is_char_literal(&text[i..]) {
            i += text[i + 1..].find('\'').map_or(1, |n| n + 2);
            Token::Str
        } else if c.is_ascii_digit() {
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                // Stop before `..` in ranges like `1..=n`.
                if bytes[i] == b'.' && bytes.get(i + 1) == Some(&b'.') {
                    break;
                }
                i += 1;
            }
            Token::Number
        } else if c.is_ascii_alphabetic() || c == b'_' {
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &text[start..i];
            if bytes.get(i) == Some(&b'!') && bytes.get(i + 1) != Some(&b'=') {
                i += 1;
                Token::Macro
            } else if KEYWORDS.contains(&word) {
                Token::Keyword
            } else if c.is_ascii_uppercase() {
                Token::Type
            } else {
                Token::Plain
            }
        } else {
            i += text[i..].chars().next().map_or(1, char::len_utf8);
            Token::Plain
        };
        match out.last_mut() {
            Some((Token::Plain, prev)) if token == Token::Plain => {
                *prev = &text[start - prev.len()..i];
            }
            _ => out.push((token, &text[start..i])),
        }
    }
    out
}

/// `'a'` or `'\n'`, as opposed to a lifetime like `'a`.
fn is_char_literal(rest: &str) -> bool {
    let mut chars = rest.chars().skip(1);
    match chars.next() {
        Some('\\') => true,
        Some(_) => chars.next() == Some('\''),
        None => false,
    }
}
