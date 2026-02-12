use opl::{desugar, expression::{Expr, eval, pretty_print}, reader};

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

#[test]
fn j1_subtract() {
    let program = "(- 5 5)";
    let actual = run(program);
    
    assert_eq!(
        0,
        actual,
    )
}

#[test]
fn j1_divide() {
    let program = "(/ 15 5)";
    let actual = run(program);

    assert_eq!(
        3,
        actual,
    )
}

#[test]
fn j1_nested_arithmetics() {
    let program = "(+ 2 (* 3 4))";
    let actual = run(program);

    assert_eq!(
        14,
        actual,
    );
}

#[test]
fn j1_deep_nesting() {
    let program = "(* (+ 1 2) (+ 10 4))";
    let actual = run(program);

    assert_eq!(
        42,
        actual,
    );
}

#[test]
fn j1_negative_numbers() {
    let program = "(* -3 5)";
    let actual = run(program);

    assert_eq!(
        -15,
        actual,
    );
}

#[test]
fn j1_complex_expression() {
    let program = "(+ (* 2 3) (/ 8 2))";
    let actual = run(program);

    assert_eq!(
        10,
        actual,
    );
}

// Helper function to run the program
fn run(program: &str) -> i32 {
    let sexpr = reader(program);
    let expr = desugar(&sexpr);
    let result = eval(&expr);
    result
}