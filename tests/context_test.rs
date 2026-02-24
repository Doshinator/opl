use opl::{Context, Expr, Value, context::plug};

#[test]
fn context_hole() {
    let ctx = Context::Hole;
    assert!(ctx.is_hole());
}

#[test]
fn test_add_left_not_hole() {
    // (+ HOLE 5)
    let ctx = Context::AddL(
        Box::new(Context::Hole), 
        Box::new(Expr::Num(5))
    );
    
    assert!(!ctx.is_hole());
    match ctx {
        Context::AddL(l, r) => {
            assert!(l.is_hole());
            assert_eq!(*r, Expr::Num(5));
        },
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn context_add_right_simple() {
    // (+ 2 HOLE)
    let ctx = Context::AddR(
        Value::Num(2),
        Box::new(Context::Hole)
    );

    assert!(!ctx.is_hole());
    match ctx {
        Context::AddR(l, r) => {
            assert_eq!(l, Value::Num(2));
            assert!(r.is_hole());
        },
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn nested_context_add_mult() {
    // (+ 2 (* HOLE 4))
    let ctx = Context::AddR(
        Value::Num(2), 
        Box::new(Context::MulL(
            Box::new(Context::Hole), 
            Box::new(Expr::Num(4))
        ))
    );

    match ctx {
        Context::AddR(left_value, inner) => {
            assert_eq!(left_value, Value::Num(2));
            
            match inner.as_ref() {
                Context::MulL(mul_inner, r) => {
                    assert!(mul_inner.is_hole());
                    assert_eq!(**r, Expr::Num(4));
                },
                _ => panic!("Expected MulL"),
            }
        },
        _ => panic!("Expected MulR"),
    }
}

#[test]
fn context_deeply_nested() {
    // (+ 1 (* 2 (- HOLE 3)))
    let ctx = Context::AddR(
        Value::Num(1),
        Box::new(Context::MulR(
            Value::Num(2),
            Box::new(Context::SubL(
                Box::new(Context::Hole),
                Box::new(Expr::Num(3))
            ))
        ))
    );
    
    // Just verify it compiles and has right structure
    match ctx {
        Context::AddR(_, inner) => match inner.as_ref() {
            Context::MulR(_, inner2) => match &**inner2 {
                Context::SubL(hole, _) => assert!(hole.is_hole()),
                _ => panic!("Expected SubL"),
            },
            _ => panic!("Expected MulR"),
        },
        _ => panic!("Expected AddR"),
    }
}


#[test]
fn test_plug_add_left() {
    let ctx = Context::AddL(
        Box::new(Context::Hole), 
        Box::new(Expr::Num(5))
    );

    let result = plug(&ctx, Expr::Num(3));

    assert_eq!(result, Expr::Add(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Num(5))
    ));
}

#[test]
fn test_plug_add_right() {
    let ctx = Context::AddR(
        Value::Num(12), 
        Box::new(Context::Hole)
    );

    let result = plug(&ctx, Expr::Num(5));

    assert_eq!(result, Expr::Add(
        Box::new(Expr::Num(12)), 
        Box::new(Expr::Num(5))
    ));
}

#[test]
fn test_plug_mult_left() {
    let ctx = Context::MulL(Box::new(Context::Hole), Box::new(Expr::Num(3)));

    let result = plug(&ctx, Expr::Num(4));

    assert_eq!(result, Expr::Mul(Box::new(Expr::Num(4)), Box::new(Expr::Num(3))));
}

#[test]
fn test_plug_mult_right() {
    let ctx = Context::MulR(Value::Num(5), Box::new(Context::Hole));

    let result = plug(&ctx, Expr::Num(7));

    assert_eq!(result, Expr::Mul(Box::new(Expr::Num(5)), Box::new(Expr::Num(7))));
}

#[test]
fn test_plug_nested_context() {
    // (+ 2 (* HOLE 4))
    let ctx = Context::AddR(
        Value::Num(2),
        Box::new(Context::MulL(
            Box::new(Context::Hole),
            Box::new(Expr::Num(4))
        ))
    );

    let result = plug(&ctx, Expr::Num(3));

    assert_eq!(
        result,
        Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Mul(
                Box::new(Expr::Num(3)),
                Box::new(Expr::Num(4))
            ))
        )
    );
}

#[test]
fn test_plug_less_left() {
    let ctx = Context::LessL(
        Box::new(Context::Hole),
        Box::new(Expr::Num(10))
    );

    let result = plug(&ctx, Expr::Num(3));

    assert_eq!(
        result,
        Expr::Less(
            Box::new(Expr::Num(3)),
            Box::new(Expr::Num(10))
        )
    );
}

#[test]
fn test_plug_less_right() {
    let ctx = Context::LessR(
        Value::Num(3),
        Box::new(Context::Hole)
    );

    let result = plug(&ctx, Expr::Num(5));

    assert_eq!(
        result,
        Expr::Less(
            Box::new(Expr::Num(3)),
            Box::new(Expr::Num(5))
        )
    );
}

#[test]
fn test_plug_if_condition() {
    // (if HOLE 1 2)
    let ctx = Context::If(
        Box::new(Context::Hole),
        Box::new(Expr::Num(1)),
        Box::new(Expr::Num(2))
    );

    let result = plug(&ctx, Expr::Bool(true));

    assert_eq!(
        result,
        Expr::If(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::Num(1)),
            Box::new(Expr::Num(2))
        )
    );
}

#[test]
fn test_plug_deep_recursive() {
    // (+ 1 (* 2 (- HOLE 3)))
    let ctx = Context::AddR(
        Value::Num(1),
        Box::new(Context::MulR(
            Value::Num(2),
            Box::new(Context::SubL(
                Box::new(Context::Hole),
                Box::new(Expr::Num(3))
            ))
        ))
    );

    let result = plug(&ctx, Expr::Num(7));

    assert_eq!(
        result,
        Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::Mul(
                Box::new(Expr::Num(2)),
                Box::new(Expr::Sub(
                    Box::new(Expr::Num(7)),
                    Box::new(Expr::Num(3))
                ))
            ))
        )
    );
}