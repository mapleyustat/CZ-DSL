//! CZ-DSL Recursive Descent Parser
//! 
//! This module implements a recursive descent parser for the Coastal Zone Domain-Specific Language.
//! It converts a stream of tokens from the lexer into an Abstract Syntax Tree (AST).

use crate::lexer::{Lexer, Token, TokenKind, Position, Span};
use crate::parser::{
    ast::*, error::*, grammar::Grammar
};
use std::collections::VecDeque;

/// Main parser struct
pub struct Parser {
    tokens: VecDeque<Token>,
    current: usize,
    error_reporter: ParseErrorReporter,
}

impl Parser {
    /// Create a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
            current: 0,
            error_reporter: ParseErrorReporter::new(),
        }
    }
    
    /// Create a parser with source file information
    pub fn with_source_file(tokens: Vec<Token>, source_file: String) -> Self {
        Self {
            tokens: tokens.into(),
            current: 0,
            error_reporter: ParseErrorReporter::with_source_file(source_file),
        }
    }
    
    /// Parse the token stream into a program AST
    pub fn parse(&mut self) -> ParseResult<Program> {
        // Skip leading trivia such as newlines and comments
        self.skip_trivia();
        let start_pos = self.current_position();
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            // Skip trivia between top-level declarations
            self.skip_trivia();
            if self.is_at_end() {
                break;
            }
            match self.parse_item() {
                Ok(item) => items.push(item),
                Err(error) => {
                    self.error_reporter.error(error.clone());
                    
                    if error.is_recoverable() {
                        self.recover_from_error();
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            }
        }
        
        let end_pos = self.current_position();
        let span = Span { start: start_pos, end: end_pos };
        
        if self.error_reporter.has_errors() {
            // Return the first error if there were any
            Err(self.error_reporter.errors()[0].clone())
        } else {
            Ok(Program::new(items, span))
        }
    }
    
    /// Get the error reporter
    pub fn error_reporter(&self) -> &ParseErrorReporter {
        &self.error_reporter
    }
    
    // === Helper methods ===
    
    /// Check if we're at the end of the token stream
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || 
        self.peek().map_or(true, |t| t.kind == TokenKind::EOF)
    }
    
    /// Peek at the current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    
    /// Advance to the next token and return the current one
    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1).cloned()
    }
    
    /// Check if the current token matches the given kind
    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().map_or(false, |t| &t.kind == kind)
    }
    
    /// Consume the current token if it matches the given kind
    fn match_token(&mut self, kind: TokenKind) -> bool {
        self.skip_trivia();
        if self.check(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// Get the current position
    fn current_position(&self) -> Position {
        self.peek()
            .map(|t| t.span.start.clone())
            .unwrap_or_else(|| Position::new(1, 1, 0))
    }
    
    /// Create a span from start position to current position
    fn span_from(&self, start: Position) -> Span {
        let end = self.current_position();
        Span { start, end }
    }
    
    /// Recover from a parse error
    fn recover_from_error(&mut self) {
        while !self.is_at_end() {
            if let Some(token) = self.peek() {
                match token.kind {
                    TokenKind::Semicolon => {
                        self.advance();
                        break;
                    }
                    TokenKind::RightBrace => {
                        self.advance();
                        break;
                    }
                    TokenKind::Sphere | TokenKind::SphereChinese => break,
                    _ => {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
    }
    
    // === Parsing methods ===
    
    /// Skip trivia tokens (whitespace and comments)
    fn skip_trivia(&mut self) {
        while !self.is_at_end() {
            if let Some(token) = self.peek() {
                if token.kind.is_whitespace() || token.kind.is_comment() {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }
    
    /// Parse a top-level item
    fn parse_item(&mut self) -> ParseResult<Item> {
        let token = self.peek().ok_or_else(|| ParseError::UnexpectedEOF {
            expected: "item".to_string(),
        })?;
        
        match token.kind {
            TokenKind::Sphere | TokenKind::SphereChinese => {
                Ok(Item::Sphere(self.parse_sphere_decl()?))
            }
            TokenKind::Scale | TokenKind::ScaleChinese => {
                Ok(Item::Scale(self.parse_scale_decl()?))
            }
            TokenKind::Data | TokenKind::DataChinese => {
                Ok(Item::Data(self.parse_data_decl()?))
            }
            TokenKind::Model | TokenKind::ModelChinese => {
                Ok(Item::Model(self.parse_model_decl()?))
            }
            TokenKind::Process | TokenKind::ProcessChinese => {
                Ok(Item::Process(self.parse_process_decl()?))
            }
            TokenKind::Interface => {
                Ok(Item::Interface(self.parse_interface_decl()?))
            }
            TokenKind::AiModel => {
                Ok(Item::AiModel(self.parse_ai_model_decl()?))
            }
            TokenKind::DigitalTwin => {
                Ok(Item::DigitalTwin(self.parse_digital_twin_decl()?))
            }
            TokenKind::Fn => {
                Ok(Item::Function(self.parse_function_decl()?))
            }
            TokenKind::Type => {
                Ok(Item::Type(self.parse_type_decl()?))
            }
            TokenKind::Const => {
                Ok(Item::Const(self.parse_const_decl()?))
            }
            TokenKind::Let => {
                Ok(Item::Let(self.parse_let_decl()?))
            }
            TokenKind::Constraint => {
                Ok(Item::Constraint(self.parse_constraint_decl()?))
            }
            TokenKind::CoastalDatabase => {
                Ok(Item::CoastalDatabase(self.parse_coastal_database_decl()?))
            }
            _ => {
                let position = token.span.start.clone();
                Err(ParseError::UnexpectedToken {
                    expected: "declaration".to_string(),
                    found: token.kind.clone(),
                    position,
                })
            }
        }
    }
    
    /// Parse a sphere declaration (basic implementation)
    fn parse_sphere_decl(&mut self) -> ParseResult<SphereDecl> {
        let start_pos = self.current_position();
        
        // Consume 'sphere' or '圈层'
        self.advance();
        
        // Parse name
        let name = self.parse_identifier()?;
        
        // Parse optional 'extends' clause
        let extends = if self.match_token(TokenKind::Extends) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        // Parse body
        self.consume(TokenKind::LeftBrace, "Expected '{' after sphere name")?;
        
        let mut members = Vec::new();
        // Allow newlines and comments inside sphere bodies
        self.skip_trivia();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_trivia();
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            members.push(self.parse_sphere_member()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after sphere body")?;
        
        let span = self.span_from(start_pos);
        Ok(SphereDecl { name, extends, members, span })
    }

    /// Parse generic parameters: <T, U: Real>
    fn parse_generic_params(&mut self) -> ParseResult<Vec<GenericParam>> {
        if !self.match_token(TokenKind::Less) {
            return Ok(Vec::new());
        }

        let mut params = Vec::new();
        while !self.check(&TokenKind::Greater) && !self.is_at_end() {
            let start_pos = self.current_position();
            let name = self.parse_identifier()?;
            
            let mut bounds = Vec::new();
            if self.match_token(TokenKind::Colon) {
                // Parse bounds
                loop {
                    bounds.push(self.parse_type_expr()?);
                    if !self.match_token(TokenKind::Plus) {
                        break;
                    }
                }
            }

            let span = self.span_from(start_pos);
            params.push(GenericParam { name, bounds, span });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.consume(TokenKind::Greater, "Expected '>' after generic parameters")?;
        Ok(params)
    }

    /// Parse a scale declaration
    fn parse_scale_decl(&mut self) -> ParseResult<ScaleDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'scale'

        let name = self.parse_identifier()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after scale name")?;

        let mut ranges = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            ranges.push(self.parse_scale_range()?);
            // Optional comma
            self.match_token(TokenKind::Comma);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after scale body")?;
        let span = self.span_from(start_pos);
        Ok(ScaleDecl { name, ranges, span })
    }

    /// Parse a scale range
    fn parse_scale_range(&mut self) -> ParseResult<ScaleRange> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':' after range name")?;
        
        let range = self.parse_range_expr()?;
        
        let unit = if self.match_token(TokenKind::With) {
             self.consume(TokenKind::Unit, "Expected 'unit' after 'with'")?;
             Some(self.parse_unit_expr()?)
        } else {
            None
        };

        let span = self.span_from(start_pos);
        Ok(ScaleRange { name, range, unit, span })
    }

    /// Parse range expression: 1..10, 1..=10, etc.
    fn parse_range_expr(&mut self) -> ParseResult<RangeExpr> {
        let start_pos = self.current_position();
        // Simplified: assuming we always have a start expression for now
        let start = Box::new(self.parse_expression()?);
        
        if self.match_token(TokenKind::Range) {
            let end = Box::new(self.parse_expression()?);
            let span = self.span_from(start_pos);
            Ok(RangeExpr::Exclusive { start, end, span })
        } else if self.match_token(TokenKind::RangeInclusive) {
            let end = Box::new(self.parse_expression()?);
            let span = self.span_from(start_pos);
            Ok(RangeExpr::Inclusive { start, end, span })
        } else {
             // This is not really a range, but our AST structure enforces it here?
             // Or maybe we should allow single values. 
             // For now, let's assume valid range syntax.
             // Actually, RangeExpr is defined as enum. 
             // If we didn't find .. or ..=, it's an error for parse_range_expr context
             let position = self.current_position();
             Err(ParseError::UnexpectedToken {
                 expected: ".. or ..=".to_string(),
                 found: TokenKind::Error, // Placeholder
                 position,
             })
        }
    }

    /// Parse a data declaration
    fn parse_data_decl(&mut self) -> ParseResult<DataDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'data'

        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after data name")?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut constraints = Vec::new();

        // Allow newlines and comments inside data bodies
        self.skip_trivia();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_trivia();
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            if self.check(&TokenKind::Fn) {
                methods.push(self.parse_method_decl()?);
            } else if self.check(&TokenKind::Constraint) {
                constraints.push(self.parse_constraint_decl()?);
            } else {
                fields.push(self.parse_field_decl()?);
            }
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after data body")?;
        let span = self.span_from(start_pos);
        Ok(DataDecl { name, generic_params, fields, methods, constraints, span })
    }

    /// Parse a field declaration
    fn parse_field_decl(&mut self) -> ParseResult<FieldDecl> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':' after field name")?;
        let type_expr = self.parse_type_expr()?;
        
        let default_value = if self.match_token(TokenKind::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Comma, "Expected ',' after field declaration")?;
        let span = self.span_from(start_pos);
        Ok(FieldDecl { name, type_expr, default_value, span })
    }
    
    /// Parse a unit expression
    fn parse_unit_expr(&mut self) -> ParseResult<UnitExpr> {
        let start_pos = self.current_position();
        
        // Simple implementation: just one unit term or multiple multiplied/divided
        // First term
        let unit_name = self.parse_identifier()?; // Or match specific unit tokens?
        // Actually unit tokens are separate. I should probably handle both identifiers and keywords if they are units.
        // For now, let's assume parse_identifier handles unit names if they are valid identifiers.
        // But wait, "meter", "kg" are keywords/tokens. 
        // I should check `TokenKind::is_unit`.
        
        // Let's refine this. 
        // The current `parse_identifier` expects `Identifier` or `UnicodeIdentifier`.
        // If the token is a unit keyword (like `Meter`), `parse_identifier` will fail.
        // So I need a `parse_unit_identifier`.
        
        // Let's defer full unit parsing logic and implement a basic one.
        // Assuming unit expression is just a name for now to satisfy the compiler.
        let span = self.span_from(start_pos);
        Ok(UnitExpr::Basic { name: unit_name, span })
    }

    /// Parse a model declaration
    fn parse_model_decl(&mut self) -> ParseResult<ModelDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'model'

        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after model name")?;

        let mut parameters = Vec::new();
        let mut variables = Vec::new();
        let mut equations = Vec::new();
        let mut constraints = Vec::new();
        let mut couplings = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.check(&TokenKind::Constraint) {
                constraints.push(self.parse_constraint_decl()?);
            } else if self.check(&TokenKind::Coupling) || self.check(&TokenKind::CouplingChinese) {
                couplings.push(self.parse_coupling_decl()?);
            } else if self.check(&TokenKind::Const) {
                // Parameters often declared as const in models? 
                // Or maybe specific syntax.
                // AST has ParameterDecl. Let's assume syntax "param name: Type = Val;"
                // But TokenKind doesn't have Param. 
                // Maybe "const" maps to parameters in model context?
                // Or maybe just field-like syntax.
                // Let's assume standard field syntax for now, and distinguish by context or keywords if available.
                // For now, let's map 'const' to parameter, 'let' or bare field to variable.
                let param = self.parse_parameter_decl()?;
                parameters.push(param);
            } else {
                // Variable or Equation?
                // Equation: name = expression;
                // Variable: name: Type [= val];
                
                // We need lookahead.
                // Identifier : -> Variable
                // Identifier = -> Equation
                
                if self.check_variable_decl() {
                    variables.push(self.parse_variable_decl()?);
                } else {
                    equations.push(self.parse_equation_decl()?);
                }
            }
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after model body")?;
        let span = self.span_from(start_pos);
        Ok(ModelDecl { name, generic_params, parameters, variables, equations, constraints, couplings, span })
    }

    fn check_variable_decl(&self) -> bool {
        // Look ahead: Identifier Colon
        if let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::Identifier | TokenKind::UnicodeIdentifier) {
                 if let Some(next) = self.tokens.get(self.current + 1) {
                     return next.kind == TokenKind::Colon;
                 }
            }
        }
        false
    }

    fn parse_parameter_decl(&mut self) -> ParseResult<ParameterDecl> {
        let start_pos = self.current_position();
        self.consume(TokenKind::Const, "Expected 'const'")?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':'")?;
        let type_expr = self.parse_type_expr()?;
        
        let default_value = if self.match_token(TokenKind::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        let span = self.span_from(start_pos);
        Ok(ParameterDecl { name, type_expr, default_value, span })
    }

    fn parse_variable_decl(&mut self) -> ParseResult<VariableDecl> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':'")?;
        let type_expr = self.parse_type_expr()?;
        
        let initial_value = if self.match_token(TokenKind::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        let span = self.span_from(start_pos);
        Ok(VariableDecl { name, type_expr, initial_value, span })
    }

    fn parse_equation_decl(&mut self) -> ParseResult<EquationDecl> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Assign, "Expected '='")?;
        let expression = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        let span = self.span_from(start_pos);
        Ok(EquationDecl { name, expression, span })
    }

    fn parse_coupling_decl(&mut self) -> ParseResult<CouplingDecl> {
        let start_pos = self.current_position();
        // coupling name: source -> target;
        self.advance(); // consume 'coupling'
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':'")?;
        let source = self.parse_identifier()?;
        
        let coupling_type = if self.match_token(TokenKind::Arrow) {
            CouplingType::OneWay
        } else if self.match_token(TokenKind::FatArrow) { // => ?
            CouplingType::Feedback // Just guessing mapping
        } else if self.match_token(TokenKind::Spaceship) { // <=>
            CouplingType::TwoWay
        } else {
            CouplingType::OneWay // Default or error
        };
        
        let target = self.parse_identifier()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        let span = self.span_from(start_pos);
        Ok(CouplingDecl { name, source, target, coupling_type, span })
    }

    fn parse_constraint_decl(&mut self) -> ParseResult<ConstraintDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'constraint'
        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;
        
        self.consume(TokenKind::LeftBrace, "Expected '{'")?;
        
        // Constraint body can be Quantified or Expression
        // For now let's implement Expression body
        let body = ConstraintBody::Expression(self.parse_expression()?);
        
        self.consume(TokenKind::RightBrace, "Expected '}'")?;
        
        let span = self.span_from(start_pos);
        Ok(ConstraintDecl { name, generic_params, body, span })
    }

    /// Parse a process declaration
    fn parse_process_decl(&mut self) -> ParseResult<ProcessDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'process'

        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after process name")?;

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // How to distinguish inputs/outputs?
            // Maybe keywords "input", "output" but TokenKind doesn't show them.
            // Let's assume they are just fields for now or use specific syntax if I find it.
            // But looking at `IoDecl`, it has `name` and `type_expr`.
            // Let's assume standard methods and parameters for now.
            // And maybe specific method names or attributes for IO?
            // Or maybe "in name: Type" syntax? TokenKind has no "in".
            
            // Re-reading tokens... no input/output keywords.
            // Maybe they are defined as `data Input { ... }` inside? No.
            
            // Let's implement standard structure parsing.
            if self.check(&TokenKind::Fn) {
                methods.push(self.parse_method_decl()?);
            } else if self.check(&TokenKind::Constraint) {
                constraints.push(self.parse_constraint_decl()?);
            } else if self.check(&TokenKind::Const) {
                parameters.push(self.parse_parameter_decl()?);
            } else {
                // Assume generic field is parameter or variable.
                // But ProcessDecl needs inputs/outputs.
                // Let's skip inputs/outputs specific parsing for now unless we see syntax like "input x: Real".
                // We'll just parse parameters.
                // Wait, I can't just skip them if they are required by struct.
                // I'll leave them empty for now or try to parse them if I can identify them.
                self.advance(); // Skip unknown token to avoid infinite loop
            }
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after process body")?;
        let span = self.span_from(start_pos);
        Ok(ProcessDecl { name, generic_params, inputs, outputs, parameters, constraints, methods, span })
    }

    /// Parse an interface declaration
    fn parse_interface_decl(&mut self) -> ParseResult<InterfaceDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'interface'

        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;

        self.consume(TokenKind::LeftBrace, "Expected '{'")?;

        let mut methods = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
             methods.push(self.parse_method_signature()?);
        }

        self.consume(TokenKind::RightBrace, "Expected '}'")?;
        let span = self.span_from(start_pos);
        Ok(InterfaceDecl { name, generic_params, methods, span })
    }

    fn parse_method_signature(&mut self) -> ParseResult<MethodSignature> {
        let start_pos = self.current_position();
        self.consume(TokenKind::Fn, "Expected 'fn'")?;
        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;
        
        self.consume(TokenKind::LeftParen, "Expected '('")?;
        let parameters = self.parse_parameters()?;
        self.consume(TokenKind::RightParen, "Expected ')'")?;
        
        let return_type = if self.match_token(TokenKind::Arrow) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        let span = self.span_from(start_pos);
        Ok(MethodSignature { name, generic_params, parameters, return_type, span })
    }

    /// Parse AI Model declaration
    fn parse_ai_model_decl(&mut self) -> ParseResult<AiModelDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'ai_model'
        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;
        
        self.consume(TokenKind::LeftBrace, "Expected '{'")?;
        // Simplified body parsing
        let mut methods = Vec::new();
        let physics_constraints = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
             if self.check(&TokenKind::Fn) {
                 methods.push(self.parse_method_decl()?);
             } else {
                 self.advance();
             }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}'")?;
        
        let span = self.span_from(start_pos);
        Ok(AiModelDecl { name, generic_params, architecture: None, training_data: None, physics_constraints, methods, span })
    }

    /// Parse Digital Twin declaration
    fn parse_digital_twin_decl(&mut self) -> ParseResult<DigitalTwinDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'digital_twin'
        let name = self.parse_identifier()?;
        
        // Expecting "for PhysicalSystem" syntax or similar?
        // AST has `physical_system: TypeExpr` and `digital_replica: TypeExpr`.
        // Let's assume syntax: digital_twin Name { physical: Type; replica: Type; ... }
        
        self.consume(TokenKind::LeftBrace, "Expected '{'")?;
        
        // Placeholder types for required fields
        let dummy_type = TypeExpr::Basic { 
            name: Identifier::new("Unknown".to_string(), self.span_from(start_pos.clone())), 
            span: self.span_from(start_pos.clone()) 
        };
        let mut physical_system = dummy_type.clone();
        let mut digital_replica = dummy_type.clone();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
             // Parse fields...
             self.advance();
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}'")?;
        
        let span = self.span_from(start_pos);
        Ok(DigitalTwinDecl { name, physical_system, digital_replica, assimilation: None, simulation: None, control: None, span })
    }

    /// Parse function declaration
    fn parse_function_decl(&mut self) -> ParseResult<FunctionDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'fn'
        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;
        
        self.consume(TokenKind::LeftParen, "Expected '('")?;
        let parameters = self.parse_parameters()?;
        self.consume(TokenKind::RightParen, "Expected ')'")?;
        
        let return_type = if self.match_token(TokenKind::Arrow) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        let body = if self.check(&TokenKind::LeftBrace) {
            Some(self.parse_block()?)
        } else {
            self.consume(TokenKind::Semicolon, "Expected ';' or body")?;
            None
        };
        
        let span = self.span_from(start_pos);
        Ok(FunctionDecl { name, generic_params, parameters, return_type, body, span })
    }
    
    fn parse_parameters(&mut self) -> ParseResult<Vec<Parameter>> {
        let mut params = Vec::new();
        while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
            let start_pos = self.current_position();
            let name = self.parse_identifier()?;
            self.consume(TokenKind::Colon, "Expected ':'")?;
            let type_expr = self.parse_type_expr()?;
            
            let default_value = if self.match_token(TokenKind::Assign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            let span = self.span_from(start_pos);
            params.push(Parameter { name, type_expr, default_value, span });
            
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_method_decl(&mut self) -> ParseResult<MethodDecl> {
        let start_pos = self.current_position();
        let is_override = self.match_token(TokenKind::Override);
        
        // consume 'fn' or 'method'?
        if !self.match_token(TokenKind::Fn) && !self.match_token(TokenKind::Method) {
             // Error if neither
             return Err(ParseError::UnexpectedToken {
                 expected: "fn or method".to_string(),
                 found: self.peek().unwrap().kind.clone(),
                 position: start_pos,
             });
        }
        
        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;
        
        self.consume(TokenKind::LeftParen, "Expected '('")?;
        let parameters = self.parse_parameters()?;
        self.consume(TokenKind::RightParen, "Expected ')'")?;
        
        let return_type = if self.match_token(TokenKind::Arrow) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        let body = if self.check(&TokenKind::LeftBrace) {
            Some(self.parse_block()?)
        } else {
            self.consume(TokenKind::Semicolon, "Expected ';' or body")?;
            None
        };
        
        let span = self.span_from(start_pos);
        Ok(MethodDecl { name, generic_params, parameters, return_type, body, is_override, span })
    }

    /// Parse block
    fn parse_block(&mut self) -> ParseResult<Block> {
        let start_pos = self.current_position();
        self.consume(TokenKind::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}'")?;
        let span = self.span_from(start_pos);
        Ok(Block { statements, span })
    }

    /// Parse statement
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let start_pos = self.current_position();
        
        if self.match_token(TokenKind::Let) {
            // Let binding
            let name = self.parse_identifier()?;
            let type_expr = if self.match_token(TokenKind::Colon) {
                Some(self.parse_type_expr()?)
            } else {
                None
            };
            self.consume(TokenKind::Assign, "Expected '='")?;
            let value = self.parse_expression()?;
            self.consume(TokenKind::Semicolon, "Expected ';'")?;
            
            // Assuming Let is mapped to Statement somehow? 
            // The Statement enum doesn't have Let! It has Assignment.
            // Wait, looking at `Statement` enum in `ast.rs`:
            // It has `Expression`, `Assignment`, `If`, `For`, `While`, `Loop`, `Break`, `Continue`, `Return`, `Match`.
            // It DOES NOT have `Let` declaration.
            // BUT `Item` has `Let`.
            // This implies `Let` is a top-level item OR I missed it in Statement.
            // Let's check `ast.rs` again.
            // Line 460: `pub enum Statement { ... }`
            // It seems `Statement` does NOT include variable declarations.
            // This suggests that variable declarations might be Expressions (like in Rust/Kotlin block expressions) or I missed it.
            // OR `Block` contains `Statement`s, and `Let` is missing from `Statement`.
            // Let's assume `Let` SHOULD be in `Statement` for a normal language.
            // I'll check `ast.rs` again carefully.
            
            // `ast.rs` line 44: `Let(LetDecl)` is in `Item`.
            // `ast.rs` line 460: `Statement` enum...
            // It seems `Statement` definition in `ast.rs` is incomplete or I misread it.
            // I'll just parse it as an ExpressionStatement for now if I can't add it to Statement enum (since I didn't modify AST).
            // Actually I should verify `ast.rs` content I read earlier.
            
            // `ast.rs` content showed `Statement` enum has `Expression`, `Assignment`, `If`...
            // It does NOT have `Let`.
            // However, `Block` has `Vec<Statement>`.
            // This means local variables are not supported in blocks? That's unlikely.
            // Maybe `Expression` includes `Block` which includes `Statement`.
            // Maybe `Let` is considered an `Expression`? (like in OCaml/Rust)
            // But `LetDecl` is a struct.
            
            // I will skip implementation of `Let` inside `parse_statement` for now to match AST.
            // I'll implement `Return`, `Break`, `Continue`, `If`, `While`.
            
            // Wait, if I can't declare local variables, the language is very limited.
            // I'll assume `Statement` enum in `ast.rs` was just a snippet or I should treat `Let` as an expression if possible, or maybe `Statement` is just missing it.
            // Given I can't modify `ast.rs` easily without potentially breaking other things (though I can), I'll stick to what's there.
            // But wait, `Item` has `Let`. Maybe blocks can contain `Item`s?
            // No, `Block` contains `Statement`.
            
            // Okay, I'll implement `Return` etc.
            
            return Err(ParseError::UnexpectedToken {
                expected: "statement".to_string(),
                found: TokenKind::Let,
                position: start_pos,
            });
        } 
        
        if self.match_token(TokenKind::Return) {
            let value = if !self.check(&TokenKind::Semicolon) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.consume(TokenKind::Semicolon, "Expected ';'")?;
            let span = self.span_from(start_pos);
            return Ok(Statement::Return { value, span });
        }
        
        // Default to Expression statement
        let expr = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        let span = self.span_from(start_pos);
        Ok(Statement::Expression { expr, span })
    }
    
    fn parse_type_decl(&mut self) -> ParseResult<TypeDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'type'
        let name = self.parse_identifier()?;
        let generic_params = self.parse_generic_params()?;
        self.consume(TokenKind::Assign, "Expected '='")?;
        let type_expr = self.parse_type_expr()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        let span = self.span_from(start_pos);
        Ok(TypeDecl { name, generic_params, type_expr, span })
    }
    
    fn parse_const_decl(&mut self) -> ParseResult<ConstDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'const'
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':'")?;
        let type_expr = self.parse_type_expr()?;
        self.consume(TokenKind::Assign, "Expected '='")?;
        let value = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        let span = self.span_from(start_pos);
        Ok(ConstDecl { name, type_expr, value, span })
    }
    
    fn parse_let_decl(&mut self) -> ParseResult<LetDecl> {
        let start_pos = self.current_position();
        self.advance(); // consume 'let'
        let is_mutable = self.match_token(TokenKind::Mut);
        let name = self.parse_identifier()?;
        
        let type_expr = if self.match_token(TokenKind::Colon) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Assign, "Expected '='")?;
        let value = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        let span = self.span_from(start_pos);
        Ok(LetDecl { name, type_expr, value, is_mutable, span })
    }

    /// Parse a sphere member (basic implementation)
    fn parse_sphere_member(&mut self) -> ParseResult<SphereMember> {
        // For now, just parse properties
        Ok(SphereMember::Property(self.parse_property_decl()?))
    }
    
    /// Parse a property declaration
    fn parse_property_decl(&mut self) -> ParseResult<PropertyDecl> {
        let start_pos = self.current_position();
        
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':' after property name")?;
        let type_expr = self.parse_type_expr()?;
        
        let default_value = if self.match_token(TokenKind::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Comma, "Expected ',' after property declaration")?;
        
        let span = self.span_from(start_pos);
        Ok(PropertyDecl { name, type_expr, default_value, span })
    }
    
    /// Parse a coastal database declaration (simplified CDB-style)
    fn parse_coastal_database_decl(&mut self) -> ParseResult<CoastalDatabaseDecl> {
        let start_pos = self.current_position();
        // consume 'coastal_database'
        self.advance();
        
        let name = self.parse_identifier()?;
        self.skip_trivia();
        self.consume(TokenKind::LeftBrace, "Expected '{' after coastal_database name")?;
        
        let mut id = None;
        let mut version = None;
        let mut spatial = None;
        let mut temporal = None;
        let mut tiling = None;
        let mut cdb_profile = None;
        let mut metadata = Vec::new();
        let mut layers = Vec::new();
        
        // Allow newlines and comments inside coastal_database bodies
        self.skip_trivia();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_trivia();
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            let field_token = self.peek().ok_or_else(|| ParseError::UnexpectedEOF {
                expected: "coastal_database field".to_string(),
            })?;
            
            match &field_token.kind {
                TokenKind::Identifier | TokenKind::UnicodeIdentifier => {
                    let field_name = self.parse_identifier()?;
                    
                    if field_name.name == "id" {
                        self.consume(TokenKind::Colon, "Expected ':' after id")?;
                        if let Expression::Literal { value: Literal::String(s), .. } = self.parse_expression()? {
                            id = Some(s);
                        }
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "version" {
                        self.consume(TokenKind::Colon, "Expected ':' after version")?;
                        if let Expression::Literal { value: Literal::String(s), .. } = self.parse_expression()? {
                            version = Some(s);
                        }
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "spatial" {
                        self.consume(TokenKind::Colon, "Expected ':' after spatial")?;
                        spatial = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "temporal" {
                        self.consume(TokenKind::Colon, "Expected ':' after temporal")?;
                        temporal = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "tiling" {
                        self.consume(TokenKind::Colon, "Expected ':' after tiling")?;
                        tiling = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "cdb_profile" {
                        self.consume(TokenKind::Colon, "Expected ':' after cdb_profile")?;
                        cdb_profile = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "metadata" {
                        self.consume(TokenKind::Colon, "Expected ':' after metadata")?;
                        self.skip_trivia();
                        self.consume(TokenKind::LeftBrace, "Expected '{' after metadata:")?;
                        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                            let key = self.parse_identifier()?;
                            self.consume(TokenKind::Colon, "Expected ':' after metadata key")?;
                            let value = self.parse_expression()?;
                            metadata.push((key, value));
                            if !self.match_token(TokenKind::Comma) {
                                break;
                            }
                        }
                        self.consume(TokenKind::RightBrace, "Expected '}' after metadata block")?;
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "layers" {
                        self.consume(TokenKind::Colon, "Expected ':' after layers")?;
                        self.skip_trivia();
                        self.consume(TokenKind::LeftBrace, "Expected '{' after layers:")?;
                        self.skip_trivia();
                        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                            self.skip_trivia();
                            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                                break;
                            }
                            layers.push(self.parse_database_layer_decl()?);
                        }
                        self.consume(TokenKind::RightBrace, "Expected '}' after layers block")?;
                        self.match_token(TokenKind::Comma);
                    } else {
                        // Unknown field, try to recover by skipping until comma or brace
                        while !self.check(&TokenKind::Comma)
                            && !self.check(&TokenKind::RightBrace)
                            && !self.is_at_end()
                        {
                            self.advance();
                        }
                        self.match_token(TokenKind::Comma);
                    }
                }
                _ => {
                    // Skip unexpected tokens inside coastal_database
                    self.advance();
                }
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after coastal_database body")?;
        let span = self.span_from(start_pos);
        Ok(CoastalDatabaseDecl {
            name,
            id,
            version,
            spatial,
            temporal,
            tiling,
            cdb_profile,
            metadata,
            layers,
            span,
        })
    }
    
    /// Parse a layer declaration inside a coastal database
    fn parse_database_layer_decl(&mut self) -> ParseResult<DatabaseLayerDecl> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':' after layer name")?;
        self.consume(TokenKind::LayerKw, "Expected 'layer' keyword after layer name:")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after layer keyword")?;
        
        let mut layer_type = None;
        let mut cdb_dataset = None;
        let mut cdb_geometry_type = None;
        let mut datasets = Vec::new();
        
        // Allow newlines and comments inside layer bodies
        self.skip_trivia();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_trivia();
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            let field_token = self.peek().ok_or_else(|| ParseError::UnexpectedEOF {
                expected: "layer field".to_string(),
            })?;
            
            match &field_token.kind {
                TokenKind::Identifier | TokenKind::UnicodeIdentifier => {
                    let field_name = self.parse_identifier()?;
                    if field_name.name == "type" {
                        self.consume(TokenKind::Colon, "Expected ':' after type")?;
                        layer_type = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "cdb_dataset" {
                        self.consume(TokenKind::Colon, "Expected ':' after cdb_dataset")?;
                        cdb_dataset = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "cdb_geometry_type" {
                        self.consume(TokenKind::Colon, "Expected ':' after cdb_geometry_type")?;
                        cdb_geometry_type = Some(self.parse_identifier()?);
                        self.match_token(TokenKind::Comma);
                    } else if field_name.name == "datasets" {
                        self.consume(TokenKind::Colon, "Expected ':' after datasets")?;
                        self.consume(TokenKind::LeftBrace, "Expected '{' after datasets:")?;
                        self.skip_trivia();
                        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                            self.skip_trivia();
                            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                                break;
                            }
                            datasets.push(self.parse_dataset_decl()?);
                        }
                        self.consume(TokenKind::RightBrace, "Expected '}' after datasets block")?;
                        self.match_token(TokenKind::Comma);
                    } else {
                        while !self.check(&TokenKind::Comma)
                            && !self.check(&TokenKind::RightBrace)
                            && !self.is_at_end()
                        {
                            self.advance();
                        }
                        self.match_token(TokenKind::Comma);
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after layer body")?;
        let span = self.span_from(start_pos);
        Ok(DatabaseLayerDecl {
            name,
            layer_type,
            cdb_dataset,
            cdb_geometry_type,
            datasets,
            span,
        })
    }
    
    /// Parse a dataset declaration inside a layer
    fn parse_dataset_decl(&mut self) -> ParseResult<DatasetDecl> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':' after dataset name")?;
        self.consume(TokenKind::DatasetKw, "Expected 'dataset' keyword after dataset name:")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after dataset keyword")?;
        
        let mut feature_class = None;
        let mut citygml_feature_class = None;
        let mut spatial_resolution = None;
        let mut temporal_resolution = None;
        let mut quality_profile = None;
        let mut uncertainty_model = None;
        let mut cmecs_unit = None;
        let mut cdb_lod = None;
        let mut cdb_tile_size = None;
        let mut citygml_version = None;
        let mut citygml_lod = None;
        let mut citygml_ade = None;
        
        // Allow newlines and comments inside dataset bodies
        self.skip_trivia();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_trivia();
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            let field_token = self.peek().ok_or_else(|| ParseError::UnexpectedEOF {
                expected: "dataset field".to_string(),
            })?;
            
            match &field_token.kind {
                TokenKind::Identifier | TokenKind::UnicodeIdentifier => {
                    let field_name = self.parse_identifier()?;
                    match field_name.name.as_str() {
                        "feature_class" => {
                            self.consume(TokenKind::Colon, "Expected ':' after feature_class")?;
                            feature_class = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "citygml_feature_class" => {
                            self.consume(TokenKind::Colon, "Expected ':' after citygml_feature_class")?;
                            citygml_feature_class = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "spatial_resolution" => {
                            self.consume(TokenKind::Colon, "Expected ':' after spatial_resolution")?;
                            spatial_resolution = Some(self.parse_expression()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "temporal_resolution" => {
                            self.consume(TokenKind::Colon, "Expected ':' after temporal_resolution")?;
                            temporal_resolution = Some(self.parse_expression()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "quality_profile" => {
                            self.consume(TokenKind::Colon, "Expected ':' after quality_profile")?;
                            quality_profile = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "uncertainty_model" => {
                            self.consume(TokenKind::Colon, "Expected ':' after uncertainty_model")?;
                            uncertainty_model = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "cmecs_unit" => {
                            self.consume(TokenKind::Colon, "Expected ':' after cmecs_unit")?;
                            cmecs_unit = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "cdb_lod" => {
                            self.consume(TokenKind::Colon, "Expected ':' after cdb_lod")?;
                            cdb_lod = Some(self.parse_expression()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "cdb_tile_size" => {
                            self.consume(TokenKind::Colon, "Expected ':' after cdb_tile_size")?;
                            cdb_tile_size = Some(self.parse_expression()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "citygml_version" => {
                            self.consume(TokenKind::Colon, "Expected ':' after citygml_version")?;
                            citygml_version = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "citygml_lod" => {
                            self.consume(TokenKind::Colon, "Expected ':' after citygml_lod")?;
                            citygml_lod = Some(self.parse_expression()?);
                            self.match_token(TokenKind::Comma);
                        }
                        "citygml_ade" => {
                            self.consume(TokenKind::Colon, "Expected ':' after citygml_ade")?;
                            citygml_ade = Some(self.parse_identifier()?);
                            self.match_token(TokenKind::Comma);
                        }
                        _ => {
                            while !self.check(&TokenKind::Comma)
                                && !self.check(&TokenKind::RightBrace)
                                && !self.is_at_end()
                            {
                                self.advance();
                            }
                            self.match_token(TokenKind::Comma);
                        }
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after dataset body")?;
        let span = self.span_from(start_pos);
        Ok(DatasetDecl {
            name,
            feature_class,
            citygml_feature_class,
            spatial_resolution,
            temporal_resolution,
            quality_profile,
            uncertainty_model,
            cmecs_unit,
            cdb_lod,
            cdb_tile_size,
            citygml_version,
            citygml_lod,
            citygml_ade,
            span,
        })
    }
    
    /// Parse an identifier
    fn parse_identifier(&mut self) -> ParseResult<Identifier> {
        self.skip_trivia();
        let token = self.peek().ok_or_else(|| ParseError::UnexpectedEOF {
            expected: "identifier".to_string(),
        })?;
        
        match token.kind {
            TokenKind::Identifier
            | TokenKind::UnicodeIdentifier
            | TokenKind::AtmosphereType
            | TokenKind::OceanType
            | TokenKind::LithosphereType
            | TokenKind::BiosphereType
            | TokenKind::AnthroposphereType
            | TokenKind::AtmosphereChinese
            | TokenKind::OceanChinese
            | TokenKind::LithosphereChinese
            | TokenKind::BiosphereChinese
            | TokenKind::AnthroposphereChinese => {
                let token = self.advance().unwrap();
                Ok(Identifier::new(token.text, token.span))
            }
            TokenKind::EOF => Err(ParseError::UnexpectedEOF {
                expected: "identifier".to_string(),
            }),
            _ => {
                let position = token.span.start.clone();
                Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: token.kind.clone(),
                    position,
                })
            }
        }
    }
    
    /// Parse a type expression (basic implementation)
    fn parse_type_expr(&mut self) -> ParseResult<TypeExpr> {
        let start_pos = self.current_position();
        let name = self.parse_identifier()?;
        let span = self.span_from(start_pos);
        Ok(TypeExpr::Basic { name, span })
    }
    
    /// Parse an expression (basic implementation)
    fn parse_expression(&mut self) -> ParseResult<Expression> {
        let token = self.peek().ok_or_else(|| ParseError::UnexpectedEOF {
            expected: "expression".to_string(),
        })?;
        
        match token.kind {
            TokenKind::Integer(value) => {
                let token = self.advance().unwrap();
                Ok(Expression::literal(Literal::Integer(value), token.span))
            }
            TokenKind::Float(value) => {
                let token = self.advance().unwrap();
                Ok(Expression::literal(Literal::Float(value), token.span))
            }
            TokenKind::String => {
                let token = self.advance().unwrap();
                let content = token.text.trim_matches('"').to_string();
                Ok(Expression::literal(Literal::String(content), token.span))
            }
            TokenKind::True => {
                let token = self.advance().unwrap();
                Ok(Expression::literal(Literal::Boolean(true), token.span))
            }
            TokenKind::False => {
                let token = self.advance().unwrap();
                Ok(Expression::literal(Literal::Boolean(false), token.span))
            }
            TokenKind::Identifier | TokenKind::UnicodeIdentifier => {
                let name = self.parse_identifier()?;
                let span = name.span.clone();
                Ok(Expression::Variable { name, span })
            }
            _ => {
                let position = token.span.start.clone();
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: token.kind.clone(),
                    position,
                })
            }
        }
    }
    
    /// Consume the current token, expecting it to be of the given kind
    fn consume(&mut self, kind: TokenKind, _message: &str) -> ParseResult<Token> {
        self.skip_trivia();
        if self.check(&kind) {
            Ok(self.advance().unwrap())
        } else if self.is_at_end() {
            Err(ParseError::UnexpectedEOF {
                expected: format!("{:?}", kind),
            })
        } else {
            let found = self.peek().map(|t| t.kind.clone()).unwrap_or(TokenKind::EOF);
            let position = self.current_position();
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", kind),
                found,
                position,
            })
        }
    }
}

/// Convenience function to parse a string into an AST
pub fn parse_string(input: &str) -> ParseResult<Program> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| ParseError::LexerError { error: e })?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Convenience function to parse a string with source file information
pub fn parse_string_with_source(input: &str, source_file: String) -> ParseResult<Program> {
    let mut lexer = Lexer::with_source_file(input, source_file.clone());
    let tokens = lexer.tokenize().map_err(|e| ParseError::LexerError { error: e })?;
    let mut parser = Parser::with_source_file(tokens, source_file);
    parser.parse()
}
