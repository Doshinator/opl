use crate::{Expr, s_expression::SExpr};

// SExpr -> Expr
pub fn desugar(sexpr: &SExpr) -> Expr {
    // Goal:
    // Num -> Expr::Num(n)
    // Sym -> add(desugar(sexpr), desugar(sexpr))
    // List -> desugar(SExpr)
    Expr::Num(5)
}