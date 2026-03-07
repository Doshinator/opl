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
    AddR(Value),
    SubR(Value),
    MulR(Value),
    DivR(Value),
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
    expr_to_value(&state.control)
}

fn is_value(expr: &Expr) -> bool {
    matches!(expr, Expr::Num(_) | Expr::Bool(_))
}

fn expr_to_value(expr: &Expr) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Bool(n) => Value::Bool(*n),
        _ => panic!("expr_to_value called on a non-expression {:?}", expr),
    }
}

fn ck_step(state: State) -> Option<State> {
    match state.control.clone() {
        // base case: If control is a value & kont stack is empty, we are done
        Expr::Num(_) | Expr::Bool(_) => {
            if state.kontinuation.is_empty() {
                None
            }
            else {
                // else we pop the frame and assign it to control
                let value = expr_to_value(&state.control);
                Some(apply_frame(value, state.kontinuation))
            }
        },
        // Binary operations - push frame
        Expr::Add(l, r) => push_binary_frame(state, &l,&r, Frame::AddL),
        Expr::Sub(l, r) => push_binary_frame(state, &l, &r, Frame::SubL),
        Expr::Mul(l, r) => push_binary_frame(state, &l, &r, Frame::MulL),
        Expr::Div(l, r) => push_binary_frame(state, &l, &r, Frame::DivL),
        // Comparison
        Expr::Less(l, r) => push_binary_frame(state, &l, &r, Frame::LessL),
        Expr::LessEq(l, r) => push_binary_frame(state, &l, &r, Frame::LessEqL),
        Expr::Greater(l, r) => push_binary_frame(state, &l, &r, Frame::GreaterL),
        Expr::GreaterEq(l, r) => push_binary_frame(state, &l, &r, Frame::GreaterEqL),
        Expr::Equal(l, r) => push_binary_frame(state, &l, &r, Frame::EqualL),

        // If
        Expr::If(cond, then_expr, else_expr) => {
            let mut kont = state.kontinuation;
            kont.push(Frame::IfK(then_expr.clone(), else_expr.clone()));
            Some(State {
                control: cond.as_ref().clone(),
                kontinuation: kont,
            })
        }
        _ => panic!("ck_step: unhandled expression: {:?}", state.control),
    }
}

// Helper: Push a binary operation frame (left side)
fn push_binary_frame<F>(
    state: State,
    left: &Expr,
    right: &Expr,
    make_frame: F,
) -> Option<State> where F: FnOnce(Box<Expr>) -> Frame
{
    let mut kont = state.kontinuation;
    kont.push(make_frame(Box::new(right.clone())));
    Some(State {
        control: left.clone(),
        kontinuation: kont,
    })
}

fn apply_frame(value: Value, mut kont: Kontinuation) -> State {
    let frame = kont.pop().expect("apply_frame: empty kont");

    match frame {
        // Left Frame, move value from within this frame, assign it the control and add frameR and move value in there
        // Arithmetic Left Frame
        Frame::AddL(right) => {
            kont.push(Frame::AddR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        Frame::SubL(right) => {
            kont.push(Frame::SubR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        Frame::MulL(right) => {
            kont.push(Frame::MulR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        Frame::DivL(right) => {
            kont.push(Frame::DivR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        // Arithmetic Right Frame
        Frame::AddR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Num(a + b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Add expects numbers"),
            }
        },
        Frame::SubR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Num(a - b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Sub expects numbers"),
            }
        },
        Frame::MulR(left_val) => {
            match (left_val, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Num(a * b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Mul expects numbers"),
            }
        },
        Frame::DivR(left_val) => {
            match (left_val, value) {
                (Value::Num(a), Value::Num(b)) => {
                    if b == 0 {
                        panic!("Division by zero");
                    }
                    State {
                        control: Expr::Num(a / b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Div expects numbers"),
            }
        },
        // Comparison - Left Frames
        Frame::LessL(right) => {
            kont.push(Frame::LessR(value));
            State { 
                control: right.as_ref().clone(), 
                kontinuation: kont,
            }
        },
        Frame::LessEqL(right) => {
            kont.push(Frame::LessEqR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        Frame::GreaterL(right) => {
            kont.push(Frame::GreaterR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        Frame::GreaterEqL(right) => {
            kont.push(Frame::GreaterEqR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        Frame::EqualL(right) => {
            kont.push(Frame::EqualR(value));
            State {
                control: right.as_ref().clone(),
                kontinuation: kont,
            }
        },
        // comparison - right frame
        Frame::LessR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Bool(a < b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Less expects numbers"),
            }
        },
        Frame::LessEqR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Bool(a <= b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: LessEq expects numbers"),
            }
        },

        Frame::GreaterR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Bool(a > b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Greater expects numbers"),
            }
        },

        Frame::GreaterEqR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Bool(a >= b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: GreaterEq expects numbers"),
            }
        },

        Frame::EqualR(left_value) => {
            match (left_value, value) {
                (Value::Num(a), Value::Num(b)) => {
                    State {
                        control: Expr::Bool(a == b),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: Equal expects numbers"),
            }
        },
        Frame::IfK(then_expr, else_expr) => {
            match value {
                Value::Bool(true) => {
                    State {
                        control: then_expr.as_ref().clone(),
                        kontinuation: kont,
                    }
                },
                Value::Bool(false) => {
                    State {
                        control: else_expr.as_ref().clone(),
                        kontinuation: kont,
                    }
                },
                _ => panic!("Type error: If condition must be boolean"),
            }
        },
    }
}

pub fn ck_eval(expr: Expr) -> Value {
    let mut state = inject(expr);

    loop {
        match ck_step(state.clone()) {
            None => return extract(&state),
            Some(next_state) => state = next_state,
        }
    }
}