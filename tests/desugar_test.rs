use opl::{Expr, desugar::desugar, eval, s_expression::SExpr};


#[test]
fn desugar_simple_num() {
    // Arrange
    let se = SExpr::Num(5);

    // Act
    let actual = desugar(&se);

    // Assert
    assert_eq!(
        Expr::Num(5),
        actual,
    )
}

#[test]
fn desugar_add() {
    // Arrange
    let se = SExpr::List(vec![
        SExpr::Sym("+".into()),
        SExpr::Num(3),
        SExpr::Num(2),
    ]);

    let expected = Expr::Add(
        Box::new(Expr::Num(3)), 
        Box::new(Expr::Num(2))
    );

    // Act
    let actual: Expr = desugar(&se);

     // Assert
    assert_eq!(
        expected,
        actual,
    )
}

#[test]
fn desugar_multiply() {
    // Arrange
    let se = SExpr::List(vec![
        SExpr::Sym("*".into()),
        SExpr::Num(3),
        SExpr::Num(4),
    ]);

    let actual = Expr::Multiply(
        Box::new(Expr::Num(3)), 
        Box::new(Expr::Num(4))
    );

    // Act
    let expected = desugar(&se);

    // Assert
    assert_eq!(
        expected,
        actual,
    )
}

#[test]
fn desugar_nested() {
    // (+ 2 (* 3 4))
    let se = SExpr::List(vec![
        SExpr::Sym("+".into()),
        SExpr::Num(2),
        SExpr::List(vec![
            SExpr::Sym("*".into()),
            SExpr::Num(3),
            SExpr::Num(4),
        ]),
    ]);

    let expected = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(
            Expr::Multiply(
                Box::new(Expr::Num(3)),
                Box::new(Expr::Num(4)),
            )
        ),
    );

    let actual = desugar(&se);

    assert_eq!(expected, actual);
}

#[test]
fn desugar_then_eval() {
    // (+ 2 (* 3 4)) = 14
    let se = SExpr::List(vec![
        SExpr::Sym("+".into()),
        SExpr::Num(2),
        SExpr::List(vec![
            SExpr::Sym("*".into()),
            SExpr::Num(3),
            SExpr::Num(4),
        ]),
    ]);

    let expr = desugar(&se);
    let result = eval(&expr);

    assert_eq!(14, result);
}