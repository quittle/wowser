use super::rule::{RuleType, Rule};
use super::lexer::{ParsedToken};

#[derive(Debug)]
pub enum ASTNode<'a> {
    Node {
        rule: Box<dyn Rule>,
        token: Option<&'a ParsedToken<'a>>,
        children: Vec<ASTNode<'a>>,
    },
    Leaf
}

#[derive(Debug)]
pub struct ParserResult<'a> {
    pub node: ASTNode<'a>,
    // child_index: usize,
    remaining_tokens: &'a [ParsedToken<'a>],
}

pub struct Parser {}

impl Parser {
    pub fn parse<'a>(&self, tokens: &'a [ParsedToken<'a>], rule: &dyn Rule) -> Result<ParserResult<'a>, &str> {
        let mut child_indices: Vec<usize> = vec!(0);
        self._parse(tokens, rule, &mut child_indices, 0)
    }

    fn _parse<'a>(&self, tokens: &'a [ParsedToken<'a>], rule: &dyn Rule, child_indices: &mut Vec<usize>, depth: usize) -> Result<ParserResult<'a>, &str> {
        println!("Parsing rule: {:?}", rule);
        if child_indices.len() == depth {
            child_indices.push(0);
        }

        if let Some(first_token) = tokens.get(0) {
            for (i, child_rule_type) in rule.children()[child_indices[depth]..].iter().enumerate() {
                let result = match child_rule_type {
                    RuleType::Token(token) => {
                        println!("token rule parse. Comparing {:?} to {:?}", token, first_token.0);
                        if token.eq(first_token.0.as_ref()) {
                            Ok(ParserResult {
                                node: ASTNode::Node {
                                    rule: rule.clone_box(),
                                    token: Some(first_token),
                                    children: vec!(),
                                },
                                // child_index: i,
                                remaining_tokens: &tokens[1..]
                            })
                        } else {
                            Err("Not a match")
                        }
                    },
                    RuleType::Rule(rule) => {
                        println!("Rule rule parse");
                        match self._parse(tokens, &**rule, child_indices, depth + 1) {
                            Ok(result) => Ok(ParserResult {
                                node: ASTNode::Node {
                                    rule: rule.clone_box(),
                                    token: None,
                                    children: vec!(result.node),
                                },
                                // child_index: i,
                                remaining_tokens: result.remaining_tokens
                            }),
                            err => err,
                        }
                    },
                    RuleType::RepeatableRule(rule) => {
                        println!("RepeatableRule rule parse");
                        let mut children = vec!();
                        let mut cur_tokens = tokens;
                        while let Ok(result) = self._parse(cur_tokens, &**rule, child_indices, depth + 1) {
                            children.push(result.node);
                            cur_tokens = result.remaining_tokens;
                        }

                        Ok(ParserResult {
                            node: ASTNode::Node {
                                rule: rule.clone_box(),
                                token: None,
                                children,
                            },
                            remaining_tokens: cur_tokens
                        })
                    },
                    RuleType::Sequence(rules) => {
                        println!("Sequence rule parse");
                        let mut children = vec!();
                        let mut cur_tokens = tokens;
                        let mut failed = false;
                        for rule in rules {
                            if let Ok(child) = self._parse(cur_tokens, &**rule, child_indices, depth + 1) {
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
                                node: ASTNode::Node {
                                    rule: rule.clone_box(),
                                    token: None,
                                    children,
                                },
                                remaining_tokens: cur_tokens,
                            })
                        }
                    }
                };

                if result.is_ok() {
                    child_indices[depth] = i;
                    return result;
                }
            }
        } else {
            return Err("No tokens left");
        }

        Err("Unable to match any child rules")
    }
}
