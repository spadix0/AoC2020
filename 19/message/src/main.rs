use std::{
    io, str,
    collections::HashMap,
    ops::Range,
};
use regex::bytes::Regex;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let (rules, msgs) = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", rules);

    let rules = RuleSet::from_rules(rules);
    //println!("{:?}", rules.patterns);

    println!("part[1]: {}", rules.count_acyclic(&msgs));
    println!("part[2]: {}", rules.count_cyclic_42_31(&msgs));
}


//----------------------------------------------------------------------------
#[derive(Debug)]
struct RuleSet {
    nodes: HashMap<u32, Rule>,
    patterns: HashMap<u32, String>,
}

impl RuleSet {
    fn from_rules(nodes: HashMap<u32, Rule>) -> RuleSet {
        let mut patterns = HashMap::with_capacity(nodes.len());
        for &r in nodes.keys() {
            gen_pat(&nodes, &mut patterns, r);
        }
        RuleSet { nodes, patterns }
    }

    fn count_acyclic(&self, msgs: &[impl AsRef<[u8]>]) -> usize {
        let pat = format!("^{}$", self.patterns.get(&0).unwrap());
        let re = Regex::new(&pat).unwrap();
        msgs.into_iter()
            .filter(|s| re.is_match(s.as_ref()))
            .count()
    }

    fn count_cyclic_42_31(&self, msgs: &[impl AsRef<[u8]>]) -> usize {
        if let (Some(pat42), Some(pat31)) = (
            self.patterns.get(&42),
            self.patterns.get(&31),
        ) {
            // need to match manually extracted rule[0] = 'X+X{n}Y{n}',
            // which regex can't represent, so start w/more relaxed
            let pat = format!("^{}{{2,}}(){}+$", pat42, pat31);
            let full = Regex::new(&pat).unwrap();

            // then manually validate constraint by counting Xs and Ys
            // (which doesn't work in general, but ok here)
            let re42 = Regex::new(&pat42).unwrap();
            let re31 = Regex::new(&pat31).unwrap();

            msgs.into_iter()
                .filter(|s| {
                    let s = s.as_ref();
                    full.captures(s)
                        .and_then(|c| c.get(1))
                        .filter(|g| {
                            let i = g.start();
                            let n42 = count_fullmatches(&re42, s, 0..i);
                            let n31 = count_fullmatches(&re31, s, i..s.len());
                            n42 > n31
                        })
                        .is_some()
                })
                .count()
        } else {
            0
        }
    }
}


fn gen_pat(
    nodes: &HashMap<u32, Rule>,
    patterns: &mut HashMap<u32, String>,
    id: u32) -> usize
{
    if let Some(s) = patterns.get(&id) {
        return s.len();
    }

    let pat = match nodes.get(&id).unwrap() {
        Term(c) => (*c).into(),
        Alts(alts) => {
            let sz: usize = alts.iter()
                .flatten()
                .map(|&r| gen_pat(nodes, patterns, r))
                .sum();

            let mut pat = String::with_capacity(4-1 + sz + alts.len());
            pat.push_str("(?:");
            for (i, a) in alts.iter().enumerate() {
                if i > 0 {
                    pat.push('|');
                }
                for s in a.iter() {
                    pat.push_str(patterns.get(s).unwrap());
                }
            }
            pat.push(')');
            pat
        },
    };

    let n = pat.len();
    patterns.insert(id, pat);
    n
}


fn count_fullmatches(regex: &Regex, s: &[u8], mut range: Range<usize>)
    -> usize
{
    let mut n = 0;
    while !range.is_empty() {
        let m = regex.find(&s[range.clone()]).unwrap();
        range.start += m.end();
        n += 1;
    }
    n
}


//----------------------------------------------------------------------------
#[derive(Debug)]
enum Rule {
    Term(char),
    Alts(Vec<Seq>),
}
use Rule::*;

type Seq = Vec<u32>;

fn read(stm: &mut impl io::Read) -> (HashMap<u32, Rule>, Vec<Vec<u8>>)
{
    use io::BufRead;
    let mut lines = io::BufReader::new(stm).lines();
    (
        (&mut lines)
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .map(|line| parse_rule(&line))
            .collect(),

        (&mut lines)
            .map(|line| line.unwrap()
                 .as_bytes().iter()
                 .cloned()
                 .collect())
            .collect(),
    )
}


fn parse_rule(s: &str) -> (u32, Rule) {
    let mut t = s.split(": ");
    let id = t.next().unwrap().parse().unwrap();
    let rhs = t.next().unwrap();

    if rhs.starts_with('"') {
        (id, Term(rhs.chars().nth(1).unwrap()))
    } else {
        (id, Alts(
            rhs.split(" | ")
                .map(|alt| {
                    alt.split_whitespace()
                        .map(|t| t.parse().unwrap())
                        .collect()
                })
                .collect()
        ))
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashSet;

    fn check_acyclic(input: &str, exp: &[&[u8]]) {
        let (rules, msgs) = read(&mut input.as_bytes());
        let rules = RuleSet::from_rules(rules);
        let exp: HashSet<_> = exp.into_iter().cloned().collect();
        for s in &msgs {
            assert_eq!(
                rules.count_acyclic(&[s]),
                exp.contains(&s[..]) as usize
            );
        }

        assert_eq!(rules.count_acyclic(&msgs), exp.len());
    }

    #[test]
    fn ex0_acyclic() {
        check_acyclic(EX0, &[b"ababbb", b"abbbab"]);
    }

    #[test]
    fn ex1_acyclic() {
        check_acyclic(EX1, &[
            b"bbabbbbaabaabba",
            b"ababaaaaaabaaab",
            b"ababaaaaabbbaba",
        ]);
    }

    #[test]
    fn ex1_cyclic() {
        let (rules, msgs) = read(&mut EX1.as_bytes());
        let rules = RuleSet::from_rules(rules);
        let exp: &[&[u8]] = &[
            b"bbabbbbaabaabba",
            b"babbbbaabbbbbabbbbbbaabaaabaaa",
            b"aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            b"bbbbbbbaaaabbbbaaabbabaaa",
            b"bbbababbbbaaaaaaaabbababaaababaabab",
            b"ababaaaaaabaaab",
            b"ababaaaaabbbaba",
            b"baabbaaaabbaaaababbaababb",
            b"abbbbabbbbaaaababbbbbbaaaababb",
            b"aaaaabbaabaaaaababaa",
            b"aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            b"aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        let exp: HashSet<_> = exp.iter().copied().collect();

        for s in &msgs {
            assert_eq!(
                rules.count_cyclic_42_31(&[s]),
                exp.contains(&s[..]) as usize
            );
        }

        assert_eq!(rules.count_cyclic_42_31(&msgs), 12);
    }

    #[test]
    fn answer1() {
        let (rules, msgs) = read(&mut INPUT.as_bytes());
        assert_eq!(RuleSet::from_rules(rules).count_acyclic(&msgs), 291);
    }

    #[test]
    fn answer2() {
        let (rules, msgs) = read(&mut INPUT.as_bytes());
        assert_eq!(RuleSet::from_rules(rules).count_cyclic_42_31(&msgs), 409);
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const EX1: &str = include_str!("../../ex1.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
