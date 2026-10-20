//! Token definitions for CZ-DSL
//! 
//! Defines all token types including keywords, operators, literals, and identifiers.
//! Supports Chinese identifiers and physical units as first-class citizens.

use logos::Logos;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Position information for tokens
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Span information for tokens
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// Token with position information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String,
}

/// All token types in CZ-DSL
#[derive(Logos, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    // === Keywords ===
    
    // Core language constructs
    #[token("sphere")]
    Sphere,
    #[token("scale")]
    Scale,
    #[token("data")]
    Data,
    #[token("model")]
    Model,
    #[token("process")]
    Process,
    #[token("interface")]
    Interface,
    #[token("coupling")]
    Coupling,
    #[token("coastal_database")]
    CoastalDatabase,
    #[token("spatial_domain")]
    SpatialDomainDeclKw,
    #[token("temporal_domain")]
    TemporalDomainDeclKw,
    #[token("tiling_scheme")]
    TilingSchemeDeclKw,
    #[token("layer")]
    LayerKw,
    #[token("dataset")]
    DatasetKw,
    #[token("feature_class")]
    FeatureClassKw,
    #[token("quality_profile")]
    QualityProfileKw,
    #[token("uncertainty_model")]
    UncertaintyModelKw,
    #[token("world_model")]
    WorldModel,
    #[token("compute_context")]
    ComputeContext,
    #[token("ai_model")]
    AiModel,
    #[token("digital_twin")]
    DigitalTwin,
    
    // Sphere types
    #[token("Atmosphere")]
    AtmosphereType,
    #[token("Ocean")]
    OceanType,
    #[token("Lithosphere")]
    LithosphereType,
    #[token("Biosphere")]
    BiosphereType,
    #[token("Anthroposphere")]
    AnthroposphereType,
    
    // Scale keywords
    #[token("spatial")]
    Spatial,
    #[token("temporal")]
    Temporal,
    #[token("molecular")]
    Molecular,
    #[token("cellular")]
    Cellular,
    #[token("organism")]
    Organism,
    #[token("ecosystem")]
    Ecosystem,
    #[token("regional")]
    Regional,
    #[token("global")]
    Global,
    
    // Data types
    #[token("observation")]
    Observation,
    #[token("simulation")]
    Simulation,
    #[token("remote_sensing")]
    RemoteSensing,
    #[token("field")]
    Field,
    #[token("time_series")]
    TimeSeries,
    #[token("uncertain")]
    Uncertain,
    
    // Control flow
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("match")]
    Match,
    #[token("loop")]
    Loop,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    
    // Declarations
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("type")]
    Type,
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("class")]
    Class,
    #[token("abstract")]
    Abstract,
    #[token("extends")]
    Extends,
    #[token("implements")]
    Implements,
    
    // Modifiers
    #[token("pub")]
    Pub,
    #[token("private")]
    Private,
    #[token("static")]
    Static,
    #[token("mut")]
    Mut,
    #[token("ref")]
    Ref,
    
    // Functions and methods
    #[token("fn")]
    Fn,
    #[token("method")]
    Method,
    #[token("override")]
    Override,
    #[token("auto")]
    Auto,
    
    // Constraints and properties
    #[token("constraint")]
    Constraint,
    #[token("property")]
    Property,
    #[token("invariant")]
    Invariant,
    #[token("requires")]
    Requires,
    #[token("ensures")]
    Ensures,
    
    // Physical units and dimensions
    #[token("with")]
    With,
    #[token("unit")]
    Unit,
    #[token("dimension")]
    Dimension,
    
    // Chinese keywords (海岸带领域专用)
    #[token("圈层")]
    SphereChinese,
    #[token("尺度")]
    ScaleChinese,
    #[token("数据")]
    DataChinese,
    #[token("模型")]
    ModelChinese,
    #[token("过程")]
    ProcessChinese,
    #[token("耦合")]
    CouplingChinese,
    #[token("大气圈")]
    AtmosphereChinese,
    #[token("海洋圈")]
    OceanChinese,
    #[token("岩石圈")]
    LithosphereChinese,
    #[token("生物圈")]
    BiosphereChinese,
    #[token("人类圈")]
    AnthroposphereChinese,
    #[token("时空")]
    SpatioTemporalChinese,
    #[token("物理量")]
    PhysicalQuantityChinese,
    #[token("不确定性")]
    UncertaintyChinese,
    
    // === Operators ===
    
    // Arithmetic operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("**")]
    Power,
    
    // Reserved for future algebra extensions (intentionally left empty)
    
    // Mathematical symbols
    #[token("∀")]  // Universal quantifier (for all)
    ForAll,
    #[token("∃")]  // Existential quantifier (there exists)
    Exists,
    #[token("∇")]  // Nabla (gradient)
    Nabla,
    #[token("∂")]  // Partial derivative
    Partial,
    #[token("∫")]  // Integral
    Integral,
    #[token("∑")]  // Summation
    Sum,
    #[token("∏")]  // Product
    Product,
    #[token("√")]  // Square root
    Sqrt,
    #[token("∞")]  // Infinity
    Infinity,
    #[token("≈")]  // Approximately equal
    Approx,
    #[token("≠")]  // Not equal (alternative)
    NotEqualAlt,
    #[token("≤")]  // Less than or equal (alternative)
    LessEqualAlt,
    #[token("≥")]  // Greater than or equal (alternative)
    GreaterEqualAlt,
    #[token("∈")]  // Element of
    ElementOf,
    #[token("∉")]  // Not element of
    NotElementOf,
    #[token("⊂")]  // Subset of
    SubsetOf,
    #[token("⊃")]  // Superset of
    SupersetOf,
    #[token("∪")]  // Union
    Union,
    #[token("∩")]  // Intersection
    Intersection,
    #[token("α")]  // Greek alpha
    Alpha,
    #[token("β")]  // Greek beta
    Beta,
    #[token("γ")]  // Greek gamma
    Gamma,
    #[token("δ")]  // Greek delta
    Delta,
    #[token("ε")]  // Greek epsilon
    Epsilon,
    #[token("ζ")]  // Greek zeta
    Zeta,
    #[token("η")]  // Greek eta
    Eta,
    #[token("θ")]  // Greek theta
    Theta,
    #[token("λ")]  // Greek lambda
    Lambda,
    #[token("μ")]  // Greek mu
    Mu,
    #[token("ν")]  // Greek nu
    Nu,
    #[token("π")]  // Greek pi
    Pi,
    #[token("ρ")]  // Greek rho
    Rho,
    #[token("σ")]  // Greek sigma
    Sigma,
    #[token("τ")]  // Greek tau
    Tau,
    #[token("φ")]  // Greek phi
    Phi,
    #[token("χ")]  // Greek chi
    Chi,
    #[token("ψ")]  // Greek psi
    Psi,
    #[token("ω")]  // Greek omega
    Omega,
    
    // Comparison operators
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<=>")]
    Spaceship,
    
    // Logical operators
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("^")]
    Xor,
    
    // Assignment operators
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    StarAssign,
    #[token("/=")]
    SlashAssign,
    #[token("%=")]
    PercentAssign,
    
    // Range operators
    #[token("..")]
    Range,
    #[token("..=")]
    RangeInclusive,
    #[token("...")]
    Ellipsis,
    
    // Arrow operators
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("<-")]
    LeftArrow,
    
    // Type operators
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("?")]
    Question,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    
    // === Delimiters ===
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("⟨")]
    LeftAngle,
    #[token("⟩")]  // Right angle bracket
    RightAngle,
    
    // Separators
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Period,
    #[token("@")]
    At,
    #[token("#", priority = 4)]
    Hash,
    #[token("$")]
    Dollar,
    
    // === Literals ===
    
    // Numeric literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Integer(i64),
    
    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),
    
    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Scientific(f64),
    
    // Complex numbers
    #[regex(r"[0-9]+(\.[0-9]+)?[+-][0-9]+(\.[0-9]+)?i")]
    Complex,
    
    // String literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    
    #[regex(r"'([^'\\]|\\.)*'")]
    Char,
    
    // Raw strings
    #[regex(r#"r"([^"])*""#)]
    RawString,
    
    // Boolean literals
    #[token("true")]
    True,
    #[token("false")]
    False,
    
    // Unit literal
    #[token("()")]
    Unit_,
    
    // === Physical Units ===
    
    // Base SI units
    #[token("meter", priority = 5)]
    #[token("m", priority = 5)]
    Meter,
    
    #[token("kilogram", priority = 5)]
    #[token("kg", priority = 5)]
    Kilogram,
    
    #[token("second", priority = 5)]
    #[token("s", priority = 5)]
    Second,
    
    #[token("ampere", priority = 5)]
    #[token("A", priority = 5)]
    Ampere,
    
    #[token("kelvin", priority = 5)]
    #[token("K", priority = 5)]
    Kelvin,
    
    #[token("mole", priority = 5)]
    #[token("mol", priority = 5)]
    Mole,
    
    #[token("candela", priority = 5)]
    #[token("cd", priority = 5)]
    Candela,
    
    // Derived units common in coastal zone science
    #[token("pascal", priority = 5)]
    #[token("Pa", priority = 5)]
    Pascal,
    
    #[token("newton", priority = 5)]
    #[token("N", priority = 5)]
    Newton,
    
    #[token("joule", priority = 5)]
    #[token("J", priority = 5)]
    Joule,
    
    #[token("watt", priority = 5)]
    #[token("W", priority = 5)]
    Watt,
    
    #[token("celsius")]
    #[token("°C")]
    Celsius,
    
    // Oceanographic units
    #[token("psu")]  // Practical Salinity Unit
    PSU,
    
    #[token("dbar")]  // Decibar
    Decibar,
    
    #[token("sverdrup")]
    #[token("Sv")]
    Sverdrup,
    
    // Atmospheric units
    #[token("hPa")]
    Hectopascal,
    
    #[token("atm")]
    Atmosphere,
    
    #[token("mmHg")]
    MillimeterMercury,
    
    // Geographic units
    #[token("degree")]
    #[token("deg")]
    #[token("°")]
    Degree,
    
    #[token("radian")]
    #[token("rad")]
    Radian,
    
    #[token("nautical_mile")]
    #[token("nmi")]
    NauticalMile,
    
    // Time units
    #[token("minute", priority = 5)]
    #[token("min", priority = 5)]
    Minute,
    
    #[token("hour", priority = 5)]
    #[token("h", priority = 5)]
    Hour,
    
    #[token("day", priority = 5)]
    #[token("d", priority = 5)]
    Day,
    
    #[token("year", priority = 5)]
    #[token("yr", priority = 5)]
    Year,
    
    // === Identifiers ===
    
    // ASCII identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    Identifier,
    
    // Unicode identifiers (including Chinese)
    #[regex(r"[\p{XID_Start}][\p{XID_Continue}]*", priority = 1)]
    UnicodeIdentifier,
    
    // === Comments ===
    
    // Line comments
    #[regex(r"//[^\n]*", priority = 3)]
    LineComment,
    
    // Block comments
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
    
    // Chinese comments (alternative style)
    #[regex(r"#[^\n]*", priority = 2)]
    ChineseComment,
    
    // Documentation comments
    #[regex(r"///[^\n]*")]
    DocComment,
    
    #[regex(r"/\*\*([^*]|\*[^/])*\*/")]
    DocBlockComment,
    
    // === Special Tokens ===
    
    // End of file
    EOF,
    
    // Error token for invalid input
    Error,
    
    // Whitespace (normally skipped)
    Whitespace,
    
    // Newline (significant in some contexts)
    #[token("\n")]
    Newline,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Keywords
            TokenKind::Sphere => write!(f, "sphere"),
            TokenKind::Scale => write!(f, "scale"),
            TokenKind::Data => write!(f, "data"),
            TokenKind::Model => write!(f, "model"),
            TokenKind::Process => write!(f, "process"),
            
            // Chinese keywords
            TokenKind::SphereChinese => write!(f, "圈层"),
            TokenKind::ScaleChinese => write!(f, "尺度"),
            TokenKind::DataChinese => write!(f, "数据"),
            
            // Operators
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            
            // Literals
            TokenKind::Integer(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String => write!(f, "string"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            
            // Units
            TokenKind::Meter => write!(f, "meter"),
            TokenKind::Kelvin => write!(f, "kelvin"),
            TokenKind::Pascal => write!(f, "pascal"),
            
            // Identifiers
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::UnicodeIdentifier => write!(f, "unicode_identifier"),
            
            // Comments
            TokenKind::LineComment => write!(f, "line_comment"),
            TokenKind::BlockComment => write!(f, "block_comment"),
            
            // Special
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Error => write!(f, "ERROR"),
            
            _ => write!(f, "{:?}", self),
        }
    }
}

