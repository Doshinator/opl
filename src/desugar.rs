use crate::{Expr, s_expression::SExpr};

// SExpr -> Expr
pub fn desugar(sexpr: &SExpr) -> Expr {
    // Goal:
    // Num -> Expr::Num(n)
    // Sym -> add(desugar(sexpr), desugar(sexpr))
    // List -> desugar(SExpr)
    match sexpr {
        SExpr::Num(n) => Expr::Num(*n),
        SExpr::Sym(s) => panic!("Unexpected symbole: {}", s),
        SExpr::List(list) => {
            if list.is_empty() {
                panic!("Empty list");
            }
            let first = &list[0];
            let rest = &list[1..];

            match first {
                SExpr::Sym(op) => {
                    match op.as_str() {
                        "+" => {
                            if rest.len() != 2 {
                                panic!("+ expects at least 2 args");
                            }
                            
                            let left = desugar(&rest[0]);
                            let right = desugar(&rest[1]);

                            Expr::Add(
                                Box::new(left), 
                                Box::new(right)
                            )
                        },
                        "*" => {
                            if rest.len() != 2 {
                                panic!("* expects at least 2 args");
                            }

                            let left = desugar(&rest[0]);
                            let right = desugar(&rest[1]);
                            
                            Expr::Multiply(
                                Box::new(left),
                                Box::new(right)
                            )
                        },
                        _ => panic!("Not a symbole operator {}", op),
                    }
                },
                _ => panic!("First element must be a symbole"),
            }


        },
    }
}