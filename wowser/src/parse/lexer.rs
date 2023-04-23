use std::fmt;

use super::token::Token;

/// Converts text into tokens
pub struct Lexer<T: Token> {
    root_token: T,
}

pub type ParsedTokenOffset = usize;

pub struct ParsedToken<'a, T: Token> {
    pub token: T,
    pub literal: &'a str,
    pub offset: ParsedTokenOffset,
    pub full_match: &'a str,
}

impl<T: Token> fmt::Debug for ParsedToken<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({}):{}", self.token, self.literal, self.offset)
    }
}

pub type ParsedTokens<'a, T> = Vec<ParsedToken<'a, T>>;

impl<T: Token> Lexer<T> {
    /// Constructs a new Lexer
    pub fn new(root_token: T) -> Lexer<T> {
        Lexer { root_token }
    }

    /// Parses a source string into a series of tokens
    pub fn parse<'a>(&self, source: &'a str) -> Option<ParsedTokens<'a, T>> {
        Self::recursive_parse(0, source, &self.root_token).map(|v| {
            v.into_iter()
                .rev()
                .filter(|token| !token.token.is_comment())
                .collect()
        })
    }

    fn recursive_parse<'a>(
        cur_source_offset: ParsedTokenOffset,
        source: &'a str,
        root_token: &T,
    ) -> Option<ParsedTokens<'a, T>> {
        if root_token.is_terminator() {
            return Some(vec![]);
        }

        let tokens = root_token.next_tokens();

        let all_tokens = tokens.iter().chain(T::get_comment_tokens());

        for token in all_tokens {
            if let Some(captures) = token.built_regex().captures(source).ok()? {
                let real_capture;
                if let Some(capture) = captures.name("token") {
                    real_capture = capture;
                } else if let Some(capture) = captures.get(1) {
                    real_capture = capture;
                } else if let Some(capture) = captures.get(0) {
                    real_capture = capture;
                } else {
                    panic!("Unable to capture token");
                }
                let real_capture = real_capture.as_str();
                let capture = captures.get(0).expect("Match must exist").as_str();
                let capture_offset = capture.len();

                let new_root_token = if token.is_comment() {
                    root_token
                } else {
                    token
                };
                if let Some(mut subpath) = Self::recursive_parse(
                    cur_source_offset + capture_offset,
                    &source[capture_offset..],
                    new_root_token,
                ) {
                    subpath.push(ParsedToken {
                        token: *token,
                        literal: real_capture,
                        offset: cur_source_offset,
                        full_match: capture,
                    });
                    return Option::Some(subpath);
                }
            }
        }

        Option::None
    }
}
