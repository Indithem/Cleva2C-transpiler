/// This is responsible for compiling the AST into C code.
///
/// 
/// Use Transpiler::transpile(ast) to achieve this. 

use super::ast::{
    statements::{Statements, Statement},
    expression::{Expression, ExpressionKind}
};

use super::c_translations::CSyntax;


// this is a struct, since we might want to add some configuration options later.
// configurations include:
//  the types of the variables etc..
pub struct Transpiler;
pub struct Formatter;

impl Formatter{
    pub fn main(s:String) -> String {
        format!("int main(){{
            {}
        }}", s)
    }
}

impl Transpiler{
    pub fn transpile(ast: &Statements) -> String{
        Formatter::main(Self::transpile_statements(ast))
    }

    fn transpile_statement(ast: &Statement) -> String {
        use Statement::*;
        match ast {
            Expression(e) => Self::transpile_expression(e),
            Assignment(var, e) =>
                format!("{} = {}", var, Self::transpile_expression(e)),
            Inplace(op, var, e) =>
                format!("{} = {} {} ({})", var, var, op.to_c_syntax(), Self::transpile_expression(e)),
        }
    }

    fn transpile_statements(ast: &Statements) -> String {
        ast.iter()
            .map(|ast| Self::transpile_statement(ast))
            .map(|mut s| {s.push('\n'); s})
            .collect()
    }            

    fn transpile_expression(ast:&Expression) -> String {
        use ExpressionKind::*;
        match &ast.kind {
            Variable(s)   => s.to_string(),
            Literal => "1".to_string(), //todo!
            Unary(op, e) => format!("{}({})", op.to_c_syntax(), Self::transpile_expression(&*e)),
            Binary(op, e1, e2) => format!("({}){}({})", Self::transpile_expression(&*e1), op.to_c_syntax(), Self::transpile_expression(&*e2)),
            Block(statements, e) => format!("{}; {}", Self::transpile_statements(&statements), Self::transpile_expression(&*e))
        }
    }
}

