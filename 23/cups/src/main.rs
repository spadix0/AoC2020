
fn main() {
    let path = std::env::args().nth(1).unwrap();
    let seed = parse(&std::fs::read_to_string(path).unwrap());
    //println!("{:?}", seed);

    let mut raft = Raft::init(&seed, seed.len());
    //println!("{:?}", raft);
    println!("part[1]: {}", raft.run(100).dump8());

    let mut raft = Raft::init(&seed, 1_000_000);
    raft.run(10_000_000);
    let c2 = raft.cups[1] as u64;
    let c3 = raft.cups[c2 as usize] as u64;
    println!("part[2]: {} * {} = {}", c2, c3, c2*c3);
}


fn parse(s: &str) -> Vec<u32> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}


#[derive(Debug)]
struct Raft {
    cups: Vec<u32>,
}

impl Raft {
    fn init(seed: &[u32], n: usize) -> Raft {
        let mut cups: Vec<_> = (1..=(n+1) as u32).into_iter().collect();
        for (&c0, &c1) in seed.into_iter().zip(&seed[1..]) {
            cups[c0 as usize] = c1;
        }

        let m = seed.len();
        if n > m {
            cups[seed[m-1] as usize] = (m + 1) as u32;
            cups[n] = seed[0];
        } else {
            cups[seed[m-1] as usize] = seed[0];
        }

        cups[0] = seed[0];

        Raft { cups }
    }

    fn run(&mut self, n: usize) -> &Raft {
        let mut c = self.cups[0];
        for _ in 0..n {
            c = self.step(c);
        }
        self.cups[0] = c;
        self
    }

    fn step(&mut self, c: u32) -> u32 {
        let p0 = self.cups[c as usize];
        let p1 = self.cups[p0 as usize];
        let p2 = self.cups[p1 as usize];
        self.cups[c as usize] = self.cups[p2 as usize];
        let mut d = c - 1;
        while d <= 0 || d == p0 || d == p1 || d == p2 {
            if d <= 0 {
                d += self.cups.len() as u32 - 1;
            } else {
                d -= 1;
            }
        }
        self.cups[p2 as usize] = self.cups[d as usize];
        self.cups[d as usize] = p0;
        self.cups[c as usize]
    }

    fn walk<'a>(&'a self, mut i: u32, mut n: u32)
        -> impl Iterator<Item=u32> + 'a
    {
        std::iter::from_fn(move || {
            if n == 0 {
                None
            } else {
                n -= 1;
                let c = i;
                i = self.cups[i as usize];
                Some(c)
            }
        })
    }

    fn dump8(&self) -> String {
        self.walk(self.cups[1], 8)
            .map(|c| std::char::from_digit(c, 10).unwrap())
            .collect()
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ex0_100() {
        let mut raft = Raft::init(&parse(EX0), 9);
        assert_eq!(raft.cups, vec![3, 2, 5, 8, 6, 4, 7, 3, 9, 1]);

        raft.run(1);
        assert_eq!(
            raft.walk(raft.cups[0], 9).collect::<Vec<_>>(),
            vec![2, 8, 9, 1, 5, 4, 6, 7, 3],
        );

        raft.run(9);
        assert_eq!(
            raft.walk(raft.cups[0], 9).collect::<Vec<_>>(),
            vec![8, 3, 7, 4, 1, 9, 2, 6, 5],
        );

        raft.run(90);
        assert_eq!(raft.dump8(), "67384529");
    }

    #[test]
    fn answer1() {
        assert_eq!(Raft::init(&parse(INPUT), 9).run(100).dump8(), "98645732");
    }

    #[test]
    #[allow(non_snake_case)]
    fn ex0_10M() {
        let mut raft = Raft::init(&parse(EX0), 1_000_000);
        raft.run(10_000_000);
        assert_eq!(raft.cups[1], 934001);
        assert_eq!(raft.cups[934001], 159792);
    }

    #[test]
    fn answer2() {
        let mut raft = Raft::init(&parse(INPUT), 1_000_000);
        raft.run(10_000_000);
        assert_eq!(raft.cups[1], 929588);
        assert_eq!(raft.cups[929588], 741727);
    }

    pub const EX0: &str = "389125467";
    pub const INPUT: &str = "364289715";
}
