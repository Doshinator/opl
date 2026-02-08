use opl::s_expression::{SExpr, reader};

#[test]
fn buid_list() {
    // Arrange
    let s = "( + 2 3 )";
    let expected = SExpr::List(
        Box::new(SExpr::Sym("+").try_into())
    );
    // Act
    let actual = reader(s);
    
    // Assert
    assert_eq!(
        expected,
        actual 
    )
}