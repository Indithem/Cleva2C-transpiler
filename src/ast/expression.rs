use super::{
    operators::{BinaryOp, UnaryOp},
    statements::Statements,
    literals::Literals,
};

/// Represents an expression in the AST
#[cfg_attr(test, derive(Clone))]
pub struct Expression {
    pub kind: ExpressionKind,
}

#[cfg_attr(test, derive(Clone))]
pub enum ExpressionKind {
    // base cases
    Variable(String),
    Literal(Literals),

    // operations
    Unary(UnaryOp, Box<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),    

    /// sequence of statements, followed by an expression
    ///
    /// The last expression is the 'resultant' of the block,
    /// think of this as let ... in ... end in OCaml.
    ///
    /// If the last expression was also made a statement(by use of `;`),
    /// then it will be parsed as a
    Block(Statements, Box<Expression>),

}
