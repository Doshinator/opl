#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Multiply(l, r) => eval(l) * eval(r),
    }
}