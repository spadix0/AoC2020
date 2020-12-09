use std::{io, collections::HashSet};

pub const MY_BAG: &str = "shiny gold";


pub trait FromReader {
    fn from_reader(stm: &mut impl io::Read) -> Self;
}


pub trait Topology {
    // part 1: find set of bags which recursively contain dst bag
    fn contains(self: &Self, dst: &str) -> HashSet<&str>;

    // part 2: count total bags recursively contained in src bag
    fn count_contents(self: &Self, src: &str) -> usize;
}


// original implementation
pub mod basic;


#[cfg(test)]
mod tests {
    // FIXME eww...  surely must be better way to share interface tests
    // across implementors

    #[macro_export]
    macro_rules! common_tests {
        ($Rules:ty) => {
            #[test]
            pub fn contains_ex0() {
                let rules = <$Rules>::from_reader(&mut EX0.as_bytes());
                assert_eq!(rules.contains(MY_BAG), [
                    "bright white", "muted yellow", "dark orange", "light red"
                ].iter().cloned().collect());
            }

            #[test]
            pub fn count_contents_ex0() {
                let rules = <$Rules>::from_reader(&mut EX0.as_bytes());
                let cases = &[
                    ("faded blue", 0),
                    ("dotted black", 0),
                    ("dark olive", 7),
                    ("vibrant plum", 11),
                    ("shiny gold", 32),
                ];
                for &(bag, exp) in cases {
                    assert_eq!(exp, rules.count_contents(bag));
                }
            }

            #[test]
            fn count_contents_ex1() {
                let rules = <$Rules>::from_reader(&mut EX1.as_bytes());
                assert_eq!(126, rules.count_contents(MY_BAG));
            }

            #[test]
            fn answer1() {
                let rules = <$Rules>::from_reader(&mut INPUT.as_bytes());
                assert_eq!(316, rules.contains(MY_BAG).len());
            }

            #[test]
            fn answer2() {
                let rules = <$Rules>::from_reader(&mut INPUT.as_bytes());
                assert_eq!(11310, rules.count_contents(MY_BAG));
            }
        };
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const EX1: &str = include_str!("../../ex1.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
