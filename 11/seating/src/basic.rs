use std::{
    mem::swap,
    iter::repeat,
};
use super::{DIRS, Seats};

// initial reference implementations w/simple for loops for comparison
// somewhat optimized for more useful results (those ABC tho...)

pub fn run_adjacent(seats: &Seats) -> Vec<u8> {
    let w = seats.width as isize;
    let dirs = [ -w-1, -w, -w+1, -1, 1, w-1, w, w+1 ];

    let n = seats.grid.len();
    let mut prev: Vec<_> = seats.grid.iter().map(|&s| s as u8).collect();
    let mut next: Vec<_> = repeat(0).take(n).collect();

    while next != prev {
        swap(&mut next, &mut prev);

        // make (most) bounds checks redundant (supposedly)
        let seats = &seats.grid[..n];
        let prev = &prev[..n];
        let next = &mut next[..n];

        for i in 0..n {
            if seats[i] {
                let mut acc = 0;
                for di in &dirs {
                    let j = (i as isize + di) as usize;
                    acc += unsafe { prev.get_unchecked(j) };
                }

                next[i] = if prev[i] != 0 {
                    acc < 4
                } else {
                    acc == 0
                } as u8;
            }
        }
    }

    next
}


pub fn run_visible(seats: &Seats) -> Vec<u8> {
    let w = seats.width as i32;
    let h = seats.grid.len() as i32 / w;

    let mut prev: Vec<_> = seats.grid.iter().map(|&s| s as u8).collect();
    let mut next: Vec<_> = repeat(0).take(seats.grid.len()).collect();

    while next != prev {
        swap(&mut next, &mut prev);

        for y0 in 0..h {
            for x0 in 0..w {
                let i0 = (w*y0 + x0) as usize;
                if unsafe { *seats.grid.get_unchecked(i0) } {
                    let mut acc = 0;
                    for (dx, dy) in DIRS {
                        let (mut x, mut y) = (x0+dx, y0+dy);
                        while 0 <= x && x < w && 0 <= y && y < h {
                            let i = (w*y + x) as usize;
                            if unsafe { *seats.grid.get_unchecked(i) } {
                                acc += unsafe { *prev.get_unchecked(i) };
                                break;
                            }
                            x += dx;
                            y += dy;
                        }
                    }

                    let sit = if unsafe { *prev.get_unchecked(i0) } != 0 {
                        acc < 5
                    } else {
                        acc == 0
                    } as u8;
                    unsafe { *next.get_unchecked_mut(i0) = sit };
                }
            }
        }
    }

    next
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{*, tests::*};

    #[test]
    fn ex0_adjacent() {
        let seats = Seats::read(&mut EX0.as_bytes());
        assert_eq!(count_occupied(&run_adjacent(&seats)), 37);
    }

    #[test]
    fn answer1() {
        let seats = Seats::read(&mut INPUT.as_bytes());
        assert_eq!(count_occupied(&run_adjacent(&seats)), 2368);
    }

    #[test]
    fn ex0_visible() {
        let seats = Seats::read(&mut EX0.as_bytes());
        assert_eq!(count_occupied(&run_visible(&seats)), 26);
    }

    #[test]
    fn answer2() {
        let seats = Seats::read(&mut INPUT.as_bytes());
        assert_eq!(count_occupied(&run_visible(&seats)), 2124);
    }
}
