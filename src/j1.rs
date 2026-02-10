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
enum Expr {
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