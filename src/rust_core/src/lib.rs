//! CZ-DSL: Coastal Zone Domain-Specific Language
//! 
//! A unified language for coastal zone complex systems modeling that supports:
//! - Multi-sphere system modeling (atmosphere, ocean, lithosphere, biosphere, anthroposphere)
//! - Multi-scale process coupling (molecular to global scales)
//! - Physical units and dimensional analysis
//! - Geometric algebra for spatial computations
//! - Chinese and Unicode identifiers
//! - AI-native constructs for machine learning integration
//! - Digital twin and real-time data assimilation

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod lexer;
pub mod parser;
pub mod cmecs;

// Re-export main types for convenience
pub use lexer::{
    Lexer, Token, TokenKind, Position, Span,
    LexError, LexResult, ErrorReporter,
    Unit, Dimension, UnitRegistry, UnitParser, UnitError
};
pub use parser::{
    Parser, Program, Item, Expression, Statement, TypeExpr,
    ParseError, ParseResult, parse_string, parse_string_with_source
};

pub use cmecs::*;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Language name
pub const LANGUAGE_NAME: &str = "CZ-DSL";

/// Language description
pub const LANGUAGE_DESCRIPTION: &str = "Coastal Zone Domain-Specific Language for complex systems modeling";

/// Tokenize a string and return all tokens
/// 
/// This is a convenience function that creates a lexer and tokenizes the input.
/// 
/// # Arguments
/// 
/// * `input` - The source code to tokenize
/// 
/// # Returns
/// 
/// A vector of tokens or a lexical error
/// 
/// # Examples
/// 
/// ```
/// use cz_dsl::tokenize;
/// 
/// let tokens = tokenize("sphere Atmosphere { temperature: Real }").unwrap();
/// assert!(tokens.len() > 0);
/// ```
pub fn tokenize(input: &str) -> LexResult<Vec<Token>> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

/// Tokenize a string with source file information
/// 
/// # Arguments
/// 
/// * `input` - The source code to tokenize
/// * `source_file` - The name of the source file (for error reporting)
/// 
/// # Returns
/// 
/// A vector of tokens or a lexical error
pub fn tokenize_with_source(input: &str, source_file: String) -> LexResult<Vec<Token>> {
    let mut lexer = Lexer::with_source_file(input, source_file);
    lexer.tokenize()
}

/// Check if a string is a valid CZ-DSL identifier
/// 
/// # Arguments
/// 
/// * `s` - The string to check
/// 
/// # Returns
/// 
/// True if the string is a valid identifier
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    
    // First character must be XID_Start or ASCII letter/underscore
    if !unicode_ident::is_xid_start(first) && !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    
    // Remaining characters must be XID_Continue or ASCII alphanumeric/underscore
    for ch in chars {
        if !unicode_ident::is_xid_continue(ch) && !ch.is_ascii_alphanumeric() && ch != '_' {
            return false;
        }
    }
    
    true
}

/// Check if a string is a CZ-DSL keyword
/// 
/// # Arguments
/// 
/// * `s` - The string to check
/// 
/// # Returns
/// 
/// True if the string is a keyword
pub fn is_keyword(s: &str) -> bool {
    TokenKind::keywords().contains(&s)
}

/// Check if a string is a physical unit
/// 
/// # Arguments
/// 
/// * `s` - The string to check
/// 
/// # Returns
/// 
/// True if the string is a recognized unit
pub fn is_unit(s: &str) -> bool {
    let registry = UnitRegistry::new();
    registry.contains(s)
}

/// Get language statistics
#[derive(Debug, Clone)]
pub struct LanguageStats {
    /// Number of keywords
    pub keyword_count: usize,
    /// Number of operators
    pub operator_count: usize,
    /// Number of physical units
    pub unit_count: usize,
    /// Supports Unicode identifiers
    pub unicode_support: bool,
    /// Supports Chinese keywords
    pub chinese_support: bool,
}

/// Get statistics about the CZ-DSL language
pub fn language_stats() -> LanguageStats {
    let registry = UnitRegistry::new();
    
    LanguageStats {
        keyword_count: TokenKind::keywords().len(),
        operator_count: 25, // Approximate count of operators
        unit_count: registry.all_units().len(),
        unicode_support: true,
        chinese_support: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_convenience_function() {
        let tokens = tokenize("sphere data").unwrap();
        assert_eq!(tokens.len(), 3); // sphere, data, EOF
        assert_eq!(tokens[0].kind, TokenKind::Sphere);
        assert_eq!(tokens[1].kind, TokenKind::Data);
        assert_eq!(tokens[2].kind, TokenKind::EOF);
    }

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("valid_name"));
        assert!(is_valid_identifier("CamelCase"));
        assert!(is_valid_identifier("_underscore"));
        assert!(is_valid_identifier("var123"));
        assert!(is_valid_identifier("变量名")); // Chinese
        assert!(is_valid_identifier("αβγ")); // Greek
        
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier("invalid-name"));
        assert!(!is_valid_identifier("invalid.name"));
    }

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("sphere"));
        assert!(is_keyword("data"));
        assert!(is_keyword("model"));
        assert!(is_keyword("process"));
        
        assert!(!is_keyword("not_a_keyword"));
        assert!(!is_keyword("variable_name"));
    }

    #[test]
    fn test_is_unit() {
        assert!(is_unit("meter"));
        assert!(is_unit("m"));
        assert!(is_unit("kelvin"));
        assert!(is_unit("K"));
        assert!(is_unit("pascal"));
        
        assert!(!is_unit("not_a_unit"));
        assert!(!is_unit("invalid_unit"));
    }

    #[test]
    fn test_language_stats() {
        let stats = language_stats();
        
        assert!(stats.keyword_count > 0);
        assert!(stats.operator_count > 0);
        assert!(stats.unit_count > 0);
        assert!(stats.unicode_support);
        assert!(stats.chinese_support);
    }

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert_eq!(LANGUAGE_NAME, "CZ-DSL");
        assert!(!LANGUAGE_DESCRIPTION.is_empty());
    }
}
