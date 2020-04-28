use super::token::Token;

/// Converts text into tokens
pub struct Lexer {
    root_token: Box<dyn Token>,
}

pub type ParsedToken<'a> = (Box<dyn Token>, &'a str);
pub type ParsedTokens<'a> = Vec<ParsedToken<'a>>;

impl Lexer {
    /// Constructs a new Lexer
    pub fn new(root_token: Box<dyn Token>) -> Lexer {
        Lexer { root_token }
    }

    /// Parses a source string into a series of tokens
    pub fn parse<'a>(&self, source: &'a str) -> Option<ParsedTokens<'a>> {
        self.recursive_parse(source, self.root_token.as_ref())
            .map(|mut v| {
                v.reverse();
                v
            })
    }

    fn recursive_parse<'a>(
        &self,
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
                let capture = captures.get(0).expect("must be present").as_str();
                if let Some(mut subpath) = self.recursive_parse(&source[capture.len()..], &*token) {
                    subpath.push((token.clone_box(), capture));
                    return Option::Some(subpath);
                }
            }
        }

        Option::None
    }
}
