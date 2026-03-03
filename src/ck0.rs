use core::panic;
use crate::{Expr, Value};

pub type Kontinuation = Vec<Frame>;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct State {
    control: Expr,
    kontinuation: Kontinuation,
}

fn inject(expr: Expr) -> State {
    State {
        control: expr,
        kontinuation: vec!  [],
    }
}

fn extract(state: &State) -> Value {
    assert!(state.kontinuation.is_empty(), "extract called on a non-final state");

    match &state.control {
        Expr::Num(n) => Value::Num(*n),
        Expr::Bool(b) => Value::Bool(*b),
        _ => panic!("extract called on a non-value {:?}", &state.control),
    }
}