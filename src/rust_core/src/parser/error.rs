//! Parser Error Types for CZ-DSL
//! 
//! This module defines error types and recovery strategies for the CZ-DSL parser.

use crate::lexer::{Position, Span, TokenKind};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Parser errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found} at {position}")]
    UnexpectedToken {
        expected: String,
        found: TokenKind,
        position: Position,
    },
    
    #[error("Unexpected end of file: expected {expected}")]
    UnexpectedEOF {
        expected: String,
    },
    
    #[error("Invalid syntax: {message} at {position}")]
    InvalidSyntax {
        message: String,
        position: Position,
    },
    
    #[error("Missing semicolon at {position}")]
    MissingSemicolon {
        position: Position,
    },
    
    #[error("Mismatched parentheses at {position}")]
    MismatchedParentheses {
        position: Position,
    },
    
    #[error("Mismatched braces at {position}")]
    MismatchedBraces {
        position: Position,
    },
    
    #[error("Mismatched brackets at {position}")]
    MismatchedBrackets {
        position: Position,
    },
    
    #[error("Invalid type expression at {position}")]
    InvalidTypeExpression {
        position: Position,
    },
    
    #[error("Invalid unit expression: {message} at {position}")]
    InvalidUnitExpression {
        message: String,
        position: Position,
    },
    
    #[error("Invalid constraint expression at {position}")]
    InvalidConstraintExpression {
        position: Position,
    },
    
    #[error("Invalid generic parameter at {position}")]
    InvalidGenericParameter {
        position: Position,
    },
    
    #[error("Duplicate identifier: {name} at {position}")]
    DuplicateIdentifier {
        name: String,
        position: Position,
    },
    
    #[error("Invalid Chinese identifier: {name} at {position}")]
    InvalidChineseIdentifier {
        name: String,
        position: Position,
    },
    
    #[error("Lexer error: {error}")]
    LexerError {
        error: crate::lexer::LexError,
    },
}

impl ParseError {
    /// Get the position where the error occurred, if available
    pub fn position(&self) -> Option<&Position> {
        match self {
            ParseError::UnexpectedToken { position, .. } => Some(position),
            ParseError::InvalidSyntax { position, .. } => Some(position),
            ParseError::MissingSemicolon { position } => Some(position),
            ParseError::MismatchedParentheses { position } => Some(position),
            ParseError::MismatchedBraces { position } => Some(position),
            ParseError::MismatchedBrackets { position } => Some(position),
            ParseError::InvalidTypeExpression { position } => Some(position),
            ParseError::InvalidUnitExpression { position, .. } => Some(position),
            ParseError::InvalidConstraintExpression { position } => Some(position),
            ParseError::InvalidGenericParameter { position } => Some(position),
            ParseError::DuplicateIdentifier { position, .. } => Some(position),
            ParseError::InvalidChineseIdentifier { position, .. } => Some(position),
            ParseError::LexerError { error } => error.position(),
            ParseError::UnexpectedEOF { .. } => None,
        }
    }
    
    /// Create a span for this error (single position)
    pub fn span(&self) -> Option<Span> {
        self.position().map(|pos| Span {
            start: pos.clone(),
            end: pos.clone(),
        })
    }
    
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ParseError::UnexpectedToken { .. } => true,
            ParseError::InvalidSyntax { .. } => true,
            ParseError::MissingSemicolon { .. } => true,
            ParseError::InvalidTypeExpression { .. } => true,
            ParseError::InvalidUnitExpression { .. } => true,
            ParseError::InvalidConstraintExpression { .. } => true,
            ParseError::InvalidGenericParameter { .. } => true,
            ParseError::InvalidChineseIdentifier { .. } => true,
            ParseError::UnexpectedEOF { .. } => false,
            ParseError::MismatchedParentheses { .. } => false,
            ParseError::MismatchedBraces { .. } => false,
            ParseError::MismatchedBrackets { .. } => false,
            ParseError::DuplicateIdentifier { .. } => true,
            ParseError::LexerError { error } => error.is_recoverable(),
        }
    }
    
    /// Get a suggested fix for the error, if available
    pub fn suggestion(&self) -> Option<String> {
        match self {
            ParseError::UnexpectedToken { expected, .. } => {
                Some(format!("Try using {}", expected))
            },
            ParseError::MissingSemicolon { .. } => {
                Some("Add a semicolon (;) at the end of the statement".to_string())
            },
            ParseError::MismatchedParentheses { .. } => {
                Some("Check that all parentheses are properly matched".to_string())
            },
            ParseError::MismatchedBraces { .. } => {
                Some("Check that all braces are properly matched".to_string())
            },
            ParseError::MismatchedBrackets { .. } => {
                Some("Check that all brackets are properly matched".to_string())
            },
            ParseError::InvalidTypeExpression { .. } => {
                Some("Check the type expression syntax".to_string())
            },
            ParseError::InvalidUnitExpression { message, .. } => {
                Some(format!("Fix unit expression: {}", message))
            },
            ParseError::DuplicateIdentifier { name, .. } => {
                Some(format!("Use a different name instead of '{}'", name))
            },
            _ => None,
        }
    }
}

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Error recovery strategy
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Skip tokens until a synchronization point
    SkipToSync,
    /// Insert a missing token
    InsertToken(TokenKind),
    /// Replace with a default value
    UseDefault,
    /// Backtrack and try alternative parsing
    Backtrack,
}

