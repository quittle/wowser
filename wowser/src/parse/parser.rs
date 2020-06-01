use super::lexer::ParsedToken;
use super::rule::{Rule, RuleType};

/// Represents a node in an AST representation of a language
#[derive(Debug)]
pub struct ASTNode<'a, T: Rule> {
    pub rule: Box<T>,
    pub token: Option<&'a ParsedToken<'a>>,
    pub children: Vec<ASTNode<'a, T>>,
}

/// The results of interpretting a rule over tokens
#[derive(Debug)]
pub struct ParserResult<'a, T: Rule> {
    pub node: ASTNode<'a, T>,
    remaining_tokens: &'a [ParsedToken<'a>],
}

/// Parses tokens into an AST representation
pub struct Parser {}

impl Parser {
    /// Performs the parsing
    pub fn parse<'a, T: Rule>(
        &self,
        tokens: &'a [ParsedToken<'a>],
        rule: &T,
    ) -> Result<ParserResult<'a, T>, &str> {
        let mut child_indices: Vec<usize> = vec![0];
        self._parse(tokens, rule, &mut child_indices, 0)
    }

    fn _parse<'a, T: Rule>(
        &self,
        tokens: &'a [ParsedToken<'a>],
        root_rule: &T,
        child_indices: &mut Vec<usize>,
        depth: usize,
    ) -> Result<ParserResult<'a, T>, &str> {
        if child_indices.len() == depth {
            child_indices.push(0);
        }

        if let Some(first_token) = tokens.get(0) {
            for child_rule_type in root_rule.children()[child_indices[depth]..].iter() {
                let result = match child_rule_type {
                    RuleType::Token(token) => {
                        if token.eq(first_token.0.as_ref()) {
                            Ok(ParserResult {
                                node: ASTNode {
                                    rule: root_rule.clone_box(),
                                    token: Some(first_token),
                                    children: vec![],
                                },
                                remaining_tokens: &tokens[1..],
                            })
                        } else {
                            Err("Not a match")
                        }
                    }
                    RuleType::Rule(rule) => {
                        match self._parse(tokens, &**rule, child_indices, depth + 1) {
                            Ok(result) => Ok(ParserResult {
                                node: ASTNode {
                                    rule: root_rule.clone_box(),
                                    token: None,
                                    children: vec![result.node],
                                },
                                remaining_tokens: result.remaining_tokens,
                            }),
                            err => err,
                        }
                    }
                    RuleType::RepeatableRule(rule) => {
                        let mut children = vec![];
                        let mut cur_tokens = tokens;
                        while let Ok(result) =
                            self._parse(cur_tokens, &**rule, child_indices, depth + 1)
                        {
                            children.push(result.node);
                            cur_tokens = result.remaining_tokens;
                        }

                        Ok(ParserResult {
                            node: ASTNode { rule: root_rule.clone_box(), token: None, children },
                            remaining_tokens: cur_tokens,
                        })
                    }
                    RuleType::Sequence(rules) => {
                        let mut children = vec![];
                        let mut cur_tokens: &[ParsedToken] = tokens;
                        let mut failed = false;
                        for rule in rules {
                            if let Ok(child) =
                                self._parse(cur_tokens, &**rule, child_indices, depth + 1)
                            {
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
                                    rule: root_rule.clone_box(),
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
        } else {
            return Err("No tokens left");
        }

        Err("Unable to match any child rules")
    }
}
