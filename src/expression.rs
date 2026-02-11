#[derive(Debug, PartialEq, Eq, Clone)]
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

pub fn pretty_print(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.to_string(),
        Expr::Add(l, r) => format!("(+ {} {})", pretty_print(l), pretty_print(r)),
        Expr::Multiply(l, r) => format!("(* {} {})", pretty_print(l), pretty_print(r)),
    }
}