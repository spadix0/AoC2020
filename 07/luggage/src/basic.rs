use std::{io, collections::{HashMap, HashSet}};

type Content = HashMap<String, usize>;

pub struct Rules(HashMap<String, Content>);


impl super::Topology for Rules {

    fn contains(self: &Self, dst: &str) -> HashSet<&str> {
        // use dst key from rules for lifetime consistency
        let dst = &self.0.get_key_value(dst).unwrap().0[..];

        struct Env<'b> {
            rules: &'b Rules,
            memos: HashMap<&'b str, bool>,
        }

        fn search(env: &mut Env, bag: &str) -> bool {
            let memo = env.memos.get(bag).cloned();
            memo.unwrap_or_else(|| {
                // NB use key from rules for lifetime consistency
                let (bag, edges) = env.rules.0.get_key_value(bag).unwrap();
                let found = edges.keys()
                    .any(|edge| search(env, edge));
                env.memos.insert(bag, found);
                found
            })
        }

        let mut env = Env { rules: self, memos: HashMap::new() };
        env.memos.insert(dst, true);
        for bag in self.0.keys() {
            search(&mut env, bag);
        }

        env.memos.iter()
            .filter_map(|(&bag, &found)| {
                if found && bag != dst { Some(bag) } else { None }
            })
            .collect()
    }


    fn count_contents(self: &Self, src: &str) -> usize {
        struct Env<'a> {
            rules: &'a Rules,
            memos: HashMap<&'a str, usize>,
        }

        fn count_rec(env: &mut Env, bag: &str) -> usize {
            let memo = env.memos.get(bag).cloned();
            memo.unwrap_or_else(|| {
                // NB use key from rules for lifetime consistency
                let (bag, edges) = env.rules.0.get_key_value(bag).unwrap();
                let n = 1 + edges.iter()
                    .map(|(edge, weight)| weight * count_rec(env, edge))
                    .sum::<usize>();
                env.memos.insert(bag, n);
                n
            })
        }

        count_rec(&mut Env{ rules: self, memos: HashMap::new() }, src) - 1
    }
}


impl super::FromReader for Rules {
    fn from_reader(stm: &mut impl io::Read) -> Self {
        use io::BufRead;
        Self(
            io::BufReader::new(stm)
                .lines()
                .map(|line| parse_rule(line.unwrap().trim()))
                .collect()
        )
    }
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


fn parse_content_item(s: &str) -> (String, usize) {
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
    use crate::{*, tests::*};

    common_tests!(Rules);

    #[test]
    fn ex0_read() {
        let rules = Rules::from_reader(&mut EX0.as_bytes());
        assert_eq!(9, rules.0.len());
        assert_eq!(0, rules.0["faded blue"].values().sum::<usize>());
        assert_eq!(11, rules.0["vibrant plum"].values().sum::<usize>());
    }
}
