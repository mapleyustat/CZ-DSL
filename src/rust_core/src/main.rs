//! CZ-DSL Command Line Interface
//! 
//! This binary provides a command-line interface for the CZ-DSL lexer and future compiler components.

use cz_dsl::{tokenize_with_source, language_stats, VERSION, LANGUAGE_NAME, LANGUAGE_DESCRIPTION};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "tokenize" | "lex" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage(&args[0]);
                process::exit(1);
            }
            tokenize_file(&args[2]);
        }
        "stats" | "info" => {
            print_language_stats();
        }
        "version" => {
            println!("{} v{}", LANGUAGE_NAME, VERSION);
        }
        "help" | "--help" | "-h" => {
            print_help(&args[0]);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program_name: &str) {
    println!("Usage: {} <command> [options]", program_name);
    println!();
    println!("Commands:");
    println!("  tokenize <file>  Tokenize a CZ-DSL source file");
    println!("  lex <file>       Alias for tokenize");
    println!("  stats            Show language statistics");
    println!("  info             Alias for stats");
    println!("  version          Show version information");
    println!("  help             Show this help message");
    println!();
    println!("Use '{} help' for more detailed information.", program_name);
}

fn print_help(program_name: &str) {
    println!("{}", LANGUAGE_NAME);
    println!("{}", LANGUAGE_DESCRIPTION);
    println!();
    println!("Version: {}", VERSION);
    println!();
    print_usage(program_name);
    println!();
    println!("Examples:");
    println!("  {} tokenize model.cz        # Tokenize model.cz file", program_name);
    println!("  {} lex -                    # Tokenize from stdin", program_name);
    println!("  {} stats                    # Show language statistics", program_name);
    println!("  {} version                  # Show version", program_name);
    println!();
    println!("The tokenize command will output all tokens found in the input,");
    println!("including their types, positions, and text content. This is useful");
    println!("for debugging lexical analysis and understanding how CZ-DSL code");
    println!("is tokenized.");
    println!();
    println!("CZ-DSL supports:");
    println!("  • English and Chinese keywords");
    println!("  • Unicode identifiers");
    println!("  • Physical units and dimensions");
    println!("  • Geometric algebra operators");
    println!("  • Multi-line and single-line comments");
    println!("  • Scientific notation and complex numbers");
}

fn tokenize_file(file_path: &str) {
    let input = if file_path == "-" {
        // Read from stdin
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Read from file
        match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                process::exit(1);
            }
        }
    };
    
    let source_name = if file_path == "-" {
        "<stdin>".to_string()
    } else {
        Path::new(file_path).file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(file_path)
            .to_string()
    };
    
    match tokenize_with_source(&input, source_name.clone()) {
        Ok(tokens) => {
            println!("Tokenization successful for '{}'", source_name);
            println!("Found {} tokens:", tokens.len());
            println!();
            
            for (i, token) in tokens.iter().enumerate() {
                println!("{:3}: {:20} {:15} '{}' at {}:{}", 
                    i + 1,
                    format!("{:?}", token.kind),
                    if token.kind.is_keyword() { "(keyword)" }
                    else if token.kind.is_operator() { "(operator)" }
                    else if token.kind.is_literal() { "(literal)" }
                    else if token.kind.is_unit() { "(unit)" }
                    else if token.kind.is_comment() { "(comment)" }
                    else { "" },
                    token.text,
                    token.span.start.line,
                    token.span.start.column
                );
            }
            
            // Print summary
            println!();
            println!("Summary:");
            let keyword_count = tokens.iter().filter(|t| t.kind.is_keyword()).count();
            let operator_count = tokens.iter().filter(|t| t.kind.is_operator()).count();
            let literal_count = tokens.iter().filter(|t| t.kind.is_literal()).count();
            let unit_count = tokens.iter().filter(|t| t.kind.is_unit()).count();
            let comment_count = tokens.iter().filter(|t| t.kind.is_comment()).count();
            let identifier_count = tokens.iter().filter(|t| 
                matches!(t.kind, cz_dsl::TokenKind::Identifier | cz_dsl::TokenKind::UnicodeIdentifier)
            ).count();
            
            println!("  Keywords:    {}", keyword_count);
            println!("  Operators:   {}", operator_count);
            println!("  Literals:    {}", literal_count);
            println!("  Units:       {}", unit_count);
            println!("  Identifiers: {}", identifier_count);
            println!("  Comments:    {}", comment_count);
        }
        Err(e) => {
            eprintln!("Tokenization failed for '{}':", source_name);
            eprintln!("Error: {}", e);
            
            // Try to provide more context if possible
            if let Some(pos) = e.position() {
                eprintln!("At line {}, column {}", pos.line, pos.column);
                
                // Show the problematic line if we can
                let lines: Vec<&str> = input.lines().collect();
                if pos.line > 0 && (pos.line as usize) <= lines.len() {
                    let line_idx = pos.line as usize - 1;
                    eprintln!("  {}: {}", pos.line, lines[line_idx]);
                    eprintln!("  {}{}", " ".repeat(pos.line.to_string().len() + 2), 
                             " ".repeat(pos.column.saturating_sub(1)) + "^");
                }
            }
            
            if let Some(suggestion) = e.suggestion() {
                eprintln!("Suggestion: {}", suggestion);
            }
            
            process::exit(1);
        }
    }
}

fn print_language_stats() {
    let stats = language_stats();
    
    println!("{} Language Statistics", LANGUAGE_NAME);
    println!("Version: {}", VERSION);
    println!();
    println!("Language Features:");
    println!("  Keywords:              {}", stats.keyword_count);
    println!("  Operators:             {}", stats.operator_count);
    println!("  Physical Units:        {}", stats.unit_count);
    println!("  Unicode Support:       {}", if stats.unicode_support { "Yes" } else { "No" });
    println!("  Chinese Support:       {}", if stats.chinese_support { "Yes" } else { "No" });
    println!();
    
    println!("Supported Token Types:");
    println!("  • Keywords (English and Chinese)");
    println!("  • Identifiers (ASCII and Unicode)");
    println!("  • Numeric literals (integers, floats, scientific notation)");
    println!("  • String and character literals");
    println!("  • Physical units (SI and domain-specific)");
    println!("  • Operators (arithmetic, logical)");
    println!("  • Comments (line and block)");
    println!("  • Delimiters and punctuation");
    println!();
    
    println!("Key Language Domains:");
    println!("  • Coastal zone modeling");
    println!("  • Multi-sphere systems (atmosphere, ocean, lithosphere, biosphere, anthroposphere)");
    println!("  • Multi-scale processes (molecular to global)");
    println!("  • Physical units and dimensional analysis");
    println!("  • AI-native constructs");
    println!("  • Digital twin and data assimilation");
}
