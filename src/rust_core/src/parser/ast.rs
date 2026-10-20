//! Abstract Syntax Tree (AST) Definitions for CZ-DSL
//! 
//! This module defines all AST node types for the Coastal Zone Domain-Specific Language.
//! The AST represents the parsed structure of CZ-DSL programs and serves as input
//! to the semantic analyzer and code generator.

use crate::lexer::{Position, Span};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete CZ-DSL program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
    pub span: Span,
}

/// Top-level items in a CZ-DSL program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    /// Sphere declaration: sphere 大气圈 { ... }
    Sphere(SphereDecl),
    /// Scale declaration: scale Spatial { ... }
    Scale(ScaleDecl),
    /// Data declaration: data CoastalObservation { ... }
    Data(DataDecl),
    /// Model declaration: model CoastalSystemModel { ... }
    Model(ModelDecl),
    /// Process declaration: process 潮汐过程 { ... }
    Process(ProcessDecl),
    /// Interface declaration: interface ModelInterface { ... }
    Interface(InterfaceDecl),
    /// AI model declaration: ai_model WavePredictor { ... }
    AiModel(AiModelDecl),
    /// Digital twin declaration: digital_twin CoastalTwin { ... }
    DigitalTwin(DigitalTwinDecl),
    /// Function declaration: fn calculate_tide() { ... }
    Function(FunctionDecl),
    /// Type declaration: type Temperature = Real with unit kelvin;
    Type(TypeDecl),
    /// Constant declaration: const GRAVITY: Real = 9.81;
    Const(ConstDecl),
    /// Variable declaration: let mut temperature: Real;
    Let(LetDecl),
    /// Constraint declaration: constraint ConservationOfMass { ... }
    Constraint(ConstraintDecl),
    /// Coastal database declaration: coastal_database HangzhouBayDB { ... }
    CoastalDatabase(CoastalDatabaseDecl),
}

/// Coastal database declaration (CDB-style CZ-DB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoastalDatabaseDecl {
    pub name: Identifier,
    pub id: Option<String>,
    pub version: Option<String>,
    pub spatial: Option<Identifier>,
    pub temporal: Option<Identifier>,
    pub tiling: Option<Identifier>,
    pub cdb_profile: Option<Identifier>,
    pub metadata: Vec<(Identifier, Expression)>,
    pub layers: Vec<DatabaseLayerDecl>,
    pub span: Span,
}

/// Layer declaration inside a coastal database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseLayerDecl {
    pub name: Identifier,
    pub layer_type: Option<Identifier>,
    pub cdb_dataset: Option<Identifier>,
    pub cdb_geometry_type: Option<Identifier>,
    pub datasets: Vec<DatasetDecl>,
    pub span: Span,
}

/// Dataset declaration inside a layer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetDecl {
    pub name: Identifier,
    pub feature_class: Option<Identifier>,
    pub citygml_feature_class: Option<Identifier>,
    pub spatial_resolution: Option<Expression>,
    pub temporal_resolution: Option<Expression>,
    pub quality_profile: Option<Identifier>,
    pub uncertainty_model: Option<Identifier>,
    pub cmecs_unit: Option<Identifier>,
    pub cdb_lod: Option<Expression>,
    pub cdb_tile_size: Option<Expression>,
    pub citygml_version: Option<Identifier>,
    pub citygml_lod: Option<Expression>,
    pub citygml_ade: Option<Identifier>,
    pub span: Span,
}

/// Sphere declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SphereDecl {
    pub name: Identifier,
    pub extends: Option<TypeExpr>,
    pub members: Vec<SphereMember>,
    pub span: Span,
}

/// Members of a sphere declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SphereMember {
    /// Property: temperature: Real with unit kelvin
    Property(PropertyDecl),
    /// Process: evaporation: Process<Water -> WaterVapor>
    Process(ProcessDecl),
    /// Boundary: surface: Interface<Atmosphere, Ocean>
    Boundary(BoundaryDecl),
    /// Constraint: constraint MassConservation { ... }
    Constraint(ConstraintDecl),
}

/// Property declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyDecl {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub default_value: Option<Expression>,
    pub span: Span,
}

/// Boundary declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryDecl {
    pub name: Identifier,
    pub interface_type: TypeExpr,
    pub span: Span,
}

/// Scale declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleDecl {
    pub name: Identifier,
    pub ranges: Vec<ScaleRange>,
    pub span: Span,
}

/// Scale range definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleRange {
    pub name: Identifier,
    pub range: RangeExpr,
    pub unit: Option<UnitExpr>,
    pub span: Span,
}

/// Data declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<MethodDecl>,
    pub constraints: Vec<ConstraintDecl>,
    pub span: Span,
}

