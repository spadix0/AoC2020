use std::{io, error, str::FromStr};
use simple_error::SimpleError as SError;

type BError = Box<dyn error::Error>;
type BResult<T> = Result<T, BError>;


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let data = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", data);

    let n1 = data.iter()
        .filter(|(p, s)| Rule1::is_valid(p, s))
        .count();
    println!("part[1]: {}", n1);

    let n2 = data.iter()
        .filter(|(p, s)| Rule2::is_valid(p, s))
        .count();
    println!("part[2]: {}", n2);
}


//----------------------------------------------------------------------------
#[derive(Copy, Clone, Default, PartialEq, Debug)]
struct Policy {
    lo: usize,
    hi: usize,
    ch: char,
}


struct Rule1;
struct Rule2;

trait Validator {
    fn is_valid(p: &Policy, s: &str) -> bool;
}

impl Validator for Rule1 {
    fn is_valid(p: &Policy, s: &str) -> bool {
        let n = s.chars()
            .filter(|&c| c == p.ch)
            .count();
        p.lo <= n && n <= p.hi
    }
}

impl Validator for Rule2 {
    fn is_valid(p: &Policy, s: &str) -> bool {
        let lo = s.chars().nth(p.lo-1);
        let hi = s.chars().nth(p.hi-1);
        (lo == Some(p.ch)) ^ (hi == Some(p.ch))
    }
}


//----------------------------------------------------------------------------
fn read(stm: &mut impl io::Read) -> Vec<(Policy, String)> {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| parse_entry(&line.unwrap()).unwrap())
        .collect()
}

fn parse_entry(s: &str) -> BResult<(Policy, String)> {
    let mut toks = s.trim().split(": ");
    Ok((
        toks.next()
            .ok_or(SError::new("bad entry"))?
            .parse()?,
        toks.next()
            .ok_or(SError::new("bad entry"))?
            .parse()?
    ))
}

impl FromStr for Policy {
    type Err = BError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut toks = s.trim().split_whitespace();
        let mut range = toks.next()
            .ok_or(SError::new("bad policy"))?
            .split("-");

        Ok(Policy{
            lo: range.next()
                .ok_or_else(|| SError::new("bad range"))?
                .parse()?,
            hi: range.next()
                .ok_or_else(|| SError::new("bad range"))?
                .parse()?,
            ch: toks.next()
                .ok_or_else(|| SError::new("bad policy"))?
                .parse()?,
        })
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(read_ex0(), [
            (Policy {lo: 1, hi: 3, ch: 'a'}, "abcde".into()),
            (Policy {lo: 1, hi: 3, ch: 'b'}, "cdefg".into()),
            (Policy {lo: 2, hi: 9, ch: 'c'}, "ccccccccc".into()),
        ]);
    }

    #[test]
    fn ex0_rule1() {
        for ((p,s), &exp) in read_ex0().iter().zip(&[true, false, true]) {
            assert_eq!(exp, Rule1::is_valid(p, s));
        }
    }

    #[test]
    fn ex0_rule2() {
        for ((p,s), &exp) in read_ex0().iter().zip(&[true, false, false]) {
            assert_eq!(exp, Rule2::is_valid(p, s));
        }
    }

    fn read_ex0() -> Vec<(Policy, String)> {
        return super::read(&mut EX0.as_bytes())
    }

    const EX0: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
}
