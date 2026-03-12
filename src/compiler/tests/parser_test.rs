use korlang_compiler::lexer::Lexer;
use korlang_compiler::parser::Parser;
use korlang_compiler::ast::{Item, Expr};
use korlang_compiler::symbols::intern;

#[test]
fn test_basic_parsing() {
    let src = "let x = 42;";
    let tokens = Lexer::new(src).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    
    assert_eq!(program.items.len(), 1);
    if let Item::Const(v) = &program.items[0] {
        assert_eq!(v.name, intern("x"));
        assert!(matches!(v.value, Expr::Literal(korlang_compiler::ast::Literal::Int(42), _)));
    } else {
        panic!("Expected constant declaration");
    }
}

#[test]
fn test_recursion_limit() {
    // Generate a deeply nested expression: (((((...)))))
    let mut src = String::new();
    let depth = 1000;
    for _ in 0..depth {
        src.push_str("(");
    }
    src.push_str("1");
    for _ in 0..depth {
        src.push_str(")");
    }
    src.push_str(";");
    
    let tokens = Lexer::new(&src).tokenize().unwrap();
    let result = Parser::new(tokens).parse_program();
    
    // Should fail cleanly with recursion depth error, not a stack overflow
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(diags.iter().any(|d| d.message.contains("recursion depth exceeded")));
}
