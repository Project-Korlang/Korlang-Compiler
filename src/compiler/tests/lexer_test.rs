use korlang_compiler::lexer::{Lexer, TokenKind};
use korlang_compiler::symbols::intern;

#[test]
fn test_basic_lexing() {
    let src = "let x = 42;";
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].kind, TokenKind::Keyword("let"));
    assert_eq!(tokens[1].kind, TokenKind::Identifier(intern("x")));
    assert_eq!(tokens[2].kind, TokenKind::Eq);
    assert_eq!(tokens[3].kind, TokenKind::IntLiteral(42));
    assert_eq!(tokens[4].kind, TokenKind::Semi);
    assert_eq!(tokens[5].kind, TokenKind::Eof);
}

#[test]
fn test_string_interpolation() {
    let src = r#"let s = "Hello @{name}!";"#;
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().unwrap();
    
    // tokens: let, s, =, StringLiteral("Hello "), InterpStart, name, InterpEnd, StringLiteral("!"), Semi, Eof
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[3].kind, TokenKind::StringLiteral("Hello ".to_string()));
    assert_eq!(tokens[4].kind, TokenKind::InterpStart);
    assert_eq!(tokens[5].kind, TokenKind::Identifier(intern("name")));
    assert_eq!(tokens[6].kind, TokenKind::InterpEnd);
    assert_eq!(tokens[7].kind, TokenKind::StringLiteral("!".to_string()));
    assert_eq!(tokens[9].kind, TokenKind::Eof);
}

#[test]
fn test_unicode_identifiers() {
    let src = "let \u{03B1} = 1;"; // alpha
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[1].kind, TokenKind::Identifier(intern("\u{03B1}")));
    assert_eq!(tokens[5].kind, TokenKind::Eof);
}
