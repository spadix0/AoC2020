use std::io;
use fxhash::FxHashSet as HashSet;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let init = Game::read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", init);

    println!("part[1]: {}", play(init.clone()).score());
    println!("part[2]: {}", play_rec(init.clone()).score());
}


fn play(mut game: Game) -> Game {
    while game.winner() == None {
        game = round(&game);
    }
    game
}


fn round(game: &Game) -> Game {
    let (c0, c1) = game.peek();
    game.round((c1 > c0) as u8)
}


fn play_rec(mut game: Game) -> Game {
    let mut memo = HashSet::default();
    while game.winner() == None {
        game = if memo.contains(&game) {
            Game { state: vec![0, 1] }
        } else {
            let next = game.round(
                if let Some(sub) = game.recurse() {
                    play_rec(sub).winner().unwrap()
                } else {
                    let (c0, c1) = game.peek();
                    (c1 > c0) as u8
                }
            );
            memo.insert(game);
            next
        }
    }

    game
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Game {
    state: Vec<u8>,
}

impl Game {
    fn read(stm: &mut impl io::Read) -> Game {
        use io::BufRead;
        let (mut p, mut n0) = (0, 0);
        let mut state = Vec::with_capacity(128);
        for line in io::BufReader::new(stm).lines() {
            let line = line.unwrap();
            if line.len() == 0 {
                p += 1;
            } else if line.starts_with("Player ") {
                // ignore
            } else {
                if p == 0 { n0 += 1 }
                state.push(line.parse().unwrap());
            }
        }

        state.push(n0);
        Game { state }
    }

    fn winner(&self) -> Option<u8> {
        let &n0 = self.state.last().unwrap();
        if n0 == 0 {
            Some(1)
        } else if n0 as usize == self.state.len()-1 {
            Some(0)
        } else {
            None
        }
    }

    fn peek(&self) -> (u8, u8) {
        let n0 = *self.state.last().unwrap() as usize;
        (self.state[0], self.state[n0])
    }

    fn round(&self, win: u8) -> Game {
        let n = self.state.len();
        let n0 = *self.state.last().unwrap() as usize;
        let mut state = Vec::with_capacity(n);
        state.extend(&self.state[1..n0]);
        if win == 0 {
            state.push(self.state[0]);
            state.push(self.state[n0]);
        }
        state.extend(&self.state[n0+1..n-1]);
        if win == 1 {
            state.push(self.state[n0]);
            state.push(self.state[0]);
        }
        state.push(if win == 0 { n0+1 } else { n0-1 } as u8);
        assert_eq!(state.len(), n);
        Game { state }
    }

    fn recurse(&self) -> Option<Game> {
        let n = self.state.len();
        let n0 = self.state[n-1] as usize;
        let c0 = self.state[0] as usize;
        let c1 = self.state[n0] as usize;
        if c0 < n0 && c1 < n-1-n0 {
            let mut state = Vec::with_capacity(c0+c1+1);
            state.extend(&self.state[1 .. 1+c0]);
            state.extend(&self.state[1+n0 .. 1+n0+c1]);
            state.push(c0 as u8);
            Some(Game { state })
        } else {
            None
        }
    }

    fn score(&self) -> usize {
        self.state[0..self.state.len()-1].into_iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (i+1) * c as usize)
            .sum()
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_play() {
        let game = play(Game::read(&mut EX0.as_bytes()));
        assert_eq!(game.state, vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1, 0]);
        assert_eq!(game.score(), 306);
    }

    #[test]
    fn answer1() {
        let game = play(Game::read(&mut INPUT.as_bytes()));
        assert_eq!(game.score(), 34127);
        assert_eq!(game.winner(), Some(1));
    }

    #[test]
    fn ex0_rec() {
        let game = play_rec(Game::read(&mut EX0.as_bytes()));
        assert_eq!(game.state, vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3, 0]);
        assert_eq!(game.score(), 291);
    }

    #[test]
    fn answer2() {
        let game = play_rec(Game::read(&mut INPUT.as_bytes()));
        assert_eq!(game.score(), 32054);
        assert_eq!(game.winner(), Some(1));
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
