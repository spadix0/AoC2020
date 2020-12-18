use std::fs::read_to_string;
use memory::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let seed = parse(&read_to_string(path).unwrap());
    //println!("{:?}", seed);

    let mut game = flat::Game::from_seed(&seed);
    println!("part[1]: {}", game.play_until(2020));
    println!("part[2]: {}", game.play_until(30_000_000));
    //println!("{}", game.size());
}
