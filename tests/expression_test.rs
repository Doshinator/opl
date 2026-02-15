use opl::{desugar, expression::{Expr, Value, eval, pretty_print}, reader};

#[test]
fn eval_num() {
    let e = Expr::Num(5);
    let n = eval(&e);
    assert_eq!(Value::Num(5), n);
}

#[test]
fn eval_add() {
    let e = Expr::Add(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Num(-4))
    );
    let n = eval(&e);
    assert_eq!(Value::Num(-1), n);
}

#[test]
fn eval_multiply() {
    let e = Expr::Mul(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Num(-4))
    );
    let n = eval(&e);
    assert_eq!(Value::Num(-12), n);
}

// ============================================================================
// J1 TESTS - NEW (Step 9: Test Suite for J1)
// ============================================================================

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

#[test]
fn j1_less_condition() {
    let program = "(< 1 10)";
    let actual = run_bool(program);
    assert_eq!(true, actual);
}

#[test]
fn j1_greater_condition() {
    let program = "(> 1 10)";
    let actual = run_bool(program);
    assert_eq!(false, actual);

    let program = "(> 10 1)";
    let actual = run_bool(program);
    assert_eq!(true, actual)
}

#[test]
fn j1_less_eq_condition() {
    let program = "(<= 10 10)";
    let actual = run_bool(program);
    assert_eq!(true, actual)
}

#[test]
fn j1_equal_condition() {
    let program = "(= 10 10)";
    let actual = run_bool(program);
    assert_eq!(true, actual);
    
    let program = "(= 5 10)";
    let actual = run_bool(program);
    assert_eq!(false, actual);
}

#[test]
fn j1_if_conditional() {
    let program = "(if (< 2 3) 10 20)";
    let actual = run(program);
    assert_eq!(10, actual);
    
    let program = "(if (= 5 2) 10 20)";
    let actual = run(program);
    assert_eq!(20, actual);
}

#[test]
fn j1_if_with_arithmatics() {
    let program = "(if (< 5 10) (+ 50 7) (* 3 4))";
    let actual = run(program);
    assert_eq!(57, actual);
}

#[test]
fn j1_if_false_branch() {
    let program = "(if false 10 20)";
    let result = run(program);
    assert_eq!(20, result);
}

#[test]
fn j1_if_nested() {
    let program = "(if (< 2 3) (if true 1 2) 3)";
    let result = run(program);
    assert_eq!(1, result);
}

#[test]
fn j1_bool_true() {
    let program = "true";
    let result = run_bool(program);
    assert_eq!(true, result);
}

#[test]
fn j1_bool_false() {
    let program = "false";
    let result = run_bool(program);
    assert_eq!(false, result);
}

#[test]
fn j1_if_true_branch() {
    let program = "(if true 10 20)";
    let result = run(program);
    assert_eq!(10, result);
}

#[test]
fn j1_complex_condition() {
    let program = "(if (> 10 5) (+ 2 3) (* 4 5))";
    let result = run(program);
    assert_eq!(5, result);
}

// Helper function to run the program
fn run(program: &str) -> i32 {
    let sexpr = reader(program);
    let expr = desugar(&sexpr);
    let result = eval(&expr);
    match result {
        Value::Num(n) => n,
        _ => panic!("Expected numeric result, got {:?}", result),
    }
}

fn run_bool(program: &str) -> bool {
    let sexpr = reader(program);
    let expr = desugar(&sexpr);
    let result = eval(&expr);
    match result {
        Value::Bool(b) => b,
        _ => panic!("Expected boolean result, got {:?}", result),
    }
}