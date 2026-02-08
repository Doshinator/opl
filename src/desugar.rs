use crate::{Expr, s_expression::SExpr};

// SExpr -> Expr
pub fn desugar(sexpr: &SExpr) -> Expr {
    // Goal:
    // Num -> Expr::Num(n)
    // Sym -> add(desugar(sexpr), desugar(sexpr))
    // List -> desugar(SExpr)
    match sexpr {
        SExpr::Num(n) => {
            Expr::Num(*n)
        },
        SExpr::Sym(s) => {
            todo!()
        },
        SExpr::List(list) => {
            todo!()
        },
        _ => todo!(),
    }
}