/// Field declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDecl {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub default_value: Option<Expression>,
    pub span: Span,
}

/// Model declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub parameters: Vec<ParameterDecl>,
    pub variables: Vec<VariableDecl>,
    pub equations: Vec<EquationDecl>,
    pub constraints: Vec<ConstraintDecl>,
    pub couplings: Vec<CouplingDecl>,
    pub span: Span,
}

/// Parameter declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDecl {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub default_value: Option<Expression>,
    pub span: Span,
}

/// Variable declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDecl {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub initial_value: Option<Expression>,
    pub span: Span,
}

/// Equation declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquationDecl {
    pub name: Identifier,
    pub expression: Expression,
    pub span: Span,
}

/// Coupling declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CouplingDecl {
    pub name: Identifier,
    pub source: Identifier,
    pub target: Identifier,
    pub coupling_type: CouplingType,
    pub span: Span,
}

/// Types of coupling between models/processes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CouplingType {
    /// One-way coupling
    OneWay,
    /// Two-way coupling
    TwoWay,
    /// Feedback coupling
    Feedback,
}

/// Process declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub inputs: Vec<IoDecl>,
    pub outputs: Vec<IoDecl>,
    pub parameters: Vec<ParameterDecl>,
    pub constraints: Vec<ConstraintDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

/// Input/Output declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IoDecl {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub span: Span,
}

/// Interface declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub methods: Vec<MethodSignature>,
    pub span: Span,
}

/// Method signature (for interfaces)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodSignature {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpr>,
    pub span: Span,
}

/// AI model declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiModelDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub architecture: Option<Expression>,
    pub training_data: Option<TypeExpr>,
    pub physics_constraints: Vec<Expression>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

/// Digital twin declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinDecl {
    pub name: Identifier,
    pub physical_system: TypeExpr,
    pub digital_replica: TypeExpr,
    pub assimilation: Option<TypeExpr>,
    pub simulation: Option<TypeExpr>,
    pub control: Option<TypeExpr>,
    pub span: Span,
}

/// Function declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpr>,
    pub body: Option<Block>,
    pub span: Span,
}

/// Method declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpr>,
    pub body: Option<Block>,
    pub is_override: bool,
    pub span: Span,
}

/// Function/method parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub default_value: Option<Expression>,
    pub span: Span,
}

/// Type declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub type_expr: TypeExpr,
    pub span: Span,
}

/// Constant declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstDecl {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub value: Expression,
    pub span: Span,
}

/// Variable declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetDecl {
    pub name: Identifier,
    pub type_expr: Option<TypeExpr>,
    pub value: Expression,
    pub is_mutable: bool,
    pub span: Span,
}

/// Constraint declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintDecl {
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub body: ConstraintBody,
    pub span: Span,
}

/// Constraint body
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintBody {
    /// Quantified expression: ∀ x: Real => x > 0
    Quantified {
        quantifier: Quantifier,
        variable: Identifier,
        variable_type: TypeExpr,
        condition: Expression,
    },
    /// Simple expression
    Expression(Expression),
}

/// Quantifiers for constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Quantifier {
    /// Universal quantifier (∀, forall)
    ForAll,
    /// Existential quantifier (∃, exists)
    Exists,
}

/// Generic parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParam {
    pub name: Identifier,
    pub bounds: Vec<TypeExpr>,
    pub span: Span,
}

/// Type expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpr {
    /// Basic type: Real, Integer, Boolean, etc.
    Basic {
        name: Identifier,
        span: Span,
    },
    /// Generic type: Vector<Real>
    Generic {
        name: Identifier,
        args: Vec<TypeExpr>,
        span: Span,
    },
    /// Function type: (Real, Real) -> Real
    Function {
        params: Vec<TypeExpr>,
        return_type: Box<TypeExpr>,
        span: Span,
    },
    /// Tuple type: (Real, Integer, String)
    Tuple {
        elements: Vec<TypeExpr>,
        span: Span,
    },
    /// Array type: [Real] or [Real; 10]
    Array {
        element_type: Box<TypeExpr>,
        size: Option<Expression>,
        span: Span,
    },
    /// Field type: ScalarField<Real>
    Field {
        field_type: FieldType,
        element_type: Box<TypeExpr>,
        span: Span,
    },
    /// Uncertain type: Uncertain<Real>
    Uncertain {
        inner_type: Box<TypeExpr>,
        span: Span,
    },
    /// Type with unit: Real with unit kelvin
    WithUnit {
        base_type: Box<TypeExpr>,
        unit: UnitExpr,
        span: Span,
    },
}

