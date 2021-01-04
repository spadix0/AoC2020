use std::io;
use fxhash::{
    FxHashMap as HashMap,
    FxHashSet as HashSet,
};

type Coord = u64;			// 2x bit packed components
const B: Coord = 32;			// bits / component -1 for separation
const M: Coord = (1 << B-1) - 1;	// component mask
const MASK: Coord = M | M<<B;		// Coord 2-tuple mask

const DIRS: &[Coord] = &[
    1 | M<<B,	// e
    M | 1<<B,	// w

    1,		// ne
    1<<B,	// nw

    M<<B,	// se
    M,		// sw
];


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let paths = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", paths);

    let floor = toggle(&paths);
    println!("part[1]: {}", floor.len());
    println!("part[2]: {}", run(floor, 100).len());
}


fn toggle(paths: &[Vec<Coord>]) -> HashSet<Coord> {
    let mut dsts = HashMap::default();
    for seq in paths {
        let p = seq.iter().fold(0, |p, dp| p+dp & MASK);
        *dsts.entry(p).or_insert(false) ^= true;
    }

    dsts.into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect()
}


fn run(floor: HashSet<Coord>, n: usize) -> HashSet<Coord> {
    (0..n).into_iter()
        .fold(floor, |f, _| step(&f))
}


fn step(prev: &HashSet<Coord>) -> HashSet<Coord> {
    let bound = prev.len() * DIRS.len();
    let mut next = HashSet::default();
    let mut hood: HashMap<_, u8> = HashMap::default();

    next.reserve(bound / 12);
    hood.reserve(bound / 2);

    for &p in prev {
        let mut n = 0;
        for dp in DIRS {
            let q = p+dp & MASK;
            if prev.contains(&q) {
                n += 1;
            } else {
                *hood.entry(q).or_insert(0) += 1;
            }
        }

        if 1 <= n && n <= 2 {
            next.insert(p);
        }
    }

    next.extend(
        hood.iter()
            .filter_map(|(p, &n)| if n == 2 { Some(p) } else { None }));
    next
}


fn read(stm: &mut impl io::Read) -> Vec<Vec<Coord>> {
    use io::BufRead;
    io::BufReader::new(stm).lines()
        .map(|line| parse_path(&line.unwrap()))
        .collect()
}


fn parse_path(s: &str) -> Vec<Coord> {
    let mut path = Vec::new();
    let mut d0 = 0;
    for c in s.chars() {
        d0 = match c {
            'e' => { path.push(DIRS[d0]); 0 },
            'w' => { path.push(DIRS[d0+1]); 0 },
            'n' => 2,
            's' => 4,
            _ => panic!(),
        }
    }

    path
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_toggle() {
        let paths = read(&mut EX0.as_bytes());
        assert_eq!(paths.len(), 20);
        assert_eq!(toggle(&paths).len(), 10);
    }

    #[test]
    fn answer1() {
        assert_eq!(toggle(&read(&mut INPUT.as_bytes())).len(), 263);
    }

    #[test]
    fn ex0_100() {
        assert_eq!(run(toggle(&read(&mut EX0.as_bytes())), 100).len(), 2208);
    }

    #[test]
    fn answer2() {
        assert_eq!(run(toggle(&read(&mut INPUT.as_bytes())), 100).len(), 3649);
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
