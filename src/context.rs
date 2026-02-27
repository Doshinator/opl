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
        Expr::Add(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            // left side is not a value - we need to recurse
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::AddL(Box::new(inner_ctx), r.clone()), redex))
            }
            // right side needs to be evaluated - recurse
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::AddR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        Expr::Sub(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::SubL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_val = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::SubR(left_val, Box::new(inner_ctx)), redex))
            }
        },
        Expr::Mul(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::MulL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::MulR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        Expr::Div(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::DivL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::DivR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        // Comparison
        Expr::Less(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::LessL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::LessR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        Expr::LessEq(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::LessEqL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::LessEqR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        Expr::Greater(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::GreaterL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::GreaterR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        Expr::GreaterEq(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::GreaterEqL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::GreaterEqR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        Expr::Equal(l, r) => {
            if is_value(l) && is_value(r) {
                Some((Context::Hole, expr.clone()))
            }
            else if !is_value(l) {
                let (inner_ctx, redex) = find_redex(l)?;
                Some((Context::EqualL(Box::new(inner_ctx), r.clone()), redex))
            }
            else {
                let left_value = expr_to_value(l);
                let (inner_ctx, redex) = find_redex(r)?;
                Some((Context::EqualR(left_value, Box::new(inner_ctx)), redex))
            }
        },
        // Condition
        Expr::If(cond, then_expr, else_expr) => {
            if !is_value(cond) {
                let (inner_ctx, redex) = find_redex(cond)?;
                Some((Context::If(Box::new(inner_ctx), then_expr.clone(), else_expr.clone()), redex))
            }
            else {
                Some((Context::Hole, expr.clone()))
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