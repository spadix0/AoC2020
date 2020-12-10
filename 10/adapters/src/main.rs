use std::{io, collections::HashMap};
use itertools::Itertools;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let diff = read_diffs(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", diff);

    let hist = histogram(&diff);
    //println!("{:?}", hist);
    println!("part[1]: {}", hist[&1] * hist[&3]);

    println!("part[2]: {}", count_combos(&diff));
}


fn histogram(diff: &[u8]) -> HashMap<u8, u32> {
    let mut hist = HashMap::with_capacity(2);
    for &d in diff {
        *hist.entry(d).or_insert(0) += 1;
    }
    hist
}


fn count_combos(diff: &[u8]) -> u64 {
    // 3-diffs "pin" their endpoints => those must both be included
    // just find runs of 1-diffs and multiply those independent combinations
    let runs = &diff.iter()
        .group_by(|&d| d);

    let mut memo = HashMap::new();

    runs.into_iter()
        .filter(|(&k, _)| k == 1)
        .map(|(_, g)| {
            sum3_combos(
                &mut memo,
                g.cloned()
                    .map_into::<i32>()
                    .sum())
        })
        .product()
}


fn sum3_combos(memo: &mut HashMap<i32, u64>, n: i32) -> u64 {
    match n {
        n if n < 0 => 0,
        n if n == 0 => 1,
        n => {
            memo.get(&n)
                .cloned()
                .unwrap_or_else(|| {
                    let v = sum3_combos(memo, n-3)
                        + sum3_combos(memo, n-2)
                        + sum3_combos(memo, n-1);
                    memo.insert(n, v);
                    v
                })
        }
    }
}


fn read_diffs(stm: &mut impl io::Read) -> Vec<u8> {
    use io::BufRead;
    let mut data: Vec<i32> = io::BufReader::new(stm)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    data.push(0);
    data.sort_unstable();
    data.push(data.last().unwrap() + 3);

    data.iter()
        .zip(data.iter().skip(1))
        .map(|(x0, x1)| (x1 - x0) as u8)
        .collect()
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_hist() {
        check_hist(&read_diffs(&mut EX0.as_bytes()), (7, 5));
    }

    #[test]
    fn ex1_hist() {
        check_hist(&read_diffs(&mut EX1.as_bytes()), (22, 10));
    }

    #[test]
    fn answer1() {
        check_hist(&read_diffs(&mut INPUT.as_bytes()), (70, 27));
    }

    #[test]
    fn ex0_combos() {
        assert_eq!(count_combos(&read_diffs(&mut EX0.as_bytes())), 8);
    }

    #[test]
    fn ex1_combos() {
        assert_eq!(count_combos(&read_diffs(&mut EX1.as_bytes())), 19208);
    }

    #[test]
    fn answer2() {
        assert_eq!(count_combos(&read_diffs(&mut INPUT.as_bytes())),
                   49607173328384);
    }

    #[test]
    fn combos() {
        let cases = [
            (1, 1),
            (2, 2),
            (3, 4),
            (4, 7),
        ];

        let mut memo = HashMap::new();
        for &(n, c) in &cases {
            assert_eq!(sum3_combos(&mut memo, n), c);
        }
    }

    fn check_hist(diff: &[u8], exp: (u32, u32)) {
        assert_eq!(
            histogram(diff),
            [(1, exp.0), (3, exp.1)].iter().cloned().collect()
        );
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const EX1: &str = include_str!("../../ex1.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
