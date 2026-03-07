use core::panic;

use crate::{Expr, Value};

// ex: (+ 1 (if (+ 2 []) 3 4)) -> AppC([+ 1] (If1C (AppC [+ 2] Hole []) 3 4) [])
#[derive(Debug, PartialEq, Clone)]
pub enum Context {
    // empty context
    Hole,
    // Arithmatic - Left side
    AddL(Box<Context>, Box<Expr>),
    SubL(Box<Context>, Box<Expr>),
    MulL(Box<Context>, Box<Expr>),
    DivL(Box<Context>, Box<Expr>),
    
    // Arithmatic - Right side 
    // Value here becaue we have left to right evaluation rules
    AddR(Value, Box<Context>),
    SubR(Value, Box<Context>),
    MulR(Value, Box<Context>),
    DivR(Value, Box<Context>),

    // Comparison - Left side
    LessL(Box<Context>, Box<Expr>),
    LessEqL(Box<Context>, Box<Expr>),
    GreaterL(Box<Context>, Box<Expr>),
    GreaterEqL(Box<Context>, Box<Expr>),
    EqualL(Box<Context>, Box<Expr>),

    // Comaprison - Right side
    LessR(Value, Box<Context>),
    LessEqR(Value, Box<Context>),
    GreaterR(Value, Box<Context>),
    GreaterEqR(Value, Box<Context>),
    EqualR(Value, Box<Context>),

    // If cond can only have context in condition
    If(Box<Context>, Box<Expr>, Box<Expr>),
}

impl Context {
    pub fn is_hole(&self) -> bool {
        matches!(self, Context::Hole)
    }
}

pub fn plug(ctx: &Context, expr: Expr) -> Expr {
    match ctx {
        Context::Hole => expr,
        // Arithemtic - Left Side
        Context::AddL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Add(Box::new(filled), right_expr.clone())
        },
        Context::SubL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Sub(Box::new(filled), right_expr.clone())
        },
        Context::MulL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Mul(Box::new(filled), right_expr.clone())
        },
        Context::DivL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Div(Box::new(filled), right_expr.clone())
        },
        // Arithmetic - Right side
        Context::AddR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Add(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::SubR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Sub(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::MulR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Mul(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::DivR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Div(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        // Comparison - Left side
        Context::LessL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Less(Box::new(filled), right_expr.clone())
        },
        Context::LessEqL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::LessEq(Box::new(filled), right_expr.clone())
        },
        Context::GreaterL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Greater(Box::new(filled), right_expr.clone())
        },
        Context::GreaterEqL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::GreaterEq(Box::new(filled), right_expr.clone())
        },
        Context::EqualL(inner_ctx, right_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::Equal(Box::new(filled), right_expr.clone())
        },
        // Comparison - Right side
        Context::LessR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Less(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::LessEqR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::LessEq(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::GreaterR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Greater(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::EqualR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::Equal(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        Context::GreaterEqR(left_val, inner_ctx) => {
            let filled = plug(inner_ctx, expr);
            Expr::GreaterEq(Box::new(value_to_expr(left_val)), Box::new(filled))
        },
        // If 
        Context::If(inner_ctx, then_expr, else_expr) => {
            let filled = plug(inner_ctx, expr);
            Expr::If(
                Box::new(filled), 
                then_expr.clone(), 
                else_expr.clone()
            )
        },
    }
}

pub fn find_redex(expr: &Expr) -> Option<(Context, Expr)> {
    match expr {
        Expr::Num(_) | Expr::Bool(_) => {
            Some((Context::Hole, expr.clone()))
        },
        // Arithematic
        Expr::Add(l, r) => find_binop_redex(expr, l, r, Context::AddL, Context::AddR),
        Expr::Sub(l, r) => find_binop_redex(expr, l, r, Context::SubL, Context::SubR),
        Expr::Mul(l, r) => find_binop_redex(expr, l, r, Context::MulL, Context::MulR),
        Expr::Div(l, r) => find_binop_redex(expr, l, r, Context::DivL, Context::DivR),
        // Comparison
        Expr::Less(l, r) => find_binop_redex(expr, l, r, Context::LessL, Context::LessR),
        Expr::LessEq(l, r) => find_binop_redex(expr, l, r, Context::LessEqL, Context::LessEqR),
        Expr::Greater(l, r) => find_binop_redex(expr, l, r, Context::GreaterL, Context::GreaterR),
        Expr::GreaterEq(l, r) => find_binop_redex(expr, l, r, Context::GreaterEqL, Context::GreaterEqR),
        Expr::Equal(l, r) => find_binop_redex(expr, l, r, Context::EqualL, Context::EqualR),
        // Condition
        Expr::If(cond, then_expr, else_expr) => {
            if is_value(cond) {
                Some((Context::Hole, expr.clone()))
            } else {
                let (inner_ctx, redex) = find_redex(cond)?;
                Some((
                    Context::If(Box::new(inner_ctx), then_expr.clone(), else_expr.clone()),
                    redex
                ))
            }
        },
        // For now, variables, lambdas, apps are not handled
        Expr::Var(_) | Expr::Lambda(_, _) | Expr::App(_, _) | Expr::Prim(_) => None,
    }
}

fn value_to_expr(val: &Value) -> Expr {
    match val {
        Value::Num(n) => Expr::Num(*n),
        Value::Bool(b) => Expr::Bool(*b),
        _ => panic!("Value is non Num | Bool.  Value: {:?}", val),
    }
}

fn is_value(expr: &Expr) -> bool {
    matches!(expr, Expr::Num(_) | Expr::Bool(_))
}

fn expr_to_value(expr: &Expr) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Bool(b) => Value::Bool(*b),
        _ => panic!("expr_to_value called on non-value"),
    }
}

