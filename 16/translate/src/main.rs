use std::{
    iter::{repeat, repeat_with},
    collections::HashSet,
    fs::read_to_string,
};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let (rules, own, nearby) = parse(&read_to_string(path).unwrap());

    let rulemap = IntervalMap::from_rules(&rules);

    let reject = find_invalid(&rulemap, nearby.iter().flatten().cloned());
    println!("part[1]: {}", reject.iter().sum::<i32>());

    let mut cons = constrain_valid(&rulemap, &own, &nearby);
    let match_ = resolve_fields(&mut cons);
    println!("part[2]: {}", hash_departure(&rules, &match_, &own));
    println!("field order: {}",
             match_.iter()
             .map(|&n| rules[n as usize].name.as_str())
             .collect::<Vec<&str>>()
             .join(", "));
}


fn find_invalid(map: &IntervalMap, values: impl IntoIterator<Item=i32>)
    -> Vec<i32>
{
    values.into_iter()
        .filter(|&v| map.get(v).is_none())
        .collect()
}


fn hash_departure(rules: &[Rule], match_: &[u32], own: &[i32]) -> i64
{
    match_.iter()
        .zip(own)
        .filter(|(&r, _)| rules[r as usize].name.starts_with("departure"))
        .map(|(_, &v)| v as i64)
        .product()
}


fn constrain_valid(map: &IntervalMap, own: &[i32], nearby: &[impl AsRef<[i32]>])
    -> Vec<HashSet<u32>>
{
    let mut cons: Vec<HashSet<u32>> = own.iter()
        .map(|&v| {
            map.get(v).unwrap().iter()
                .cloned()
                .collect()
        })
        .collect();

    let mut upd: Vec<&[u32]> = Vec::with_capacity(cons.len());
    for t in nearby {
        upd.splice(0..upd.len(), t.as_ref().iter().map(|&v| {
            map.get(v).unwrap_or(&[])
        }));
        if upd.iter().all(|s| s.len() > 0) {
            for (c, u) in cons.iter_mut().zip(&upd) {
                c.retain(|v| u.contains(v));
            }
        }
    }

    cons
}


fn resolve_fields(cons: &mut [HashSet<u32>]) -> Vec<u32> {
    let n = cons.len();

    // collect reverse edges of bipartite graph
    // and frontier set of nodes to resolve
    let mut front: Vec<u32> = Vec::new();
    let mut rev: Vec<HashSet<u32>> = repeat_with(HashSet::new)
        .take(n)
        .collect();

    for (src, fwd) in cons.iter().enumerate() {
        if fwd.len() == 1 {
            front.push(src as u32);
        } else {
            for &dst in fwd {
                rev[dst as usize]
                    .insert(src as u32);
            }
        }
    }

    let mut match_: Vec<_> = repeat(n as u32).take(n).collect();

    // match frontier src to single dst and follow reverse edges
    // to remove all other forward edges that refer to dst
    while let Some(src) = front.pop() {
        let src = src as usize;
        let fwd = &cons[src];
        assert_eq!(fwd.len(), 1);
        let dst = fwd.iter().next().cloned().unwrap();

        assert_eq!(match_[src], n as u32);
        match_[src] = dst;

        for &s in &rev[dst as usize] {
            let f = &mut cons[s as usize];
            f.remove(&dst);
            if f.len() == 1 {
                front.push(s);
            }
        }
    }

    match_
}


//----------------------------------------------------------------------------
// map i32 value range intervals to sets of u32 node IDs
struct IntervalMap {
    ranges: Vec<i32>,
    index: Vec<u32>,
    nodes: Vec<u32>,
}


impl IntervalMap {
    fn from_rules(rules: &[Rule]) -> IntervalMap {
        let ranges = range_breaks(rules);

        // first pass - collect count of rules in each range
        let mut count = count_nodes(rules, &ranges);
        let index = accumulate(&count);

        // reuse to track offsets
        count.clear();
        count.resize(index.len(), 0);

        // second pass - collect mapping from range to set of rules
        // FIXME this begs to be compressed, but... meh
        let mut nodes: Vec<u32> = repeat(0)
            .take(*index.last().unwrap() as usize)
            .collect();

        for (v, rule) in rules.iter().enumerate() {
            for r in &rule.ranges {
                let i0 = ranges.binary_search(&r[0]).unwrap();
                let i1 = ranges.binary_search(&r[1]).unwrap_or_else(|k| k-1);
                for i in i0 ..= i1 {
                    let j = index[i] + count[i];
                    nodes[j as usize] = v as u32;
                    count[i] += 1;
                }
            }
        }

        IntervalMap { ranges, index, nodes }
    }

    fn get<'a>(&'a self, value: i32) -> Option<&'a [u32]> {
        if self.ranges[0] > value || value >= *self.ranges.last().unwrap() {
            return None;
        }

        let i = self.ranges.binary_search(&value).unwrap_or_else(|j| j-1);
        let j0 = self.index[i] as usize;
        let j1 = self.index[i+1] as usize;
        if j0 == j1 {
            None
        } else {
            Some(&self.nodes[j0..j1])
        }
    }
}


