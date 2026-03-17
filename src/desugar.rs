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
                _ => Expr::Var(s.clone()),
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
                        "λ" | "lambda" => {
                            if rest.len() != 2 {
                                panic!("lambda expects 2 args: (λ (params...) body)");
                            }
                            
                            // Extract parameters from (x y z)
                            let params = extract_params(&rest[0]);
                            
                            // Desugar the body
                            let body = desugar(&rest[1]);
                            
                            Expr::Lambda(params, Box::new(body))
                        },
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
                        _ => {
                            // Not a known operator, treat as function application
                            // (f arg1 arg2) where f is a variable
                            let func = Expr::Var(op.clone());
                            let args: Vec<Expr> = rest.iter().map(desugar).collect();
                            Expr::App(Box::new(func), args)
                        },
                    }
                },
                _ => {
                    // First element is not a symbol (could be a lambda)
                    // Example: ((λ (x) x) 5)
                    let func = desugar(first);
                    let args: Vec<Expr> = rest.iter().map(desugar).collect();
                    Expr::App(Box::new(func), args)
                }
            }
        },
    }
}

fn extract_params(sexpr: &SExpr) -> Vec<String> {
    match sexpr {
        SExpr::List(items) => {
            items.iter().map(|item| {
                match item {
                    SExpr::Sym(name) => name.clone(),
                    _ => panic!("Parameter must be a symbol, got: {:?}", item),
                }
            }).collect()
        },
        _ => panic!("Parameters must be a list, got: {:?}", sexpr),
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