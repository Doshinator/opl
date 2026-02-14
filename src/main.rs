use opl::expression::{Expr, eval, pretty_print};

fn main() {
    println!("---- Start -----");
    let e = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Mul(
            Box::new(Expr::Num(3)),
            Box::new(Expr::Num(4))
        )),
    );
    println!("{} = {:?}", 
        pretty_print(&e),
        eval(&e)
    );
    println!("---- End -----");
}