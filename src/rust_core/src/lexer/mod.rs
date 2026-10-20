//! CZ-DSL Lexical Analyzer
//! 
//! This module implements the lexical analysis for the Coastal Zone Domain-Specific Language.
//! It supports:
//! - Keywords for spheres, scales, data, processes, etc.
//! - Chinese and Unicode identifiers
//! - Physical units and dimensions
//! - Scientific literals with units
//! - Multi-language comments

pub mod tokens;
pub mod lexer;
pub mod units;
pub mod error;

pub use tokens::*;
pub use lexer::*;
pub use units::*;
pub use error::*;

#[cfg(test)]
mod tests;
