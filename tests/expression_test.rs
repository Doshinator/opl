use opl::expression::{Expr, eval};

#[test]
fn eval_num() {
    let e = Expr::Num(5);
    let n = eval(&e);
    assert_eq!(5, n);
}

#[test]
fn eval_add() {
    let e = Expr::Add(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Num(-4))
    );
    let n = eval(&e);
    assert_eq!(-1, n);
}

#[test]
fn eval_multiply() {
    let e = Expr::Multiply(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Num(-4))
    );
    let n = eval(&e);
    assert_eq!(-12, n);
}
