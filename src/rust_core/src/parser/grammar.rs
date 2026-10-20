//! CZ-DSL Grammar Definition
//! 
//! This file defines the formal grammar for CZ-DSL in Extended Backus-Naur Form (EBNF).
//! The grammar supports both English and Chinese keywords, physical units,
//! and multi-scale coastal zone modeling constructs.

/// CZ-DSL Grammar in EBNF
/// 
/// This grammar defines the complete syntax for the Coastal Zone Domain-Specific Language.
/// It supports:
/// - English and Chinese keywords
/// - Physical units and dimensional analysis
/// - Multi-sphere system modeling
/// - Type annotations and constraints
/// 
/// # Grammar Rules
/// 
/// ```ebnf
/// (* Top-level program structure *)
/// program = { declaration | statement } ;
/// 
/// (* Declarations *)
/// declaration = sphere_decl
///             | scale_decl
///             | data_decl
///             | model_decl
///             | process_decl
///             | interface_decl
///             | coupling_decl
///             | ai_model_decl
///             | digital_twin_decl
///             | function_decl
///             | type_decl
///             | const_decl
///             | let_decl ;
/// 
/// (* Sphere Declaration *)
/// sphere_decl = ("sphere" | "圈层") identifier [ "extends" type_expr ] 
///               "{" { sphere_member } "}" ;
/// 
/// sphere_member = property_decl
///               | process_decl
///               | boundary_decl
///               | constraint_decl ;
/// 
/// property_decl = identifier ":" type_expr [ "=" expression ] "," ;
/// 
/// boundary_decl = "boundaries" ":" "{" { boundary_member } "}" ;
/// boundary_member = identifier ":" "Interface" "<" type_expr "," type_expr ">" "," ;
/// 
/// (* Scale Declaration *)
/// scale_decl = ("scale" | "尺度") identifier "{" { scale_member } "}" ;
/// 
/// scale_member = scale_range_decl ;
/// scale_range_decl = identifier ":" range_expr [ unit_expr ] "," ;
/// 
/// (* Data Declaration *)
/// data_decl = ("data" | "数据") identifier [ generic_params ] 
///             "{" { data_member } "}" ;
/// 
/// data_member = field_decl
///             | method_decl
///             | constraint_decl ;
/// 
/// field_decl = identifier ":" type_expr [ "=" expression ] "," ;
/// 
/// (* Model Declaration *)
/// model_decl = ("model" | "模型") identifier [ generic_params ]
///              "{" { model_member } "}" ;
/// 
/// model_member = parameter_decl
///              | variable_decl
///              | equation_decl
///              | constraint_decl
///              | coupling_decl ;
/// 
/// parameter_decl = "parameters" ":" "{" { parameter_member } "}" ;
/// parameter_member = identifier ":" type_expr [ "=" expression ] "," ;
/// 
/// equation_decl = "equations" ":" "{" { equation_member } "}" ;
/// equation_member = identifier ":" expression "," ;
/// 
/// (* Process Declaration *)
/// process_decl = ("process" | "过程") identifier [ generic_params ]
///                "{" { process_member } "}" ;
/// 
/// process_member = input_decl
///                | output_decl
///                | parameter_decl
///                | constraint_decl
///                | method_decl ;
/// 
/// input_decl = "inputs" ":" "{" { io_member } "}" ;
/// output_decl = "outputs" ":" "{" { io_member } "}" ;
/// io_member = identifier ":" type_expr "," ;
/// 
/// (* AI Model Declaration *)
/// ai_model_decl = ("ai_model" | "AI模型") identifier [ generic_params ]
///                 "{" { ai_model_member } "}" ;
/// 
/// ai_model_member = architecture_decl
///                 | training_data_decl
///                 | physics_constraints_decl
///                 | method_decl ;
/// 
/// architecture_decl = "architecture" ":" type_expr "," ;
/// training_data_decl = "training_data" ":" type_expr "," ;
/// physics_constraints_decl = "physics_constraints" ":" "[" { expression } "]" "," ;
/// 
/// (* Digital Twin Declaration *)
/// digital_twin_decl = ("digital_twin" | "数字孪生") identifier
///                     "{" { digital_twin_member } "}" ;
/// 
/// digital_twin_member = physical_system_decl
///                     | digital_replica_decl
///                     | assimilation_decl
///                     | simulation_decl
///                     | control_decl ;
/// 
/// physical_system_decl = "physical_system" ":" type_expr "," ;
/// digital_replica_decl = "digital_replica" ":" type_expr "," ;
/// assimilation_decl = "assimilation" ":" type_expr "," ;
/// simulation_decl = "simulation" ":" type_expr "," ;
/// control_decl = "control" ":" type_expr "," ;
/// 
/// (* Function Declaration *)
/// function_decl = ("fn" | "method") identifier [ generic_params ]
///                 "(" [ parameter_list ] ")" [ ":" type_expr ]
///                 ( block_stmt | ";" ) ;
/// 
/// parameter_list = parameter { "," parameter } ;
/// parameter = identifier ":" type_expr ;
/// 
/// (* Type Declaration *)
/// type_decl = "type" identifier [ generic_params ] "=" type_expr ";" ;
/// 
/// (* Variable Declarations *)
/// const_decl = "const" identifier ":" type_expr "=" expression ";" ;
/// let_decl = ("let" | "mut") identifier [ ":" type_expr ] "=" expression ";" ;
/// 
/// (* Constraint Declaration *)
/// constraint_decl = "constraint" identifier [ generic_params ]
///                   "{" constraint_body "}" ;
/// 
/// constraint_body = quantified_expr | expression ;
/// quantified_expr = quantifier identifier ":" type_expr "=>" expression ;
/// quantifier = "∀" | "forall" | "∃" | "exists" ;
/// 
/// (* Type Expressions *)
/// type_expr = basic_type
///           | generic_type
///           | function_type
///           | tuple_type
///           | array_type
///           | field_type
///           | uncertain_type
///           | unit_type ;
/// 
/// basic_type = "Real" | "Integer" | "Complex" | "Boolean" | "String"
///            | "Vector3D" | "Point2D" | "Point3D" | "GeoPoint"
///            | "Multivector" | "Scalar" | "Vector" | "Bivector" | "Trivector"
///            | identifier ;
/// 
/// generic_type = identifier "<" type_list ">" ;
/// type_list = type_expr { "," type_expr } ;
/// 
/// function_type = "(" [ type_list ] ")" "->" type_expr ;
/// tuple_type = "(" type_list ")" ;
/// array_type = "[" type_expr "]" | "[" type_expr ";" expression "]" ;
/// 
/// field_type = "ScalarField" "<" type_expr ">"
///            | "VectorField" "<" type_expr ">"
///            | "TensorField" "<" type_expr ">" ;
/// 
/// uncertain_type = "Uncertain" "<" type_expr ">" ;
/// 
/// unit_type = type_expr "with" "unit" unit_expr ;
/// 
/// (* Unit Expressions *)
/// unit_expr = unit_term { ( "*" | "/" ) unit_term } ;
/// unit_term = unit_factor [ "^" integer_literal ] ;
/// unit_factor = unit_identifier | "(" unit_expr ")" ;
/// 
/// unit_identifier = "meter" | "m" | "kilogram" | "kg" | "second" | "s"
///                 | "kelvin" | "K" | "pascal" | "Pa" | "newton" | "N"
///                 | "joule" | "J" | "watt" | "W" | "celsius" | "°C"
///                 | "psu" | "dbar" | "sverdrup" | "Sv" | "hPa" | "atm"
///                 | "degree" | "°" | "radian" | "rad" | "minute" | "min"
///                 | "hour" | "h" | "day" | "d" | "year" | "yr"
///                 | identifier ;
/// 
/// (* Statements *)
/// statement = expression_stmt
///           | assignment_stmt
///           | if_stmt
///           | for_stmt
///           | while_stmt
///           | match_stmt
///           | loop_stmt
///           | break_stmt
///           | continue_stmt
///           | return_stmt
///           | block_stmt ;
/// 
/// expression_stmt = expression ";" ;
/// assignment_stmt = lvalue assignment_op expression ";" ;
/// assignment_op = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;
/// 
/// if_stmt = "if" expression block_stmt [ "else" ( if_stmt | block_stmt ) ] ;
/// for_stmt = "for" identifier "in" expression block_stmt ;
/// while_stmt = "while" expression block_stmt ;
/// loop_stmt = "loop" block_stmt ;
/// break_stmt = "break" [ expression ] ";" ;
/// continue_stmt = "continue" ";" ;
/// return_stmt = "return" [ expression ] ";" ;
/// 
/// match_stmt = "match" expression "{" { match_arm } "}" ;
/// match_arm = pattern [ "if" expression ] "=>" ( expression | block_stmt ) "," ;
/// 
/// block_stmt = "{" { statement } "}" ;
/// 
/// (* Expressions *)
/// expression = logical_or_expr ;
/// 
/// logical_or_expr = logical_and_expr { "||" logical_and_expr } ;
/// logical_and_expr = equality_expr { "&&" equality_expr } ;
/// equality_expr = relational_expr { ( "==" | "!=" | "≠" ) relational_expr } ;
/// relational_expr = additive_expr { ( "<" | "<=" | ">" | ">=" | "≤" | "≥" ) additive_expr } ;
/// 
/// additive_expr = multiplicative_expr { ( "+" | "-" ) multiplicative_expr } ;
/// multiplicative_expr = power_expr { ( "*" | "/" | "%" ) power_expr } ;
/// 
/// power_expr = unary_expr [ "**" power_expr ] ;
/// 
/// unary_expr = ( "+" | "-" | "!" | "~" | "†" | "∇" | "∂" ) unary_expr
///            | postfix_expr ;
/// 
/// postfix_expr = primary_expr { postfix_op } ;
/// postfix_op = "." identifier [ "(" [ argument_list ] ")" ]
///            | "[" expression "]"
///            | "(" [ argument_list ] ")" ;
/// 
/// primary_expr = literal
///              | identifier
///              | "(" expression ")"
///              | array_expr
///              | tuple_expr
///              | field_access
///              | method_call
///              | lambda_expr ;
/// 
/// array_expr = "[" [ expression_list ] "]" ;
/// tuple_expr = "(" expression_list ")" ;
/// expression_list = expression { "," expression } ;
/// argument_list = expression { "," expression } ;
/// 
/// field_access = expression "." identifier ;
/// method_call = expression "." identifier "(" [ argument_list ] ")" ;
/// 
/// lambda_expr = "|" [ parameter_list ] "|" ( expression | block_stmt ) ;
/// 
/// (* Literals *)
/// literal = numeric_literal
///         | string_literal
///         | char_literal
///         | boolean_literal
///         | unit_literal ;
/// 
/// numeric_literal = integer_literal
///                 | float_literal
///                 | scientific_literal
///                 | complex_literal ;
/// 
/// integer_literal = digit { digit } ;
/// float_literal = digit { digit } "." digit { digit } ;
/// scientific_literal = ( integer_literal | float_literal ) ( "e" | "E" ) [ "+" | "-" ] digit { digit } ;
/// complex_literal = ( integer_literal | float_literal | scientific_literal ) "i" ;
/// 
/// string_literal = "\"" { string_char } "\""
///                | "r\"" { raw_string_char } "\"" ;
/// 
/// char_literal = "'" char_char "'" ;
/// 
/// boolean_literal = "true" | "false" ;
/// unit_literal = "()" ;
/// 
/// (* Patterns *)
/// pattern = literal_pattern
///         | identifier_pattern
///         | wildcard_pattern
///         | tuple_pattern
///         | array_pattern
///         | struct_pattern ;
/// 
/// literal_pattern = literal ;
/// identifier_pattern = identifier ;
/// wildcard_pattern = "_" ;
/// tuple_pattern = "(" [ pattern_list ] ")" ;
/// array_pattern = "[" [ pattern_list ] "]" ;
/// struct_pattern = identifier "{" [ field_pattern_list ] "}" ;
/// 
/// pattern_list = pattern { "," pattern } ;
/// field_pattern_list = field_pattern { "," field_pattern } ;
/// field_pattern = identifier ":" pattern ;
/// 
/// (* Generic Parameters *)
/// generic_params = "<" generic_param_list ">" ;
/// generic_param_list = generic_param { "," generic_param } ;
/// generic_param = identifier [ ":" type_bound ] ;
/// type_bound = type_expr { "+" type_expr } ;
/// 
/// (* Range Expressions *)
/// range_expr = expression ".." expression
///            | expression "..=" expression
///            | ".." expression
///            | expression ".." ;
/// 
/// (* Left-hand Values *)
/// lvalue = identifier
///        | lvalue "." identifier
///        | lvalue "[" expression "]" ;
/// 
/// (* Identifiers *)
/// identifier = ascii_identifier | unicode_identifier ;
/// ascii_identifier = ( letter | "_" ) { letter | digit | "_" } ;
/// unicode_identifier = xid_start { xid_continue } ;
/// 
/// (* Character Classes *)
/// letter = "a" .. "z" | "A" .. "Z" ;
/// digit = "0" .. "9" ;
/// xid_start = (* Unicode XID_Start characters *) ;
/// xid_continue = (* Unicode XID_Continue characters *) ;
/// 
/// string_char = (* Any character except " and \ *) | escape_sequence ;
/// raw_string_char = (* Any character except " *) ;
/// char_char = (* Any character except ' and \ *) | escape_sequence ;
/// 
/// escape_sequence = "\\" ( "n" | "t" | "r" | "\\" | "\"" | "'" | "0" ) ;
/// ```
/// 
/// # Grammar Notes
/// 
/// 1. **Precedence**: Operators follow standard mathematical precedence.
/// 
/// 2. **Associativity**: Most operators are left-associative except for assignment and
///    exponentiation which are right-associative.
/// 
/// 3. **Unicode Support**: The grammar supports Unicode identifiers and mathematical symbols.
/// 
/// 4. **Physical Units**: Units are first-class citizens in the type system and can be
///    combined using standard mathematical operations.
/// 
/// 5. **Chinese Keywords**: The grammar supports both English and Chinese keywords for
///    major language constructs to support international collaboration.
/// 

