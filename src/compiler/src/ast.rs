use crate::diag::Span;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Fun(FunDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    TypeAlias(TypeAliasDecl),
    View(ViewDecl),
    Resource(ResourceDecl),
    Const(VarDecl),
    Stmt(Stmt),
    Interface(InterfaceDecl),
    Sealed(SealedDecl),
}

#[derive(Debug, Clone)]
pub struct InterfaceDecl {
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub methods: Vec<FunSig>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: String,
    pub constraints: Vec<TypeRef>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunSig {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Option<TypeRef>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SealedDecl {
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub receiver: Option<TypeRef>,
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub ret: Option<TypeRef>,
    pub body: Block,
    pub nogc: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: TypeRef,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub implements: Vec<TypeRef>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FieldDecl {
    pub name: String,
    pub ty: TypeRef,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub variants: Vec<VariantDecl>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariantDecl {
    pub name: String,
    pub payload: Vec<TypeRef>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TypeAliasDecl {
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub target: TypeRef,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ViewDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<ViewNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ViewNode {
    pub name: String,
    pub args: Vec<ViewArg>,
    pub children: Vec<ViewNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ViewArg {
    pub name: Option<String>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ResourceDecl {
    pub name: String,
    pub resource_type: String,
    pub entries: Vec<ResourceEntry>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ResourceEntry {
    pub key: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub mutable: bool,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Var(VarDecl),
    Expr(Expr, Span),
    Return(Option<Expr>, Span),
    Break(Span),
    Continue(Span),
    If(Expr, Block, Option<Box<Stmt>>, Span),
    While(Expr, Block, Span),
    For(String, Expr, Block, Span),
    Match(Expr, Vec<MatchArm>, Span),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub tail: Option<Box<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pat: Pattern,
    pub body: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal, Span),
    Ident(String, Span),
    StructLit { name: String, fields: Vec<(String, Expr)>, span: Span },
    Unary { op: UnaryOp, expr: Box<Expr>, span: Span },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr>, span: Span },
    Assign { left: Box<Expr>, op: AssignOp, right: Box<Expr>, span: Span },
    Call { callee: Box<Expr>, args: Vec<Expr>, span: Span },
    Member { target: Box<Expr>, name: String, span: Span },
    Index { target: Box<Expr>, index: Box<Expr>, span: Span },
    If { cond: Box<Expr>, then_block: Block, else_block: Block, span: Span },
    Match { expr: Box<Expr>, arms: Vec<MatchArm>, span: Span },
    Block(Block),
    Array(Vec<Expr>, Span),
    Tensor(Vec<Vec<Expr>>, Span),
    Interpolated { parts: Vec<Expr>, span: Span },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Neg,
    Pos,
    BitNot,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    DotAdd,
    DotSub,
    DotMul,
    DotDiv,
    MatMul,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    NullCoalesce,
    Pipe,
    Arrow,
}

#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Ident(String, Span),
    Wildcard(Span),
    Literal(Literal, Span),
    Tuple(Vec<Pattern>, Span),
    Variant { name: String, args: Vec<Pattern>, span: Span },
    Struct { name: String, fields: Vec<(String, Pattern)>, span: Span },
    Is(TypeRef, Box<Pattern>, Span),
}

#[derive(Debug, Clone)]
pub enum TypeRef {
    Named(String, Vec<TypeRef>, Span),
    Tuple(Vec<TypeRef>, Span),
    Array(Box<TypeRef>, Span),
    Tensor { elem: Box<TypeRef>, shape: Vec<ShapeDim>, span: Span },
    Optional(Box<TypeRef>, Span),
    NonNull(Box<TypeRef>, Span),
}

#[derive(Debug, Clone)]
pub enum ShapeDim {
    Int(i64),
    Ident(String),
    Unknown,
}
