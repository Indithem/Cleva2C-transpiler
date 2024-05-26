use strum::IntoEnumIterator;

use super::ast::{
    Expression, ExpressionKind,
    BinaryOp, InplaceOp, UnaryOp,
    Statement,
};
use super::{Transpiler, Formatter, CSyntax};

#[test]
fn binary_ops() {
    let x = Box::new(Expression {
        kind: ExpressionKind::Variable("x".to_string()),
    });

    for op in BinaryOp::iter().filter(|op| *op!=BinaryOp::ShiftCircularLeft && *op!=BinaryOp::ShiftCircularRight) {
        let ast = vec![Statement::Expression(Expression {
            kind: ExpressionKind::Binary(
                op.clone(),
                x.clone(),
                Box::new(Expression {
                    kind: ExpressionKind::Variable("y".to_string()),
                }),
            ),
        })];
    
        let translation = Transpiler::transpile(&ast);
    
        assert_eq!(translation, Formatter::main(format!("(x){}(y)\n", op.to_c_syntax())));
    }
}

#[test]
fn unary_ops() {
    let x = Box::new(Expression {
        kind: ExpressionKind::Variable("x".to_string()),
    });

    for op in UnaryOp::iter() {
        let ast = vec![Statement::Expression(Expression {
            kind: ExpressionKind::Unary(
                op.clone(),
                x.clone(),
            ),
        })];

        let translation = Transpiler::transpile(&ast);

        assert_eq!(translation, Formatter::main(format!("{}(x)\n", op.to_c_syntax())));

    }
}

#[test]
fn inplace_ops() {

    for op in InplaceOp::iter().filter(|op| *op!=InplaceOp::ShiftCircularLeft && *op!=InplaceOp::ShiftCircularRight) {
        let ast = vec![Statement::Inplace(
            op.clone(),
            "x".to_string(),
            Expression {
                kind: ExpressionKind::Variable("y".to_string()),
            },
        )];

        let translation= Transpiler::transpile(&ast);
    
        assert_eq!(translation, Formatter::main(format!("x = x {} (y)\n", op.to_c_syntax())));
    }
}
