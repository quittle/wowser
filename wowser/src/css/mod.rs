mod css_interpreter;
mod css_rule;
mod css_token;

pub use css_interpreter::*;
pub use css_rule::*;
pub use css_token::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::*;

    fn parse(document: &str) -> CssDocument {
        let lexer = Lexer::new(Box::new(CssToken::Document));
        let tokens = Box::new(lexer.parse(document).expect("Failed to lex"));
        let ast = Parser {}.parse(&tokens, &CssRule::Document).expect("Failed to parse");
        let css_document = CssInterpreter {}.interpret(&ast).expect("Failed to interpret");
        css_document
    }

    #[test]
    fn empty_config() {
        assert_eq!(CssDocument { blocks: vec![] }, parse(""), "Empty document without spaces");
        assert_eq!(CssDocument { blocks: vec![] }, parse("  "), "Empty document with spaces");
    }

    #[test]
    fn simple_config() {
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelector { selectors: vec!["foo".to_string()] }],
                    properties: vec![]
                }]
            },
            parse("foo { }"),
            "Minimal block with a tag selector"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelector { selectors: vec!["foo".to_string()] }],
                    properties: vec![CssProperty {
                        key: "key".to_string(),
                        value: "value".to_string(),
                    }]
                }]
            },
            parse("foo { key: value; }"),
            "Minimal block with a tag selector and single key-value"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelector {
                        selectors: vec!["foo".to_string(), "bar".to_string()]
                    }],
                    properties: vec![]
                }]
            },
            parse("foo bar{}"),
            "Multiple tag selectors with minimal spacing"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![
                        CssSelector { selectors: vec!["foo".to_string()] },
                        CssSelector { selectors: vec!["bar".to_string()] }
                    ],
                    properties: vec![]
                }]
            },
            parse("foo, bar { }"),
            "Multiple selectors with comma"
        );
    }

    #[test]
    fn complex_config() {
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![
                        CssSelector { selectors: vec!["foo".to_string(), "#bar".to_string()] },
                        CssSelector { selectors: vec![".class".to_string()] }
                    ],
                    properties: vec![
                        CssProperty { key: "hi".to_string(), value: "'there'".to_string() },
                        CssProperty { key: "display".to_string(), value: "none".to_string() }
                    ]
                }]
            },
            parse("foo #bar, .class { hi: 'there'; display: none; }"),
            "More complex format"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![
                    CssBlock {
                        selectors: vec![CssSelector { selectors: vec!["foo".to_string()] }],
                        properties: vec![CssProperty {
                            key: "key".to_string(),
                            value: "'value-with_symbols'".to_string()
                        },]
                    },
                    CssBlock {
                        selectors: vec![
                            CssSelector { selectors: vec!["bar".to_string()] },
                            CssSelector { selectors: vec!["baz".to_string()] }
                        ],
                        properties: vec![
                            CssProperty { key: "k".to_string(), value: "v".to_string() },
                            CssProperty { key: "v".to_string(), value: "k".to_string() }
                        ]
                    }
                ]
            },
            parse("foo{key:'value-with_symbols';}bar,baz{k:v;v:k;}"),
            "Minimal, complex format"
        );
    }
}