pub struct Grammar;

impl Grammar {
    /// Get the grammar rules as a string
    pub fn rules() -> &'static str {
        include_str!("grammar.rs")
    }
    
    /// Check if an identifier is a reserved keyword
    pub fn is_keyword(identifier: &str) -> bool {
        matches!(identifier,
            // English keywords
            "sphere" | "scale" | "data" | "model" | "process" | "interface" | "coupling" |
            "world_model" | "compute_context" | "ai_model" | "digital_twin" |
            "Atmosphere" | "Ocean" | "Lithosphere" | "Biosphere" | "Anthroposphere" |
            "spatial" | "temporal" | "molecular" | "cellular" | "organism" | 
            "ecosystem" | "regional" | "global" |
            "observation" | "simulation" | "remote_sensing" | "field" | 
            "time_series" | "uncertain" |
            "if" | "else" | "for" | "while" | "match" | "loop" | "break" | 
            "continue" | "return" |
            "let" | "const" | "type" | "enum" | "struct" | "class" | "abstract" | 
            "extends" | "implements" |
            "pub" | "private" | "static" | "mut" | "ref" |
            "fn" | "method" | "override" | "auto" |
            "constraint" | "property" | "invariant" | "requires" | "ensures" |
            "multivector" | "scalar" | "vector" | "bivector" | "trivector" |
            "geometric_product" | "outer_product" | "inner_product" | "dual" |
            "with" | "unit" | "dimension" | "true" | "false" |
            
            // Chinese keywords
            "圈层" | "尺度" | "数据" | "模型" | "过程" | "耦合" |
            "大气圈" | "海洋圈" | "岩石圈" | "生物圈" | "人类圈" |
            "时空" | "物理量" | "不确定性"
        )
    }
    
    /// Get operator precedence (higher number = higher precedence)
    pub fn operator_precedence(op: &str) -> Option<u8> {
        match op {
            // Assignment (lowest precedence)
            "=" | "+=" | "-=" | "*=" | "/=" | "%=" => Some(1),
            
            // Logical OR
            "||" => Some(2),
            
            // Logical AND
            "&&" => Some(3),
            
            // Equality
            "==" | "!=" | "≠" => Some(4),
            
            // Relational
            "<" | "<=" | ">" | ">=" | "≤" | "≥" => Some(5),
            
            // Additive
            "+" | "-" => Some(6),
            
            // Multiplicative
            "*" | "/" | "%" => Some(7),
            
            // Geometric algebra
            "∧" | "∨" | "⌋" | "⌊" | "×" | "·" | "∘" => Some(8),
            
            // Power (highest precedence)
            "**" => Some(9),
            
            _ => None,
        }
    }
    
    /// Check if an operator is right-associative
    pub fn is_right_associative(op: &str) -> bool {
        matches!(op, "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "**")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keyword_recognition() {
        assert!(Grammar::is_keyword("sphere"));
        assert!(Grammar::is_keyword("圈层"));
        assert!(Grammar::is_keyword("Atmosphere"));
        assert!(!Grammar::is_keyword("my_variable"));
        assert!(!Grammar::is_keyword("custom_identifier"));
    }
    
    #[test]
    fn test_operator_precedence() {
        assert_eq!(Grammar::operator_precedence("="), Some(1));
        assert_eq!(Grammar::operator_precedence("+"), Some(6));
        assert_eq!(Grammar::operator_precedence("*"), Some(7));
        assert_eq!(Grammar::operator_precedence("∧"), Some(8));
        assert_eq!(Grammar::operator_precedence("**"), Some(9));
        assert_eq!(Grammar::operator_precedence("unknown"), None);
    }
    
    #[test]
    fn test_associativity() {
        assert!(Grammar::is_right_associative("="));
        assert!(Grammar::is_right_associative("**"));
        assert!(!Grammar::is_right_associative("+"));
        assert!(!Grammar::is_right_associative("*"));
    }
}
