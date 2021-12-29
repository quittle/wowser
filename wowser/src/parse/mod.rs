//! Contains the basic logic for "compiling" a language.
//! You will need to create your own Token, Rule, and Interpeter subclasses
//! to work with this module. See `super::math_parse` for a demo implementation.
//!
//! 1. Use the Lexer to first generate tokens from a string
//! 2. Then use the Parser to convert the string of tokens into an AST
//! 3. Finally use Interpreter to run through your AST, generating output from it

mod interpreter;
mod lexer;
mod parser;
mod rule;
mod token;

pub use interpreter::Interpreter;
pub use lexer::{Lexer, ParsedToken, ParsedTokenOffset, ParsedTokens};
pub use parser::{ASTNode, Parser, ParserResult};
pub use rule::{Rule, RuleType};
pub use token::Token;
