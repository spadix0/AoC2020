use std::{
    io,
    iter::once,
    ops::BitOr,
};
use fxhash::{
    FxHashSet as HashSet,
    FxHashMap as HashMap,
};

type Point = u128;			// bit packed components

const B: Point = 32;			// bits / component -1 for separation
const M: Point = (1 << B-1) - 1;	// component mask


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let seed = read(&mut std::fs::File::open(path).unwrap());

    println!("part[1]: {}", sim(&seed, 3, 6).len());
    println!("part[2]: {}", sim(&seed, 4, 6).len());
}


fn sim(seed: &HashSet<Point>, dims: usize, steps: usize) -> HashSet<Point>
{
    let dirs = cache_dirs(dims);
    let mask = (0..dims).into_iter()
        .map(|i| M << B*(i as Point))
        .fold(0, Point::bitor);

    let mut state = step(seed, &dirs, mask);
    for _ in 1..steps {
        state = step(&state, &dirs, mask);
    }

    state
}


fn step(prev: &HashSet<Point>, dirs: &[Point], mask: Point) -> HashSet<Point>
{
    let bound = prev.len() * dirs.len();
    let mut next = HashSet::default();
    let mut hood: HashMap<_, u8> = HashMap::default();

    next.reserve(bound / 12);
    hood.reserve(bound / 2);

    for &p in prev {
        let mut n = 0;
        for dp in dirs {
            let q = p+dp & mask;
            if prev.contains(&q) {
                n += 1;
            } else {
                *hood.entry(q).or_insert(0) += 1;
            }
        }

        if 2 <= n && n <= 3 {
            next.insert(p);
        }
    }

    next.extend(
        hood.iter()
            .filter(|&(_, &n)| n == 3)
            .map(|(p, _)| p));

    next
}


fn read(stm: &mut impl io::Read) -> HashSet<Point> {
    use io::BufRead;
    let mut pts = HashSet::default();

    for (y, line) in io::BufReader::new(stm).lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                pts.insert((y<<B | x) as Point);
            }
        }
    }

    pts
}


fn cache_dirs(dims: usize) -> Vec<Point> {
    let mut dirs = Vec::new();

    for i in 0..dims {
        dirs = dirs.iter()
            .cloned()
            .chain(once(0))
            .flat_map(|p| {
                (-1 ..= 1).into_iter()
                    .map(move |d| p | ((d as Point) & M) << B*(i as Point))
                    .filter(|&n| n != 0)
            })
            .collect()
    }

    dirs
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirs() {
        assert_eq!(cache_dirs(1).len(), 2);
        assert_eq!(cache_dirs(2).len(), 8);
        assert_eq!(cache_dirs(3).len(), 26);
        assert_eq!(cache_dirs(4).len(), 80);
    }

    #[test]
    fn ex0_3d() {
        assert_eq!(sim(&read(&mut EX0.as_bytes()), 3, 6).len(), 112);
    }

    #[test]
    fn answer1() {
        assert_eq!(sim(&read(&mut INPUT.as_bytes()), 3, 6).len(), 202);
    }

    #[test]
    fn ex0_4d() {
        assert_eq!(sim(&read(&mut EX0.as_bytes()), 4, 6).len(), 848);
    }

    #[test]
    fn answer2() {
        assert_eq!(sim(&read(&mut INPUT.as_bytes()), 4, 6).len(), 2028);
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
