use crate::ast::*;
use crate::diag::{Diagnostic, Span};
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    diags: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            diags: Vec::new(),
        }
    }

    pub fn parse_program(mut self) -> Result<Program, Vec<Diagnostic>> {
        let mut items = Vec::new();
        while !self.at_eof() {
            match self.parse_item() {
                Ok(item) => items.push(item),
                Err(_) => self.synchronize(),
            }
        }

        if self.diags.is_empty() {
            Ok(Program { items })
        } else {
            Err(self.diags)
        }
    }

    fn parse_item(&mut self) -> Result<Item, ()> {
        if self.match_keyword("@nogc") {
            self.expect_keyword("fun")?;
            return self.parse_fun(true).map(Item::Fun);
        }
        if self.match_keyword("fun") {
            return self.parse_fun(false).map(Item::Fun);
        }
        if self.match_keyword("struct") {
            return self.parse_struct().map(Item::Struct);
        }
        if self.match_keyword("enum") {
            return self.parse_enum().map(Item::Enum);
        }
        if self.match_keyword("type") {
            return self.parse_type_alias().map(Item::TypeAlias);
        }
        if self.match_keyword("view") {
            return self.parse_view().map(Item::View);
        }
        if self.match_keyword("resource") {
            return self.parse_resource().map(Item::Resource);
        }
        if self.check_keyword("let") || self.check_keyword("var") {
            return self.parse_var_decl().map(Item::Const);
        }
        self.parse_stmt().map(Item::Stmt)
    }

    fn parse_fun(&mut self, nogc: bool) -> Result<FunDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        let params = self.parse_param_list()?;
        let ret = if self.match_kind(TokenKind::Arrow) {
            Some(self.parse_type_ref()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        let end = body.span.end;
        Ok(FunDecl { name, params, ret, body, nogc, span: Span::new(start.start, end) })
    }

    fn parse_struct(&mut self) -> Result<StructDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        self.expect_kind(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while !self.check_kind(TokenKind::RBrace) && !self.at_eof() {
            let field_name = self.expect_ident()?;
            self.expect_kind(TokenKind::Colon)?;
            let ty = self.parse_type_ref()?;
            let semi = self.expect_kind(TokenKind::Semi)?;
            fields.push(FieldDecl { name: field_name, ty, span: semi.span });
        }
        let end = self.expect_kind(TokenKind::RBrace)?.span;
        Ok(StructDecl { name, fields, span: Span::new(start.start, end.end) })
    }

    fn parse_enum(&mut self) -> Result<EnumDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        self.expect_kind(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        while !self.check_kind(TokenKind::RBrace) && !self.at_eof() {
            let vname = self.expect_ident()?;
            let mut payload = Vec::new();
            if self.match_kind(TokenKind::LParen) {
                if !self.check_kind(TokenKind::RParen) {
                    payload.push(self.parse_type_ref()?);
                    while self.match_kind(TokenKind::Comma) {
                        payload.push(self.parse_type_ref()?);
                    }
                }
                self.expect_kind(TokenKind::RParen)?;
            }
            let semi = self.expect_kind(TokenKind::Semi)?;
            variants.push(VariantDecl { name: vname, payload, span: semi.span });
        }
        let end = self.expect_kind(TokenKind::RBrace)?.span;
        Ok(EnumDecl { name, variants, span: Span::new(start.start, end.end) })
    }

    fn parse_type_alias(&mut self) -> Result<TypeAliasDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        self.expect_kind(TokenKind::Eq)?;
        let target = self.parse_type_ref()?;
        let end = self.expect_kind(TokenKind::Semi)?.span;
        Ok(TypeAliasDecl { name, target, span: Span::new(start.start, end.end) })
    }

    fn parse_view(&mut self) -> Result<ViewDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        let params = self.parse_param_list()?;
        let body = self.parse_view_block()?;
        let end = body.last().map(|n| n.span).unwrap_or(start);
        Ok(ViewDecl { name, params, body, span: Span::new(start.start, end.end) })
    }

    fn parse_view_block(&mut self) -> Result<Vec<ViewNode>, ()> {
        self.expect_kind(TokenKind::LBrace)?;
        let mut nodes = Vec::new();
        while !self.check_kind(TokenKind::RBrace) && !self.at_eof() {
            let name = self.expect_ident()?;
            let args = self.parse_view_args()?;
            let children = if self.check_kind(TokenKind::LBrace) {
                self.parse_view_block()?
            } else {
                Vec::new()
            };
            let semi = self.expect_kind(TokenKind::Semi)?;
            nodes.push(ViewNode { name, args, children, span: semi.span });
        }
        self.expect_kind(TokenKind::RBrace)?;
        Ok(nodes)
    }

    fn parse_view_args(&mut self) -> Result<Vec<ViewArg>, ()> {
        self.expect_kind(TokenKind::LParen)?;
        let mut args = Vec::new();
        if !self.check_kind(TokenKind::RParen) {
            args.push(self.parse_view_arg()?);
            while self.match_kind(TokenKind::Comma) {
                args.push(self.parse_view_arg()?);
            }
        }
        self.expect_kind(TokenKind::RParen)?;
        Ok(args)
    }

    fn parse_view_arg(&mut self) -> Result<ViewArg, ()> {
        let start = self.current_span();
        let mut name = None;
        if matches!(self.current().kind, TokenKind::Identifier(_)) && self.peek_kind(TokenKind::Colon) {
            name = Some(self.expect_ident()?);
            self.expect_kind(TokenKind::Colon)?;
        }
        let value = self.parse_expr()?;
        Ok(ViewArg { name, value, span: Span::new(start.start, self.prev_span().end) })
    }

    fn parse_resource(&mut self) -> Result<ResourceDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        self.expect_kind(TokenKind::LParen)?;
        let resource_type = self.parse_qualified_ident()?;
        self.expect_kind(TokenKind::RParen)?;
        self.expect_kind(TokenKind::LBrace)?;
        let mut entries = Vec::new();
        while !self.check_kind(TokenKind::RBrace) && !self.at_eof() {
            let key = self.expect_ident()?;
            self.expect_kind(TokenKind::Colon)?;
            let value = self.parse_expr()?;
            let semi = self.expect_kind(TokenKind::Semi)?;
            entries.push(ResourceEntry { key, value, span: semi.span });
        }
        let end = self.expect_kind(TokenKind::RBrace)?.span;
        Ok(ResourceDecl { name, resource_type, entries, span: Span::new(start.start, end.end) })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ()> {
        if self.match_keyword("let") {
            return self.parse_var_decl_with(true).map(Stmt::Var);
        }
        if self.match_keyword("var") {
            return self.parse_var_decl_with(false).map(Stmt::Var);
        }
        if self.match_keyword("return") {
            let start = self.prev_span();
            let expr = if self.check_kind(TokenKind::Semi) {
                None
            } else {
                Some(self.parse_expr()?)
            };
            let end = self.expect_kind(TokenKind::Semi)?.span;
            return Ok(Stmt::Return(expr, Span::new(start.start, end.end)));
        }
        if self.match_keyword("break") {
            let start = self.prev_span();
            let end = self.expect_kind(TokenKind::Semi)?.span;
            return Ok(Stmt::Break(Span::new(start.start, end.end)));
        }
        if self.match_keyword("continue") {
            let start = self.prev_span();
            let end = self.expect_kind(TokenKind::Semi)?.span;
            return Ok(Stmt::Continue(Span::new(start.start, end.end)));
        }
        if self.match_keyword("if") {
            let start = self.prev_span();
            let cond = self.parse_expr()?;
            let then_block = self.parse_block()?;
            let else_stmt = if self.match_keyword("else") {
                if self.check_keyword("if") {
                    Some(Box::new(self.parse_stmt()?))
                } else {
                    Some(Box::new(Stmt::Block(self.parse_block()?)))
                }
            } else {
                None
            };
            let end = match &else_stmt {
                Some(stmt) => match &**stmt {
                    Stmt::Block(b) => b.span.end,
                    Stmt::If(_, _, _, s) => s.end,
                    _ => then_block.span.end,
                },
                None => then_block.span.end,
            };
            return Ok(Stmt::If(cond, then_block, else_stmt, Span::new(start.start, end)));
        }
        if self.match_keyword("while") {
            let start = self.prev_span();
            let cond = self.parse_expr()?;
            let body = self.parse_block()?;
            let end = body.span.end;
            return Ok(Stmt::While(cond, body, Span::new(start.start, end)));
        }
        if self.match_keyword("for") {
            let start = self.prev_span();
            let name = self.expect_ident()?;
            self.expect_keyword("in")?;
            let iter = self.parse_expr()?;
            let body = self.parse_block()?;
            let end = body.span.end;
            return Ok(Stmt::For(name, iter, body, Span::new(start.start, end)));
        }
        if self.match_keyword("match") {
            let start = self.prev_span();
            let expr = self.parse_expr()?;
            let arms = self.parse_match_arms()?;
            let end = arms.last().map(|a| a.span.end).unwrap_or(start.end);
            return Ok(Stmt::Match(expr, arms, Span::new(start.start, end)));
        }
        if self.check_kind(TokenKind::LBrace) {
            return self.parse_block().map(Stmt::Block);
        }

        let expr = self.parse_expr()?;
        let end = self.expect_kind(TokenKind::Semi)?.span;
        Ok(Stmt::Expr(expr, Span::new(end.start, end.end)))
    }

    fn parse_var_decl(&mut self) -> Result<VarDecl, ()> {
        if self.match_keyword("let") {
            self.parse_var_decl_with(true)
        } else if self.match_keyword("var") {
            self.parse_var_decl_with(false)
        } else {
            self.error("expected 'let' or 'var'");
            Err(())
        }
    }

    fn parse_var_decl_with(&mut self, immutable: bool) -> Result<VarDecl, ()> {
        let start = self.prev_span();
        let name = self.expect_ident()?;
        let ty = if self.match_kind(TokenKind::Colon) {
            Some(self.parse_type_ref()?)
        } else {
            None
        };
        self.expect_kind(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        let end = if self.match_kind(TokenKind::Semi) {
            self.prev_span()
        } else {
            // Allow implicit semicolon to support expression-tail blocks.
            self.prev_span()
        };
        Ok(VarDecl { mutable: !immutable, name, ty, value, span: Span::new(start.start, end.end) })
    }

    fn parse_block(&mut self) -> Result<Block, ()> {
        let start = self.expect_kind(TokenKind::LBrace)?.span;
        let mut stmts = Vec::new();
        let mut tail = None;
        while !self.check_kind(TokenKind::RBrace) && !self.at_eof() {
            if self.check_keyword("let") || self.check_keyword("var") || self.check_keyword("return") ||
                self.check_keyword("break") || self.check_keyword("continue") || self.check_keyword("if") ||
                self.check_keyword("while") || self.check_keyword("for") || self.check_keyword("match") ||
                self.check_kind(TokenKind::LBrace) {
                stmts.push(self.parse_stmt()?);
                continue;
            }
            let expr = self.parse_expr()?;
            if self.match_kind(TokenKind::Semi) {
                stmts.push(Stmt::Expr(expr, self.prev_span()));
            } else {
                tail = Some(Box::new(expr));
                break;
            }
        }
        let end = self.expect_kind(TokenKind::RBrace)?.span;
        Ok(Block { stmts, tail, span: Span::new(start.start, end.end) })
    }

    fn parse_expr(&mut self) -> Result<Expr, ()> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr, ()> {
        let mut lhs = self.parse_prefix()?;

        loop {
            if self.match_kind(TokenKind::LParen) {
                let mut args = Vec::new();
                if !self.check_kind(TokenKind::RParen) {
                    args.push(self.parse_expr()?);
                    while self.match_kind(TokenKind::Comma) {
                        args.push(self.parse_expr()?);
                    }
                }
                let end = self.expect_kind(TokenKind::RParen)?.span;
                let span = Span::new(self.span_of(&lhs).start, end.end);
                lhs = Expr::Call { callee: Box::new(lhs), args, span };
                continue;
            }
            if self.match_kind(TokenKind::Dot) {
                let name = self.expect_ident()?;
                let span = Span::new(self.span_of(&lhs).start, self.prev_span().end);
                lhs = Expr::Member { target: Box::new(lhs), name, span };
                continue;
            }
            if self.match_kind(TokenKind::LBracket) {
                let index = self.parse_expr()?;
                let end = self.expect_kind(TokenKind::RBracket)?.span;
                let span = Span::new(self.span_of(&lhs).start, end.end);
                lhs = Expr::Index { target: Box::new(lhs), index: Box::new(index), span };
                continue;
            }
            if self.match_kind(TokenKind::Question) {
                // Treat try operator as a no-op for now to keep parsing self-hosted code.
                continue;
            }

            let (l_bp, r_bp, op) = match self.infix_binding_power() {
                Some(v) => v,
                None => break,
            };
            if l_bp < min_bp {
                break;
            }
            self.advance();
            let rhs = self.parse_expr_bp(r_bp)?;
            let span = Span::new(self.span_of(&lhs).start, self.span_of(&rhs).end);
            lhs = match op {
                InfixOp::Binary(op) => Expr::Binary { left: Box::new(lhs), op, right: Box::new(rhs), span },
                InfixOp::Assign(op) => Expr::Assign { left: Box::new(lhs), op, right: Box::new(rhs), span },
            };
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ()> {
        let tok = self.current().clone();
        match tok.kind {
            TokenKind::IntLiteral(v) => {
                self.advance();
                Ok(Expr::Literal(Literal::Int(v), tok.span))
            }
            TokenKind::FloatLiteral(v) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(v), tok.span))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                if self.check_kind(TokenKind::InterpStart) {
                    self.parse_interpolated_string(Expr::Literal(Literal::String(s), tok.span))
                } else {
                    Ok(Expr::Literal(Literal::String(s), tok.span))
                }
            }
            TokenKind::CharLiteral(c) => {
                self.advance();
                Ok(Expr::Literal(Literal::Char(c), tok.span))
            }
            TokenKind::BoolLiteral(b) => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(b), tok.span))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                if name == "tensor" && self.check_kind(TokenKind::LBracket) {
                    return self.parse_tensor_literal(tok.span);
                }
                if self.check_kind(TokenKind::LBrace) {
                    return self.parse_struct_lit(name, tok.span);
                }
                Ok(Expr::Ident(name, tok.span))
            }
            TokenKind::Keyword("if") => self.parse_if_expr(),
            TokenKind::Keyword("match") => self.parse_match_expr(),
            TokenKind::Keyword(k) if k.starts_with('@') => {
                let name = k.to_string();
                self.advance();
                Ok(Expr::Ident(name, tok.span))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect_kind(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBracket => self.parse_array_literal(),
            TokenKind::LBrace => {
                let block = self.parse_block()?;
                Ok(Expr::Block(block))
            }
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_expr_bp(70)?;
                let span = Span::new(tok.span.start, self.span_of(&expr).end);
                Ok(Expr::Unary { op: UnaryOp::Neg, expr: Box::new(expr), span })
            }
            TokenKind::Plus => {
                self.advance();
                let expr = self.parse_expr_bp(70)?;
                let span = Span::new(tok.span.start, self.span_of(&expr).end);
                Ok(Expr::Unary { op: UnaryOp::Pos, expr: Box::new(expr), span })
            }
            TokenKind::Not => {
                self.advance();
                let expr = self.parse_expr_bp(70)?;
                let span = Span::new(tok.span.start, self.span_of(&expr).end);
                Ok(Expr::Unary { op: UnaryOp::Not, expr: Box::new(expr), span })
            }
            _ => {
                self.error_at(tok.span, "unexpected token in expression");
                Err(())
            }
        }
    }

    fn parse_if_expr(&mut self) -> Result<Expr, ()> {
        let start = self.current_span();
        self.advance();
        let cond = self.parse_expr()?;
        let then_block = self.parse_block()?;
        self.expect_keyword("else")?;
        let else_block = if self.check_keyword("if") {
            let if_expr = self.parse_if_expr()?;
            let span = self.span_of(&if_expr);
            Block { stmts: Vec::new(), tail: Some(Box::new(if_expr)), span }
        } else {
            self.parse_block()?
        };
        let span = Span::new(start.start, else_block.span.end);
        Ok(Expr::If { cond: Box::new(cond), then_block, else_block, span })
    }

    fn parse_match_expr(&mut self) -> Result<Expr, ()> {
        let start = self.current_span();
        self.advance();
        let expr = self.parse_expr()?;
        let arms = self.parse_match_arms()?;
        let end = arms.last().map(|a| a.span.end).unwrap_or(start.end);
        Ok(Expr::Match { expr: Box::new(expr), arms, span: Span::new(start.start, end) })
    }

    fn parse_match_arms(&mut self) -> Result<Vec<MatchArm>, ()> {
        self.expect_kind(TokenKind::LBrace)?;
        let mut arms = Vec::new();
        while !self.check_kind(TokenKind::RBrace) && !self.at_eof() {
            let pat = self.parse_pattern()?;
            self.expect_kind(TokenKind::FatArrow)?;
            let body = if self.check_kind(TokenKind::LBrace) {
                Expr::Block(self.parse_block()?)
            } else {
                self.parse_expr()?
            };
            let end = if self.match_kind(TokenKind::Semi) {
                self.prev_span()
            } else {
                self.span_of(&body)
            };
            arms.push(MatchArm { pat, body, span: Span::new(end.start, end.end) });
        }
        self.expect_kind(TokenKind::RBrace)?;
        Ok(arms)
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ()> {
        let tok = self.current().clone();
        match tok.kind {
            TokenKind::Identifier(ref name) if name == "_" => {
                self.advance();
                Ok(Pattern::Wildcard(tok.span))
            }
            TokenKind::Identifier(mut name) => {
                self.advance();
                while self.match_kind(TokenKind::Dot) {
                    let part = self.expect_ident()?;
                    name = format!("{}.{}", name, part);
                }
                if self.match_kind(TokenKind::LBrace) {
                    let mut fields = Vec::new();
                    if !self.check_kind(TokenKind::RBrace) {
                        loop {
                            let field = self.expect_ident()?;
                            let pat = if self.match_kind(TokenKind::Colon) {
                                self.parse_pattern()?
                            } else {
                                Pattern::Ident(field.clone(), self.prev_span())
                            };
                            fields.push((field, pat));
                            if !self.match_kind(TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    let end = self.expect_kind(TokenKind::RBrace)?.span;
                    return Ok(Pattern::Struct { name, fields, span: Span::new(tok.span.start, end.end) });
                }
                if self.match_kind(TokenKind::LParen) {
                    let mut args = Vec::new();
                    if !self.check_kind(TokenKind::RParen) {
                        args.push(self.parse_pattern()?);
                        while self.match_kind(TokenKind::Comma) {
                            args.push(self.parse_pattern()?);
                        }
                    }
                    let end = self.expect_kind(TokenKind::RParen)?.span;
                    return Ok(Pattern::Variant { name, args, span: Span::new(tok.span.start, end.end) });
                }
                Ok(Pattern::Ident(name, tok.span))
            }
            TokenKind::IntLiteral(v) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Int(v), tok.span))
            }
            TokenKind::FloatLiteral(v) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Float(v), tok.span))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Ok(Pattern::Literal(Literal::String(s), tok.span))
            }
            TokenKind::CharLiteral(c) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Char(c), tok.span))
            }
            TokenKind::BoolLiteral(b) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(b), tok.span))
            }
            TokenKind::LParen => {
                self.advance();
                let mut elems = Vec::new();
                if !self.check_kind(TokenKind::RParen) {
                    elems.push(self.parse_pattern()?);
                    while self.match_kind(TokenKind::Comma) {
                        elems.push(self.parse_pattern()?);
                    }
                }
                let end = self.expect_kind(TokenKind::RParen)?.span;
                Ok(Pattern::Tuple(elems, Span::new(tok.span.start, end.end)))
            }
            _ => {
                self.error_at(tok.span, "invalid pattern");
                Err(())
            }
        }
    }

    fn parse_array_literal(&mut self) -> Result<Expr, ()> {
        let start = self.expect_kind(TokenKind::LBracket)?.span;
        let mut items = Vec::new();
        if !self.check_kind(TokenKind::RBracket) {
            items.push(self.parse_expr()?);
            while self.match_kind(TokenKind::Comma) {
                items.push(self.parse_expr()?);
            }
        }
        let end = self.expect_kind(TokenKind::RBracket)?.span;
        Ok(Expr::Array(items, Span::new(start.start, end.end)))
    }

    fn parse_struct_lit(&mut self, name: String, start: Span) -> Result<Expr, ()> {
        self.expect_kind(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        if !self.check_kind(TokenKind::RBrace) {
            loop {
                let field = self.expect_ident()?;
                self.expect_kind(TokenKind::Colon)?;
                let value = self.parse_expr()?;
                fields.push((field, value));
                if self.match_kind(TokenKind::Comma) {
                    if self.check_kind(TokenKind::RBrace) {
                        break;
                    }
                    continue;
                }
                break;
            }
        }
        let end = self.expect_kind(TokenKind::RBrace)?.span;
        Ok(Expr::StructLit { name, fields, span: Span::new(start.start, end.end) })
    }

    fn parse_tensor_literal(&mut self, start: Span) -> Result<Expr, ()> {
        self.expect_kind(TokenKind::LBracket)?;
        let mut rows = Vec::new();
        if !self.check_kind(TokenKind::RBracket) {
            rows.push(self.parse_tensor_row()?);
            while self.match_kind(TokenKind::Comma) {
                rows.push(self.parse_tensor_row()?);
            }
        }
        let end = self.expect_kind(TokenKind::RBracket)?.span;
        Ok(Expr::Tensor(rows, Span::new(start.start, end.end)))
    }

    fn parse_tensor_row(&mut self) -> Result<Vec<Expr>, ()> {
        self.expect_kind(TokenKind::LBracket)?;
        let mut row = Vec::new();
        if !self.check_kind(TokenKind::RBracket) {
            row.push(self.parse_expr()?);
            while self.match_kind(TokenKind::Comma) {
                row.push(self.parse_expr()?);
            }
        }
        self.expect_kind(TokenKind::RBracket)?;
        Ok(row)
    }

    fn parse_interpolated_string(&mut self, first: Expr) -> Result<Expr, ()> {
        let mut parts = vec![first];
        while self.match_kind(TokenKind::InterpStart) {
            let expr = self.parse_expr()?;
            self.expect_kind(TokenKind::InterpEnd)?;
            parts.push(expr);
            if let TokenKind::StringLiteral(s) = self.current().kind.clone() {
                let span = self.current().span;
                self.advance();
                parts.push(Expr::Literal(Literal::String(s), span));
            } else {
                break;
            }
        }
        let span = Span::new(self.span_of(&parts[0]).start, self.span_of(parts.last().unwrap()).end);
        Ok(Expr::Interpolated { parts, span })
    }

    fn parse_type_ref(&mut self) -> Result<TypeRef, ()> {
        let mut base = if self.match_kind(TokenKind::LParen) {
            let start = self.prev_span();
            let mut elems = Vec::new();
            if !self.check_kind(TokenKind::RParen) {
                elems.push(self.parse_type_ref()?);
                while self.match_kind(TokenKind::Comma) {
                    elems.push(self.parse_type_ref()?);
                }
            }
            let end = self.expect_kind(TokenKind::RParen)?.span;
            TypeRef::Tuple(elems, Span::new(start.start, end.end))
        } else if self.match_kind(TokenKind::LBracket) {
            let start = self.prev_span();
            let inner = self.parse_type_ref()?;
            let end = self.expect_kind(TokenKind::RBracket)?.span;
            TypeRef::Array(Box::new(inner), Span::new(start.start, end.end))
        } else {
            let name = self.parse_qualified_ident()?;
            let span = self.prev_span();
            if name == "Tensor" && self.match_kind(TokenKind::Lt) {
                let elem = self.parse_type_ref()?;
                self.expect_kind(TokenKind::Comma)?;
                let shape = self.parse_shape_ref()?;
                let end = self.expect_kind(TokenKind::Gt)?.span;
                TypeRef::Tensor { elem: Box::new(elem), shape, span: Span::new(span.start, end.end) }
            } else {
                TypeRef::Named(name, span)
            }
        };

        if self.match_kind(TokenKind::Question) {
            let span = self.prev_span();
            base = TypeRef::Optional(Box::new(base), span);
        } else if self.match_kind(TokenKind::Not) {
            let span = self.prev_span();
            base = TypeRef::NonNull(Box::new(base), span);
        }

        Ok(base)
    }

    fn parse_shape_ref(&mut self) -> Result<Vec<ShapeDim>, ()> {
        self.expect_kind(TokenKind::LBracket)?;
        let mut dims = Vec::new();
        if !self.check_kind(TokenKind::RBracket) {
            dims.push(self.parse_shape_dim()?);
            while self.match_kind(TokenKind::Comma) {
                dims.push(self.parse_shape_dim()?);
            }
        }
        self.expect_kind(TokenKind::RBracket)?;
        Ok(dims)
    }

    fn parse_shape_dim(&mut self) -> Result<ShapeDim, ()> {
        let tok = self.current().clone();
        match tok.kind {
            TokenKind::IntLiteral(v) => {
                self.advance();
                Ok(ShapeDim::Int(v))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                if name == "_" {
                    Ok(ShapeDim::Unknown)
                } else {
                    Ok(ShapeDim::Ident(name))
                }
            }
            _ => {
                self.error_at(tok.span, "invalid shape dimension");
                Err(())
            }
        }
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ()> {
        self.expect_kind(TokenKind::LParen)?;
        let mut params = Vec::new();
        if !self.check_kind(TokenKind::RParen) {
            params.push(self.parse_param()?);
            while self.match_kind(TokenKind::Comma) {
                params.push(self.parse_param()?);
            }
        }
        self.expect_kind(TokenKind::RParen)?;
        Ok(params)
    }

    fn parse_param(&mut self) -> Result<Param, ()> {
        let start = self.current_span();
        let name = self.expect_ident()?;
        self.expect_kind(TokenKind::Colon)?;
        let ty = self.parse_type_ref()?;
        Ok(Param { name, ty, span: Span::new(start.start, self.prev_span().end) })
    }

    fn parse_qualified_ident(&mut self) -> Result<String, ()> {
        let mut name = self.expect_ident()?;
        while self.match_kind(TokenKind::Dot) {
            let part = self.expect_ident()?;
            name.push('.');
            name.push_str(&part);
        }
        Ok(name)
    }

    fn infix_binding_power(&self) -> Option<(u8, u8, InfixOp)> {
        match self.current().kind {
            TokenKind::Star => Some((60, 61, InfixOp::Binary(BinaryOp::Mul))),
            TokenKind::Slash => Some((60, 61, InfixOp::Binary(BinaryOp::Div))),
            TokenKind::Percent => Some((60, 61, InfixOp::Binary(BinaryOp::Mod))),
            TokenKind::DotStar => Some((60, 61, InfixOp::Binary(BinaryOp::DotMul))),
            TokenKind::DotSlash => Some((60, 61, InfixOp::Binary(BinaryOp::DotDiv))),
            TokenKind::At => Some((60, 61, InfixOp::Binary(BinaryOp::MatMul))),
            TokenKind::Plus => Some((50, 51, InfixOp::Binary(BinaryOp::Add))),
            TokenKind::Minus => Some((50, 51, InfixOp::Binary(BinaryOp::Sub))),
            TokenKind::DotPlus => Some((50, 51, InfixOp::Binary(BinaryOp::DotAdd))),
            TokenKind::DotMinus => Some((50, 51, InfixOp::Binary(BinaryOp::DotSub))),
            TokenKind::Lt => Some((40, 41, InfixOp::Binary(BinaryOp::Lt))),
            TokenKind::LtEq => Some((40, 41, InfixOp::Binary(BinaryOp::LtEq))),
            TokenKind::Gt => Some((40, 41, InfixOp::Binary(BinaryOp::Gt))),
            TokenKind::GtEq => Some((40, 41, InfixOp::Binary(BinaryOp::GtEq))),
            TokenKind::EqEq => Some((35, 36, InfixOp::Binary(BinaryOp::Eq))),
            TokenKind::NotEq => Some((35, 36, InfixOp::Binary(BinaryOp::NotEq))),
            TokenKind::AndAnd => Some((30, 31, InfixOp::Binary(BinaryOp::And))),
            TokenKind::OrOr => Some((25, 26, InfixOp::Binary(BinaryOp::Or))),
            TokenKind::NullCoalesce => Some((20, 20, InfixOp::Binary(BinaryOp::NullCoalesce))),
            TokenKind::Pipe => Some((15, 16, InfixOp::Binary(BinaryOp::Pipe))),
            TokenKind::Arrow => Some((15, 16, InfixOp::Binary(BinaryOp::Arrow))),
            TokenKind::Eq => Some((10, 10, InfixOp::Assign(AssignOp::Assign))),
            TokenKind::PlusEq => Some((10, 10, InfixOp::Assign(AssignOp::AddAssign))),
            TokenKind::MinusEq => Some((10, 10, InfixOp::Assign(AssignOp::SubAssign))),
            TokenKind::StarEq => Some((10, 10, InfixOp::Assign(AssignOp::MulAssign))),
            TokenKind::SlashEq => Some((10, 10, InfixOp::Assign(AssignOp::DivAssign))),
            TokenKind::PercentEq => Some((10, 10, InfixOp::Assign(AssignOp::ModAssign))),
            _ => None,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn current_span(&self) -> Span {
        self.current().span
    }

    fn prev_span(&self) -> Span {
        self.tokens[self.pos.saturating_sub(1)].span
    }

    fn span_of(&self, expr: &Expr) -> Span {
        match expr {
            Expr::Literal(_, s) => *s,
            Expr::Ident(_, s) => *s,
            Expr::StructLit { span, .. } => *span,
            Expr::Unary { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
            Expr::Assign { span, .. } => *span,
            Expr::Call { span, .. } => *span,
            Expr::Member { span, .. } => *span,
            Expr::Index { span, .. } => *span,
            Expr::If { span, .. } => *span,
            Expr::Match { span, .. } => *span,
            Expr::Block(b) => b.span,
            Expr::Array(_, s) => *s,
            Expr::Tensor(_, s) => *s,
            Expr::Interpolated { span, .. } => *span,
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.at_eof() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).unwrap()
    }

    fn at_eof(&self) -> bool {
        matches!(self.current().kind, TokenKind::Eof)
    }

    fn check_kind(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    fn peek_kind(&self, kind: TokenKind) -> bool {
        if self.pos + 1 >= self.tokens.len() {
            return false;
        }
        self.tokens[self.pos + 1].kind == kind
    }

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.check_kind(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_keyword(&mut self, kw: &str) -> bool {
        if self.check_keyword(kw) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check_keyword(&self, kw: &str) -> bool {
        matches!(self.current().kind, TokenKind::Keyword(k) if k == kw)
    }

    fn expect_keyword(&mut self, kw: &str) -> Result<(), ()> {
        if self.check_keyword(kw) {
            self.advance();
            Ok(())
        } else {
            self.error(&format!("expected keyword '{kw}'"));
            Err(())
        }
    }

    fn expect_kind(&mut self, kind: TokenKind) -> Result<Token, ()> {
        if self.check_kind(kind.clone()) {
            Ok(self.advance().clone())
        } else {
            self.error(&format!("expected {:?}", kind));
            Err(())
        }
    }

    fn expect_ident(&mut self) -> Result<String, ()> {
        match self.current().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            _ => {
                self.error("expected identifier");
                Err(())
            }
        }
    }

    fn error(&mut self, msg: &str) {
        let span = self.current_span();
        self.diags.push(Diagnostic::new(msg, span));
    }

    fn error_at(&mut self, span: Span, msg: &str) {
        self.diags.push(Diagnostic::new(msg, span));
    }

    fn synchronize(&mut self) {
        while !self.at_eof() {
            if matches!(self.current().kind, TokenKind::Semi | TokenKind::RBrace) {
                self.advance();
                break;
            }
            if self.check_keyword("fun") || self.check_keyword("struct") || self.check_keyword("enum") ||
                self.check_keyword("type") || self.check_keyword("view") || self.check_keyword("resource") ||
                self.check_keyword("let") || self.check_keyword("var") || self.check_keyword("if") ||
                self.check_keyword("while") || self.check_keyword("for") || self.check_keyword("match") ||
                self.check_keyword("return") {
                break;
            }
            self.advance();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum InfixOp {
    Binary(BinaryOp),
    Assign(AssignOp),
}
