use std::{iter::Peekable, str::Chars};

pub enum SExpr {
    Num(i64),
    Sym(String),
    List(Vec<SExpr>),
}

// "(+ 2 (* 3 4))" -> list[+, 2, list[*, 3, 4]]
/*
    case white space - skip
    case number
    case non-number
    case (
 */

pub fn reader(str: &str) -> SExpr {
    todo!()
}