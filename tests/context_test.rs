use opl::{Context, Expr};

#[test]
fn context_hole() {
    let c = Context::Hole;
    assert_eq!(c, Context::Hole);
}

#[test]
fn add_left_context() {
    // (+ HOLE 5)
    let c = Context::AddL(
        Box::new(Context::Hole), 
        Box::new(Expr::Num(5))
    );
    match c {
        Context::AddL(_, _) => assert!(true),
        _ => panic!("Wrong variant"),
    }
}