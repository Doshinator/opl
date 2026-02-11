use opl::{desugar::desugar, eval, j1::{Expr, pretty_print}, reader};

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
fn j1_subtract() {
    let program = "(- 10 3)";
    let result = run(program);

    assert_eq!(
        7,
        result,
    )
}



fn run(program: &str) -> i64 {
    let sexpr = reader(program);
    let expr = desugar(&sexpr);
    eval(&expr)
}