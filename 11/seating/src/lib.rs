use std::{
    io,
    iter::repeat,
};

pub mod dod;
pub mod basic;


pub const DIRS: &[(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1), (-1, 0),
    (1, 0), (-1, 1), (0, 1), (1, 1)
];


#[derive(Debug)]
pub struct Seats {
    grid: Vec<bool>,
    width: usize,
}


impl Seats {
    pub fn read(stm: &mut impl io::Read) -> Seats {
        use io::BufRead;
        let mut width = 0;
        let mut grid = Vec::new();

        for line in io::BufReader::new(stm).lines() {
            let line = line.unwrap();
            if width == 0 {
                width = line.len() + 2;
                grid.extend(repeat(false).take(width));
            } else {
                assert_eq!(width, line.len() + 2);
            }
            grid.push(false);
            grid.extend(line.chars().map(|c| c == 'L'));
            grid.push(false);
        }

        // pad w/floor on all sides to simplify (some) boundary conditions
        grid.extend(repeat(false).take(width));

        Seats { grid, width }
    }
}


pub fn count_occupied(seats: &Vec<u8>) -> u32 {
    seats.iter()
        .cloned()
        .map(|x| x as u32)
        .sum()
}


#[cfg(any(test, feature="bench"))]
pub mod tests {
    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
