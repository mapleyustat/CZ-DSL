//! Error handling for the CZ-DSL lexer
//! 
//! Provides comprehensive error types and recovery mechanisms for lexical analysis.

use crate::lexer::{Position, Span};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Lexical analysis errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LexError {
    #[error("Unexpected character '{character}' at {position}")]
    UnexpectedCharacter {
        character: char,
        position: Position,
    },
    
    #[error("Unterminated string literal at {position}")]
    UnterminatedString {
        position: Position,
    },
    
    #[error("Unterminated block comment at {position}")]
    UnterminatedBlockComment {
        position: Position,
    },
    
    #[error("Invalid numeric literal '{literal}' at {position}")]
    InvalidNumericLiteral {
        literal: String,
        position: Position,
    },
    
    #[error("Invalid Unicode escape sequence at {position}")]
    InvalidUnicodeEscape {
        position: Position,
    },
    
    #[error("Invalid character literal at {position}")]
    InvalidCharLiteral {
        position: Position,
    },
    
    #[error("Invalid unit '{unit}' at {position}")]
    InvalidUnit {
        unit: String,
        position: Position,
    },
    
    #[error("Invalid Chinese identifier at {position}")]
    InvalidChineseIdentifier {
        position: Position,
    },
    
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    
    #[error("IO error: {message}")]
    IoError {
        message: String,
    },
    
    #[error("Encoding error: invalid UTF-8 sequence at {position}")]
    EncodingError {
        position: Position,
    },
}

impl LexError {
    /// Get the position where the error occurred, if available
    pub fn position(&self) -> Option<&Position> {
        match self {
            LexError::UnexpectedCharacter { position, .. } => Some(position),
            LexError::UnterminatedString { position } => Some(position),
            LexError::UnterminatedBlockComment { position } => Some(position),
            LexError::InvalidNumericLiteral { position, .. } => Some(position),
            LexError::InvalidUnicodeEscape { position } => Some(position),
            LexError::InvalidCharLiteral { position } => Some(position),
            LexError::InvalidUnit { position, .. } => Some(position),
            LexError::InvalidChineseIdentifier { position } => Some(position),
            LexError::EncodingError { position } => Some(position),
            _ => None,
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
            LexError::UnexpectedCharacter { .. } => true,
            LexError::InvalidNumericLiteral { .. } => true,
            LexError::InvalidUnit { .. } => true,
            LexError::InvalidChineseIdentifier { .. } => true,
            LexError::UnterminatedString { .. } => false,
            LexError::UnterminatedBlockComment { .. } => false,
            LexError::UnexpectedEOF => false,
            LexError::IoError { .. } => false,
            LexError::EncodingError { .. } => false,
            _ => true,
        }
    }
    
    /// Get a suggested fix for the error, if available
    pub fn suggestion(&self) -> Option<String> {
        match self {
            LexError::UnexpectedCharacter { character, .. } => {
                Some(format!("Remove or escape the character '{}'", character))
            },
            LexError::UnterminatedString { .. } => {
                Some("Add closing quote to terminate the string".to_string())
            },
            LexError::UnterminatedBlockComment { .. } => {
                Some("Add */ to close the block comment".to_string())
            },
            LexError::InvalidNumericLiteral { literal, .. } => {
                Some(format!("Check the format of numeric literal '{}'", literal))
            },
            LexError::InvalidUnit { unit, .. } => {
                Some(format!("Check if '{}' is a valid unit name", unit))
            },
            _ => None,
        }
    }
}

/// Result type for lexical operations
pub type LexResult<T> = Result<T, LexError>;

/// Error recovery strategy
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Skip the current character and continue
    SkipCharacter,
    /// Skip to the next whitespace or delimiter
    SkipToDelimiter,
    /// Skip to the end of the current line
    SkipToEndOfLine,
    /// Insert a missing token
    InsertToken,
    /// Replace with a default token
    ReplaceWithDefault,
}

/// Error recovery context
#[derive(Debug, Clone)]
pub struct RecoveryContext {
    pub strategy: RecoveryStrategy,
    pub position: Position,
    pub message: String,
}

impl RecoveryContext {
    pub fn new(strategy: RecoveryStrategy, position: Position, message: String) -> Self {
        Self {
            strategy,
            position,
            message,
        }
    }
}

/// Error reporter for collecting and formatting lexical errors
#[derive(Debug, Clone)]
pub struct ErrorReporter {
    errors: Vec<LexError>,
    warnings: Vec<String>,
    source_file: Option<String>,
}

impl ErrorReporter {
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
    pub fn error(&mut self, error: LexError) {
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
    pub fn errors(&self) -> &[LexError] {
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
    
    /// Format all errors and warnings for display
    pub fn format_messages(&self, source_lines: &[String]) -> String {
        let mut output = String::new();
        
        // Format errors
        for error in &self.errors {
            output.push_str(&self.format_error(error, source_lines));
            output.push('\n');
        }
        
        // Format warnings
        for warning in &self.warnings {
            output.push_str(&format!("Warning: {}\n", warning));
        }
        
        output
    }
    
    /// Format a single error with context
    fn format_error(&self, error: &LexError, source_lines: &[String]) -> String {
        let mut output = String::new();
        
        // Error header
        if let Some(ref file) = self.source_file {
            output.push_str(&format!("Error in {}: ", file));
        } else {
            output.push_str("Error: ");
        }
        output.push_str(&error.to_string());
        output.push('\n');
        
        // Show source context if position is available
        if let Some(position) = error.position() {
            if position.line > 0 && (position.line - 1) < source_lines.len() {
                let line_idx = position.line - 1;
                let line = &source_lines[line_idx];
                
                // Line number and content
                output.push_str(&format!("{:4} | {}\n", position.line, line));
                
                // Pointer to error location
                let pointer_line = format!("{:4} | {}{}", 
                    "", 
                    " ".repeat(position.column.saturating_sub(1)), 
                    "^"
                );
                output.push_str(&pointer_line);
                output.push('\n');
            }
        }
        
        // Add suggestion if available
        if let Some(suggestion) = error.suggestion() {
            output.push_str(&format!("Suggestion: {}\n", suggestion));
        }
        
        output
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Diagnostic information for IDE integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub span: Span,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
    pub source: String,
    pub related_information: Vec<DiagnosticRelatedInformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    pub span: Span,
    pub message: String,
}

impl From<LexError> for Diagnostic {
    fn from(error: LexError) -> Self {
        let span = error.span().unwrap_or_else(|| Span {
            start: Position::new(1, 1, 0),
            end: Position::new(1, 1, 0),
        });
        
        Diagnostic {
            span,
            severity: DiagnosticSeverity::Error,
            message: error.to_string(),
            code: Some("LEX001".to_string()),
            source: "cz-dsl-lexer".to_string(),
            related_information: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_position() {
        let pos = Position::new(10, 5, 100);
        let error = LexError::UnexpectedCharacter {
            character: '@',
            position: pos.clone(),
        };
        
        assert_eq!(error.position(), Some(&pos));
        assert!(error.is_recoverable());
    }
    
    #[test]
    fn test_error_reporter() {
        let mut reporter = ErrorReporter::new();
        
        let error = LexError::UnexpectedCharacter {
            character: '@',
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
        let error = LexError::InvalidNumericLiteral {
            literal: "123.45.67".to_string(),
            position: Position::new(5, 10, 50),
        };
        
        let diagnostic: Diagnostic = error.into();
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert!(diagnostic.message.contains("Invalid numeric literal"));
    }
}