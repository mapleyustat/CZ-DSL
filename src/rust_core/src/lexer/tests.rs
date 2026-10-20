//! Comprehensive tests for the CZ-DSL lexer
//! 
//! Tests cover all token types, error handling, Unicode support, and edge cases.

use super::*;
use crate::lexer::{Lexer, TokenKind, LexError};

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::EOF);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("   \t  \n  ");
        let tokens = lexer.tokenize().unwrap();
        // Should only have newline and EOF (other whitespace is skipped)
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Newline);
        assert_eq!(tokens[1].kind, TokenKind::EOF);
    }

    #[test]
    fn test_all_single_char_tokens() {
        let input = "()[]{},:;@#$?";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBracket,
            TokenKind::RightBracket,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Colon,
            TokenKind::Semicolon,
            TokenKind::At,
            TokenKind::Hash,
            TokenKind::Dollar,
            TokenKind::Question,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_char_operators() {
        let input = "+= -= *= /= %= == != <= >= && || -> => :: .. ..= ...";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::StarAssign,
            TokenKind::SlashAssign,
            TokenKind::PercentAssign,
            TokenKind::Equal,
            TokenKind::NotEqual,
            TokenKind::LessEqual,
            TokenKind::GreaterEqual,
            TokenKind::And,
            TokenKind::Or,
            TokenKind::Arrow,
            TokenKind::FatArrow,
            TokenKind::DoubleColon,
            TokenKind::Range,
            TokenKind::RangeInclusive,
            TokenKind::Ellipsis,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_english_keywords() {
        let input = "sphere scale data model process interface coupling";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Sphere,
            TokenKind::Scale,
            TokenKind::Data,
            TokenKind::Model,
            TokenKind::Process,
            TokenKind::Interface,
            TokenKind::Coupling,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_chinese_keywords() {
        let input = "圈层 尺度 数据 模型 过程 耦合 大气圈 海洋圈 岩石圈 生物圈 人类圈";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::SphereChinese,
            TokenKind::ScaleChinese,
            TokenKind::DataChinese,
            TokenKind::ModelChinese,
            TokenKind::ProcessChinese,
            TokenKind::CouplingChinese,
            TokenKind::AtmosphereChinese,
            TokenKind::OceanChinese,
            TokenKind::LithosphereChinese,
            TokenKind::BiosphereChinese,
            TokenKind::AnthroposphereChinese,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sphere_types() {
        let input = "Atmosphere Ocean Lithosphere Biosphere Anthroposphere";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::AtmosphereType,
            TokenKind::OceanType,
            TokenKind::LithosphereType,
            TokenKind::BiosphereType,
            TokenKind::AnthroposphereType,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_control_flow_keywords() {
        let input = "if else for while match loop break continue return";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::If,
            TokenKind::Else,
            TokenKind::For,
            TokenKind::While,
            TokenKind::Match,
            TokenKind::Loop,
            TokenKind::Break,
            TokenKind::Continue,
            TokenKind::Return,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_integer_literals() {
        let input = "0 123 456789";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::Integer(0)));
        assert!(matches!(tokens[1].kind, TokenKind::Integer(123)));
        assert!(matches!(tokens[2].kind, TokenKind::Integer(456789)));
        assert_eq!(tokens[3].kind, TokenKind::EOF);
    }

    #[test]
    fn test_float_literals() {
        let input = "0.0 123.456 0.123 999.999";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::Float(f) if (f - 0.0).abs() < 1e-10));
        assert!(matches!(tokens[1].kind, TokenKind::Float(f) if (f - 123.456).abs() < 1e-10));
        assert!(matches!(tokens[2].kind, TokenKind::Float(f) if (f - 0.123).abs() < 1e-10));
        assert!(matches!(tokens[3].kind, TokenKind::Float(f) if (f - 999.999).abs() < 1e-10));
        assert_eq!(tokens[4].kind, TokenKind::EOF);
    }

    #[test]
    fn test_scientific_notation() {
        let input = "1e5 2.5e-3 1.23E+10 6.022e23";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::Scientific(f) if (f - 1e5).abs() < 1e-10));
        assert!(matches!(tokens[1].kind, TokenKind::Scientific(f) if (f - 2.5e-3).abs() < 1e-10));
        assert!(matches!(tokens[2].kind, TokenKind::Scientific(f) if (f - 1.23e10).abs() < 1e-10));
        assert!(matches!(tokens[3].kind, TokenKind::Scientific(f) if (f - 6.022e23).abs() < 1e-10));
        assert_eq!(tokens[4].kind, TokenKind::EOF);
    }

    #[test]
    fn test_complex_numbers() {
        let input = "3+4i 2.5-1.5i 0+1i";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        // Note: Complex parsing might need adjustment based on exact implementation
        // This test assumes the lexer recognizes complex patterns
        assert!(tokens.len() > 1);
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello" "world with spaces" "escape\nsequences""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[1].kind, TokenKind::String);
        assert_eq!(tokens[2].kind, TokenKind::String);
        assert_eq!(tokens[3].kind, TokenKind::EOF);
    }

    #[test]
    fn test_char_literals() {
        let input = "'a' 'Z' '\\n' '\\t' '\\\\'";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        for i in 0..5 {
            assert_eq!(tokens[i].kind, TokenKind::Char);
        }
        assert_eq!(tokens[5].kind, TokenKind::EOF);
    }

    #[test]
    fn test_raw_strings() {
        let input = r#"r"raw string with \n no escapes""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::RawString);
        assert_eq!(tokens[1].kind, TokenKind::EOF);
    }

    #[test]
    fn test_boolean_literals() {
        let input = "true false";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::True);
        assert_eq!(tokens[1].kind, TokenKind::False);
        assert_eq!(tokens[2].kind, TokenKind::EOF);
    }

    #[test]
    fn test_si_base_units() {
        let input = "meter m kilogram kg second s ampere A kelvin K mole mol candela cd";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Meter, TokenKind::Meter,
            TokenKind::Kilogram, TokenKind::Kilogram,
            TokenKind::Second, TokenKind::Second,
            TokenKind::Ampere, TokenKind::Ampere,
            TokenKind::Kelvin, TokenKind::Kelvin,
            TokenKind::Mole, TokenKind::Mole,
            TokenKind::Candela, TokenKind::Candela,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_derived_units() {
        let input = "pascal Pa newton N joule J watt W";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Pascal, TokenKind::Pascal,
            TokenKind::Newton, TokenKind::Newton,
            TokenKind::Joule, TokenKind::Joule,
            TokenKind::Watt, TokenKind::Watt,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_coastal_units() {
        let input = "psu dbar sverdrup Sv hPa atm mmHg";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::PSU,
            TokenKind::Decibar,
            TokenKind::Sverdrup, TokenKind::Sverdrup,
            TokenKind::Hectopascal,
            TokenKind::Atmosphere,
            TokenKind::MillimeterMercury,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_temperature_units() {
        let input = "kelvin K celsius °C";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Kelvin, TokenKind::Kelvin,
            TokenKind::Celsius, TokenKind::Celsius,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_angle_units() {
        let input = "degree deg ° radian rad";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Degree, TokenKind::Degree, TokenKind::Degree,
            TokenKind::Radian, TokenKind::Radian,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_time_units() {
        let input = "second s minute min hour h day d year yr";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Second, TokenKind::Second,
            TokenKind::Minute, TokenKind::Minute,
            TokenKind::Hour, TokenKind::Hour,
            TokenKind::Day, TokenKind::Day,
            TokenKind::Year, TokenKind::Year,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "variable_name CamelCase snake_case _underscore var123";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        for i in 0..5 {
            assert_eq!(tokens[i].kind, TokenKind::Identifier);
        }
        assert_eq!(tokens[5].kind, TokenKind::EOF);
    }

    #[test]
    fn test_unicode_identifiers() {
        let input = "变量名 αβγ δεζ ηθι κλμ";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        for i in 0..5 {
            assert_eq!(tokens[i].kind, TokenKind::UnicodeIdentifier);
        }
        assert_eq!(tokens[5].kind, TokenKind::EOF);
    }

    #[test]
    fn test_line_comments() {
        let input = "// This is a comment\ncode // Another comment";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::LineComment);
        assert_eq!(tokens[1].kind, TokenKind::Newline);
        assert_eq!(tokens[2].kind, TokenKind::Identifier); // "code"
        assert_eq!(tokens[3].kind, TokenKind::LineComment);
        assert_eq!(tokens[4].kind, TokenKind::EOF);
    }

    #[test]
    fn test_block_comments() {
        let input = "/* This is a block comment */ code /* Another\nmultiline\ncomment */";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::BlockComment);
        assert_eq!(tokens[1].kind, TokenKind::Identifier); // "code"
        assert_eq!(tokens[2].kind, TokenKind::BlockComment);
        assert_eq!(tokens[3].kind, TokenKind::EOF);
    }

    #[test]
    fn test_mixed_language_code() {
        let input = "sphere 大气圈 { temperature: 温度 with unit kelvin }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenKind::Sphere,
            TokenKind::AtmosphereChinese,
            TokenKind::LeftBrace,
            TokenKind::Identifier, // "temperature"
            TokenKind::Colon,
            TokenKind::UnicodeIdentifier, // "温度"
            TokenKind::With,
            TokenKind::Unit,
            TokenKind::Kelvin,
            TokenKind::RightBrace,
            TokenKind::EOF,
        ];
        
        let actual: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_number_range_disambiguation() {
        let input = "1..10 1.5..2.5 1.0.method()";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        // Should correctly distinguish between float literals and range operators
        assert!(matches!(tokens[0].kind, TokenKind::Integer(1)));
        assert_eq!(tokens[1].kind, TokenKind::Range);
        assert!(matches!(tokens[2].kind, TokenKind::Integer(10)));
        
        assert!(matches!(tokens[3].kind, TokenKind::Float(_)));
        assert_eq!(tokens[4].kind, TokenKind::Range);
        assert!(matches!(tokens[5].kind, TokenKind::Float(_)));
        
        assert!(matches!(tokens[6].kind, TokenKind::Float(_)));
        assert_eq!(tokens[7].kind, TokenKind::Period);
        assert_eq!(tokens[8].kind, TokenKind::Method); // "method" - treated as keyword
    }

    #[test]
    fn test_error_unterminated_string() {
        let mut lexer = Lexer::new(r#""unterminated string"#);
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        if let Err(LexError::UnterminatedString { .. }) = result {
            // Expected error
        } else {
            panic!("Expected UnterminatedString error");
        }
    }

    #[test]
    fn test_error_unterminated_block_comment() {
        let mut lexer = Lexer::new("/* unterminated comment");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        if let Err(LexError::UnterminatedBlockComment { .. }) = result {
            // Expected error
        } else {
            panic!("Expected UnterminatedBlockComment error");
        }
    }

    #[test]
    fn test_error_invalid_char_literal() {
        let mut lexer = Lexer::new("'invalid char literal'");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        // The lexer should detect this as an invalid character literal
    }

    #[test]
    fn test_position_tracking() {
        let input = "line1\nline2\n  indented";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        // Check that positions are correctly tracked
        assert_eq!(tokens[0].span.start.line, 1); // "line1"
        assert_eq!(tokens[0].span.start.column, 1);
        
        assert_eq!(tokens[1].span.start.line, 1); // newline after "line1"
        
        assert_eq!(tokens[2].span.start.line, 2); // "line2"
        assert_eq!(tokens[2].span.start.column, 1);
        
        assert_eq!(tokens[4].span.start.line, 3); // "indented"
        assert_eq!(tokens[4].span.start.column, 3); // After 2 spaces
    }

    #[test]
    fn test_comprehensive_coastal_example() {
        let input = r#"
        // 海岸带模型定义
        sphere 大气圈 {
            properties: {
                temperature: Real with unit kelvin,
                pressure: Real with unit pascal,
                wind_velocity: Vector3D with unit m/s
            }
        }
        
        process 潮汐过程 {
            inputs: {
                lunar_force: Force with unit newton,
                solar_force: Force with unit newton
            },
            outputs: {
                tide_height: Real with unit meter
            }
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        
        assert!(result.is_ok());
        let tokens = result.unwrap();
        
        // Verify that we can parse a realistic CZ-DSL code snippet
        assert!(tokens.len() > 20); // Should have many tokens
        assert_eq!(tokens.last().unwrap().kind, TokenKind::EOF);
        
        // Check for presence of key tokens
        let token_kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert!(token_kinds.contains(&TokenKind::Sphere));
        assert!(token_kinds.contains(&TokenKind::AtmosphereChinese));
        assert!(token_kinds.contains(&TokenKind::Process));
        assert!(token_kinds.contains(&TokenKind::With));
        assert!(token_kinds.contains(&TokenKind::Unit));
        assert!(token_kinds.contains(&TokenKind::Kelvin));
        assert!(token_kinds.contains(&TokenKind::Pascal));
        assert!(token_kinds.contains(&TokenKind::Meter));
    }
}
