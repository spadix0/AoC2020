#![allow(non_snake_case)]

pub mod sparse;
pub mod flat;


pub trait Game {
    fn from_seed(seed: &[u32]) -> Box<dyn Game> where Self: Sized;
    fn play_until(&mut self, turn: u32) -> u32;
    fn size(&self) -> usize;
}


pub fn parse(s: &str) -> Vec<u32> {
    s.trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    #[macro_export]
    macro_rules! common_tests {
        ($Game:ty) => {
            #[test]
            fn ex0_2020() {
                let mut game = <$Game>::from_seed(&parse(&EX0));
                assert_eq!(game.play_until(2020), 436);
            }

            #[test]
            fn ex1_2020() {
                let mut game = <$Game>::from_seed(&parse("1,3,2"));
                assert_eq!(game.play_until(2020), 1);
            }

            #[test]
            fn ex2_2020() {
                let mut game = <$Game>::from_seed(&parse("2,1,3"));
                assert_eq!(game.play_until(2020), 10);
            }

            #[test]
            fn ex3_2020() {
                let mut game = <$Game>::from_seed(&parse("1,2,3"));
                assert_eq!(game.play_until(2020), 27);
            }

            #[test]
            fn ex4_2020() {
                let mut game = <$Game>::from_seed(&parse("2,3,1"));
                assert_eq!(game.play_until(2020), 78);
            }

            #[test]
            fn ex5_2020() {
                let mut game = <$Game>::from_seed(&parse("3,2,1"));
                assert_eq!(game.play_until(2020), 438);
            }

            #[test]
            fn ex6_2020() {
                let mut game = <$Game>::from_seed(&parse("3,1,2"));
                assert_eq!(game.play_until(2020), 1836);
            }

            #[test]
            fn ex0_30M() {
                let mut game = <$Game>::from_seed(&parse(&EX0));
                assert_eq!(game.play_until(30_000_000), 175594);
            }

            #[test]
            fn ex1_30M() {
                let mut game = <$Game>::from_seed(&parse("1,3,2"));
                assert_eq!(game.play_until(30_000_000), 2578);
            }

            #[test]
            fn ex2_30M() {
                let mut game = <$Game>::from_seed(&parse("2,1,3"));
                assert_eq!(game.play_until(30_000_000), 3544142);
            }

            #[test]
            fn ex3_30M() {
                let mut game = <$Game>::from_seed(&parse("1,2,3"));
                assert_eq!(game.play_until(30_000_000), 261214);
            }

            #[test]
            fn ex4_30M() {
                let mut game = <$Game>::from_seed(&parse("2,3,1"));
                assert_eq!(game.play_until(30_000_000), 6895259);
            }

            #[test]
            fn ex5_30M() {
                let mut game = <$Game>::from_seed(&parse("3,2,1"));
                assert_eq!(game.play_until(30_000_000), 18);
            }

            #[test]
            fn ex6_30M() {
                let mut game = <$Game>::from_seed(&parse("3,1,2"));
                assert_eq!(game.play_until(30_000_000), 362);
            }

            #[test]
            fn answer1() {
                let mut game = <$Game>::from_seed(&parse(&INPUT));
                assert_eq!(game.play_until(2020), 234);
            }

            #[test]
            fn answer2() {
                let mut game = <$Game>::from_seed(&parse(&INPUT));
                assert_eq!(game.play_until(30_000_000), 8984);
            }
        };
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