impl TokenKind {
    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self,
            TokenKind::Sphere | TokenKind::Scale | TokenKind::Data | TokenKind::Model |
            TokenKind::Process | TokenKind::Interface | TokenKind::Coupling |
            TokenKind::CoastalDatabase | TokenKind::SpatialDomainDeclKw |
            TokenKind::TemporalDomainDeclKw | TokenKind::TilingSchemeDeclKw |
            TokenKind::LayerKw | TokenKind::DatasetKw | TokenKind::FeatureClassKw |
            TokenKind::QualityProfileKw | TokenKind::UncertaintyModelKw |
            TokenKind::SphereChinese | TokenKind::ScaleChinese | TokenKind::DataChinese |
            TokenKind::ModelChinese | TokenKind::ProcessChinese | TokenKind::CouplingChinese |
            TokenKind::If | TokenKind::Else | TokenKind::For | TokenKind::While |
            TokenKind::Match | TokenKind::Let | TokenKind::Const | TokenKind::Type |
            TokenKind::Fn | TokenKind::Method | TokenKind::Class | TokenKind::Abstract
        )
    }
    
    /// Check if this token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
            TokenKind::Equal | TokenKind::NotEqual | TokenKind::Less | TokenKind::Greater |
            TokenKind::And | TokenKind::Or | TokenKind::Not | TokenKind::Assign
        )
    }
    
    /// Check if this token is a literal
    pub fn is_literal(&self) -> bool {
        matches!(self,
            TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::Scientific(_) |
            TokenKind::Complex | TokenKind::String | TokenKind::Char |
            TokenKind::True | TokenKind::False | TokenKind::Unit_
        )
    }
    
    /// Check if this token is a physical unit
    pub fn is_unit(&self) -> bool {
        matches!(self,
            TokenKind::Meter | TokenKind::Kilogram | TokenKind::Second |
            TokenKind::Kelvin | TokenKind::Pascal | TokenKind::Newton |
            TokenKind::Joule | TokenKind::Watt | TokenKind::PSU | TokenKind::Decibar |
            TokenKind::Degree | TokenKind::Radian | TokenKind::Hour | TokenKind::Day
        )
    }
    
    /// Check if this token is a comment
    pub fn is_comment(&self) -> bool {
        matches!(self,
            TokenKind::LineComment | TokenKind::BlockComment |
            TokenKind::ChineseComment | TokenKind::DocComment | TokenKind::DocBlockComment
        )
    }
    
    /// Check if this token is whitespace
    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::Newline)
    }
    
    /// Get the precedence of an operator (higher number = higher precedence)
    pub fn precedence(&self) -> Option<u8> {
        match self {
            // Arithmetic operators
            TokenKind::Power => Some(8),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(7),
            TokenKind::Plus | TokenKind::Minus => Some(6),
            
            // Comparison operators
            TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual => Some(5),
            TokenKind::Equal | TokenKind::NotEqual => Some(4),
            
            // Logical operators
            TokenKind::And => Some(3),
            TokenKind::Xor => Some(2),
            TokenKind::Or => Some(1),
            
            // Assignment (lowest precedence)
            TokenKind::Assign | TokenKind::PlusAssign | TokenKind::MinusAssign => Some(0),
            
            _ => None,
        }
    }
    
    /// Check if this operator is left-associative
    pub fn is_left_associative(&self) -> bool {
        match self {
            TokenKind::Power => false,  // Right-associative
            TokenKind::Assign | TokenKind::PlusAssign | TokenKind::MinusAssign => false,  // Right-associative
            _ => true,  // Most operators are left-associative
        }
    }
}

