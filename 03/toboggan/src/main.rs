use std::io;

const ALL_SLOPES: &[(usize, usize)] = &[
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let forest = Forest::read(&mut std::fs::File::open(path).unwrap());
    //println!("{}", forest.dumped());

    println!("part[1]: {}", forest.count_trees(3, 1));

    let paths = all_paths(&forest);
    println!("part[2]: {:?} {}", paths, paths.iter().product::<u64>());
}


fn all_paths(forest: &Forest) -> Vec<u64> {
    ALL_SLOPES.iter()
        .map(|&(dx, dy)| forest.count_trees(dx, dy))
        .collect()
}


//----------------------------------------------------------------------------
#[derive(Clone, Default, PartialEq, Debug)]
struct Forest {
    grid: Vec<Vec<bool>>,
}

impl Forest {
    fn read(stm: &mut impl io::Read) -> Forest {
        use io::BufRead;
        Forest {
            grid: io::BufReader::new(stm)
                .lines()
                .map(|line| parse_row(&line.unwrap()))
                .collect(),
        }
    }

    #[allow(dead_code)]
    fn dumped(&self) -> String {
        self.grid.iter()
            .map(|row| {
                row.iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn count_trees(&self, dx: usize, dy: usize) -> u64 {
        self.grid.iter()
            .step_by(dy)
            .enumerate()
            .map(|(y_dy, row)| row[dx * y_dy % row.len()] as u64)
            .sum()
    }
}

fn parse_row(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| c == '#')
        .collect()
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dump() {
        assert_eq!(read_ex0().dumped(), EX0.trim());
    }

    #[test]
    fn ex0_3_1() {
        assert_eq!(read_ex0().count_trees(3, 1), 7);
    }

    #[test]
    fn ex0_all() {
        assert_eq!(all_paths(&read_ex0()), [2, 7, 3, 4, 2]);
    }

    fn read_ex0() -> Forest {
        return Forest::read(&mut EX0.as_bytes())
    }

    const EX0: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
}
