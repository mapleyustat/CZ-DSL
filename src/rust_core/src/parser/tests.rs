//! Tests for the CZ-DSL parser
//! 
//! This module contains comprehensive tests for the parser functionality.

use super::*;
use crate::lexer::{Lexer, TokenKind};

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_empty_input() {
        let result = parse_string("");
        assert!(result.is_ok());
        let program = result.unwrap();
        assert!(program.items.is_empty());
    }

    #[test]
    fn test_parse_simple_sphere() {
        let input = r#"
        sphere Atmosphere {
            temperature: Real,
            pressure: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.items.len(), 1);
        
        match &program.items[0] {
            Item::Sphere(sphere) => {
                assert_eq!(sphere.name.name, "Atmosphere");
                assert_eq!(sphere.members.len(), 2);
                assert!(sphere.extends.is_none());
            }
            _ => panic!("Expected sphere declaration"),
        }
    }

    #[test]
    fn test_parse_chinese_sphere() {
        let input = r#"
        圈层 大气圈 {
            温度: Real,
            压力: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.items.len(), 1);
        
        match &program.items[0] {
            Item::Sphere(sphere) => {
                assert_eq!(sphere.name.name, "大气圈");
                assert_eq!(sphere.members.len(), 2);
            }
            _ => panic!("Expected sphere declaration"),
        }
    }

    #[test]
    fn test_parse_sphere_with_extends() {
        let input = r#"
        sphere Ocean extends WaterBody {
            salinity: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        match &program.items[0] {
            Item::Sphere(sphere) => {
                assert_eq!(sphere.name.name, "Ocean");
                assert!(sphere.extends.is_some());
                if let Some(TypeExpr::Basic { name, .. }) = &sphere.extends {
                    assert_eq!(name.name, "WaterBody");
                }
            }
            _ => panic!("Expected sphere declaration"),
        }
    }

    #[test]
    fn test_parse_property_with_default_value() {
        let input = r#"
        sphere TestSphere {
            temperature: Real = 273.15,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        match &program.items[0] {
            Item::Sphere(sphere) => {
                match &sphere.members[0] {
                    SphereMember::Property(prop) => {
                        assert_eq!(prop.name.name, "temperature");
                        assert!(prop.default_value.is_some());
                        if let Some(Expression::Literal { value: Literal::Float(val), .. }) = &prop.default_value {
                            assert!((val - 273.15).abs() < 1e-10);
                        }
                    }
                    _ => panic!("Expected property"),
                }
            }
            _ => panic!("Expected sphere declaration"),
        }
    }

    #[test]
    fn test_parse_multiple_spheres() {
        let input = r#"
        sphere Atmosphere {
            temperature: Real,
        }
        
        sphere Ocean {
            salinity: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.items.len(), 2);
        
        match (&program.items[0], &program.items[1]) {
            (Item::Sphere(sphere1), Item::Sphere(sphere2)) => {
                assert_eq!(sphere1.name.name, "Atmosphere");
                assert_eq!(sphere2.name.name, "Ocean");
            }
            _ => panic!("Expected two sphere declarations"),
        }
    }

    #[test]
    fn test_parse_mixed_language() {
        let input = r#"
        sphere 大气圈 {
            temperature: Real,
            温度: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        match &program.items[0] {
            Item::Sphere(sphere) => {
                assert_eq!(sphere.name.name, "大气圈");
                assert_eq!(sphere.members.len(), 2);
                
                match (&sphere.members[0], &sphere.members[1]) {
                    (SphereMember::Property(prop1), SphereMember::Property(prop2)) => {
                        assert_eq!(prop1.name.name, "temperature");
                        assert_eq!(prop2.name.name, "温度");
                    }
                    _ => panic!("Expected properties"),
                }
            }
            _ => panic!("Expected sphere declaration"),
        }
    }

    #[test]
    fn test_parse_error_missing_brace() {
        let input = r#"
        sphere BadSphere {
            temperature: Real,
        // Missing closing brace
        "#;
        
        let result = parse_string(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        println!("parse_error_missing_brace err = {:?}", err);
        
        match err {
            ParseError::UnexpectedEOF { .. } => {}
            ParseError::UnexpectedToken { found, .. } if found == TokenKind::EOF => {}
            _ => panic!("Expected EOF-related parse error"),
        }
    }

    #[test]
    fn test_parse_error_missing_colon() {
        let input = r#"
        sphere BadSphere {
            temperature Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found, .. } => {
                assert!(expected.contains("Colon"));
                assert_eq!(found, TokenKind::Identifier);
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_parse_error_invalid_identifier() {
        let input = r#"
        sphere 123InvalidName {
            temperature: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_err());
        
        // Should fail because identifiers can't start with numbers
        match result.unwrap_err() {
            ParseError::UnexpectedToken { .. } => {
                // Expected error type
            }
            ParseError::LexerError { .. } => {
                // Also acceptable - lexer might catch this first
            }
            _ => panic!("Expected parsing error for invalid identifier"),
        }
    }

    #[test]
    fn test_parse_with_source_file() {
        let input = r#"
        sphere TestSphere {
            temperature: Real,
        }
        "#;
        
        let result = parse_string_with_source(input, "test.cz".to_string());
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_coastal_database() {
        let input = r#"
        coastal_database HangzhouBayCZDB {
            id: "CZDB-HZB-001",
            version: "1.0.0",
            spatial: HangzhouBay_SRM,
            temporal: Scenario_2023,
            tiling: CZDB_TileScheme_250m,
            cdb_profile: CDB_Global_Profile,
            metadata: {
                title: "杭州湾海岸带综合环境数据库",
                region: "Hangzhou Bay, China"
            },
            layers: {
                bathymetry: layer {
                    type: BathymetryLayer,
                    cdb_dataset: CDB_Elevation,
                    cdb_geometry_type: CDB_TIN,
                    datasets: {
                        depth_grid: dataset {
                            feature_class: GriddedBathymetry,
                            citygml_feature_class: CityGML_Terrain,
                            spatial_resolution: 250.0,
                            temporal_resolution: 10.0,
                            quality_profile: QC_Reviewed,
                            uncertainty_model: GaussianErrorModel,
                            cmecs_unit: CMECS_Substrate,
                            cdb_lod: 2.0,
                            cdb_tile_size: 250.0,
                            citygml_version: CityGML_3_0,
                            citygml_lod: 2.0,
                            citygml_ade: CityGML_Coastal_ADE
                        }
                    }
                }
            }
        }
        "#;
        
        let result = parse_string(input);
        let program = match result {
            Ok(p) => p,
            Err(e) => panic!("parse error in coastal_database test: {:?}", e),
        };
        assert_eq!(program.items.len(), 1);
        
        match &program.items[0] {
            Item::CoastalDatabase(db) => {
                assert_eq!(db.name.name, "HangzhouBayCZDB");
                assert_eq!(db.id.as_deref(), Some("CZDB-HZB-001"));
                assert_eq!(db.version.as_deref(), Some("1.0.0"));
                assert!(db.cdb_profile.is_some());
                assert_eq!(db.layers.len(), 1);
                let layer = &db.layers[0];
                assert!(layer.cdb_dataset.is_some());
                assert!(layer.cdb_geometry_type.is_some());
                assert_eq!(layer.datasets.len(), 1);
                let dataset = &layer.datasets[0];
                assert!(dataset.feature_class.is_some());
                assert!(dataset.citygml_feature_class.is_some());
                assert!(dataset.spatial_resolution.is_some());
                assert!(dataset.temporal_resolution.is_some());
                assert!(dataset.quality_profile.is_some());
                assert!(dataset.uncertainty_model.is_some());
                assert!(dataset.cmecs_unit.is_some());
                assert!(dataset.cdb_lod.is_some());
                assert!(dataset.cdb_tile_size.is_some());
                assert!(dataset.citygml_version.is_some());
                assert!(dataset.citygml_lod.is_some());
                assert!(dataset.citygml_ade.is_some());
            }
            _ => panic!("Expected coastal_database declaration"),
        }
    }


    #[test]
    fn test_expression_parsing() {
        // Test basic literal expressions
        let input = r#"
        sphere TestSphere {
            int_val: Integer = 42,
            float_val: Real = 3.14,
            string_val: String = "hello",
            bool_val: Boolean = true,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        match &program.items[0] {
            Item::Sphere(sphere) => {
                assert_eq!(sphere.members.len(), 4);
                
                // Check each property's default value
                for member in &sphere.members {
                    if let SphereMember::Property(prop) = member {
                        assert!(prop.default_value.is_some());
                        match prop.name.name.as_str() {
                            "int_val" => {
                                if let Some(Expression::Literal { value: Literal::Integer(42), .. }) = &prop.default_value {
                                    // Correct
                                } else {
                                    panic!("Expected integer literal 42");
                                }
                            }
                            "float_val" => {
                                if let Some(Expression::Literal { value: Literal::Float(val), .. }) = &prop.default_value {
                                    assert!((val - 3.14).abs() < 1e-10);
                                } else {
                                    panic!("Expected float literal 3.14");
                                }
                            }
                            "string_val" => {
                                if let Some(Expression::Literal { value: Literal::String(s), .. }) = &prop.default_value {
                                    assert_eq!(s, "hello");
                                } else {
                                    panic!("Expected string literal 'hello'");
                                }
                            }
                            "bool_val" => {
                                if let Some(Expression::Literal { value: Literal::Boolean(true), .. }) = &prop.default_value {
                                    // Correct
                                } else {
                                    panic!("Expected boolean literal true");
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => panic!("Expected sphere declaration"),
        }
    }

    #[test]
    fn test_unicode_identifiers() {
        let input = r#"
        sphere αβγ {
            δεζ: Real,
            ηθι: Real,
        }
        "#;
        
        let result = parse_string(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        match &program.items[0] {
            Item::Sphere(sphere) => {
                assert_eq!(sphere.name.name, "αβγ");
                assert_eq!(sphere.members.len(), 2);
            }
            _ => panic!("Expected sphere declaration"),
        }
    }


    #[test]
    fn test_error_reporter() {
        let input = r#"
        sphere BadSphere {
            invalid syntax here
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
        assert!(parser.error_reporter().has_errors());
        assert!(!parser.error_reporter().errors().is_empty());
    }
}