// flattened, sorted and uniquified range endpoints
fn range_breaks(rules: &[Rule]) -> Vec<i32> {
    let mut ranges: Vec<i32> = rules.iter()
        .flat_map(|rule| {
            rule.ranges.iter()
                .map(|&r| r[0])
                .chain(
                    rule.ranges.iter()
                        .map(|&r| r[1]+1))
        })
        .collect();

    ranges.sort_unstable();
    ranges.dedup();

    ranges
}


fn count_nodes(rules: &[Rule], ranges: &[i32]) -> Vec<u32> {
    let mut count: Vec<u32> = repeat(0).take(ranges.len()).collect();
    for rule in rules {
        for r in &rule.ranges {
            let i = ranges.binary_search(&r[0]).unwrap();
            let j = ranges.binary_search(&r[1]).unwrap_or_else(|k| k-1);
            for k in i..=j {
                count[k] += 1;
            }
        }
    }

    count
}


fn accumulate(vals: &[u32]) -> Vec<u32> {
    vals.iter()
        .scan(0, |acc, n| {
            let i = *acc;
            *acc += n;
            Some(i)
        })
        .collect()
}


//----------------------------------------------------------------------------
#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<[i32; 2]>,
}


fn parse(s: &str) -> (Vec<Rule>, Vec<i32>, Vec<Vec<i32>>) {
    let mut grps = s.trim().split("\n\n");
    let fields = grps.next().unwrap()
        .split('\n')
        .map(|l| parse_rule(l))
        .collect();

    let mut lines = grps.next().unwrap().split("\n");
    let hdr = lines.next().unwrap();
    assert_eq!(hdr, "your ticket:");
    let own = parse_ticket(lines.next().unwrap());

    let mut lines = grps.next().unwrap().split("\n");
    let hdr = lines.next().unwrap();
    assert_eq!(hdr, "nearby tickets:");
    let nearby = lines.map(parse_ticket).collect();

    (fields, own, nearby)
}


fn parse_rule(s: &str) -> Rule {
    let mut t = s.split(": ");
    Rule {
        name: t.next().unwrap().into(),
        ranges: t.next().unwrap()
            .split(" or ")
            .map(|r| {
                let mut t = r.split('-')
                    .map(|v| v.parse().unwrap());
                [ t.next().unwrap(), t.next().unwrap() ]
            })
            .collect()
    }
}


fn parse_ticket(s: &str) -> Vec<i32> {
    s.split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;

    fn parse_and_build(s: &str)
        -> (Vec<Rule>, IntervalMap, Vec<i32>, Vec<Vec<i32>>)
    {
        let (rules, own, nearby) = parse(s);
        let rulemap = IntervalMap::from_rules(&rules);
        (rules, rulemap, own, nearby)
    }

    fn parse_and_match(s: &str) -> (Vec<Rule>, Vec<u32>, Vec<i32>)
    {
        let (rules, rulemap, own, nearby) = parse_and_build(s);
        let mut cons = constrain_valid(&rulemap, &own, &nearby);
        let match_ = resolve_fields(&mut cons);
        (rules, match_, own)
    }

    #[test]
    fn ex0_invalid() {
        let (rules, rulemap, own, nearby) = parse_and_build(EX0);
        assert!(
            rules.iter().map(|r| &r.name)
                .eq(&[ "class", "row", "seat" ])
        );
        assert_eq!(own, &[ 7, 1, 14 ]);
        assert_eq!(
            find_invalid(&rulemap, nearby.iter().flatten().cloned()),
            &[ 4, 55, 12 ]
        );
    }

    #[test]
    fn ex1_invalid() {
        let (rules, rulemap, own, nearby) = parse_and_build(EX1);
        assert!(
            rules.iter().map(|r| &r.name)
                .eq(&[ "class", "row", "seat" ])
        );
        assert_eq!(own, &[ 11, 12, 13 ]);
        assert!(find_invalid(&rulemap, nearby.iter().flatten().cloned())
                .is_empty());
    }

    #[test]
    fn answer1() {
        let (_, rulemap, _, nearby) = parse_and_build(INPUT);
        assert_eq!(
            find_invalid(&rulemap, nearby.iter().flatten().cloned())
                .iter().sum::<i32>(),
            29759
        );
    }

    #[test]
    fn ex0_match() {
        let (rules, match_, _) = parse_and_match(EX0);
        assert!(
            match_.iter().map(|&r| &rules[r as usize].name)
                .eq(&[ "row", "class", "seat" ]));
    }

    #[test]
    fn ex1_match() {
        let (rules, match_, _) = parse_and_match(EX1);
        assert!(
            match_.iter().map(|&r| &rules[r as usize].name)
                .eq(&[ "row", "class", "seat" ]));
    }

    #[test]
    fn answer2() {
        let (rules, match_, own) = parse_and_match(INPUT);
        assert_eq!(hash_departure(&rules, &match_, &own), 1307550234719);
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const EX1: &str = include_str!("../../ex1.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