// Helper function for finding redexes in binary operations.
// Handles the common pattern: check if both operands are values (redex),
// recurse left if needed, or recurse right if needed.
fn find_binop_redex<L, R>(
    expr: &Expr,
    l: &Expr,
    r: &Expr,
    make_left_ctx: L,
    make_right_ctx: R,
) -> Option<(Context, Expr)> 
where 
    L: Fn(Box<Context>, Box<Expr>) -> Context,
    R: Fn(Value, Box<Context>) -> Context,
{
    if is_value(l) && is_value(r) {
        Some((Context::Hole, expr.clone()))
    }
    else if !is_value(l) {
        let (inner_ctx, redex) = find_redex(l)?;
        Some((make_left_ctx(Box::new(inner_ctx), Box::new(r.clone())), redex))
    }
    else {
        let left_val = expr_to_value(l);
        let (inner_ctx, redex) = find_redex(r)?;
        Some((make_right_ctx(left_val, Box::new(inner_ctx)), redex))
    }
}

pub fn reduce(expr: &Expr) -> Expr {
    match expr {
        Expr::Num(n) => Expr::Num(*n),
        Expr::Bool(b) => Expr::Bool(*b),
        // Arithmetic
        Expr::Add(l, r) => {
            match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Num(a + b),
                _ => panic!("Type error: + expects two numbers"),
            }
        },
        Expr::Sub(l, r) => {
             match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Num(a - b),
                _ => panic!("Type error: - expects two numbers"),
            }
        },
        Expr::Mul(l, r) => {
             match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Num(a * b),
                _ => panic!("Type error: * expects two numbers"),
            }
        },
        Expr::Div(l, r) => {
             match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    if b == 0 {
                        panic!("Division by zero");
                    }
                    Expr::Num(a / b)
                },
                _ => panic!("Type error: / expects two numbers"),
            }
        },
        // comparison
        Expr::Less(l, r) => {
            match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Bool(a < b),
                _ => panic!("Type error: < expects two numbers"),
            }
        },
        Expr::LessEq(l, r) => {
            match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Bool(a <= b),
                _ => panic!("Type error: <= expects two numbers"),
            }
        },
        Expr::Greater(l, r) => {
            match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Bool(a > b),
                _ => panic!("Type error: > expects two numbers"),
            }
        },
        Expr::GreaterEq(l, r) => {
            match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Bool(a >= b),
                _ => panic!("Type error: >= expects two numbers"),
            }
        },
        Expr::Equal(l, r) => {
            match (expr_to_value(l), expr_to_value(r)) {
                (Value::Num(a), Value::Num(b)) => Expr::Bool(a == b),
                _ => panic!("Type error: = expects two numbers"),
            }
        },
        // If
        Expr::If(cond, then_expr, else_expr) => {
            match expr_to_value(cond) {
                Value::Bool(true) => then_expr.as_ref().clone(),
                Value::Bool(false) => else_expr.as_ref().clone(),
                _ => panic!("Type error: if condition must be boolean"),
            }
        },
        _ => panic!("reduce called on non-redex: {:?}", expr),
    }
}

fn small_step(expr: &Expr) -> Option<Expr> {
    let (ctx, redex) = find_redex(&expr)?;

    if ctx.is_hole() && is_value(&redex) {
        return None;
    }

    let reduced = reduce(&redex);
    Some(plug(&ctx, reduced))
}

pub fn small_step_eval(expr: Expr) -> Value {
    let mut current = expr;

    loop {
        match small_step(&current) {
            None => {
                return expr_to_value(&current);
            },
            Some(next_expr) => {
                current = next_expr;
            },
        }
    }
}