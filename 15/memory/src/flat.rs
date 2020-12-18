
#[derive(Debug)]
pub struct Game {
    mem: Vec<u32>,
    prev_turn: u32,
    turn: u32,
}

impl super::Game for Game {
    fn from_seed(seed: &[u32]) -> Box<dyn super::Game> {
        use std::iter::repeat;
        let mut mem: Vec<_> = repeat(0).take(30_000_000).collect();
        let mut tp = 0;

        for (t, &n) in seed.iter().enumerate() {
            let t = 1 + t as u32;
            tp = mem[n as usize];
            if tp == 0 { tp = t }
            mem[n as usize] = t;
        }

        Box::new(Game {
            mem: mem,
            prev_turn: tp,
            turn: seed.len() as u32,
        })
    }

    fn play_until(&mut self, turn: u32) -> u32 {
        let mut n = 0;
        while self.turn < turn {
            n = self.turn - self.prev_turn;
            self.turn += 1;
            self.prev_turn = self.mem[n as usize];
            if self.prev_turn == 0 { self.prev_turn = self.turn; }
            self.mem[n as usize] = self.turn;
        }

        n
    }

    fn size(&self) -> usize {
        self.mem.len()
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use crate::{*, tests::*};

    common_tests!(super::Game);
}
