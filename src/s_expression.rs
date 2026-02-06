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
    let mut it = str.chars().peekable();
    read_expr(&mut it)
}

fn read_expr(it: &mut Peekable<Chars>) -> SExpr {
    skip_white_space(it);

    match it.peek() {
        Some('(') => read_list(it),
        Some(_) => read_atom(it),
        None => panic!("Unexpected input"),
    }
}

fn skip_white_space(it: &mut Peekable<Chars>) {
    while let Some(c) = it.peek() {
        if c.is_whitespace() {
            it.next();
        }
        else {
            break;
        }
    }
}