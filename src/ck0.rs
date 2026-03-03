use crate::{Context, Expr, Value};

pub type Kontinuation = Vec<Frame>;

pub enum Frame {
    // Artithmetic Left side
    AddL(Box<Expr>),
    SubL(Box<Expr>),
    MulL(Box<Expr>),
    DivL(Box<Expr>),
    // Arithmetic Right side
    AddR(Box<Expr>),
    SubR(Box<Expr>),
    MulR(Box<Expr>),
    DivR(Box<Expr>),
    // Comparison Left side
    LessL(Box<Expr>),
    LessEqL(Box<Expr>),
    GreaterL(Box<Expr>),
    GreaterEqL(Box<Expr>),
    EqualL(Box<Expr>),
    // Comparisons - Right side
    LessR(Value),
    LessEqR(Value),
    GreaterR(Value),
    GreaterEqR(Value),
    EqualR(Value),
    // If
    IfK(Box<Expr>, Box<Expr>),
}

struct Machine {
    control: Expr,
    kontinuation: Kontinuation,
}



fn inject(expr: Expr) -> (Expr, Context) {
    todo!()
}

fn extract((value, Kret): (Value, Machine)) -> Value {
    todo!()
}