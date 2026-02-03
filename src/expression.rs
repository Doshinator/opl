
#[cfg(test)]
mod tests_expression {
    use super::*;

    #[test]
    fn eval_num() {
        let expression: Expression = ENum::new(5);
        assert_eq!(5, expression.eval());
    }

    #[test]
    fn eval_add() {
        let expression: Expression = EAdd::new(
            ENum::new(5),
            ENum::new(3)
        );

        assert_eq!(8, expression.eval());
    }

    #[test]
    fn eval_add_negatives() {
        let expression: Expression = EAdd::new(
            ENum::new(-5),
            ENum::new(3)
        );

        assert_eq!(-2, expression.eval());
    }

    #[test]
    fn eval_multiply() {
        let expression: Expression = EMultiply::new(
            ENum::new(8),
            ENum::new(3)
        );

        assert_eq!(24, expression.eval());
    }

    #[test]
    fn eval_multiply() {
        let expression: Expression = EMultiply::new(
            ENum::new(-8),
            ENum::new(8)
        );

        assert_eq!(-64, expression.eval());
    }
}
