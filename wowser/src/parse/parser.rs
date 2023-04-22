use super::lexer::ParsedToken;
use super::rule::{Rule, RuleType};

/// Represents a node in an AST representation of a language
#[derive(Debug)]
pub struct ASTNode<'a, R: Rule> {
    pub rule: R,
    pub token: Option<&'a ParsedToken<'a, R::Token>>,
    pub children: Vec<ASTNode<'a, R>>,
}

impl<'a, R: Rule> ASTNode<'a, R> {
    /// A very rough approximation of where the current node is located. This may beuseful for
    /// relative comparisons but may be v6ery far from the actual byte offset
    pub fn get_first_token(&self) -> Option<&'a ParsedToken<'a, R::Token>> {
        if let Some(token) = self.token {
            Some(token)
        } else {
            self.children
                .iter()
                .find_map(|child| child.get_first_token())
        }
    }

    pub fn rebuild_full_text(&self) -> String {
        let mut ret = String::new();

        if let Some(token) = self.token {
            ret += token.full_match;
        }

        for node in &self.children {
            ret += &node.rebuild_full_text();
        }

        ret
    }
}

/// The results of interpretting a rule over tokens
#[derive(Debug)]
pub struct ParserResult<'a, R: Rule> {
    pub node: ASTNode<'a, R>,
    remaining_tokens: &'a [ParsedToken<'a, R::Token>],
}

/// Parses tokens into an AST representation
pub struct Parser {}

impl Parser {
    /// Performs the parsing
    ///
    /// Tips for debugging "Unable to match any child rules"
    /// 1. When rules have multiple children, sort the longest, most complex ones first. A rule will only use the first matched child.
    /// 2. If a rule is invalid due to parent rule's presence, split it so there are two: one for the default state, and one for child rules.
    /// 3. Stack overflows occur when a rule is defined recursively and the recursive match is not the last value.
    pub fn parse<'a, R: Rule>(
        &self,
        tokens: &'a [ParsedToken<'a, R::Token>],
        rule: &R,
    ) -> Result<ParserResult<'a, R>, &str> {
        Self::_parse(tokens, rule)
    }

    fn _parse<'a, 'b, R: Rule>(
        tokens: &'a [ParsedToken<'a, R::Token>],
        root_rule: &'b R,
    ) -> Result<ParserResult<'a, R>, &'static str> {
        let first_token = tokens.get(0).ok_or("No tokens left")?;

        for child_rule_type in root_rule.children().iter() {
            let result = match child_rule_type {
                RuleType::Token(token) => {
                    if token.eq(&first_token.token) {
                        Ok(ParserResult {
                            node: ASTNode {
                                rule: *root_rule,
                                token: Some(first_token),
                                children: vec![],
                            },
                            remaining_tokens: &tokens[1..],
                        })
                    } else {
                        Err("Not a match")
                    }
                }
                RuleType::Rule(rule) => match Self::_parse(tokens, rule) {
                    Ok(result) => Ok(ParserResult {
                        node: ASTNode {
                            rule: *root_rule,
                            token: None,
                            children: vec![result.node],
                        },
                        remaining_tokens: result.remaining_tokens,
                    }),
                    err => err,
                },
                RuleType::RepeatableRule(rule) => {
                    let mut children = vec![];
                    let mut cur_tokens = tokens;
                    while let Ok(result) = Self::_parse(cur_tokens, rule) {
                        children.push(result.node);
                        cur_tokens = result.remaining_tokens;
                    }

                    Ok(ParserResult {
                        node: ASTNode {
                            rule: *root_rule,
                            token: None,
                            children,
                        },
                        remaining_tokens: cur_tokens,
                    })
                }
                RuleType::Sequence(rules) => {
                    let mut children = vec![];
                    let mut cur_tokens: &[ParsedToken<'a, R::Token>] = tokens;
                    let mut failed = false;

                    for rule in rules {
                        if let Ok(child) = Self::_parse(cur_tokens, rule) {
                            children.push(child.node);
                            cur_tokens = child.remaining_tokens;
                        } else {
                            failed = true;
                            break;
                        }
                    }

                    if failed {
                        Err("Failed to parse sequence")
                    } else {
                        Ok(ParserResult {
                            node: ASTNode {
                                rule: *root_rule,
                                token: None,
                                children,
                            },
                            remaining_tokens: cur_tokens,
                        })
                    }
                }
            };

            if result.is_ok() {
                return result;
            }
        }

        Err("Unable to match any child rules")
    }
}
