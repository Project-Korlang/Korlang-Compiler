use korlang_compiler::lexer::Lexer;
use korlang_compiler::parser::Parser;
use korlang_compiler::sema::Sema;

#[test]
fn test_basic_sema() {
    let src = "let x: Int = 42;";
    let tokens = Lexer::new(src).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    
    let mut sema = Sema::new();
    let result = sema.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_type_mismatch() {
    let src = r#"let x: Int = "hello";"#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    
    let mut sema = Sema::new();
    let result = sema.check_program(&program);
    
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(diags.iter().any(|d| d.message.contains("type mismatch")));
}

#[test]
fn test_nogc_violation() {
    let src = r#"
        @nogc fun test() {
            let arr = [1, 2, 3];
        }
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    
    let mut sema = Sema::new();
    let result = sema.check_program(&program);
    
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(diags.iter().any(|d| d.message.contains("allocation not allowed in @nogc")));
}
