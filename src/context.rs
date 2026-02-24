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

fn value_to_expr(val: &Value) -> Expr {
    match val {
        Value::Num(n) => Expr::Num(*n),
        Value::Bool(b) => Expr::Bool(*b),
    }
}