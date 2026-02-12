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
            diags.push(Diagnostic::new(
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
            "fun" | "let" | "var" | "if" | "else" | "match" | "for" | "while" |
            "break" | "continue" | "return" | "view" | "resource" | "state" |
            "spawn" | "@nogc" | "import" | "as" | "struct" | "enum" | "type" |
            "in" => TokenKind::Keyword(Box::leak(s.into_boxed_str())),
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
                return Err(Diagnostic::new("invalid hex literal", Span::new(start_pos, self.position())));
            }
            let s: String = self.chars[hex_start..self.pos].iter().collect();
            let v = i64::from_str_radix(&s, 16)
                .map_err(|_| Diagnostic::new("invalid hex literal", Span::new(start_pos, self.position())))?;
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
                .map_err(|_| Diagnostic::new("invalid float literal", Span::new(start_pos, self.position())))?;
            Ok(Token { kind: TokenKind::FloatLiteral(v), span: Span::new(start_pos, self.position()) })
        } else {
            let v = s.parse::<i64>()
                .map_err(|_| Diagnostic::new("invalid int literal", Span::new(start_pos, self.position())))?;
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
                    return Err(Diagnostic::new("unterminated escape", Span::new(start, self.position())));
                }
                let esc = self.peek();
                let real = match esc {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '0' => '\0',
                    '\\' => '\\',
                    '"' => '"',
                    '{' => '{',
                    '}' => '}',
                    _ => return Err(Diagnostic::new("invalid escape", Span::new(start, self.position()))),
                };
                out.push(real);
                self.advance();
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
        Err(Diagnostic::new("unterminated string", Span::new(start, self.position())))
    }

    fn lex_char(&mut self) -> Result<Token, Diagnostic> {
        let start = self.position();
        self.advance();
        if self.is_eof() {
            return Err(Diagnostic::new("unterminated char", Span::new(start, self.position())));
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
                _ => return Err(Diagnostic::new("invalid escape", Span::new(start, self.position()))),
            };
            self.advance();
            real
        } else {
            self.advance();
            c
        };
        if self.is_eof() || self.peek() != '\'' {
            return Err(Diagnostic::new("unterminated char", Span::new(start, self.position())));
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
                    _ => return Err(Diagnostic::new("unexpected character", Span::new(start, self.position()))),
                }
            }
        };
        Ok(Token { kind: tok, span: Span::new(start, self.position()) })
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            while !self.is_eof() && self.peek().is_whitespace() {
                self.advance();
            }
            if self.is_eof() {
                return;
            }
            if self.peek() == '/' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                while !self.is_eof() && self.peek() != '\n' {
                    self.advance();
                }
                continue;
            }
            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();
                while !self.is_eof() {
                    if self.peek() == '*' && self.peek_next() == '/' {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn peek(&self) -> char {
        self.chars[self.pos]
    }

    fn peek_next(&self) -> char {
        if self.pos + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.pos + 1]
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
