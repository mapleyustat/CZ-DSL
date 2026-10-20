//! Main lexer implementation for CZ-DSL
//! 
//! This module implements the lexical analyzer that converts source text into tokens.
//! It supports Unicode identifiers, Chinese keywords, and physical units.

use crate::lexer::{TokenKind, Token, Position, Span, LexError, LexResult, ErrorReporter, UnitParser};
use logos::Logos;
use std::str::Chars;
use std::iter::Peekable;
use unicode_ident::{is_xid_start, is_xid_continue};
use unicode_normalization::UnicodeNormalization;

/// Main lexer struct
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: Position,
    current_char: Option<char>,
    error_reporter: ErrorReporter,
    unit_parser: UnitParser,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.next();
        
        Self {
            input,
            chars,
            position: Position::new(1, 1, 0),
            current_char,
            error_reporter: ErrorReporter::new(),
            unit_parser: UnitParser::new(),
        }
    }
    
    /// Create a lexer with a specific source file name
    pub fn with_source_file(input: &'a str, source_file: String) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.next();
        
        Self {
            input,
            chars,
            position: Position::new(1, 1, 0),
            current_char,
            error_reporter: ErrorReporter::with_source_file(source_file),
            unit_parser: UnitParser::new(),
        }
    }
    
    /// Tokenize the entire input and return all tokens
    pub fn tokenize(&mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();
        
        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token.kind == TokenKind::EOF;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(error) => {
                    self.error_reporter.error(error.clone());
                    
                    // Try to recover from the error
                    if error.is_recoverable() {
                        self.recover_from_error();
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            }
        }
        
        if self.error_reporter.has_errors() {
            // Return the first error if there were any
            Err(self.error_reporter.errors()[0].clone())
        } else {
            Ok(tokens)
        }
    }
    
    /// Get the next token from the input
    pub fn next_token(&mut self) -> LexResult<Token> {
        // Skip whitespace (except newlines which might be significant)
        self.skip_whitespace();
        
        let start_pos = self.position.clone();
        
        match self.current_char {
            None => Ok(self.make_token(TokenKind::EOF, start_pos.clone(), start_pos, "")),
            Some(ch) => {
                match ch {
                    // Single-character tokens
                    '(' => self.single_char_token(TokenKind::LeftParen),
                    ')' => self.single_char_token(TokenKind::RightParen),
                    '[' => self.single_char_token(TokenKind::LeftBracket),
                    ']' => self.single_char_token(TokenKind::RightBracket),
                    '{' => self.single_char_token(TokenKind::LeftBrace),
                    '}' => self.single_char_token(TokenKind::RightBrace),
                    ',' => self.single_char_token(TokenKind::Comma),
                    ';' => self.single_char_token(TokenKind::Semicolon),
                    '@' => self.single_char_token(TokenKind::At),
                    '#' => self.single_char_token(TokenKind::Hash),
                    '$' => self.single_char_token(TokenKind::Dollar),
                    '?' => self.single_char_token(TokenKind::Question),
                    
                    // Multi-character operators
                    '+' => self.plus_or_assign(),
                    '-' => self.minus_or_arrow(),
                    '*' => self.star_or_power(),
                    '/' => self.slash_or_comment(),
                    '%' => self.percent_or_assign(),
                    '=' => self.equal_or_assign(),
                    '!' => self.not_or_not_equal(),
                    '<' => self.less_or_arrow(),
                    '>' => self.greater_or_equal(),
                    '&' => self.ampersand_or_and(),
                    '|' => self.pipe_or_or(),
                    '^' => self.single_char_token(TokenKind::Xor),
                    ':' => self.colon_or_double_colon(),
                    '.' => self.dot_or_range(),
                    '°' => self.degree_or_celsius(),
                    '→' => self.single_char_token(TokenKind::Arrow),
                    
                    // Mathematical symbols
                    '∀' => self.single_char_token(TokenKind::ForAll),
                    '∃' => self.single_char_token(TokenKind::Exists),
                    '∇' => self.single_char_token(TokenKind::Nabla),
                    '∂' => self.single_char_token(TokenKind::Partial),
                    '∫' => self.single_char_token(TokenKind::Integral),
                    '∑' => self.single_char_token(TokenKind::Sum),
                    '∏' => self.single_char_token(TokenKind::Product),
                    '√' => self.single_char_token(TokenKind::Sqrt),
                    '∞' => self.single_char_token(TokenKind::Infinity),
                    '≈' => self.single_char_token(TokenKind::Approx),
                    '≠' => self.single_char_token(TokenKind::NotEqualAlt),
                    '≤' => self.single_char_token(TokenKind::LessEqualAlt),
                    '≥' => self.single_char_token(TokenKind::GreaterEqualAlt),
                    '∈' => self.single_char_token(TokenKind::ElementOf),
                    '∉' => self.single_char_token(TokenKind::NotElementOf),
                    '⊂' => self.single_char_token(TokenKind::SubsetOf),
                    '⊃' => self.single_char_token(TokenKind::SupersetOf),
                    '∪' => self.single_char_token(TokenKind::Union),
                    '∩' => self.single_char_token(TokenKind::Intersection),
                    
                    // String literals
                    '"' => self.string_literal(),
                    '\'' => self.char_literal(),
                    
                    // Raw strings
                    'r' if self.peek_char() == Some('"') => self.raw_string_literal(),
                    
                    // Numbers
                    '0'..='9' => self.numeric_literal(),
                    
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => self.identifier_or_keyword(),
                    
                    // Unicode identifiers (including Chinese)
                    ch if is_xid_start(ch) => self.unicode_identifier(),
                    
                    // Newlines (might be significant)
                    '\n' => self.single_char_token(TokenKind::Newline),
                    
                    // Unexpected character
                    _ => Err(LexError::UnexpectedCharacter {
                        character: ch,
                        position: self.position.clone(),
                    }),
                }
            }
        }
    }
    
    /// Get error reporter
    pub fn error_reporter(&self) -> &ErrorReporter {
        &self.error_reporter
    }
    
    /// Get mutable error reporter
    pub fn error_reporter_mut(&mut self) -> &mut ErrorReporter {
        &mut self.error_reporter
    }
    
    // === Helper methods ===
    
    /// Advance to the next character
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char {
            self.position.offset += ch.len_utf8();
            
            if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            } else {
                self.position.column += 1;
            }
        }
        
        self.current_char = self.chars.next();
        self.current_char
    }
    
    /// Peek at the next character without advancing
    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }
    
    /// Skip whitespace characters (except newlines)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// Create a token with the given kind and span
    fn make_token(&self, kind: TokenKind, start: Position, end: Position, text: &str) -> Token {
        Token {
            kind,
            span: Span { start, end },
            text: text.to_string(),
        }
    }
    
    /// Create a single-character token
    fn single_char_token(&mut self, kind: TokenKind) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let ch = self.current_char.unwrap();
        self.advance();
        let end_pos = self.position.clone();
        
        Ok(self.make_token(kind, start_pos, end_pos, &ch.to_string()))
    }
    
    /// Handle + and +=
    fn plus_or_assign(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '+'
        
        if self.current_char == Some('=') {
            self.advance(); // consume '='
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::PlusAssign, start_pos, end_pos, "+="))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Plus, start_pos, end_pos, "+"))
        }
    }
    
    /// Handle - and -= and ->
    fn minus_or_arrow(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '-'
        
        match self.current_char {
            Some('=') => {
                self.advance(); // consume '='
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::MinusAssign, start_pos, end_pos, "-="))
            }
            Some('>') => {
                self.advance(); // consume '>'
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Arrow, start_pos, end_pos, "->"))
            }
            _ => {
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Minus, start_pos, end_pos, "-"))
            }
        }
    }
    
    /// Handle * and ** and *=
    fn star_or_power(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '*'
        
        match self.current_char {
            Some('*') => {
                self.advance(); // consume second '*'
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Power, start_pos, end_pos, "**"))
            }
            Some('=') => {
                self.advance(); // consume '='
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::StarAssign, start_pos, end_pos, "*="))
            }
            _ => {
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Star, start_pos, end_pos, "*"))
            }
        }
    }
    
    /// Handle / and /= and comments
    fn slash_or_comment(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '/'
        
        match self.current_char {
            Some('/') => self.line_comment(),
            Some('*') => self.block_comment(),
            Some('=') => {
                self.advance(); // consume '='
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::SlashAssign, start_pos, end_pos, "/="))
            }
            _ => {
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Slash, start_pos, end_pos, "/"))
            }
        }
    }
    
    /// Handle % and %=
    fn percent_or_assign(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '%'
        
        if self.current_char == Some('=') {
            self.advance(); // consume '='
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::PercentAssign, start_pos, end_pos, "%="))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Percent, start_pos, end_pos, "%"))
        }
    }
    
    /// Handle = and == and =>
    fn equal_or_assign(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '='
        
        match self.current_char {
            Some('=') => {
                self.advance(); // consume second '='
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Equal, start_pos, end_pos, "=="))
            }
            Some('>') => {
                self.advance(); // consume '>'
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::FatArrow, start_pos, end_pos, "=>"))
            }
            _ => {
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Assign, start_pos, end_pos, "="))
            }
        }
    }
    
    /// Handle ! and !=
    fn not_or_not_equal(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '!'
        
        if self.current_char == Some('=') {
            self.advance(); // consume '='
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::NotEqual, start_pos, end_pos, "!="))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Not, start_pos, end_pos, "!"))
        }
    }
    
    /// Handle < and <= and <- and <=>
    fn less_or_arrow(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '<'
        
        match self.current_char {
            Some('=') => {
                self.advance(); // consume '='
                if self.current_char == Some('>') {
                    self.advance(); // consume '>'
                    let end_pos = self.position.clone();
                    Ok(self.make_token(TokenKind::Spaceship, start_pos, end_pos, "<=>"))
                } else {
                    let end_pos = self.position.clone();
                    Ok(self.make_token(TokenKind::LessEqual, start_pos, end_pos, "<="))
                }
            }
            Some('-') => {
                self.advance(); // consume '-'
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::LeftArrow, start_pos, end_pos, "<-"))
            }
            _ => {
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Less, start_pos, end_pos, "<"))
            }
        }
    }
    
    /// Handle > and >=
    fn greater_or_equal(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '>'
        
        if self.current_char == Some('=') {
            self.advance(); // consume '='
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::GreaterEqual, start_pos, end_pos, ">="))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Greater, start_pos, end_pos, ">"))
        }
    }
    
    /// Handle & and &&
    fn ampersand_or_and(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '&'
        
        if self.current_char == Some('&') {
            self.advance(); // consume second '&'
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::And, start_pos, end_pos, "&&"))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Ampersand, start_pos, end_pos, "&"))
        }
    }
    
    /// Handle | and ||
    fn pipe_or_or(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '|'
        
        if self.current_char == Some('|') {
            self.advance(); // consume second '|'
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Or, start_pos, end_pos, "||"))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Pipe, start_pos, end_pos, "|"))
        }
    }
    
    /// Handle : and ::
    fn colon_or_double_colon(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume ':'
        
        if self.current_char == Some(':') {
            self.advance(); // consume second ':'
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::DoubleColon, start_pos, end_pos, "::"))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Colon, start_pos, end_pos, ":"))
        }
    }
    
    /// Handle . and .. and ..= and ...
    fn dot_or_range(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '.'
        
        match self.current_char {
            Some('.') => {
                self.advance(); // consume second '.'
                if self.current_char == Some('=') {
                    self.advance(); // consume '='
                    let end_pos = self.position.clone();
                    Ok(self.make_token(TokenKind::RangeInclusive, start_pos, end_pos, "..="))
                } else if self.current_char == Some('.') {
                    self.advance(); // consume third '.'
                    let end_pos = self.position.clone();
                    Ok(self.make_token(TokenKind::Ellipsis, start_pos, end_pos, "..."))
                } else {
                    let end_pos = self.position.clone();
                    Ok(self.make_token(TokenKind::Range, start_pos, end_pos, ".."))
                }
            }
            _ => {
                let end_pos = self.position.clone();
                Ok(self.make_token(TokenKind::Period, start_pos, end_pos, "."))
            }
        }
    }
    
    /// Handle ° (degree symbol) and °C
    fn degree_or_celsius(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        self.advance(); // consume '°'
        
        if self.current_char == Some('C') {
            self.advance(); // consume 'C'
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Celsius, start_pos, end_pos, "°C"))
        } else {
            let end_pos = self.position.clone();
            Ok(self.make_token(TokenKind::Degree, start_pos, end_pos, "°"))
        }
    }
    
    /// Parse line comment
    fn line_comment(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let mut content = String::from("//");
        
        self.advance(); // consume second '/'
        
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            content.push(ch);
            self.advance();
        }
        
        let end_pos = self.position.clone();
        Ok(self.make_token(TokenKind::LineComment, start_pos, end_pos, &content))
    }
    
    /// Parse block comment
    fn block_comment(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let mut content = String::from("/*");
        
        self.advance(); // consume '*'
        
        while let Some(ch) = self.current_char {
            content.push(ch);
            
            if ch == '*' && self.peek_char() == Some('/') {
                self.advance(); // consume '*'
                content.push('/');
                self.advance(); // consume '/'
                break;
            }
            
            self.advance();
        }
        
        if !content.ends_with("*/") {
            return Err(LexError::UnterminatedBlockComment {
                position: start_pos,
            });
        }
        
        let end_pos = self.position.clone();
        Ok(self.make_token(TokenKind::BlockComment, start_pos, end_pos, &content))
    }
    
    /// Parse string literal
    fn string_literal(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let mut content = String::new();
        
        self.advance(); // consume opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // consume closing quote
                let end_pos = self.position.clone();
                return Ok(self.make_token(TokenKind::String, start_pos, end_pos, &content));
            }
            
            if ch == '\\' {
                self.advance(); // consume backslash
                match self.current_char {
                    Some('n') => content.push('\n'),
                    Some('t') => content.push('\t'),
                    Some('r') => content.push('\r'),
                    Some('\\') => content.push('\\'),
                    Some('"') => content.push('"'),
                    Some('\'') => content.push('\''),
                    Some('0') => content.push('\0'),
                    Some(escape_ch) => {
                        // Invalid escape sequence - add both characters
                        content.push('\\');
                        content.push(escape_ch);
                    }
                    None => {
                        return Err(LexError::UnterminatedString {
                            position: start_pos,
                        });
                    }
                }
            } else {
                content.push(ch);
            }
            
            self.advance();
        }
        
        Err(LexError::UnterminatedString {
            position: start_pos,
        })
    }
    
    /// Parse character literal
    fn char_literal(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        
        self.advance(); // consume opening quote
        
        let ch = match self.current_char {
            Some('\'') => {
                return Err(LexError::InvalidCharLiteral {
                    position: start_pos,
                });
            }
            Some('\\') => {
                self.advance(); // consume backslash
                match self.current_char {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('\\') => '\\',
                    Some('"') => '"',
                    Some('\'') => '\'',
                    Some('0') => '\0',
                    _ => {
                        return Err(LexError::InvalidCharLiteral {
                            position: start_pos,
                        });
                    }
                }
            }
            Some(ch) => ch,
            None => {
                return Err(LexError::InvalidCharLiteral {
                    position: start_pos,
                });
            }
        };
        
        self.advance(); // consume character
        
        if self.current_char != Some('\'') {
            return Err(LexError::InvalidCharLiteral {
                position: start_pos,
            });
        }
        
        self.advance(); // consume closing quote
        let end_pos = self.position.clone();
        
        Ok(self.make_token(TokenKind::Char, start_pos, end_pos, &format!("'{}'", ch)))
    }
    
    /// Parse raw string literal
    fn raw_string_literal(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        
        self.advance(); // consume 'r'
        self.advance(); // consume '"'
        
        let mut content = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // consume closing quote
                let end_pos = self.position.clone();
                return Ok(self.make_token(TokenKind::RawString, start_pos, end_pos, &content));
            }
            
            content.push(ch);
            self.advance();
        }
        
        Err(LexError::UnterminatedString {
            position: start_pos,
        })
    }
    
    /// Parse numeric literal (integer, float, scientific notation)
    fn numeric_literal(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let mut content = String::new();
        let mut has_dot = false;
        let mut has_exponent = false;
        
        // Collect digits and decimal point
        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => {
                    content.push(ch);
                    self.advance();
                }
                '.' if !has_dot && !has_exponent => {
                    // Check if next character is a digit (to distinguish from range operator)
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch.is_ascii_digit() {
                            has_dot = true;
                            content.push(ch);
                            self.advance();
                        } else {
                            break; // This is a range operator, not part of the number
                        }
                    } else {
                        break;
                    }
                }
                'e' | 'E' if !has_exponent => {
                    // Check if this is actually an exponent or a geographic suffix (E/e)
                    // It's an exponent if followed by a digit or +/-
                    let is_exponent = if let Some(next_ch) = self.peek_char() {
                        next_ch.is_ascii_digit() || next_ch == '+' || next_ch == '-'
                    } else {
                        false
                    };

                    if is_exponent {
                        has_exponent = true;
                        content.push(ch);
                        self.advance();
                        
                        // Handle optional sign after exponent
                        if let Some(sign_ch) = self.current_char {
                            if sign_ch == '+' || sign_ch == '-' {
                                content.push(sign_ch);
                                self.advance();
                            }
                        }
                    } else {
                        // Not an exponent (likely a unit or direction like 'E'ast), stop parsing number
                        break;
                    }
                }
                'i' if has_dot || has_exponent => {
                    // Complex number
                    content.push(ch);
                    self.advance();
                    let end_pos = self.position.clone();
                    return Ok(self.make_token(TokenKind::Complex, start_pos, end_pos, &content));
                }
                _ => break,
            }
        }
        
        let end_pos = self.position.clone();
        
        // Determine token type
        if has_exponent {
            match content.parse::<f64>() {
                Ok(value) => Ok(self.make_token(TokenKind::Scientific(value), start_pos, end_pos, &content)),
                Err(_) => Err(LexError::InvalidNumericLiteral {
                    literal: content,
                    position: start_pos,
                }),
            }
        } else if has_dot {
            match content.parse::<f64>() {
                Ok(value) => Ok(self.make_token(TokenKind::Float(value), start_pos, end_pos, &content)),
                Err(_) => Err(LexError::InvalidNumericLiteral {
                    literal: content,
                    position: start_pos,
                }),
            }
        } else {
            match content.parse::<i64>() {
                Ok(value) => Ok(self.make_token(TokenKind::Integer(value), start_pos, end_pos, &content)),
                Err(_) => Err(LexError::InvalidNumericLiteral {
                    literal: content,
                    position: start_pos,
                }),
            }
        }
    }
    
    /// Parse ASCII identifier or keyword
    fn identifier_or_keyword(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let mut content = String::new();
        
        // First character (already validated)
        if let Some(ch) = self.current_char {
            content.push(ch);
            self.advance();
        }
        
        // Continue with alphanumeric characters and underscores
        while let Some(ch) = self.current_char {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                content.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let end_pos = self.position.clone();
        
        // Check if it's a keyword or unit
        let token_kind = self.classify_identifier(&content);
        
        Ok(self.make_token(token_kind, start_pos, end_pos, &content))
    }
    
    /// Parse Unicode identifier (including Chinese)
    fn unicode_identifier(&mut self) -> LexResult<Token> {
        let start_pos = self.position.clone();
        let mut content = String::new();
        
        // First character (already validated as XID_Start)
        if let Some(ch) = self.current_char {
            content.push(ch);
            self.advance();
        }
        
        // Continue with XID_Continue characters
        while let Some(ch) = self.current_char {
            if is_xid_continue(ch) {
                content.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let end_pos = self.position.clone();
        
        // Normalize Unicode (NFC normalization)
        let normalized: String = content.nfc().collect();
        
        // Check if it's a Chinese keyword
        let token_kind = self.classify_unicode_identifier(&normalized);
        
        Ok(self.make_token(token_kind, start_pos, end_pos, &normalized))
    }
    
    /// Classify an ASCII identifier as keyword, unit, or identifier
    fn classify_identifier(&self, content: &str) -> TokenKind {
        // Check for keywords first
        match content {
            // Core language constructs
            "sphere" => TokenKind::Sphere,
            "scale" => TokenKind::Scale,
            "data" => TokenKind::Data,
            "model" => TokenKind::Model,
            "process" => TokenKind::Process,
            "interface" => TokenKind::Interface,
            "coupling" => TokenKind::Coupling,
            "coastal_database" => TokenKind::CoastalDatabase,
            "world_model" => TokenKind::WorldModel,
            "compute_context" => TokenKind::ComputeContext,
            "ai_model" => TokenKind::AiModel,
            "digital_twin" => TokenKind::DigitalTwin,
            
            // Sphere types
            "Atmosphere" => TokenKind::AtmosphereType,
            "Ocean" => TokenKind::OceanType,
            "Lithosphere" => TokenKind::LithosphereType,
            "Biosphere" => TokenKind::BiosphereType,
            "Anthroposphere" => TokenKind::AnthroposphereType,
            
            // Scale keywords
            "spatial" => TokenKind::Spatial,
            "temporal" => TokenKind::Temporal,
            "molecular" => TokenKind::Molecular,
            "cellular" => TokenKind::Cellular,
            "organism" => TokenKind::Organism,
            "ecosystem" => TokenKind::Ecosystem,
            "regional" => TokenKind::Regional,
            "global" => TokenKind::Global,
            
            // Coastal database constructs
            "layer" => TokenKind::LayerKw,
            "dataset" => TokenKind::DatasetKw,
            
            // Data types
            "observation" => TokenKind::Observation,
            "simulation" => TokenKind::Simulation,
            "remote_sensing" => TokenKind::RemoteSensing,
            "field" => TokenKind::Field,
            "time_series" => TokenKind::TimeSeries,
            "uncertain" => TokenKind::Uncertain,
            
            // Control flow
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "match" => TokenKind::Match,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            
            // Declarations
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "type" => TokenKind::Type,
            "enum" => TokenKind::Enum,
            "struct" => TokenKind::Struct,
            "class" => TokenKind::Class,
            "abstract" => TokenKind::Abstract,
            "extends" => TokenKind::Extends,
            "implements" => TokenKind::Implements,
            
            // Modifiers
            "pub" => TokenKind::Pub,
            "private" => TokenKind::Private,
            "static" => TokenKind::Static,
            "mut" => TokenKind::Mut,
            "ref" => TokenKind::Ref,
            
            // Functions and methods
            "fn" => TokenKind::Fn,
            "method" => TokenKind::Method,
            "override" => TokenKind::Override,
            "auto" => TokenKind::Auto,
            
            // Constraints and properties
            "constraint" => TokenKind::Constraint,
            "property" => TokenKind::Property,
            "invariant" => TokenKind::Invariant,
            "requires" => TokenKind::Requires,
            "ensures" => TokenKind::Ensures,
            
            // Physical units and dimensions
            "with" => TokenKind::With,
            "unit" => TokenKind::Unit,
            "dimension" => TokenKind::Dimension,
            
            // Base SI units
            "meter" => TokenKind::Meter,
            "m" => TokenKind::Meter,
            "kilogram" => TokenKind::Kilogram,
            "kg" => TokenKind::Kilogram,
            "second" => TokenKind::Second,
            "s" => TokenKind::Second,
            "ampere" => TokenKind::Ampere,
            "A" => TokenKind::Ampere,
            "kelvin" => TokenKind::Kelvin,
            "K" => TokenKind::Kelvin,
            "mole" => TokenKind::Mole,
            "mol" => TokenKind::Mole,
            "candela" => TokenKind::Candela,
            "cd" => TokenKind::Candela,
            
            // Derived units
            "pascal" => TokenKind::Pascal,
            "Pa" => TokenKind::Pascal,
            "newton" => TokenKind::Newton,
            "N" => TokenKind::Newton,
            "joule" => TokenKind::Joule,
            "J" => TokenKind::Joule,
            "watt" => TokenKind::Watt,
            "W" => TokenKind::Watt,
            "celsius" => TokenKind::Celsius,
            
            // Coastal units
            "psu" => TokenKind::PSU,
            "dbar" => TokenKind::Decibar,
            "sverdrup" => TokenKind::Sverdrup,
            "Sv" => TokenKind::Sverdrup,
            "hPa" => TokenKind::Hectopascal,
            "atm" => TokenKind::Atmosphere,
            "mmHg" => TokenKind::MillimeterMercury,
            
            // Geographic units
            "degree" => TokenKind::Degree,
            "deg" => TokenKind::Degree,
            "radian" => TokenKind::Radian,
            "rad" => TokenKind::Radian,
            "nautical_mile" => TokenKind::NauticalMile,
            "nmi" => TokenKind::NauticalMile,
            
            // Time units
            "minute" => TokenKind::Minute,
            "min" => TokenKind::Minute,
            "hour" => TokenKind::Hour,
            "h" => TokenKind::Hour,
            "day" => TokenKind::Day,
            "d" => TokenKind::Day,
            "year" => TokenKind::Year,
            "yr" => TokenKind::Year,
            
            // Boolean literals
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            
            // Default to identifier
            _ => TokenKind::Identifier,
        }
    }
    
    /// Classify a Unicode identifier (including Chinese keywords)
    fn classify_unicode_identifier(&self, content: &str) -> TokenKind {
        // Check for Chinese keywords
        match content {
            "圈层" => TokenKind::SphereChinese,
            "尺度" => TokenKind::ScaleChinese,
            "数据" => TokenKind::DataChinese,
            "模型" => TokenKind::ModelChinese,
            "过程" => TokenKind::ProcessChinese,
            "耦合" => TokenKind::CouplingChinese,
            "大气圈" => TokenKind::AtmosphereChinese,
            "海洋圈" => TokenKind::OceanChinese,
            "岩石圈" => TokenKind::LithosphereChinese,
            "生物圈" => TokenKind::BiosphereChinese,
            "人类圈" => TokenKind::AnthroposphereChinese,
            "时空" => TokenKind::SpatioTemporalChinese,
            "物理量" => TokenKind::PhysicalQuantityChinese,
            "不确定性" => TokenKind::UncertaintyChinese,
            _ => TokenKind::UnicodeIdentifier,
        }
    }
    
    /// Recover from a lexical error
    fn recover_from_error(&mut self) {
        // Simple recovery: skip the current character
        if self.current_char.is_some() {
            self.advance();
        }
    }
}
    