/// Field types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    Scalar,
    Vector,
    Tensor,
}

/// Unit expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnitExpr {
    /// Basic unit: meter, kelvin, etc.
    Basic {
        name: Identifier,
        span: Span,
    },
    /// Compound unit: m/s, kg*m/s^2
    Compound {
        terms: Vec<UnitTerm>,
        span: Span,
    },
}

/// Unit term in compound units
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitTerm {
    pub unit: Identifier,
    pub exponent: i32,
    pub in_numerator: bool, // true for numerator, false for denominator
    pub span: Span,
}

/// Range expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RangeExpr {
    /// Exclusive range: 1..10
    Exclusive {
        start: Box<Expression>,
        end: Box<Expression>,
        span: Span,
    },
    /// Inclusive range: 1..=10
    Inclusive {
        start: Box<Expression>,
        end: Box<Expression>,
        span: Span,
    },
    /// Open start: ..10
    ToEnd {
        end: Box<Expression>,
        span: Span,
    },
    /// Open end: 1..
    FromStart {
        start: Box<Expression>,
        span: Span,
    },
}

/// Statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Expression statement: expression;
    Expression {
        expr: Expression,
        span: Span,
    },
    /// Assignment: variable = expression;
    Assignment {
        lvalue: LValue,
        op: AssignmentOp,
        rvalue: Expression,
        span: Span,
    },
    /// If statement: if condition { ... } else { ... }
    If {
        condition: Expression,
        then_block: Block,
        else_block: Option<Block>,
        span: Span,
    },
    /// For loop: for item in iterable { ... }
    For {
        variable: Identifier,
        iterable: Expression,
        body: Block,
        span: Span,
    },
    /// While loop: while condition { ... }
    While {
        condition: Expression,
        body: Block,
        span: Span,
    },
    /// Loop: loop { ... }
    Loop {
        body: Block,
        span: Span,
    },
    /// Break: break [expression];
    Break {
        value: Option<Expression>,
        span: Span,
    },
    /// Continue: continue;
    Continue {
        span: Span,
    },
    /// Return: return [expression];
    Return {
        value: Option<Expression>,
        span: Span,
    },
    /// Match: match expression { ... }
    Match {
        expr: Expression,
        arms: Vec<MatchArm>,
        span: Span,
    },
}

/// Assignment operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssignmentOp {
    Assign,      // =
    AddAssign,   // +=
    SubAssign,   // -=
    MulAssign,   // *=
    DivAssign,   // /=
    ModAssign,   // %=
}

/// Left-hand values for assignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LValue {
    /// Variable: identifier
    Variable {
        name: Identifier,
        span: Span,
    },
    /// Field access: object.field
    FieldAccess {
        object: Box<LValue>,
        field: Identifier,
        span: Span,
    },
    /// Index access: array[index]
    IndexAccess {
        object: Box<LValue>,
        index: Expression,
        span: Span,
    },
}

/// Match arm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
    pub span: Span,
}

/// Patterns for match expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    /// Literal pattern: 42, "hello"
    Literal {
        value: Literal,
        span: Span,
    },
    /// Variable pattern: x
    Variable {
        name: Identifier,
        span: Span,
    },
    /// Wildcard pattern: _
    Wildcard {
        span: Span,
    },
    /// Tuple pattern: (x, y, z)
    Tuple {
        elements: Vec<Pattern>,
        span: Span,
    },
    /// Array pattern: [x, y, z]
    Array {
        elements: Vec<Pattern>,
        span: Span,
    },
    /// Struct pattern: Point { x, y }
    Struct {
        name: Identifier,
        fields: Vec<FieldPattern>,
        span: Span,
    },
}

/// Field pattern in struct patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldPattern {
    pub name: Identifier,
    pub pattern: Pattern,
    pub span: Span,
}

