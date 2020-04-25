#[macro_use]
extern crate lazy_static;

mod html;

// use html::Html;
use std::env;
use std::fs;
use regex::Regex;
use std::fmt;

trait TokenClone {
    fn clone_box(&self) -> Box<dyn Token>;
    fn b(self) -> Box<dyn Token>;
}

impl<T: 'static + Token + Clone> TokenClone for T {
    fn clone_box(&self) -> Box<dyn Token> {
        Box::new(self.clone())
    }

    fn b(self) -> Box<dyn Token> {
        Box::new(self)
    }
}

trait Token: TokenClone + fmt::Debug {
    fn built_regex(&self) -> Regex {
        Regex::new(format!("^{}", self.regex()).as_str()).expect("valid regex")
    }

    fn eq(&self, other: &dyn Token) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }

    fn regex(&self) -> &str;
    fn next_tokens(&self) -> Vec<Box<dyn Token>>;
    fn is_terminator(&self) -> bool;
}

trait RuleClone {
    fn clone_box(&self) -> Box<dyn Rule>;

    fn b(self) -> Box<dyn Rule>;
}

impl<T: 'static + Rule + Clone> RuleClone for T {
    fn clone_box(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }

    fn b(self) -> Box<dyn Rule> {
        Box::new(self)
    }
}

enum RuleType {
    /// Single, unrepeatable rule
    Rule(Box<dyn Rule>),
    /// Rule can repeat 0+ times in a row, greedily consuming
    RepeatableRule(Box<dyn Rule>),
    /// A sequence of rules that need to be matched
    Sequence(Vec<Box<dyn Rule>>),
    /// Single Token
    Token(Box<dyn Token>),
}

// (1 + 2 + 3) / 3
// Expression [ Parens [ "(" Number("1") ? ] ]
// Expression [ Parens [ "(" Expression [ BinaryExpression [ Number("1") BinaryOperator("+") Number("2") ] ? ] ] ]
// Expression [ Parens [ "(" Expression [ BinaryExpression [ BinaryExpression [ Number("1") BinaryOperator("+") Number("2") ] BinaryOperator("+") Number("3") ] ")" ] ? ] ]
// Expression [ BinaryExpression [ Parens [ "(" Expression [ BinaryExpression [ BinaryExpression [ Number("1") BinaryOperator("+") Number("2") ] BinaryOperator("+") Number("3") ] ] ")" ] BinaryOperator("/") Number("3") ] ]

trait Rule: RuleClone + fmt::Debug {
    /// One of these children must match for the rule to match
    fn children(&self) -> Vec<RuleType>;
}

#[derive(Clone, Debug)]
enum MathRule {
    Document, // Root isn't referenced
    DocumentBody,
    Statement,
    Expression,
    BinaryExpression,
    // Parens, // Not yet
    BinaryOperator,
    // Add, // Not needed?
    Semicolon,
    Number,
    Terminator,
}

impl Rule for MathRule {
    fn children(&self) -> Vec<RuleType> {
        match self {
            MathRule::Document => vec!(RuleType::Sequence(vec!(MathRule::DocumentBody.b(), MathRule::Terminator.b()))),
            MathRule::DocumentBody => vec!(RuleType::RepeatableRule(MathRule::Statement.b())),
            MathRule::Statement => vec!(RuleType::Sequence(vec!(MathRule::Expression.b(), MathRule::Semicolon.b()))),
            
            MathRule::Expression => vec!(RuleType::Rule(MathRule::BinaryExpression.b()), RuleType::Rule(MathRule::Number.b())),
            MathRule::BinaryExpression => vec!(RuleType::Sequence(vec!(MathRule::Number.b(), MathRule::BinaryOperator.b(), MathRule::Expression.b()))),
            MathRule::BinaryOperator => vec!(RuleType::Token(MathToken::Plus.b())),
            // MathRule::Add => vec!(RuleType::Tokens(vec!(MathToken::Number.b(), MathToken::Plus.b(), MathToken::Number.b()))),
            MathRule::Semicolon => vec!(RuleType::Token(MathToken::Semicolon.b())),
            MathRule::Number => vec!(RuleType::Token(MathToken::Number.b())),
            MathRule::Terminator => vec!(RuleType::Token(MathToken::Terminator.b())),
        }
    }
}

// #[derive(Clone, Debug)]
// enum HtmlToken {
//     TagOpen,
//     TagName,
//     TagClose,
//     Whitespace,
//     TagAttribute,
// }

// impl Token for HtmlToken {
//     fn regex(&self) -> &str {
//         match self {
//             HtmlToken::TagOpen => r"<",
//             HtmlToken::TagName => r"\w[\w\d-]*",
//             HtmlToken::TagClose => r">",
//             HtmlToken::Whitespace => r"\s+",
//             HtmlToken::TagAttribute => r"\w+",
//         }
//     }

