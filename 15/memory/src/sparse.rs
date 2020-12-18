use fxhash::FxHashMap;

#[derive(Debug)]
pub struct Game {
    mem: FxHashMap<u32, u32>,
    prev_turn: u32,
    turn: u32,
}

impl super::Game for Game {
    fn from_seed(seed: &[u32]) -> Box<dyn super::Game> {
        let mut mem: FxHashMap<u32, u32> = FxHashMap::default();
        let mut tp = 0;
        for (t, &n) in seed.iter().enumerate() {
            tp = mem.get(&n).copied().unwrap_or(t as u32);
            mem.insert(n, t as u32);
        }

        Box::new(Game {
            mem: mem,
            prev_turn: tp,
            turn: seed.len() as u32 - 1,
        })
    }

    fn play_until(&mut self, turn: u32) -> u32 {
        let mut n = 0;
        while self.turn < turn-1 {
            n = self.turn - self.prev_turn;
            self.turn += 1;
            self.prev_turn = self.mem.get(&n).copied().unwrap_or(self.turn);
            self.mem.insert(n, self.turn);
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
