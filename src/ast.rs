use crate::tree_display::{TreeDisplay, TreeNode};
use derive::TreeDisplay;

pub type NameId = u32;
pub type Operator = u32;
pub type Span = u32;

// ──── Spanned lexeme ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T> {
  pub kind: T,
  pub span: Span,
}

impl<T> From<(T, Span)> for Spanned<T> {
  fn from((kind, span): (T, Span)) -> Self {
    Self { kind, span }
  }
}

// ──── Ast ───────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub enum Ast {
  Identifier(#[tree(child)] Identifier),
  Integer(#[tree(child)] Integer),
  Float(Float),
  Char(Char),
  String(#[tree(child)] StringLiteral),
  Tuple(Tuple),
  Array(Array),
  Let(#[tree(child)] Let),
  Record(Record),
  Call(#[tree(child)] Call),
  Index(Index),
  With(With),
  Block(#[tree(child)] Block),
  Prefix(Prefix),
  Binary(#[tree(child)] Binary),
  Postfix(Postfix),
  Range(#[tree(child)] Range),
  Return(#[tree(child)] Return),
  Yield(#[tree(child)] Yield),
  Break(Break),
  Continue(Continue),
  If(#[tree(child)] If),
  Match(#[tree(child)] Match),
  For(#[tree(child)] For),
  While(#[tree(child)] While),
  Loop(Loop),
  Do(Do),
  Fn(#[tree(child)] Fn),
  Attribute(Attribute),
  RecordDef(RecordDef),
  EnumDef(EnumDef),
  VariantDef(VariantDef),
  ImplDef(ImplDef),
  TraitDef(TraitDef),
  AbilityDef(AbilityDef),
  AttributeValue(AttributeValue),
  Ptr(Ptr),
  SpanType(SpanType),
  ArrayType(ArrayType),
  DynArrayType(DynArrayType),
  MapType(MapType),
  NamedTuple(NamedTuple),
  Error(Span),
}

// ──── Wrapper types ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Elements(#[tree(child)] pub Vec<(Box<Ast>, Option<Span>)>);

impl TreeDisplay for (Box<Ast>, Option<Span>) {
  fn tree(&self) -> TreeNode {
    self.0.tree()
  }
}

// ──── Literal ───────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, TreeDisplay)]
pub struct Identifier {
  pub id: NameId,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Integer(pub Span);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Float(pub Span);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Char(pub Span);

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct StringLiteral(pub Span);

// ──── Aggregate ──────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple {
  pub opener: Span,
  pub elements: Elements,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
  pub opener: Span,
  pub elements: Elements,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
  pub ty: Box<Ast>,
  pub opener: Span,
  pub args: Args,
  pub closer: Span,
}

// ──── Argument ──────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Arg {
  pub label: Option<(Span, Span)>,
  #[tree(child)]
  pub value: Ast,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Args(#[tree(child)] pub Vec<(Arg, Option<Span>)>);

impl TreeDisplay for (Arg, Option<Span>) {
  fn tree(&self) -> TreeNode {
    self.0.tree()
  }
}

// ──── Let ───────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Let {
  pub kw_let: Span,
  pub kw_mut: Option<Span>,
  #[tree(child)]
  pub binding: Box<Ast>,
  #[tree(child)]
  pub ty: MaybeTypeAnnotation,
  pub op_eq: Span,
  #[tree(child)]
  pub value: Box<Ast>,
}

// ──── Type Annotation ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAnnotation {
  pub span: Span,
  pub value: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct MaybeTypeAnnotation(pub Option<TypeAnnotation>);

// ──── Function call ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Call {
  #[tree(child)]
  pub target: Box<Ast>,
  pub opener: Span,
  #[tree(child)]
  pub args: Args,
  pub closer: Span,
}

// ──── Array/tuple access ────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
  pub target: Box<Ast>,
  pub access: Box<Ast>,
}

// ──── With ──────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct With {
  pub target: Box<Ast>,
  pub kw_with: Span,
  pub opener: Span,
  pub args: Args,
  pub closer: Span,
}

// ──── Block ─────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Block {
  pub opener: Span,
  #[tree(child)]
  pub elements: Elements,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Do {
  pub kw: Span,
  pub body: Box<Ast>,
}

// ──── Operators ─────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefix {
  pub op: Spanned<Operator>,
  pub rhs: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Binary {
  #[tree(child)]
  pub lhs: Box<Ast>,
  pub op: Spanned<Operator>,
  #[tree(child)]
  pub rhs: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Postfix {
  pub lhs: Box<Ast>,
  pub op: Spanned<Operator>,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Range {
  #[tree(child)]
  pub begin: MaybeAst,
  pub op: Span,
  #[tree(child)]
  pub end: MaybeAst,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Return {
  pub kw: Span,
  #[tree(child)]
  pub value: MaybeAst,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Yield {
  pub kw: Span,
  #[tree(child)]
  pub value: MaybeAst,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Break {
  pub kw: Span,
  pub label: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Continue {
  pub kw: Span,
  pub label: Option<Span>,
}

// ──── If/Else ───────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct If {
  pub kw: Span,
  #[tree(child)]
  pub condition: Box<Ast>,
  #[tree(child)]
  pub block: Box<Ast>,
  #[tree(child)]
  pub eelse: MaybeElse,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Else {
  pub kw: Span,
  #[tree(child)]
  pub block: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct MaybeElse(#[tree(child)] pub Option<Else>);

// ──── Match ─────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Match {
  pub kw: Span,
  #[tree(child)]
  pub subject: Box<Ast>,
  pub opener: Span,
  #[tree(child)]
  pub cases: MatchCases,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct MatchCase {
  #[tree(child)]
  pub pattern: Box<Ast>,
  #[tree(child)]
  pub guard: Option<(Span, Box<Ast>)>,
  pub arrow: Span,
  #[tree(child)]
  pub body: Box<Ast>,
}

impl TreeDisplay for (Span, Box<Ast>) {
  fn tree(&self) -> TreeNode {
    self.1.tree()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct MatchCases(#[tree(child)] pub Vec<(MatchCase, Option<Span>)>);

impl TreeDisplay for (MatchCase, Option<Span>) {
  fn tree(&self) -> TreeNode {
    self.0.tree()
  }
}

// ──── Loops ─────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct For {
  pub kw: Span,
  #[tree(child)]
  pub binding: Box<Ast>,
  pub kw_in: Span,
  #[tree(child)]
  pub iterable: Box<Ast>,
  #[tree(child)]
  pub body: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct While {
  pub kw: Span,
  #[tree(child)]
  pub condition: Box<Ast>,
  #[tree(child)]
  pub body: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop {
  pub kw: Span,
  pub body: Box<Ast>,
}

// ──── Function ──────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Fn {
  pub kw_fn: Span,
  pub opener: Span,
  #[tree(child)]
  pub params: Params,
  pub closer: Span,
  #[tree(child)]
  pub return_ty: MaybeAst,
  pub kw_with: Option<Span>,
  #[tree(child)]
  pub abilities: Abilities,
  pub kw_where: Option<Span>,
  #[tree(child)]
  pub constraints: Constraints,
  #[tree(child)]
  pub body: MaybeAst,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Param {
  pub label: Option<Identifier>,
  pub name: Identifier,
  pub colon: Span,
  #[tree(child)]
  pub ty: Box<Ast>,
  pub eq: Option<Span>,
  #[tree(child)]
  pub default: MaybeAst,
}

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Params(#[tree(child)] pub Vec<(Param, Option<Span>)>);

impl TreeDisplay for (Param, Option<Span>) {
  fn tree(&self) -> TreeNode {
    self.0.tree()
  }
}

// ──── Pointer ───────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ptr {
  pub op: Span,
  pub kind: PtrKind,
  pub kw: Option<Span>,
  pub ty: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtrKind {
  Const,
  Mut,
  Dyn,
}

// ──── Span/Array ────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanType {
  pub opener: Span,
  pub closer: Span,
  pub ty: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayType {
  pub opener: Span,
  pub size: Box<Ast>,
  pub closer: Span,
  pub ty: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynArrayType {
  pub opener: Span,
  pub kw_dyn: Span,
  pub closer: Span,
  pub ty: Box<Ast>,
}

// ──── Map ───────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapType {
  pub opener: Span,
  pub key: Box<Ast>,
  pub colon: Span,
  pub value: Box<Ast>,
  pub closer: Span,
}

// ──── Named tuple ─────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedField {
  pub name: Span,
  pub colon: Span,
  pub ty: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedFields(pub Vec<(NamedField, Option<Span>)>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedTuple {
  pub opener: Span,
  pub fields: NamedFields,
  pub closer: Span,
}

// ──── Attribute ─────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
  pub at: Span,
  pub opener: Span,
  pub attrs: Elements,
  pub closer: Span,
  pub target: Box<Ast>,
}

// ──── Abilities / Constraints ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Abilities(#[tree(child)] pub Vec<(Box<Ast>, Option<Span>)>);

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct Constraints(#[tree(child)] pub Vec<(Box<Ast>, Option<Span>)>);

// ──── Record definition ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordDef {
  pub kw: Span,
  pub opener: Span,
  pub fields: Fields,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
  pub name: Identifier,
  pub colon: Span,
  pub ty: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fields(pub Vec<(Field, Option<Span>)>);

// ──── Enum definition ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDef {
  pub kw: Span,
  pub base_ty: Box<Ast>,
  pub opener: Span,
  pub cases: EnumCases,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
  pub name: Identifier,
  pub eq: Span,
  pub value: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCases(pub Vec<(EnumCase, Option<Span>)>);

// ──── Variant definition ─────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantDef {
  pub kw: Span,
  pub opener: Span,
  pub cases: VariantCases,
  pub closer: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
  pub name: Identifier,
  pub ty: MaybeTypeAnnotation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCases(pub Vec<(VariantCase, Option<Span>)>);

// ──── Impl definition ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplDef {
  pub kw: Span,
  pub ttrait: MaybeAst,
  pub kw_for: Option<Span>,
  pub ty: Box<Ast>,
  pub kw_where: Option<Span>,
  pub constraints: Constraints,
  pub body: Box<Ast>,
}

// ──── Trait definition ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitDef {
  pub kw: Span,
  pub kw_where: Option<Span>,
  pub constraints: Constraints,
  pub opener: Span,
  pub members: Fields,
  pub closer: Span,
}

// ──── Ability definition ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityDef {
  pub kw: Span,
  pub opener: Span,
  pub members: Fields,
  pub closer: Span,
}

// ──── Attribute value ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeValue {
  pub kw: Span,
  pub opener: Span,
  pub fields: Args,
  pub closer: Span,
}

// ──── MaybeAst ──────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, TreeDisplay)]
pub struct MaybeAst(#[tree(child)] pub Option<Box<Ast>>);

// ──── Constructors ──────────────────────────────────────────────────────────────────────────────

pub fn identifier(span: Span, id: NameId) -> Ast {
  Ast::Identifier(Identifier { span, id })
}

pub fn integer(span: Span) -> Ast {
  Ast::Integer(Integer(span))
}

pub fn float(span: Span) -> Ast {
  Ast::Float(Float(span))
}

pub fn char(span: Span) -> Ast {
  Ast::Char(Char(span))
}

pub fn string(span: Span) -> Ast {
  Ast::String(StringLiteral(span))
}

pub fn tuple(opener: Span, elements: Vec<(Ast, Option<Span>)>, closer: Span) -> Ast {
  Ast::Tuple(Tuple {
    opener,
    elements: Elements(
      elements
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect::<Vec<_>>(),
    ),
    closer,
  })
}

pub fn array(opener: Span, elements: Vec<(Ast, Option<Span>)>, closer: Span) -> Ast {
  Ast::Array(Array {
    opener,
    elements: Elements(
      elements
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect::<Vec<_>>(),
    ),
    closer,
  })
}

pub fn record(ty: Box<Ast>, opener: Span, args: Vec<(Arg, Option<Span>)>, closer: Span) -> Ast {
  Ast::Record(Record {
    ty,
    opener,
    args: Args(args),
    closer,
  })
}

pub fn llet(
  kw_let: Span,
  kw_mut: Option<Span>,
  binding: Ast,
  ty: Option<(Span, Ast)>,
  op_eq: Span,
  value: Ast,
) -> Ast {
  Ast::Let(Let {
    kw_let,
    kw_mut,
    binding: Box::new(binding),
    ty: MaybeTypeAnnotation(ty.map(|(colon, ty)| TypeAnnotation {
      span: colon,
      value: Box::new(ty),
    })),
    op_eq,
    value: Box::new(value),
  })
}

pub fn call(target: Box<Ast>, opener: Span, args: Vec<(Arg, Option<Span>)>, closer: Span) -> Ast {
  Ast::Call(Call {
    target,
    opener,
    args: Args(args),
    closer,
  })
}

pub fn index(target: Box<Ast>, access: Box<Ast>) -> Ast {
  Ast::Index(Index { target, access })
}

pub fn with(
  target: Box<Ast>,
  kw_with: Span,
  opener: Span,
  args: Vec<(Arg, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::With(With {
    target,
    kw_with,
    opener,
    args: Args(args),
    closer,
  })
}

pub fn block(opener: Span, elements: Vec<(Ast, Option<Span>)>, closer: Span) -> Ast {
  Ast::Block(Block {
    opener,
    elements: Elements(
      elements
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect::<Vec<_>>(),
    ),
    closer,
  })
}

pub fn prefix(op: Spanned<Operator>, rhs: Box<Ast>) -> Ast {
  Ast::Prefix(Prefix { op, rhs })
}

pub fn binary(lhs: Box<Ast>, op: Spanned<Operator>, rhs: Box<Ast>) -> Ast {
  Ast::Binary(Binary { lhs, op, rhs })
}

pub fn postfix(lhs: Box<Ast>, op: Spanned<Operator>) -> Ast {
  Ast::Postfix(Postfix { lhs, op })
}

pub fn error(span: Span) -> Ast {
  Ast::Error(span)
}

pub fn range(begin: Option<Ast>, op: Span, end: Option<Ast>) -> Ast {
  Ast::Range(Range {
    begin: MaybeAst(begin.map(Box::new)),
    op,
    end: MaybeAst(end.map(Box::new)),
  })
}

pub fn rreturn(kw: Span, value: Option<Ast>) -> Ast {
  Ast::Return(Return {
    kw,
    value: MaybeAst(value.map(Box::new)),
  })
}

pub fn yyield(kw: Span, value: Option<Ast>) -> Ast {
  Ast::Yield(Yield {
    kw,
    value: MaybeAst(value.map(Box::new)),
  })
}

pub fn bbreak(kw: Span, label: Option<Span>) -> Ast {
  Ast::Break(Break { kw, label })
}

pub fn ccontinue(kw: Span, label: Option<Span>) -> Ast {
  Ast::Continue(Continue { kw, label })
}

pub fn iif(kw: Span, condition: Ast, block: Ast, eelse: Option<Else>) -> Ast {
  Ast::If(If {
    kw,
    condition: Box::new(condition),
    block: Box::new(block),
    eelse: MaybeElse(eelse),
  })
}

pub fn eelse(kw: Span, block: Ast) -> Else {
  Else {
    kw,
    block: Box::new(block),
  }
}

pub fn mmatch(
  kw: Span,
  subject: Ast,
  opener: Span,
  cases: Vec<(MatchCase, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::Match(Match {
    kw,
    subject: Box::new(subject),
    opener,
    cases: MatchCases(cases),
    closer,
  })
}

pub fn match_case(pattern: Ast, guard: Option<(Span, Ast)>, arrow: Span, body: Ast) -> MatchCase {
  MatchCase {
    pattern: Box::new(pattern),
    guard: guard.map(|(kw, v)| (kw, Box::new(v))),
    arrow,
    body: Box::new(body),
  }
}

pub fn ffor(kw: Span, binding: Ast, kw_in: Span, iterable: Ast, body: Ast) -> Ast {
  Ast::For(For {
    kw,
    binding: Box::new(binding),
    kw_in,
    iterable: Box::new(iterable),
    body: Box::new(body),
  })
}

pub fn wwhile(kw: Span, condition: Ast, body: Ast) -> Ast {
  Ast::While(While {
    kw,
    condition: Box::new(condition),
    body: Box::new(body),
  })
}

pub fn lloop(kw: Span, body: Ast) -> Ast {
  Ast::Loop(Loop {
    kw,
    body: Box::new(body),
  })
}

pub fn ddo(kw: Span, body: Ast) -> Ast {
  Ast::Do(Do {
    kw,
    body: Box::new(body),
  })
}

pub fn ffn(
  kw_fn: Span,
  opener: Span,
  params: Vec<(Param, Option<Span>)>,
  closer: Span,
  return_ty: Option<Ast>,
  kw_with: Option<Span>,
  abilities: Vec<(Ast, Option<Span>)>,
  kw_where: Option<Span>,
  constraints: Vec<(Ast, Option<Span>)>,
  body: Option<Ast>,
) -> Ast {
  Ast::Fn(Fn {
    kw_fn,
    opener,
    params: Params(params),
    closer,
    return_ty: MaybeAst(return_ty.map(Box::new)),
    kw_with,
    abilities: Abilities(
      abilities
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect(),
    ),
    kw_where,
    constraints: Constraints(
      constraints
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect(),
    ),
    body: MaybeAst(body.map(Box::new)),
  })
}

pub fn attribute(
  at: Span,
  opener: Span,
  attrs: Vec<(Ast, Option<Span>)>,
  closer: Span,
  target: Ast,
) -> Ast {
  Ast::Attribute(Attribute {
    at,
    opener,
    attrs: Elements(attrs.into_iter().map(|(v, c)| (Box::new(v), c)).collect()),
    closer,
    target: Box::new(target),
  })
}

pub fn record_def(kw: Span, opener: Span, fields: Vec<(Field, Option<Span>)>, closer: Span) -> Ast {
  Ast::RecordDef(RecordDef {
    kw,
    opener,
    fields: Fields(fields),
    closer,
  })
}

pub fn field(name_span: Span, name_id: NameId, colon: Span, ty: Ast) -> Field {
  Field {
    name: Identifier {
      span: name_span,
      id: name_id,
    },
    colon,
    ty: Box::new(ty),
  }
}

pub fn enum_def(
  kw: Span,
  base_ty: Ast,
  opener: Span,
  cases: Vec<(EnumCase, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::EnumDef(EnumDef {
    kw,
    base_ty: Box::new(base_ty),
    opener,
    cases: EnumCases(cases),
    closer,
  })
}

pub fn enum_case(name_span: Span, name_id: NameId, eq: Span, value: Ast) -> EnumCase {
  EnumCase {
    name: Identifier {
      span: name_span,
      id: name_id,
    },
    eq,
    value: Box::new(value),
  }
}

pub fn variant_def(
  kw: Span,
  opener: Span,
  cases: Vec<(VariantCase, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::VariantDef(VariantDef {
    kw,
    opener,
    cases: VariantCases(cases),
    closer,
  })
}

pub fn variant_case(name_span: Span, name_id: NameId, ty: Option<(Span, Ast)>) -> VariantCase {
  VariantCase {
    name: Identifier {
      span: name_span,
      id: name_id,
    },
    ty: MaybeTypeAnnotation(ty.map(|(colon, t)| TypeAnnotation {
      span: colon,
      value: Box::new(t),
    })),
  }
}

pub fn impl_def(
  kw: Span,
  ttrait: Option<Ast>,
  kw_for: Option<Span>,
  ty: Ast,
  kw_where: Option<Span>,
  constraints: Vec<(Ast, Option<Span>)>,
  body: Ast,
) -> Ast {
  Ast::ImplDef(ImplDef {
    kw,
    ttrait: MaybeAst(ttrait.map(Box::new)),
    kw_for,
    ty: Box::new(ty),
    kw_where,
    constraints: Constraints(
      constraints
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect(),
    ),
    body: Box::new(body),
  })
}

pub fn trait_def(
  kw: Span,
  kw_where: Option<Span>,
  constraints: Vec<(Ast, Option<Span>)>,
  opener: Span,
  members: Vec<(Field, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::TraitDef(TraitDef {
    kw,
    kw_where,
    constraints: Constraints(
      constraints
        .into_iter()
        .map(|(v, c)| (Box::new(v), c))
        .collect(),
    ),
    opener,
    members: Fields(members),
    closer,
  })
}

pub fn ability_def(
  kw: Span,
  opener: Span,
  members: Vec<(Field, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::AbilityDef(AbilityDef {
    kw,
    opener,
    members: Fields(members),
    closer,
  })
}

pub fn attribute_value(
  kw: Span,
  opener: Span,
  fields: Vec<(Arg, Option<Span>)>,
  closer: Span,
) -> Ast {
  Ast::AttributeValue(AttributeValue {
    kw,
    opener,
    fields: Args(fields),
    closer,
  })
}

pub fn ptr(op: Span, kind: PtrKind, kw: Option<Span>, ty: Ast) -> Ast {
  Ast::Ptr(Ptr {
    op,
    kind,
    kw,
    ty: Box::new(ty),
  })
}

pub fn span_type(opener: Span, closer: Span, ty: Ast) -> Ast {
  Ast::SpanType(SpanType {
    opener,
    closer,
    ty: Box::new(ty),
  })
}

pub fn array_type(opener: Span, size: Ast, closer: Span, ty: Ast) -> Ast {
  Ast::ArrayType(ArrayType {
    opener,
    size: Box::new(size),
    closer,
    ty: Box::new(ty),
  })
}

pub fn dyn_array_type(opener: Span, kw_dyn: Span, closer: Span, ty: Ast) -> Ast {
  Ast::DynArrayType(DynArrayType {
    opener,
    kw_dyn,
    closer,
    ty: Box::new(ty),
  })
}

pub fn map_type(opener: Span, key: Ast, colon: Span, value: Ast, closer: Span) -> Ast {
  Ast::MapType(MapType {
    opener,
    key: Box::new(key),
    colon,
    value: Box::new(value),
    closer,
  })
}

pub fn named_field(name: Span, colon: Span, ty: Ast) -> NamedField {
  NamedField {
    name,
    colon,
    ty: Box::new(ty),
  }
}

pub fn named_tuple(opener: Span, fields: Vec<(NamedField, Option<Span>)>, closer: Span) -> Ast {
  Ast::NamedTuple(NamedTuple {
    opener,
    fields: NamedFields(fields),
    closer,
  })
}

pub fn param(
  label_span: Option<Span>,
  label_id: Option<NameId>,
  name_span: Span,
  name_id: NameId,
  colon: Span,
  ty: Ast,
  eq: Option<Span>,
  default: Option<Ast>,
) -> Param {
  let label = match (label_span, label_id) {
    (Some(label_span), Some(label_id)) => Some(Identifier {
      span: label_span,
      id: label_id,
    }),
    _ => None,
  };

  Param {
    label,
    name: Identifier {
      span: name_span,
      id: name_id,
    },
    colon,
    ty: Box::new(ty),
    eq,
    default: MaybeAst(default.map(Box::new)),
  }
}
