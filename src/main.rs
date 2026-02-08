use opl::{expression::{Expr, eval}};

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

    println!("---- End -----");
}
