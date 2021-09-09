use super::token::Token;

/// Converts text into tokens
pub struct Lexer {
    root_token: Box<dyn Token>,
}

pub type ParsedTokenOffset = usize;

#[derive(Debug)]
pub struct ParsedToken<'a> {
    pub token: Box<dyn Token>,
    pub literal: &'a str,
    pub offset: ParsedTokenOffset,
}

pub type ParsedTokens<'a> = Vec<ParsedToken<'a>>;

impl Lexer {
    /// Constructs a new Lexer
    pub fn new(root_token: Box<dyn Token>) -> Lexer {
        Lexer { root_token }
    }

    /// Parses a source string into a series of tokens
    pub fn parse<'a>(&self, source: &'a str) -> Option<ParsedTokens<'a>> {
        self.recursive_parse(0, source, self.root_token.as_ref())
            .map(|mut v| {
                v.reverse();
                v
            })
    }

    fn recursive_parse<'a>(
        &self,
        cur_source_offset: ParsedTokenOffset,
        source: &'a str,
        root_token: &dyn Token,
    ) -> Option<ParsedTokens<'a>> {
        if root_token.is_terminator() {
            let vec: ParsedTokens<'a> = Vec::new();
            return Option::Some(vec);
        }

        let tokens = root_token.next_tokens();
        for token in tokens.into_iter() {
            if let Some(captures) = token.built_regex().captures(source) {
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
                if let Some(mut subpath) = self.recursive_parse(
                    cur_source_offset + capture_offset,
                    &source[capture_offset..],
                    &*token,
                ) {
                    subpath.push(ParsedToken {
                        token: token.clone_box(),
                        literal: real_capture,
                        offset: cur_source_offset,
                    });
                    return Option::Some(subpath);
                }
            }
        }

        Option::None
    }
}