/// Block of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Literal: 42, "hello", true
    Literal {
        value: Literal,
        span: Span,
    },
    /// Variable reference: identifier
    Variable {
        name: Identifier,
        span: Span,
    },
    /// Binary operation: left op right
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
        span: Span,
    },
    /// Unary operation: op operand
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
        span: Span,
    },
    /// Function call: function(args)
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
        span: Span,
    },
    /// Method call: object.method(args)
    MethodCall {
        object: Box<Expression>,
        method: Identifier,
        args: Vec<Expression>,
        span: Span,
    },
    /// Field access: object.field
    FieldAccess {
        object: Box<Expression>,
        field: Identifier,
        span: Span,
    },
    /// Index access: array[index]
    IndexAccess {
        object: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },
    /// Array literal: [1, 2, 3]
    Array {
        elements: Vec<Expression>,
        span: Span,
    },
    /// Tuple literal: (1, 2, 3)
    Tuple {
        elements: Vec<Expression>,
        span: Span,
    },
    /// Range: 1..10, 1..=10
    Range {
        range: Box<RangeExpr>,
        span: Span,
    },
    /// Lambda: |x, y| x + y
    Lambda {
        params: Vec<Parameter>,
        body: Box<Expression>,
        span: Span,
    },
    /// Block expression: { statements; expression }
    Block {
        block: Block,
        span: Span,
    },
    /// If expression: if condition { then } else { else }
    If {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Option<Box<Expression>>,
        span: Span,
    },
    /// Match expression: match expr { arms }
    Match {
        expr: Box<Expression>,
        arms: Vec<MatchArm>,
        span: Span,
    },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Arithmetic
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    Modulo,     // %
    Power,      // **
    
    // Comparison
    Equal,          // ==
    NotEqual,       // !=, ≠
    Less,           // <
    LessEqual,      // <=, ≤
    Greater,        // >
    GreaterEqual,   // >=, ≥
    
    // Logical
    And,        // &&
    Or,         // ||
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Plus,       // +
    Minus,      // -
    Not,        // !
    Tilde,      // ~ (reverse)
    Dagger,     // † (conjugate)
    Nabla,      // ∇ (gradient)
    Partial,    // ∂ (partial derivative)
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// Integer: 42
    Integer(i64),
    /// Float: 3.14
    Float(f64),
    /// Complex: 3+4i
    Complex { real: f64, imag: f64 },
    /// String: "hello"
    String(String),
    /// Character: 'a'
    Char(char),
    /// Boolean: true, false
    Boolean(bool),
    /// Unit: ()
    Unit,
}

/// Identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Utility functions for AST construction
impl Program {
    pub fn new(items: Vec<Item>, span: Span) -> Self {
        Self { items, span }
    }
}

impl Expression {
    /// Create a literal expression
    pub fn literal(value: Literal, span: Span) -> Self {
        Expression::Literal { value, span }
    }
    
    /// Create a variable reference
    pub fn variable(name: String, span: Span) -> Self {
        Expression::Variable {
            name: Identifier::new(name, span.clone()),
            span,
        }
    }
    
    /// Create a binary operation
    pub fn binary(left: Expression, op: BinaryOp, right: Expression, span: Span) -> Self {
        Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
            span,
        }
    }
    
    /// Create a unary operation
    pub fn unary(op: UnaryOp, operand: Expression, span: Span) -> Self {
        Expression::Unary {
            op,
            operand: Box::new(operand),
            span,
        }
    }
    
    /// Get the span of this expression
    pub fn span(&self) -> &Span {
        match self {
            Expression::Literal { span, .. } => span,
            Expression::Variable { span, .. } => span,
            Expression::Binary { span, .. } => span,
            Expression::Unary { span, .. } => span,
            Expression::Call { span, .. } => span,
            Expression::MethodCall { span, .. } => span,
            Expression::FieldAccess { span, .. } => span,
            Expression::IndexAccess { span, .. } => span,
            Expression::Array { span, .. } => span,
            Expression::Tuple { span, .. } => span,
            Expression::Range { span, .. } => span,
            Expression::Lambda { span, .. } => span,
            Expression::Block { span, .. } => span,
            Expression::If { span, .. } => span,
            Expression::Match { span, .. } => span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Position;
    
    #[test]
    fn test_identifier_creation() {
        let span = Span {
            start: Position::new(1, 1, 0),
            end: Position::new(1, 5, 4),
        };
        let id = Identifier::new("test".to_string(), span.clone());
        assert_eq!(id.name, "test");
        assert_eq!(id.span, span);
    }
    
    #[test]
    fn test_expression_creation() {
        let span = Span {
            start: Position::new(1, 1, 0),
            end: Position::new(1, 3, 2),
        };
        
        let expr = Expression::literal(Literal::Integer(42), span.clone());
        match expr {
            Expression::Literal { value: Literal::Integer(42), .. } => {},
            _ => panic!("Expected integer literal"),
        }
    }
    
    #[test]
    fn test_binary_expression() {
        let span = Span {
            start: Position::new(1, 1, 0),
            end: Position::new(1, 5, 4),
        };
        
        let left = Expression::literal(Literal::Integer(1), span.clone());
        let right = Expression::literal(Literal::Integer(2), span.clone());
        let expr = Expression::binary(left, BinaryOp::Add, right, span);
        
        match expr {
            Expression::Binary { op: BinaryOp::Add, .. } => {},
            _ => panic!("Expected binary add expression"),
        }
    }
}