/// Synchronization points for error recovery
#[derive(Debug, Clone, PartialEq)]
pub enum SyncPoint {
    /// Semicolon (end of statement)
    Semicolon,
    /// Right brace (end of block)
    RightBrace,
    /// Keywords that start new declarations
    Declaration,
    /// End of file
    EOF,
}

/// Error recovery context
#[derive(Debug, Clone)]
pub struct RecoveryContext {
    pub strategy: RecoveryStrategy,
    pub sync_points: Vec<SyncPoint>,
    pub position: Position,
    pub message: String,
}

impl RecoveryContext {
    pub fn new(
        strategy: RecoveryStrategy,
        sync_points: Vec<SyncPoint>,
        position: Position,
        message: String,
    ) -> Self {
        Self {
            strategy,
            sync_points,
            position,
            message,
        }
    }
    
    /// Create a recovery context for skipping to synchronization point
    pub fn skip_to_sync(position: Position, message: String) -> Self {
        Self::new(
            RecoveryStrategy::SkipToSync,
            vec![SyncPoint::Semicolon, SyncPoint::RightBrace, SyncPoint::Declaration],
            position,
            message,
        )
    }
    
    /// Create a recovery context for inserting a missing token
    pub fn insert_token(token: TokenKind, position: Position, message: String) -> Self {
        Self::new(
            RecoveryStrategy::InsertToken(token),
            vec![],
            position,
            message,
        )
    }
}

/// Parser diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseDiagnostic {
    pub span: Span,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
    pub source: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

impl From<ParseError> for ParseDiagnostic {
    fn from(error: ParseError) -> Self {
        let span = error.span().unwrap_or_else(|| Span {
            start: Position::new(1, 1, 0),
            end: Position::new(1, 1, 0),
        });
        
        let suggestions = error.suggestion().into_iter().collect();
        
        ParseDiagnostic {
            span,
            severity: DiagnosticSeverity::Error,
            message: error.to_string(),
            code: Some("PARSE001".to_string()),
            source: "cz-dsl-parser".to_string(),
            suggestions,
        }
    }
}

/// Error reporter for collecting and formatting parser errors
#[derive(Debug, Clone)]
pub struct ParseErrorReporter {
    errors: Vec<ParseError>,
    warnings: Vec<String>,
    source_file: Option<String>,
}

impl ParseErrorReporter {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            source_file: None,
        }
    }
    
    pub fn with_source_file(source_file: String) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            source_file: Some(source_file),
        }
    }
    
    /// Report an error
    pub fn error(&mut self, error: ParseError) {
        self.errors.push(error);
    }
    
    /// Report a warning
    pub fn warning(&mut self, message: String) {
        self.warnings.push(message);
    }
    
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Get all errors
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
    
    /// Get all warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
    
    /// Clear all errors and warnings
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }
    
    /// Convert errors to diagnostics
    pub fn diagnostics(&self) -> Vec<ParseDiagnostic> {
        self.errors.iter().map(|e| e.clone().into()).collect()
    }
}

impl Default for ParseErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Position;
    
    #[test]
    fn test_error_position() {
        let pos = Position::new(10, 5, 100);
        let error = ParseError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: TokenKind::LeftParen,
            position: pos.clone(),
        };
        
        assert_eq!(error.position(), Some(&pos));
        assert!(error.is_recoverable());
    }
    
    #[test]
    fn test_error_suggestion() {
        let error = ParseError::MissingSemicolon {
            position: Position::new(1, 5, 4),
        };
        
        assert!(error.suggestion().is_some());
        assert!(error.suggestion().unwrap().contains("semicolon"));
    }
    
    #[test]
    fn test_error_reporter() {
        let mut reporter = ParseErrorReporter::new();
        
        let error = ParseError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: TokenKind::LeftParen,
            position: Position::new(1, 5, 4),
        };
        
        reporter.error(error);
        reporter.warning("This is a warning".to_string());
        
        assert!(reporter.has_errors());
        assert_eq!(reporter.errors().len(), 1);
        assert_eq!(reporter.warnings().len(), 1);
    }
    
    #[test]
    fn test_diagnostic_conversion() {
        let error = ParseError::InvalidSyntax {
            message: "Invalid expression".to_string(),
            position: Position::new(5, 10, 50),
        };
        
        let diagnostic: ParseDiagnostic = error.into();
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert!(diagnostic.message.contains("Invalid expression"));
    }
}
