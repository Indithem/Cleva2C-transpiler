use super::CSyntax;

use super::super::ast::{BinaryOp, InplaceOp, UnaryOp};

impl CSyntax for BinaryOp {
    fn to_c_syntax(&self) -> String {
        use BinaryOp::*;
        match self {
            Add => "+",
            Subtract => "-",
            Multiply => "*",
            Divide => "/",
            Modulus => "%",
            Eq => "==",
            Neq => "!=",
            Lt => "<",
            Gt => ">",
            Lte => "<=",
            Gte => ">=",
            And => "&&",
            Or => "||",
            BitwiseAnd => "&",
            BitwiseOr => "|",
            Xor => "^",
            ShiftLeft => "<<",
            ShiftRight => ">>",
            ShiftCircularLeft => todo!("Need a specific function in C for this"),
            ShiftCircularRight => todo!("Need a specific function in C for this"),
        }
        .to_string()
    }
}

impl CSyntax for InplaceOp{
    fn to_c_syntax(&self) -> String {
        self.to_binary_op().to_c_syntax()
    }
}

impl CSyntax for UnaryOp {
    fn to_c_syntax(&self) -> String {
        use UnaryOp::*;
        match self {
            Negate => "-",
            Not => "!",
            BitwiseNot => "~",
            Increment => "++",
            Decrement => "--",
        }
        .to_string()
    }
}