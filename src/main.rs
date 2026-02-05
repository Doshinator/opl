use opl::{expression::{Expr, eval}, reader, s_expression::SExpr};

fn main() {
    println!("---- Start -----");
    let e = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Multiply(
            Box::new(Expr::Num(3)),
            Box::new(Expr::Num(4))
        )),
    );
    println!("{}", eval(&e));

    // (+ 2 (* 3 4))
    // hand written
    let s_exprs = SExpr::SExprList(vec![
        Box::new(SExpr::SESym("+".into())),
        Box::new(SExpr::SExprNum(2)),
            Box::new(SExpr::SExprList(vec![
                Box::new(SExpr::SESym("*".into())),
                Box::new(SExpr::SExprNum(3)),
                Box::new(SExpr::SExprNum(4)),
            ]))
    ]);

    println!("---- End -----");
}
