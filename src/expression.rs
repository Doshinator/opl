use core::panic;
use std::collections::HashMap;

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

pub type Env = HashMap<String, Value>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(i32),
    Bool(bool),
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

    // ---- J2 -------
    // Var
    Var(String),

    // Functions
    Lambda(Vec<String>, Box<Expr>), // (param, body)

    // Application, a.k.a calling a function 
    //((λ (x) (+ x 1)) 5) == App(Lambda([x], Expr::Add(Expr::Var(x), Expr::Num(1)), Expr::Num(5)))
    //  ↑              ↑
    //  Function       Argument
    App(Box<Expr>, Vec<Expr>), // (function, args)
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
    Closure {
        params: Vec<String>, // ["id", "name", ...]
        body: Box<Expr>, // (+ x y)
        env: Env, // ex. ["id": "55735", "name": "Bob", ...]
    },
}

pub fn eval(expr: &Expr, env: &Env) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Add(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
                _ => panic!("Type error: + expects two numbers"),
            }
        },
        Expr::Mul(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a * b),
                _ => panic!("Type error: * expects two numbers"),
            }
        },
        Expr::Sub(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a - b),
                _ => panic!("Type error: - expects two numbers"),
            }
        },
        Expr::Div(l, r) => {
            match (eval(l, env), eval(r, env)) {
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
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a < b)
                },
                _ => panic!("Type error: < expects two numbers"),
            }
        },
        Expr::LessEq(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a <= b)
                },
                _ => panic!("Type error: <= expects two numbers"),
            }
        },
        Expr::Greater(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a > b)
                },
                _ => panic!("Type error: > expects two numbers"),
            }
        },
        Expr::GreaterEq(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a >= b)
                },
                _ => panic!("Type error: ?= expects two numbers"),
            }
        },
        Expr::Equal(l, r) => {
            match (eval(l, env), eval(r, env)) {
                (Value::Num(a), Value::Num(b)) => {
                    Value::Bool(a == b)
                },
                _ => panic!("Type error: == expects two numbers"),
            }
        },
        Expr::If(cond, then_expr, else_expr) => {
            match eval(cond, env) {
                Value::Bool(true) => eval(then_expr, env),
                Value::Bool(false) => eval(else_expr, env),
                _ => panic!("Type error: if condition must be a boolean"),
            }
        },
        Expr::Var(name) => {
            env.get(name)
                .cloned()
                .unwrap_or_else(|| panic!("Unbound variable: {}", name))
        },
        Expr::Lambda(param, body) => {
            Value::Closure { 
                params: param.clone(), 
                body: body.clone(), 
                env: env.clone() 
            }
        },
        Expr::App(func_expr, args_expr) => {
            let func = eval(func_expr, env);

            let args: Vec<Value> = args_expr
                .iter()
                .map(|args| eval(args, env))
                .collect();

            match func {
                Value::Closure { params, body, env: closur_env } => {
                    if params.len() != args.len() {
                        panic!(
                            "Arity mismatch: expected {} args, got {}",
                            params.len(),
                            args.len()
                        );
                    }
                
                    let mut new_env = closur_env.clone();

                    for (param, arg) in params.iter().zip(args.iter()) {
                        new_env.insert(param.clone(), arg.clone());
                    }

                    eval(&body, &new_env)
                },
                _ => panic!("Attempted to call non-function: {:?}", func),
            }
        },
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