//     fn next_tokens(&self) -> Vec<Box<dyn Token>> {
//         match self {
//             HtmlToken::TagOpen => vec!(Box::new(HtmlToken::TagName)),
//             _ => vec!(Box::new(HtmlToken::TagName)),
//         }
//     }

//     fn is_terminator(&self) -> bool {
//         true
//     }
// }

#[derive(Clone, Debug, PartialEq)]
enum MathToken {
    Document,
    Number,
    Plus,
    Whitespace,
    Semicolon,
    Terminator
}

impl Token for MathToken {
    fn regex(&self) -> &str {
        match self {
            MathToken::Document => "",
            MathToken::Number => r"\s*\d+(\.\d+)?",
            MathToken::Plus => r"\s*\+",
            MathToken::Whitespace => r"\s+",
            MathToken::Semicolon => r";",
            MathToken::Terminator => r"^$",
        }
    }

    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            MathToken::Document => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Number), Box::new(MathToken::Terminator)),
            MathToken::Number => vec!(Box::new(MathToken::Plus), Box::new(MathToken::Semicolon), Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)),
            MathToken::Plus => vec!(Box::new(MathToken::Number)),
            MathToken::Whitespace => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)),
            MathToken::Semicolon => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)), // eh
            MathToken::Terminator => vec!(),
        }
    }

    fn is_terminator(&self) -> bool {
        match self {
            MathToken::Terminator => true,
            _ => false
        }
    }
}


struct Lexer {
    // source: Box<str>,
    root_token: Box<dyn Token>,
    // loc: usize,
}

type ParsedToken<'a> = (Box<dyn Token>, &'a str);
type ParsedTokens<'a> = Vec<ParsedToken<'a>>;

impl Lexer {
    fn new(root_token: Box<dyn Token>) -> Lexer {
        // let mut target: String = String::new();
        // target = source.to_string();

        Lexer {
            // source: Box::from(target),
            root_token,
            // loc: 0
        }
    }

    pub fn parse<'a>(&self, source: &'a str) -> Option<ParsedTokens<'a>> {
        self.recursive_parse(source, self.root_token.as_ref()).map(|mut v| { v.reverse(); v })
        
        // while self.loc < self.source.len() {
        //     if (self.source[self.loc] == )
        // }
    }

    fn recursive_parse<'a>(&self, source: &'a str, root_token: &dyn Token) -> Option<ParsedTokens<'a>> { // pair of token + it's value
        if root_token.is_terminator() {
            let vec: ParsedTokens<'a>= Vec::new();
            // vec.push((root_token.clone_box(), "",));
            return Option::Some(vec);
        }

        let tokens = root_token.next_tokens();
        for token in tokens.into_iter() {
            if let Some(captures) = token.built_regex().captures(source) {
                println!("match found!: {:?}", captures);
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

#[derive(Debug)]
enum ASTNode<'a> {
    Node {
        rule: Box<dyn Rule>,
        token: Option<&'a ParsedToken<'a>>,
        children: Vec<ASTNode<'a>>,
    },
    Leaf
}

#[derive(Debug)]
struct ParserResult<'a> {
    node: ASTNode<'a>,
    // child_index: usize,
    remaining_tokens: &'a [ParsedToken<'a>],
}

struct Parser {}

impl Parser {

    fn parse<'a>(&self, tokens: &'a [ParsedToken<'a>], rule: &dyn Rule) -> Result<ParserResult<'a>, &str> {
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

struct Interpreter {}

impl Interpreter {
    fn interpret(&self, ast: ParserResult) -> Option<i32> {
        match ast.node {
            ASTNode::Leaf => None,
            ASTNode::Node { rule, token, children } => {
                match *rule {
                    MathRule::Document { Document } => {
                        self.interpret(children[0])
                    },
                    _ => None
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let html_document_file = args.get(1).expect("HTML document not passed in");
    let html_document = fs::read_to_string(html_document_file)
        .expect("Unable to read file")
        ;//.into_boxed_str();
    // let html_document: Box<str> = Box::from(html_document.to_owned());
    // let foo = build_foo_holder(html_document);
    // let html = Html::load(&html_document);
    // println!("doctype: {:?}", html.get_doctype());
    // Html::load(file);

    let lexer = Lexer::new(Box::new(MathToken::Document));
    let tokens = lexer.parse(html_document.as_str());
    println!("Tokens: {:?}", tokens);
    if let Some(tokens) = tokens {
        let parser = Parser {};
        let ast = parser.parse(&tokens, &MathRule::Document);
        println!("AST: {:?}", ast);
        if let Ok(ast) = ast {
            let interpreter = Interpreter {};
            let result = interpreter.interpret(ast);
        }
    }
}
