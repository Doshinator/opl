use std::{iter::Peekable, str::Chars};

pub enum SExpr {
    SESym(String),
    SExprNum(i64),
    SExprList(Vec<Box<SExpr>>),
}

// "(+ 2 (* 3 4))" -> list[+, 2, list[*, 3, 4]]
pub fn reader(str: &str) -> SExpr {
    // case number
    // case non-number
    // case (

    for c in str.chars() {
        print!("{}", c);
    }
    // parse
    // finish impl of reader to turn string into SExpr
    SExpr::SExprNum(5)
}

pub fn read_expr(char: &mut Peekable<Chars>) -> SExpr {
    /*
    skip whitespace

    if next char == '('
        consume (
        let elements = []
        while next char != ')'
            elements.push(read_expr(char))
        consume ')'
        return List(elements)
    else if next char is digit or '-'
        read number
        return Num(n)
    
    else
        read symbol
        return Symbol(name)
    */
}