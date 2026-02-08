use opl::{desugar::{self, desugar}, expression::{Expr, eval}, s_expression::SExpr};

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
    println!("---- End -----");
}
