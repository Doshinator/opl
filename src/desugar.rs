use crate::{Expr, s_expression::SExpr};

// SExpr -> Expr
pub fn desugar(sexpr: &SExpr) -> Expr {
    // Goal:
    // Num -> Expr::Num(n)
    // Sym -> add(desugar(sexpr), desugar(sexpr))
    // List -> desugar(SExpr)
    match sexpr {
        SExpr::Num(n) => Expr::Num(*n),
        SExpr::Sym(s) => {
            match s.as_str() {
                "true" => Expr::Bool(true),
                "false" => Expr::Bool(false),
                _ => panic!("Unexpected symbol {}", s),
            }
        },
        SExpr::List(list) => {
            if list.is_empty() {
                panic!("Empty list");
            }
            let first = &list[0];
            let rest = &list[1..];

            match first {
                SExpr::Sym(op) => {
                    match op.as_str() {
                        "+" => desugar_binary_op(op, rest),
                        "-" => desugar_binary_op(op, rest),
                        "*" => desugar_binary_op(op, rest),
                        "/" => desugar_binary_op(op, rest),
                        "<" => desugar_conditional(op, rest),
                        "<=" => desugar_conditional(op, rest),
                        ">" => desugar_conditional(op, rest),
                        ">=" => desugar_conditional(op, rest),
                        "=" => desugar_conditional(op, rest),
                        "if" => {
                            if rest.len() != 3 {
                                panic!("if expects 3 args: (if condition then-expr else-expr)")
                            }

                            let cond = desugar(&rest[0]);
                            let then_expr = desugar(&rest[1]);
                            let else_expr = desugar(&rest[2]);
                            
                            Expr::If(
                                Box::new(cond), 
                                Box::new(then_expr), 
                                Box::new(else_expr)
                            )
                        },
                        _ => panic!("Unknown operator: {}", op),
                    }
                },
                _ => panic!("First element must be a symbole"),
            }
        },
    }
}

fn desugar_binary_op(op: &str, rest: &[SExpr]) -> Expr {
    if rest.len() != 2 {
        panic!("{} expects at least 2 args", op);
    }
    
    let left = desugar(&rest[0]);
    let right = desugar(&rest[1]);

    match op {
        "+" => Expr::Add(Box::new(left), Box::new(right)),
        "-" => Expr::Sub(Box::new(left), Box::new(right)),
        "*" => Expr::Mul(Box::new(left), Box::new(right)),
        "/" => Expr::Div(Box::new(left), Box::new(right)),
        _ => panic!("{} is not a supported binary operator", op),
    }
}

fn desugar_conditional(op: &str, rest: &[SExpr]) -> Expr {
    if rest.len() != 2 {
        panic!("{} expected at least 2 args", op);
    }

    match op {
        "<" => {
            let left = desugar(&rest[0]);
            let right = desugar(&rest[1]);
            Expr::Less(Box::new(left), Box::new(right))
        },
        "<=" => {
            let left = desugar(&rest[0]);
            let right = desugar(&rest[1]);
            Expr::LessEq(Box::new(left), Box::new(right))
        },
        ">" => {
            let left = desugar(&rest[0]);
            let right = desugar(&rest[1]);
            Expr::Greater(Box::new(left), Box::new(right))
        },
        ">=" => {
            let left = desugar(&rest[0]);
            let right = desugar(&rest[1]);
            Expr::GreaterEq(Box::new(left), Box::new(right))
        },
        "=" => {
            let left = desugar(&rest[0]);
            let right = desugar(&rest[1]);
            Expr::Equal(Box::new(left), Box::new(right))
        },
        _ => panic!("{} is not a supported conditional operator", op),
    }
}