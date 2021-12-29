use super::lexer::ParsedToken;
use super::rule::{Rule, RuleType};
use super::Token;

/// Represents a node in an AST representation of a language
#[derive(Debug)]
pub struct ASTNode<'a, R: Rule<T>, T: Token> {
    pub rule: R,
    pub token: Option<&'a ParsedToken<'a, T>>,
    pub children: Vec<ASTNode<'a, R, T>>,
}

impl<'a, R: Rule<T>, T: Token> ASTNode<'a, R, T> {
    /// A very rough approximation of where the current node is located. This may beuseful for
    /// relative comparisons but may be v6ery far from the actual byte offset
    pub fn get_first_token(&self) -> Option<&'a ParsedToken<'a, T>> {
        if let Some(token) = self.token {
            Some(token)
        } else {
            self.children
                .iter()
                .find_map(|child| child.get_first_token())
        }
    }
}

/// The results of interpretting a rule over tokens
#[derive(Debug)]
pub struct ParserResult<'a, R: Rule<T>, T: Token> {
    pub node: ASTNode<'a, R, T>,
    remaining_tokens: &'a [ParsedToken<'a, T>],
}

/// Parses tokens into an AST representation
pub struct Parser {}

impl Parser {
    /// Performs the parsing
    pub fn parse<'a, R: Rule<T>, T: Token>(
        &self,
        tokens: &'a [ParsedToken<'a, T>],
        rule: &R,
    ) -> Result<ParserResult<'a, R, T>, &str> {
        let mut child_indices: Vec<usize> = vec![0];
        self._parse(tokens, rule, &mut child_indices, 0)
    }

    fn _parse<'a, 'b, R: Rule<T>, T: Token>(
        &self,
        tokens: &'a [ParsedToken<'a, T>],
        root_rule: &'b R,
        child_indices: &mut Vec<usize>,
        depth: usize,
    ) -> Result<ParserResult<'a, R, T>, &str> {
        if depth >= 100 {
            return Err("Code too complex");
        }

        if child_indices.len() == depth {
            child_indices.push(0);
        }

        if let Some(first_token) = tokens.get(0) {
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
                    RuleType::Rule(rule) => {
                        match self._parse(tokens, rule, child_indices, depth + 1) {
                            Ok(result) => Ok(ParserResult {
                                node: ASTNode {
                                    rule: *root_rule,
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
                            self._parse(cur_tokens, rule, child_indices, depth + 1)
                        {
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
                        let mut cur_tokens: &[ParsedToken<'a, T>] = tokens;
                        let mut failed = false;
                        for rule in rules {
                            if let Ok(child) =
                                self._parse(cur_tokens, rule, child_indices, depth + 1)
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
        } else {
            return Err("No tokens left");
        }

        Err("Unable to match any child rules")
    }
}
