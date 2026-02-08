use opl::{Expr, desugar::desugar, eval, s_expression::SExpr};


#[test]
fn desugar_simple_num() {
    // Arrange
    let expected = Expr::Num(5);
    let sexpr = SExpr::Num(5);

    // Act
    let actual = desugar(&sexpr);

    // Assert
    assert_eq!(
        expected,
        actual,
    )
}

#[test]
fn desugar_add() {
    // Arrange
    let expected = Expr::Add(
        Box::new(Expr::Num(3)), 
        Box::new(Expr::Num(2))
    );
    let sexpr = SExpr::List(vec![
        SExpr::Sym("+".into()),
        SExpr::Num(3),
        SExpr::Num(2),
    ]);

    // Act
    let actual: Expr = desugar(&sexpr);

     // Assert
    assert_eq!(
        expected,
        actual,
    )
}