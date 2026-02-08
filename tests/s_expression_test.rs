use opl::s_expression::{SExpr, reader};

#[test]
fn read_number() {
    // Arrange
    let s = "2";
    let expected = SExpr::Num(2);

    // Act
    let actual = reader(s);
    
    // Assert
    assert_eq!(
        expected,
        actual 
    );
}

#[test]
fn read_symbol() {
    // Arrange
    let s = "+";
    let expected = SExpr::Sym("+".into());

    // Act
    let actual = reader(s);
    
    // Assert
    assert_eq!(
        expected,
        actual 
    );
}

#[test]
fn read_list() {
    // Arrange
    let s = "(+ 1 2)";
    let expected = SExpr::List(vec![
        SExpr::Sym("+".into()),
        SExpr::Num(1),
        SExpr::Num(2),
    ]);

    // Act
    let actual = reader(s);

    // Arrange
    assert_eq!(
        expected,
        actual,
    )
}

#[test]
fn read_nested_list() {
    // Arrange
    let s = "(+ 2 (* 3 4))";
    let expected = SExpr::List(vec![
        SExpr::Sym("+".into()),
        SExpr::Num(2),
        SExpr::List(vec![
            SExpr::Sym("*".into()),
            SExpr::Num(3),
            SExpr::Num(4),
        ]),
    ]);

    // Act
    let actual = reader(s);
    assert_eq!(
        actual, 
        expected
    );
}