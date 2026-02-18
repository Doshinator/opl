use crate::Expr;

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
    AddR(Box<Expr>, Box<Context>),
    SubR(Box<Expr>, Box<Context>),
    MulR(Box<Expr>, Box<Context>),
    DivR(Box<Expr>, Box<Context>),

    // Comparison - Left side
    LessL(Box<Context>, Box<Expr>),
    LessEqL(Box<Context>, Box<Expr>),
    GreaterL(Box<Context>, Box<Expr>),
    GreaterEqL(Box<Context>, Box<Expr>),
    EqualL(Box<Context>, Box<Expr>),

    // Comaprison - Right side
    LessR(Box<Expr>, Box<Context>),
    LessEqR(Box<Expr>, Box<Context>),
    GreaterR(Box<Expr>, Box<Context>),
    GreaterEqR(Box<Expr>, Box<Context>),
    EqualR(Box<Expr>, Box<Context>),

    // If cond can only have context in condition
    If(Box<Context>, Box<Expr>, Box<Expr>),
}
