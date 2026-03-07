#[cfg(test)]
mod tests {
    use opl::{Expr, Value, ck_machine::ck_eval};

    #[test]
    fn test_ck_simple_value() {
        let expr = Expr::Num(42);
        let result = ck_eval(expr);
        assert_eq!(result, Value::Num(42));
    }

    #[test]
    fn test_ck_simple_add() {
        let expr = Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Num(3))
        );
        let result = ck_eval(expr);
        assert_eq!(result, Value::Num(5));
    }

    #[test]
    fn test_ck_nested() {
        // (+ 2 (* 3 4))
        let expr = Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Mul(
                Box::new(Expr::Num(3)),
                Box::new(Expr::Num(4))
            ))
        );
        let result = ck_eval(expr);
        assert_eq!(result, Value::Num(14));
    }

    #[test]
    fn test_ck_if() {
        // (if (< 2 3) 10 20)
        let expr = Expr::If(
            Box::new(Expr::Less(
                Box::new(Expr::Num(2)),
                Box::new(Expr::Num(3))
            )),
            Box::new(Expr::Num(10)),
            Box::new(Expr::Num(20))
        );
        let result = ck_eval(expr);
        assert_eq!(result, Value::Num(10));
    }
}