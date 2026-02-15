use crate::diag::{Diagnostic, Position, Span};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),

    Keyword(&'static str),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    DotPlus,
    DotMinus,
    DotStar,
    DotSlash,
    At,

    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,

    EqEq,
    FatArrow,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,

    AndAnd,
    OrOr,
    Not,

    Arrow,
    Pipe,
    NullCoalesce,
    Question,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Comma,
    Semi,
    Colon,
    Dot,

    InterpStart, // "@{"
    InterpEnd,   // "}"

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
struct InterpContext {
    resume_string: bool,
    depth: usize,
}

pub struct Lexer<'a> {
    src: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    pending: VecDeque<Token>,
    in_string: bool,
    interp_stack: Vec<InterpContext>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            pending: VecDeque::new(),
            in_string: false,
            interp_stack: Vec::new(),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, Vec<Diagnostic>> {
        let mut tokens = Vec::new();
        let mut diags = Vec::new();

        while !self.is_eof() || self.in_string || !self.pending.is_empty() || self.in_interpolation() {
            if let Some(tok) = self.pending.pop_front() {
                tokens.push(tok);
                continue;
            }

            if self.in_string && !self.in_interpolation() {
                match self.lex_string_segment() {
                    Ok(Some(t)) => tokens.push(t),
                    Ok(None) => {}
                    Err(d) => diags.push(d),
                }
                continue;
            }

            self.skip_whitespace_and_comments();
            if self.is_eof() {
                break;
            }

            let c = self.peek();
            let tok = match c {
                'a'..='z' | 'A'..='Z' | '_' => self.lex_ident_or_keyword(),
                '0'..='9' => self.lex_number(),
                '"' => {
                    self.advance();
                    self.in_string = true;
                    match self.lex_string_segment() {
                        Ok(Some(t)) => Ok(t),
                        Ok(None) => continue,
                        Err(d) => Err(d),
                    }
                }
                '\'' => self.lex_char(),
                _ => self.lex_symbol(),
            };

            match tok {
                Ok(t) => tokens.push(t),
                Err(d) => diags.push(d),
            }
        }

        if self.in_interpolation() {
            diags.push(Diagnostic::error(
                "unterminated interpolation",
                Span::new(self.position(), self.position()),
            ));
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span::new(self.position(), self.position()),
        });

        if diags.is_empty() {
            Ok(tokens)
        } else {
            Err(diags)
        }
    }

    fn lex_ident_or_keyword(&mut self) -> Result<Token, Diagnostic> {
        let start_pos = self.position();
        let start_idx = self.pos;
        self.advance();
        while !self.is_eof() {
            match self.peek() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.advance(),
                _ => break,
            }
        }
        let s: String = self.chars[start_idx..self.pos].iter().collect();
        let kind = match s.as_str() {
            "fun" | "gpu" | "let" | "var" | "if" | "else" | "match" | "for" | "while" |
            "break" | "continue" | "return" | "view" | "resource" | "state" |
            "spawn" | "@nogc" | "import" | "as" | "struct" | "enum" | "type" |
            "in" | "mut" | "interface" | "sealed" | "implements" | "class" => TokenKind::Keyword(Box::leak(s.into_boxed_str())),
            "true" => TokenKind::BoolLiteral(true),
            "false" => TokenKind::BoolLiteral(false),
            _ => TokenKind::Identifier(s),
        };
        Ok(Token { kind, span: Span::new(start_pos, self.position()) })
    }

    fn lex_number(&mut self) -> Result<Token, Diagnostic> {
        let start_pos = self.position();
        let start = self.pos;
        if self.peek() == '0' && (self.peek_next() == 'x' || self.peek_next() == 'X') {
            self.advance();
            self.advance();
            let hex_start = self.pos;
            while !self.is_eof() && self.peek().is_ascii_hexdigit() {
                self.advance();
            }
            if hex_start == self.pos {
                return Err(Diagnostic::error("invalid hex literal", Span::new(start_pos, self.position())));
            }
            let s: String = self.chars[hex_start..self.pos].iter().collect();
            let v = i64::from_str_radix(&s, 16)
                .map_err(|_| Diagnostic::error("invalid hex literal", Span::new(start_pos, self.position())))?;
            return Ok(Token { kind: TokenKind::IntLiteral(v), span: Span::new(start_pos, self.position()) });
        }
        if self.peek() == '0' && (self.peek_next() == 'b' || self.peek_next() == 'B') {
            self.advance();
            self.advance();
            let bin_start = self.pos;
            while !self.is_eof() && matches!(self.peek(), '0' | '1') {
                self.advance();
            }
            if bin_start == self.pos {
                return Err(Diagnostic::error("invalid binary literal", Span::new(start_pos, self.position())));
            }
            let s: String = self.chars[bin_start..self.pos].iter().collect();
            let v = i64::from_str_radix(&s, 2)
                .map_err(|_| Diagnostic::error("invalid binary literal", Span::new(start_pos, self.position())))?;
            return Ok(Token { kind: TokenKind::IntLiteral(v), span: Span::new(start_pos, self.position()) });
        }
        while !self.is_eof() && self.peek().is_ascii_digit() {
            self.advance();
        }
        let mut is_float = false;
        if !self.is_eof() && self.peek() == '.' {
            if self.peek_next().is_ascii_digit() {
                is_float = true;
                self.advance();
                while !self.is_eof() && self.peek().is_ascii_digit() {
                    self.advance();
                }
            }
        }
        if !self.is_eof() && (self.peek() == 'e' || self.peek() == 'E') {
            is_float = true;
            self.advance();
            if self.peek() == '+' || self.peek() == '-' {
                self.advance();
            }
            while !self.is_eof() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let s: String = self.chars[start..self.pos].iter().collect();
        if is_float {
            let v = s.parse::<f64>()
                .map_err(|_| Diagnostic::error("invalid float literal", Span::new(start_pos, self.position())))?;
            Ok(Token { kind: TokenKind::FloatLiteral(v), span: Span::new(start_pos, self.position()) })
        } else {
            let v = s.parse::<i64>()
                .map_err(|_| Diagnostic::error("invalid int literal", Span::new(start_pos, self.position())))?;
            Ok(Token { kind: TokenKind::IntLiteral(v), span: Span::new(start_pos, self.position()) })
        }
    }

    fn lex_string_segment(&mut self) -> Result<Option<Token>, Diagnostic> {
        let start = self.position();
        let mut out = String::new();
        while !self.is_eof() {
            let c = self.peek();
            if c == '"' {
                let end_pos = self.position();
                self.advance();
                self.in_string = false;
                let span = Span::new(start, end_pos);
                if out.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(Token { kind: TokenKind::StringLiteral(out), span }));
            }
            if c == '\\' {
                self.advance();
                if self.is_eof() {
                    return Err(Diagnostic::error("unterminated escape", Span::new(start, self.position())));
                }
                let esc = self.peek();
                let (real, consume) = match esc {
                    'n' => ('\n', true),
                    'r' => ('\r', true),
                    't' => ('\t', true),
                    '0' => ('\0', true),
                    '\\' => ('\\', true),
                    '"' => ('"', true),
                    '{' => ('{', true),
                    '}' => ('}', true),
                    'u' => {
                        self.advance();
                        if self.is_eof() || self.peek() != '{' {
                            return Err(Diagnostic::error("invalid unicode escape", Span::new(start, self.position())));
                        }
                        self.advance();
                        let hex_start = self.pos;
                        while !self.is_eof() && self.peek().is_ascii_hexdigit() {
                            self.advance();
                        }
                        if self.is_eof() || self.peek() != '}' {
                            return Err(Diagnostic::error("invalid unicode escape", Span::new(start, self.position())));
                        }
                        let hex: String = self.chars[hex_start..self.pos].iter().collect();
                        self.advance();
                        let code = u32::from_str_radix(&hex, 16)
                            .map_err(|_| Diagnostic::error("invalid unicode escape", Span::new(start, self.position())))?;
                        let ch = std::char::from_u32(code)
                            .ok_or_else(|| Diagnostic::error("invalid unicode escape", Span::new(start, self.position())))?;
                        (ch, false)
                    }
                    _ => return Err(Diagnostic::error("invalid escape", Span::new(start, self.position()))),
                };
                out.push(real);
                if consume {
                    self.advance();
                }
                continue;
            }
            if c == '{' {
                let interp_start = self.position();
                if !out.is_empty() {
                    let span = Span::new(start, interp_start);
                    self.advance();
                    self.push_interpolation(true);
                    let interp_span = Span::new(interp_start, self.position());
                    self.pending.push_back(Token { kind: TokenKind::InterpStart, span: interp_span });
                    return Ok(Some(Token { kind: TokenKind::StringLiteral(out), span }));
                }
                self.advance();
                self.push_interpolation(true);
                let interp_span = Span::new(interp_start, self.position());
                return Ok(Some(Token { kind: TokenKind::InterpStart, span: interp_span }));
            }
            out.push(c);
            self.advance();
        }
        Err(Diagnostic::error("unterminated string", Span::new(start, self.position())))
    }

    fn lex_char(&mut self) -> Result<Token, Diagnostic> {
        let start = self.position();
        self.advance();
        if self.is_eof() {
            return Err(Diagnostic::error("unterminated char", Span::new(start, self.position())));
        }
        let c = self.peek();
        let ch = if c == '\\' {
            self.advance();
            let esc = self.peek();
            let real = match esc {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '0' => '\0',
                '\\' => '\\',
                '\'' => '\'',
                'u' => {
                    self.advance();
                    if self.is_eof() || self.peek() != '{' {
                        return Err(Diagnostic::error("invalid unicode escape", Span::new(start, self.position())));
                    }
                    self.advance();
                    let hex_start = self.pos;
                    while !self.is_eof() && self.peek().is_ascii_hexdigit() {
                        self.advance();
                    }
                    if self.is_eof() || self.peek() != '}' {
                        return Err(Diagnostic::error("invalid unicode escape", Span::new(start, self.position())));
                    }
                    let hex: String = self.chars[hex_start..self.pos].iter().collect();
                    self.advance();
                    let code = u32::from_str_radix(&hex, 16)
                        .map_err(|_| Diagnostic::error("invalid unicode escape", Span::new(start, self.position())))?;
                    std::char::from_u32(code)
                        .ok_or_else(|| Diagnostic::error("invalid unicode escape", Span::new(start, self.position())))?
                }
                _ => return Err(Diagnostic::error("invalid escape", Span::new(start, self.position()))),
            };
            self.advance();
            real
        } else {
            self.advance();
            c
        };
        if self.is_eof() || self.peek() != '\'' {
            return Err(Diagnostic::error("unterminated char", Span::new(start, self.position())));
        }
        self.advance();
        Ok(Token { kind: TokenKind::CharLiteral(ch), span: Span::new(start, self.position()) })
    }

    fn lex_symbol(&mut self) -> Result<Token, Diagnostic> {
        let start = self.position();
        let c = self.peek();
        let next = self.peek_next();

        if c == '}' && self.in_interpolation() {
            self.advance();
            if let Some(top) = self.interp_stack.last_mut() {
                if top.depth == 0 {
                    let ctx = self.interp_stack.pop().unwrap();
                    if ctx.resume_string {
                        self.in_string = true;
                    }
                    return Ok(Token { kind: TokenKind::InterpEnd, span: Span::new(start, self.position()) });
                }
                top.depth -= 1;
                return Ok(Token { kind: TokenKind::RBrace, span: Span::new(start, self.position()) });
            }
        }
        if c == '{' && self.in_interpolation() {
            self.advance();
            if let Some(top) = self.interp_stack.last_mut() {
                top.depth += 1;
            }
            return Ok(Token { kind: TokenKind::LBrace, span: Span::new(start, self.position()) });
        }
        if c == '@' && next == '"' {
            self.advance();
            self.advance();
            let mut raw = String::new();
            while !self.is_eof() && self.peek() != '"' {
                raw.push(self.peek());
                self.advance();
            }
            if self.is_eof() {
                return Err(Diagnostic::error("unterminated raw string", Span::new(start, self.position())));
            }
            self.advance();
            return Ok(Token { kind: TokenKind::StringLiteral(raw), span: Span::new(start, self.position()) });
        }
        if (c == '@' && next.is_ascii_alphabetic()) || (c == '@' && next == '_') {
            self.advance();
            let ident_start = self.pos;
            while !self.is_eof() {
                match self.peek() {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.advance(),
                    _ => break,
                }
            }
            let name: String = self.chars[ident_start..self.pos].iter().collect();
            let full = format!("@{}", name);
            let kind = match name.as_str() {
                "nogc" | "import" | "bridge" => TokenKind::Keyword(Box::leak(full.into_boxed_str())),
                _ => TokenKind::Identifier(full),
            };
            return Ok(Token { kind, span: Span::new(start, self.position()) });
        }

        let tok = match (c, next) {
            ('-', '>') => { self.advance(); self.advance(); TokenKind::Arrow }
            ('|', '>') => { self.advance(); self.advance(); TokenKind::Pipe }
            ('?', ':') => { self.advance(); self.advance(); TokenKind::NullCoalesce }
            ('=', '=') => { self.advance(); self.advance(); TokenKind::EqEq }
            ('=', '>') => { self.advance(); self.advance(); TokenKind::FatArrow }
            ('!', '=') => { self.advance(); self.advance(); TokenKind::NotEq }
            ('<', '=') => { self.advance(); self.advance(); TokenKind::LtEq }
            ('>', '=') => { self.advance(); self.advance(); TokenKind::GtEq }
            ('&', '&') => { self.advance(); self.advance(); TokenKind::AndAnd }
            ('|', '|') => { self.advance(); self.advance(); TokenKind::OrOr }
            ('.', '+') => { self.advance(); self.advance(); TokenKind::DotPlus }
            ('.', '-') => { self.advance(); self.advance(); TokenKind::DotMinus }
            ('.', '*') => { self.advance(); self.advance(); TokenKind::DotStar }
            ('.', '/') => { self.advance(); self.advance(); TokenKind::DotSlash }
            ('+', '=') => { self.advance(); self.advance(); TokenKind::PlusEq }
            ('-', '=') => { self.advance(); self.advance(); TokenKind::MinusEq }
            ('*', '=') => { self.advance(); self.advance(); TokenKind::StarEq }
            ('/', '=') => { self.advance(); self.advance(); TokenKind::SlashEq }
            ('%', '=') => { self.advance(); self.advance(); TokenKind::PercentEq }
            ('@', '{') => {
                self.advance();
                self.advance();
                self.push_interpolation(false);
                TokenKind::InterpStart
            }
            _ => {
                self.advance();
                match c {
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Star,
                    '/' => TokenKind::Slash,
                    '%' => TokenKind::Percent,
                    '@' => TokenKind::At,
                    '=' => TokenKind::Eq,
                    '<' => TokenKind::Lt,
                    '>' => TokenKind::Gt,
                    '!' => TokenKind::Not,
                    '?' => TokenKind::Question,
                    '(' => TokenKind::LParen,
                    ')' => TokenKind::RParen,
                    '{' => TokenKind::LBrace,
                    '}' => TokenKind::RBrace,
                    '[' => TokenKind::LBracket,
                    ']' => TokenKind::RBracket,
                    ',' => TokenKind::Comma,
                    ';' => TokenKind::Semi,
                    ':' => TokenKind::Colon,
                    '.' => TokenKind::Dot,
                    _ => return Err(Diagnostic::error("unexpected character", Span::new(start, self.position()))),
                }
            }
        };
        Ok(Token { kind: tok, span: Span::new(start, self.position()) })
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let before = self.pos;
            self.skip_whitespace();
            if self.is_eof() {
                return;
            }
            if self.skip_comment() {
                continue;
            }
            if before == self.pos {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.peek() == '/' && self.peek_next() == '/' {
            self.advance();
            self.advance();
            while !self.is_eof() && self.peek() != '\n' {
                self.advance();
            }
            return true;
        }
        if self.peek() == '/' && self.peek_next() == '*' {
            self.advance();
            self.advance();
            let mut depth = 1;
            while !self.is_eof() {
                if self.peek() == '/' && self.peek_next() == '*' {
                    depth += 1;
                    self.advance();
                    self.advance();
                    continue;
                }
                if self.peek() == '*' && self.peek_next() == '/' {
                    self.advance();
                    self.advance();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                self.advance();
            }
            return true;
        }
        false
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn peek(&self) -> char {
        self.chars.get(self.pos).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.chars.get(self.pos + 1).copied().unwrap_or('\0')
    }

    fn next_char(&mut self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            let c = self.peek();
            self.advance();
            Some(c)
        }
    }

    fn advance(&mut self) {
        if self.is_eof() { return; }
        if self.peek() == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.pos += 1;
    }

    fn position(&self) -> Position {
        Position::new(self.line, self.col, self.pos)
    }

    fn in_interpolation(&self) -> bool {
        !self.interp_stack.is_empty()
    }

    fn push_interpolation(&mut self, resume_string: bool) {
        self.interp_stack.push(InterpContext {
            resume_string,
            depth: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_layout_matches_expected() {
        let source = "abc";
        let lexer = Lexer::new(source);
        assert_eq!(lexer.src, source);
        assert_eq!(lexer.chars.len(), source.chars().count());
        assert_eq!(lexer.pos, 0);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 1);
    }

    #[test]
    fn next_char_reads_utf8() {
        let mut lexer = Lexer::new("\né");
        assert_eq!(lexer.next_char(), Some('\n'));
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.col, 1);
        assert_eq!(lexer.next_char(), Some('é'));
        assert!(lexer.is_eof());
    }

    #[test]
    fn peek_returns_zero_at_eof() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.peek(), '\0');
        assert_eq!(lexer.peek_next(), '\0');
    }

    #[test]
    fn skip_whitespace_handles_spaces_tabs_newlines() {
        let mut lexer = Lexer::new(" \t\nfoo");
        lexer.skip_whitespace();
        assert_eq!(lexer.peek(), 'f');
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.col, 1);
    }

    #[test]
    fn skip_comment_handles_line_comment() {
        let mut lexer = Lexer::new("// comment\nnext");
        assert!(lexer.skip_comment());
        assert_eq!(lexer.peek(), '\n');
    }

    fn tokens(src: &str) -> Vec<Token> {
        Lexer::new(src).tokenize().unwrap()
    }

    fn has_keyword(tokens: &[Token], keyword: &str) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::Keyword(k) if k == keyword))
    }

    fn has_identifier(tokens: &[Token], name: &str) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::Identifier(ref s) if s == name))
    }

    fn has_int_literal(tokens: &[Token], value: i64) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::IntLiteral(v) if v == value))
    }

    fn has_float_literal(tokens: &[Token], value: f64) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::FloatLiteral(v) if (v - value).abs() < f64::EPSILON))
    }

    fn has_string_literal(tokens: &[Token], value: &str) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::StringLiteral(ref s) if s == value))
    }

    fn has_char_literal(tokens: &[Token], value: char) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::CharLiteral(v) if v == value))
    }

    fn has_interp_tokens(tokens: &[Token]) -> bool {
        tokens.iter().any(|t| matches!(t.kind, TokenKind::InterpStart))
            && tokens.iter().any(|t| matches!(t.kind, TokenKind::InterpEnd))
    }

    #[test]
    fn skip_multiline_comment() {
        let tokens = tokens("/* comment */ fun main() -> Int {}");
        assert!(has_keyword(&tokens, "fun"));
    }

    #[test]
    fn skip_nested_multiline_comment() {
        let tokens = tokens("/* outer /* inner */ outer */ fun main() -> Int {}");
        assert!(has_keyword(&tokens, "fun"));
    }

    #[test]
    fn identifier_parsing() {
        let tokens = tokens("myIdentifier");
        assert!(has_identifier(&tokens, "myIdentifier"));
    }

    #[test]
    fn integer_decimal_parsing() {
        let tokens = tokens("12345");
        assert!(has_int_literal(&tokens, 12345));
    }

    #[test]
    fn integer_hex_parsing() {
        let tokens = tokens("0x2A");
        assert!(has_int_literal(&tokens, 42));
    }

    #[test]
    fn integer_binary_parsing() {
        let tokens = tokens("0b1010");
        assert!(has_int_literal(&tokens, 10));
    }

    #[test]
    fn float_standard_parsing() {
        let tokens = tokens("1.5");
        assert!(has_float_literal(&tokens, 1.5));
    }

    #[test]
    fn float_scientific_parsing() {
        let tokens = tokens("1e3");
        assert!(has_float_literal(&tokens, 1e3));
    }

    #[test]
    fn string_literal_parsing() {
        let tokens = tokens("\"hello\"");
        assert!(has_string_literal(&tokens, "hello"));
    }

    #[test]
    fn string_escape_sequences() {
        let tokens = tokens("\"line\\n\\t\\r\\\\\\\"end\"");
        assert!(has_string_literal(&tokens, "line\n\t\r\\\"end"));
    }

    #[test]
    fn unicode_escape_sequence() {
        let tokens = tokens("\"emoji\\u{1F600}\"");
        assert!(has_string_literal(&tokens, "emoji😀"));
    }

    #[test]
    fn string_interpolation_tokens() {
        let tokens = tokens("\"value @{num}\"");
        assert!(has_interp_tokens(&tokens));
    }

    #[test]
    fn raw_string_literal() {
        let tokens = tokens("@\"C:\\path\\file\"");
        assert!(has_string_literal(&tokens, "C:\\path\\file"));
    }

    #[test]
    fn char_literal_parsing() {
        let tokens = tokens("'a'");
        assert!(has_char_literal(&tokens, 'a'));
    }

    #[test]
    fn char_escape_parsing() {
        let tokens = tokens("'\\n'");
        assert!(has_char_literal(&tokens, '\n'));
    }

}
