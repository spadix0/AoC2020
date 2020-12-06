use std::{
    collections::HashSet,
    fs::read_to_string,
};

type Entry = HashSet<char>;
type Group = Vec<Entry>;


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let groups = parse_groups(&read_to_string(path).unwrap());

    println!("part[1]: {}", part1(&groups));
    println!("part[2]: {}", part2(&groups));
}


fn part1(groups: &Vec<Group>) -> usize {
    groups.iter()
        .map(count_anyone_yes)
        .sum()
}


fn part2(groups: &Vec<Group>) -> usize {
    groups.iter()
        .map(count_everyone_yes)
        .sum()
}


fn count_anyone_yes(group: &Group) -> usize {
    group.iter()
        // running union
        .fold(Entry::new(),
              |mut acc, g| { acc.extend(g); acc })
        .len()
}


fn count_everyone_yes(group: &Group) -> usize {
    group.iter()
        // running intersection
        .fold(group[0].clone(),
              |mut acc, g| { acc.retain(|e| g.contains(e)); acc })
        .len()
}


fn parse_groups(slurpee: &str) -> Vec<Group> {
    slurpee.split("\n\n")
        .map(parse_group)
        .collect()
}


fn parse_group(group: &str) -> Group {
    group.split("\n")
        .filter(|line| line.len() > 0)
        .map(|entry| entry.chars().collect())
        .collect()
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_anyone() {
        for (group, &exp) in parse_groups(EX0).iter()
            .zip(&[ 3, 3, 3, 1, 1 ])
        {
            assert_eq!(exp, count_anyone_yes(group));
        }
    }

    #[test]
    fn ex0_everyone() {
        for (group, &exp) in parse_groups(EX0).iter()
            .zip(&[ 3, 0, 1, 1, 1 ])
        {
            assert_eq!(exp, count_everyone_yes(group));
        }
    }

    #[test]
    fn ex0_part1() {
        assert_eq!(part1(&parse_groups(EX0)), 11);
    }

    #[test]
    fn ex0_part2() {
        assert_eq!(part2(&parse_groups(EX0)), 6);
    }

    #[test]
    fn answer1() {
        assert_eq!(part1(&parse_groups(INPUT)), 6590);
    }

    #[test]
    fn answer2() {
        assert_eq!(part2(&parse_groups(INPUT)), 3288);
    }

    const EX0: &str = include_str!("../../ex0.txt");
    const INPUT: &str = include_str!("../../input.txt");
}
