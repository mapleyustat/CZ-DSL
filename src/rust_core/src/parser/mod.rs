//! CZ-DSL Parser Module
//! 
//! This module implements the syntax analyzer for the Coastal Zone Domain-Specific Language.
//! It builds upon the lexer to construct Abstract Syntax Trees (AST) from token streams.

pub mod grammar;
pub mod ast;
pub mod recursive_parser;
pub mod error;

pub use grammar::*;
pub use ast::*;
pub use error::*;
pub use recursive_parser::{Parser, parse_string, parse_string_with_source};

#[cfg(test)]
mod tests;