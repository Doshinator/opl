use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpr {
    Num(i32),
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

// should return a list
fn read_list(it: &mut Peekable<Chars>) -> SExpr {
    // (+ e e)
    it.next(); // consume + 2 3 )
    let mut elements = Vec::new();
    loop {
        skip_white_space(it);

        match it.peek() {
            Some(')') => {
                it.next();
                break;
            },
            Some(_) => {
                let expr = read_expr(it);
                elements.push(expr);
            },
            None => panic!("Unexpected input"),
        }
    }

    SExpr::List(elements)
}


// A number or a symbol 
// (+ 2 3) put everything inside the () into a
fn read_atom(it: &mut Peekable<Chars>) -> SExpr {
    let mut s = String::new();

    while let Some(ch) = it.peek() {
        if ch.is_whitespace() || *ch == '(' || *ch == ')' {
            break;
        }
        s.push(it.next().unwrap());
    }

    if let Ok(n) = s.parse::<i32>() {
        SExpr::Num(n)
    } else {
        SExpr::Sym(s)
    }
}