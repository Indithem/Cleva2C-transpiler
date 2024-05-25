//! All the syntactical operators (such as +-*&&^) are originated here.
//!
//! If you add any other kinds of operators, be sure to 
//! also modify them in Expressions and Statements modules.
//! Also implement ClevaSyntax trait for them.(in the transpiler/reserved_keywords module)
//! 


/// For operating on two expressions
/// 
/// Type checking will be done later.
#[cfg_attr(test, derive(strum::EnumIter, PartialEq))]
#[derive(Clone)]
pub enum BinaryOp {
    // for numbers
    /// The `+` operator
    Add,
    /// The `-` operator
    Subtract,
    /// The `*` operator
    Multiply,
    /// The `/` operator
    Divide,
    /// The `%` operator
    Modulus,

    // for bits
    /// The `&` operator
    BitwiseAnd,
    /// The `|` operator
    BitwiseOr,

    // for binary
    /// The `&&` operator
    And,
    /// The `||` operator
    Or,
    /// The `^` operator
    /// 
    /// ##Note
    /// There is no `BitwiseXor` operator, hence `^` is the same as `BitwiseXor`
    Xor,
    /// The `<<` operator
    ShiftLeft,
    /// The `>>` operator
    ShiftRight,
    /// The `<<@` operator
    ShiftCircularLeft,
    /// The `>>@` operator
    ShiftCircularRight,

    // comparisions
    /// The `<` operator
    Lt,
    /// The `>` operator
    Gt,
    /// The `<=` operator
    Lte,
    /// The `>=` operator
    Gte,
    /// The `==` operator
    Eq,
    /// The `!=` operator
    Neq,
}

/// For operating on one expression
/// 
/// Type checking will be done later.
#[cfg_attr(test, derive(strum::EnumIter, PartialEq))]
#[derive(Clone)]
pub enum UnaryOp {
    // for numbers
    /// The `-` operator
    Negate,
    /// The `++` operator
    // todo! Make postfix and prefix increment and decrement
    Increment,
    /// The `--` operator
    Decrement,

    // for bits
    /// The `~` operator
    BitwiseNot,

    // for binary
    /// The `!` operator
    Not,
}

/// For inplace operations
/// such as +=
/// 
/// These are just some syntac sugars.
/// x #= y is the same as x = x#y.
#[cfg_attr(test, derive(strum::EnumIter, PartialEq))]
#[derive(Clone)]
pub enum InplaceOp{
    /// The `+=` operator
    Add,
    /// The `-=` operator
    Subtract,
    /// The `*=` operator
    Multiply,
    /// The `/=` operator
    Divide,
    /// The `%=` operator
    Modulus,
    /// The `&=` operator
    BitwiseAnd,
    /// The `|=` operator
    BitwiseOr,
    /// The `^=` operator
    BitwiseXor,
    /// The `&&=` operator
    And,
    /// The `||=` operator
    Or,
    /// The `^=` operator
    Xor,
    /// The `<<=` operator
    ShiftLeft,
    /// The `>>=` operator
    ShiftRight,
    /// The `<<@=` operator
    ShiftCircularLeft,
    /// The `>>@=` operator
    ShiftCircularRight,
}

impl InplaceOp {
    pub fn to_binary_op(&self) -> BinaryOp {
        match self {
            InplaceOp::Add => BinaryOp::Add,
            InplaceOp::Subtract => BinaryOp::Subtract,
            InplaceOp::Multiply => BinaryOp::Multiply,
            InplaceOp::Divide => BinaryOp::Divide,
            InplaceOp::Modulus => BinaryOp::Modulus,
            InplaceOp::BitwiseAnd => BinaryOp::BitwiseAnd,
            InplaceOp::BitwiseOr => BinaryOp::BitwiseOr,
            InplaceOp::BitwiseXor => BinaryOp::Xor,
            InplaceOp::And => BinaryOp::And,
            InplaceOp::Or => BinaryOp::Or,
            InplaceOp::Xor => BinaryOp::Xor,
            InplaceOp::ShiftLeft => BinaryOp::ShiftLeft,
            InplaceOp::ShiftRight => BinaryOp::ShiftRight,
            InplaceOp::ShiftCircularLeft => BinaryOp::ShiftCircularLeft,
            InplaceOp::ShiftCircularRight => BinaryOp::ShiftCircularRight,
        }
    }
}