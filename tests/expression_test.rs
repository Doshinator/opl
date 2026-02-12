use opl::{desugar, expression::{Expr, eval}, reader};

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
    let e = Expr::Mul(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Num(-4))
    );
    let n = eval(&e);
    assert_eq!(-12, n);
}


#[test]
fn j1_add() {
    let program = "(+ 2 3)";
    let result = run(program);

    assert_eq!(
        5,
        result,
    )
}

#[test]
fn j1_mul() {
    let program = "(* 3 4)";
    let actual = run(program);

    assert_eq!(
        12,
        actual,
    )
}

#[test]
fn j1_num() {
    let program = "5";
    let result = run(program);

    assert_eq!(
        5,
        result,
    )
}

// Helper function to run the program
fn run(program: &str) -> i32 {
    let sexpr = reader(program);
    let expr = desugar(&sexpr);
    let result = eval(&expr);
    result
}