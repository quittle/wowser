mod css_colors;
mod css_interpreter;
mod css_rule;
mod css_token;
mod nodes;
mod properties;

pub use css_colors::*;
pub use css_interpreter::*;
pub use css_rule::*;
pub use css_token::*;
pub use nodes::*;
pub use properties::*;

use crate::parse::*;

pub fn parse_css(document: &str) -> Result<CssDocument, String> {
    let lexer = Lexer::new(CssToken::Document);
    let tokens = lexer.parse(document).ok_or("Failed to lex CSS")?;
    let ast = Parser {}.parse(&tokens, &CssRule::Document)?;
    let document = CssInterpreter {}
        .interpret(&ast)
        .ok_or("Failed to interpret Css")?;
    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(document: &str) -> CssDocument {
        parse_css(document).expect("Failed to parse CSS")
    }

    #[test]
    fn empty_config() {
        assert_eq!(
            CssDocument { blocks: vec![] },
            parse(""),
            "Empty document without spaces"
        );
        assert_eq!(
            CssDocument { blocks: vec![] },
            parse("  "),
            "Empty document with spaces"
        );
    }

    #[test]
    fn simple_config() {
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelectorChain {
                        item: CssSelectorChainItem::Tag("foo".into()),
                        next: None
                    }],
                    properties: vec![]
                }]
            },
            parse("foo { }"),
            "Minimal block with a tag selector"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelectorChain {
                        item: CssSelectorChainItem::Tag("foo".into()),
                        next: None
                    }],
                    properties: vec![CssProperty::new_rc("key", "value")]
                }]
            },
            parse("foo { key: value; }"),
            "Minimal block with a tag selector and single key-value"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelectorChain {
                        item: CssSelectorChainItem::Tag("foo".into()),
                        next: Some(Box::new(CssSelectorChain {
                            item: CssSelectorChainItem::Tag("bar".into()),
                            next: None
                        }))
                    },],
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
                        CssSelectorChain {
                            item: CssSelectorChainItem::Tag("foo".into()),
                            next: None
                        },
                        CssSelectorChain {
                            item: CssSelectorChainItem::Tag("bar".into()),
                            next: None
                        },
                    ],
                    properties: vec![]
                }]
            },
            parse("foo, bar { }"),
            "Multiple selectors with comma"
        );
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![CssSelectorChain {
                        item: CssSelectorChainItem::Tag("foo".into()),
                        next: None
                    }],
                    properties: vec![
                        CssProperty::new_rc("key", "value"),
                        CssProperty::new_rc("key2", "value2"),
                    ]
                }]
            },
            parse("foo { key: value; key2: value2 }"),
            "Trailing CSS Property without semicolon"
        );
    }

    #[test]
    fn complex_config() {
        assert_eq!(
            CssDocument {
                blocks: vec![CssBlock {
                    selectors: vec![
                        CssSelectorChain {
                            item: CssSelectorChainItem::Tag("foo".into()),
                            next: Some(Box::new(CssSelectorChain {
                                item: CssSelectorChainItem::Id("bar".into()),
                                next: None
                            }))
                        },
                        CssSelectorChain {
                            item: CssSelectorChainItem::Class("class".into()),
                            next: None
                        }
                    ],
                    properties: vec![
                        CssProperty::new_rc("hi", "'there'"),
                        CssProperty::new_rc("display", "none")
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
                        selectors: vec![CssSelectorChain {
                            item: CssSelectorChainItem::Tag("foo".into()),
                            next: None
                        }],
                        properties: vec![CssProperty::new_rc("key", "'value-with_symbols'"),]
                    },
                    CssBlock {
                        selectors: vec![
                            CssSelectorChain {
                                item: CssSelectorChainItem::Tag("bar".into()),
                                next: None
                            },
                            CssSelectorChain {
                                item: CssSelectorChainItem::Tag("baz".into()),
                                next: None
                            },
                        ],
                        properties: vec![
                            CssProperty::new_rc("k", "v"),
                            CssProperty::new_rc("v", "k"),
                        ]
                    }
                ]
            },
            parse("foo{key:'value-with_symbols';}bar,baz{k:v;v:k;}"),
            "Minimal, complex format"
        );
    }
}
