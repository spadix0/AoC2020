use std::{io, collections::{HashMap, HashSet}};

type Content = HashMap<String, usize>;
type Rules = HashMap<String, Content>;

const MY_BAG: &str = "shiny gold";


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let rules = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", rules);
    //println!("{:?}", contains(&rules, MY_BAG));

    println!("part[1]: {}", contains(&rules, MY_BAG).len());
    println!("part[2]: {}", count_contents(&rules, MY_BAG));
}


fn contains<'a>(rules: &'a Rules, dst: &str) -> HashSet<&'a str> {
    // use dst key from rules for lifetime consistency
    let dst = &rules.get_key_value(dst).unwrap().0[..];

    struct Env<'b> {
        rules: &'b Rules,
        memos: HashMap<&'b str, bool>,
    }

    fn search(env: &mut Env, bag: &str) -> bool {
        let memo = env.memos.get(bag).cloned();
        memo.unwrap_or_else(|| {
            // NB use key from rules for lifetime consistency
            let (bag, edges) = env.rules.get_key_value(bag).unwrap();
            let found = edges.keys()
                .any(|edge| search(env, edge));
            env.memos.insert(bag, found);
            found
        })
    }

    let mut env = Env { rules, memos: HashMap::new() };
    env.memos.insert(dst, true);
    for bag in rules.keys() {
        search(&mut env, bag);
    }

    env.memos.iter()
        .filter_map(|(&bag, &found)| {
            if found && bag != dst { Some(bag) } else { None }
        })
        .collect()
}


fn count_contents(rules: &Rules, src: &str) -> usize {
    struct Env<'a> {
        rules: &'a Rules,
        memos: HashMap<&'a str, usize>,
    }

    fn count_rec(env: &mut Env, bag: &str) -> usize {
        let memo = env.memos.get(bag).cloned();
        memo.unwrap_or_else(|| {
            // NB use key from rules for lifetime consistency
            let (bag, edges) = env.rules.get_key_value(bag).unwrap();
            let n = 1 + edges.iter()
                .map(|(edge, weight)| weight * count_rec(env, edge))
                .sum::<usize>();
            env.memos.insert(bag, n);
            n
        })
    }

    count_rec(&mut Env{ rules, memos: HashMap::new() }, src) - 1
}


fn read(stm: &mut impl io::Read) -> Rules {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| parse_rule(line.unwrap().trim()))
        .collect()
}


fn parse_rule(rule: &str) -> (String, Content) {
    let mut toks = rule.trim_end_matches('.')
        .split(" bags contain ");
    let lhs = toks.next().unwrap();
    let rhs = toks.next().unwrap();
    (
        lhs.into(),
        if rhs == "no other bags" {
            Content::new()
        } else {
            rhs.split(", ")
                .map(parse_content_item)
                .collect()
        }
    )
}


fn parse_content_item(s: &str) -> (String, usize)
{
    let mut toks = s.trim_end_matches('s')
        .strip_suffix(" bag").unwrap()
        .splitn(2, " ");
    let n = toks.next().unwrap().parse().unwrap();
    (toks.next().unwrap().into(), n)
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_read() {
        let rules = read(&mut EX0.as_bytes());
        assert_eq!(9, rules.len());
        assert_eq!(0, rules["faded blue"].values().sum::<usize>());
        assert_eq!(11, rules["vibrant plum"].values().sum::<usize>());
    }

    #[test]
    fn contains_ex0() {
        let rules = read(&mut EX0.as_bytes());
        assert_eq!(contains(&rules, &MY_BAG), [
            "bright white", "muted yellow", "dark orange", "light red"
        ].iter().cloned().collect());
    }

    #[test]
    fn count_contents_ex0() {
        let rules = read(&mut EX0.as_bytes());
        let cases = &[
            ("faded blue", 0),
            ("dotted black", 0),
            ("dark olive", 7),
            ("vibrant plum", 11),
            ("shiny gold", 32),
        ];
        for &(bag, exp) in cases {
            assert_eq!(exp, count_contents(&rules, bag));
        }
    }

    #[test]
    fn count_contents_ex1() {
        let rules = read(&mut EX1.as_bytes());
        assert_eq!(126, count_contents(&rules, &MY_BAG));
    }

    #[test]
    fn answer1() {
        let rules = read(&mut INPUT.as_bytes());
        assert_eq!(316, contains(&rules, &MY_BAG).len());
    }

    #[test]
    fn answer2() {
        let rules = read(&mut INPUT.as_bytes());
        assert_eq!(11310, count_contents(&rules, &MY_BAG));
    }

    const EX0: &str = include_str!("../../ex0.txt");
    const EX1: &str = include_str!("../../ex1.txt");
    const INPUT: &str = include_str!("../../input.txt");
}