/// Utility functions for token classification
impl TokenKind {
    /// Get all keywords as a vector
    pub fn keywords() -> Vec<&'static str> {
        vec![
            "sphere", "scale", "data", "model", "process", "interface", "coupling",
            "world_model", "compute_context", "ai_model", "digital_twin",
            "Atmosphere", "Ocean", "Lithosphere", "Biosphere", "Anthroposphere",
            "spatial", "temporal", "molecular", "cellular", "organism", "ecosystem", "regional", "global",
            "observation", "simulation", "remote_sensing", "field", "time_series", "uncertain",
            "if", "else", "for", "while", "match", "loop", "break", "continue", "return",
            "let", "const", "type", "enum", "struct", "class", "abstract", "extends", "implements",
            "pub", "private", "static", "mut", "ref",
            "fn", "method", "override", "auto",
            "constraint", "property", "invariant", "requires", "ensures",
            "with", "unit", "dimension",
            "圈层", "尺度", "数据", "模型", "过程", "耦合",
            "大气圈", "海洋圈", "岩石圈", "生物圈", "人类圈",
            "时空", "物理量", "不确定性",
        ]
    }
    
    /// Get all physical units as a vector
    pub fn units() -> Vec<&'static str> {
        vec![
            "meter", "m", "kilogram", "kg", "second", "s", "ampere", "A",
            "kelvin", "K", "mole", "mol", "candela", "cd",
            "pascal", "Pa", "newton", "N", "joule", "J", "watt", "W",
            "celsius", "°C", "psu", "dbar", "sverdrup", "Sv",
            "hPa", "atm", "mmHg", "degree", "deg", "°", "radian", "rad",
            "nautical_mile", "nmi", "minute", "min", "hour", "h", "day", "d", "year", "yr",
        ]
    }
}
