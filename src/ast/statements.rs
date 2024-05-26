use super::{
    expression::Expression,
    operators::InplaceOp
};

/// A statement is a sequence of expressions
/// all expressions are of type (), the 'unit' type
pub type Statements = Vec<Statement>;

#[cfg_attr(test, derive(Clone))]
pub enum Statement {
    Expression(Expression),

    /// variable assignment/creation
    Assignment(String, Expression),

    Inplace(InplaceOp, String, Expression),

}