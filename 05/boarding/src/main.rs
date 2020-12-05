use std::{io, collections::HashSet};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let seats = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", seats);

    println!("part[1]: {}", seats.iter().max().unwrap());
    println!("part[2]: {}", find_empty_interior_seat(&seats));
}


fn read(stm: &mut impl io::Read) -> HashSet<usize> {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| seat_id(parse_seat(&line.unwrap())))
        .collect()
}


fn parse_seat(bsp: &str) -> (usize, usize) {
    (
        bsp[..7].chars().enumerate()
            .map(|(i, c)| ((c == 'B') as usize) << 6-i)
            .sum(),
        bsp[7..].chars().enumerate()
            .map(|(i, c)| ((c == 'R') as usize) << 2-i)
            .sum(),
    )
}


fn seat_id(pos: (usize, usize)) -> usize {
    8*pos.0 + pos.1
}


fn find_empty_interior_seat(seats: &HashSet<usize>) -> usize {
    // interior range specified somewhat inconsistently as:
    // "Your seat wasn't at the very front or back, though;
    //  the seats with IDs +1 and -1 from yours will be in your list."
    // => checking both because it doesn't matter for this input
    let idmin = seats.iter().min().unwrap();
    let idmax = seats.iter().max().unwrap();

    // front most and back most row
    let rmin = idmin / 8;
    let rmax = idmax / 8;

    // interior range
    let inmin = (8*(rmin + 1)).max(idmin + 1);
    let inmax = (8*rmax - 1).min(idmax - 1);

    let mut empties = (inmin..=inmax).into_iter()
        .filter(|id| !seats.contains(id));
    let empty = empties.next().unwrap();
    assert_eq!(0, empties.count());

    empty
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_cases() {
        let cases = [
            ("FBFBBFFRLR", (44, 5), 357),
            ("BFFFBBFRRR", (70, 7), 567),
            ("FFFBBBFRRR", (14, 7), 119),
            ("BBFFBBFRLL", (102, 4), 820),
        ];
        for &(bsp, pos, id) in &cases {
            assert_eq!(pos, parse_seat(bsp));
            assert_eq!(id, seat_id(pos));
        }
    }

    #[test]
    fn not_front() {
        let seats: HashSet<usize> = [
                17,     19, 20, 21, 22, 23,	// *not* it
            24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35,     37, 38, 39,	// should be this one
            40, 41, 42, 43, 44
        ].iter().cloned().collect();
        assert_eq!(36, find_empty_interior_seat(&seats));
    }

    #[test]
    fn not_back() {
        let seats: HashSet<usize> = [
                17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,     28, 29, 30, 31,	// should be this one
            32, 33, 34, 35, 36, 37, 38, 39,
                41, 42, 43, 44			// *not* this one
        ].iter().cloned().collect();
        assert_eq!(27, find_empty_interior_seat(&seats));
    }

    #[test]
    fn front_edge() {
        let seats: HashSet<usize> = [
                                        23,
                25, 26, 27, 28, 29, 30, 31,
            32,
        ].iter().cloned().collect();
        assert_eq!(24, find_empty_interior_seat(&seats));
    }

    #[test]
    fn back_edge() {
        let seats: HashSet<usize> = [
                                        23,
            24, 25, 26, 27, 28, 29, 30,
            32,
        ].iter().cloned().collect();
        assert_eq!(31, find_empty_interior_seat(&seats));
    }

    #[test]
    fn answer1() {
        let seats = read(&mut INPUT.as_bytes());
        assert_eq!(Some(&848), seats.iter().max());
    }

    #[test]
    fn answer2() {
        let seats = read(&mut INPUT.as_bytes());
        assert_eq!(682, find_empty_interior_seat(&seats));
    }

    const INPUT: &str = include_str!("../../input.txt");
}
