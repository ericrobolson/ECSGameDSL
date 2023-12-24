use crate::parser::{ListType, Listable, Primitive, PrimitiveType};

/// Intermediate representation of compiler primitives.
#[derive(Debug, Clone, PartialEq)]
pub enum IR {
    Struct(Commentable<Struct>),
    Expression(Commentable<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Commentable<T> {
    pub comments: Vec<String>,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub id: String,
    pub args: Vec<ExpressionArg>,
    pub body: Vec<ExpressionStatement>,
    pub return_type: Listable<Primitive>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionStatement {
    Comment(String),
    Literal(Literal),
    Return(Box<ExpressionStatement>),
    Call {
        id: String,
        args: Vec<ExpressionStatement>,
    },
    NativeExpression(NativeExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NativeExpression {
    Add {
        lhs: Box<ExpressionStatement>,
        rhs: Box<ExpressionStatement>,
    },
    Multiply {
        lhs: Box<ExpressionStatement>,
        rhs: Box<ExpressionStatement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Bool(bool),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionArg {
    pub id: String,
    pub ty: PrimitiveType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub id: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub id: String,
    pub ty: ListType<Primitive>,
}
