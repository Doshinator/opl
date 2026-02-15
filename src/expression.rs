use core::panic;

/**
 * Expr = 
  Num(i32)
  Bool(bool)
  Var(String)
  Add(Expr, Expr)
  Mul(Expr, Expr)
  Sub(Expr, Expr)
  Div(Expr, Expr)
  If(Expr, Expr, Expr)
  Lambda(params: Vec<String>, body: Box<Expr>)
  App(fn_expr: Box<Expr>, args: Vec<Expr>)

 */

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(i32),
    Bool(bool),
    Var(String),
    Prim(Prim),

    // Binary
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    // Comparison
    Less(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),

    // Conditionals
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    // Functions
    Lambda(Vec<String>, Box<Expr>),

    // Application
    App(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prim {
    Add,
    Mul,
    Sub,
    Div,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equal,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Num(i32),
    Bool(bool),
}

pub fn eval(expr: &Expr) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Add(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
                _ => panic!("Type error: + expects two numbers"),
            }
        },
        Expr::Mul(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a * b),
                _ => panic!("Type error: * expects two numbers"),
            }
        },
        Expr::Sub(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a - b),
                _ => panic!("Type error: - expects two numbers"),
            }
        },
        Expr::Div(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    if b == 0 {
                        panic!("divide by zero error");
                    }
                    Value::Num(a / b)
                },
                _ => panic!("Type error: / expects two numbers"),
            }
        },
        Expr::Less(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a < b)
                },
                _ => panic!("Type error: < expects two numbers"),
            }
        },
        Expr::LessEq(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a <= b)
                },
                _ => panic!("Type error: <= expects two numbers"),
            }
        },
        Expr::Greater(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a > b)
                },
                _ => panic!("Type error: > expects two numbers"),
            }
        },
        Expr::GreaterEq(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a >= b)
                },
                _ => panic!("Type error: ?= expects two numbers"),
            }
        },
        Expr::Equal(l, r) => {
            match (eval(l), eval(r)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a == b)
                },
                _ => panic!("Type error: == expects two numbers"),
            }
        },
        Expr::If(cond, then_expr, else_expr) => {
            match eval(cond) {
                Value::Bool(true) => eval(then_expr),
                Value::Bool(false) => eval(else_expr),
                _ => panic!("Type error: if condition must be a boolean"),
            }
        },
        Expr::Var(_) => panic!("Variables not yet implemented (needed for J2+)"),
        Expr::Lambda(_, _) => panic!("Lambda not yet implemented (needed for J2+)"),
        Expr::App(_, _) => panic!("Application not yet implemented (needed for J2+)"),
        Expr::Prim(_) => panic!("Primitives not yet implemented"),
    }
}

pub fn pretty_print(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Var(v) => v.clone(),

        Expr::Add(l, r) => 
            format!("(+ {} {})", pretty_print(l), pretty_print(r)),

        Expr::Mul(l, r) => 
            format!("(* {} {})", pretty_print(l), pretty_print(r)),

        Expr::Sub(l, r) => 
            format!("(- {} {})", pretty_print(l), pretty_print(r)),

        Expr::Div(l, r) => 
            format!("(/ {} {})", pretty_print(l), pretty_print(r)),

        Expr::Less(l, r) => 
            format!("(< {} {})", pretty_print(l), pretty_print(r)),

        Expr::LessEq(l, r) => 
            format!("(<= {} {})", pretty_print(l), pretty_print(r)),

        Expr::Greater(l, r) => 
            format!("(> {} {})", pretty_print(l), pretty_print(r)),

        Expr::GreaterEq(l, r) => 
            format!("(>= {} {})", pretty_print(l), pretty_print(r)),

        Expr::Equal(l, r) =>
            format!("(= {} {}", pretty_print(l), pretty_print(r)),

        Expr::If(c, t, f) => 
            format!("(if {} {} {})", 
                pretty_print(c),
                pretty_print(t),
                pretty_print(f)
            ),
        
        Expr::Lambda(params, body) => {
            let param_str = params.join(" ");
            format!(
                "(-> ({}) ({})",
                param_str,
                pretty_print(body),
            )
        },
            
        Expr::App(fun, args) => {
            let args_str=  args
            .iter()
            .map(|a| pretty_print(a))
            .collect::<Vec<_>>()
            .join(" ");
            
            format!(
                "({} {})",
                pretty_print(fun),
                args_str,
            )
        },

        Expr::Prim(p) => match p {
            Prim::Add => "+".into(),
            Prim::Mul => "*".into(),
            Prim::Sub => "-".into(),
            Prim::Div => "/".into(),
            Prim::Less => "<".into(),
            Prim::LessEq => "<=".into(),
            Prim::Greater => ">".into(),
            Prim::GreaterEq => ">=".into(),
            Prim::Equal => "=".into(),
        },
    }
}