use opl::{Context, Expr, Value};

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
