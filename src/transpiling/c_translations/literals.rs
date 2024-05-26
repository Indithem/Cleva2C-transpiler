use super::super::ast::Literals;

impl super::CSyntax for Literals {
    fn to_c_syntax(&self) -> String {
        use Literals::*;
        match self {
            Number(n) => n.to_string(),
            Boolean(b) => b.to_string(),
            String(s) => format!("\"{}\"", s),
        }
    }
}