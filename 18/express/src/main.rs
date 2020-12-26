#[macro_use]
extern crate lazy_static;

use std::{
    io,
    collections::HashMap,
    iter::Peekable,
    slice::Iter,
};


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let expr = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", expr);

    println!("part[1]: {}", checksum(&eval_all(&expr, &[])));
    println!("part[2]: {}", checksum(&eval_all(&expr, &[(Op('+'), 2)])));
}


fn eval_all(expr: &[Vec<Token>], bind: &[(Token, i32)]) -> Vec<i64> {
    expr.iter()
        .map(|e| Parser::for_tokens(e, bind).eval(0))
        .collect()
}


fn checksum(data: &[i64]) -> i64 {
    data.iter().sum()
}


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum Token {
    Op(char),
    Num(i32),
}
use Token::*;

struct Parser<'a> {
    toks: Peekable<Iter<'a, Token>>,
    bind: HashMap<Token, i32>,
}

impl Parser<'_> {
    fn for_tokens<'a>(toks: &'a [Token], bind: &[(Token, i32)]) -> Parser<'a> {
        Parser {
            toks: toks.iter().peekable(),
            bind: [
                (Op('+'), 1),
                (Op('*'), 1),
            ].iter()
                .cloned()
                .chain(bind.iter().cloned())
                .collect()
        }
    }

    fn eval(&mut self, bind: i32) -> i64 {
        let mut val = match self.toks.next().unwrap() {
            Op('(') => {
                let v = self.eval(0);
                let t = self.toks.next();
                assert_eq!(t, Some(&Op(')')));
                v
            },
            &Num(v) => v as i64,
            _ => panic!()
        };

        loop {
            let b = self.toks.peek()
                .cloned()
                .and_then(|t| self.bind.get(t).cloned())
                .unwrap_or(0);
            if b <= bind {
                return val;
            }

            match self.toks.next() {
                Some(Op('*')) => val *= self.eval(b),
                Some(Op('+')) => val += self.eval(b),
                None => return val,
                _ => panic!()
            }
        }
    }
}


fn read(stm: &mut impl io::Read) -> Vec<Vec<Token>> {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| tokenize(&line.unwrap()))
        .collect()
}


fn tokenize(s: &str) -> Vec<Token> {
    use regex::Regex;
    lazy_static! {
        static ref TOKEN_RE: Regex = Regex::new(r"\d+|[(+*)]").unwrap();
    }

    TOKEN_RE.find_iter(s)
        .map(|m| {
            let t = m.as_str();
            if t.chars().all(|c| c.is_digit(10)) {
                Num(t.parse().unwrap())
            } else {
                Op(t.parse().unwrap())
            }
        })
        .collect()
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ex0_ltr() {
        assert_eq!(
            eval_all(&read(&mut EX0.as_bytes()), &[]),
            &[ 71, 51, 26, 437, 12240, 13632 ]
        );
    }

    #[test]
    fn ex0_prec() {
        assert_eq!(
            eval_all(&read(&mut EX0.as_bytes()), &[(Op('+'), 2)]),
            &[ 231, 51, 46, 1445, 669060, 23340 ]
        );
    }

    #[test]
    fn answer1() {
        assert_eq!(
            checksum(&eval_all(&read(&mut INPUT.as_bytes()), &[])),
            12956356593940
        );
    }

    #[test]
    fn answer2() {
        assert_eq!(
            checksum(&eval_all(&read(&mut INPUT.as_bytes()), &[(Op('+'), 2)])),
            94240043727614
        );
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
