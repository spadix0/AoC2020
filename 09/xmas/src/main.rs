use std::{io, collections::HashMap};

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();

    // override default preamble/window size for smaller example
    // NB use 5 for ex0
    let n = args.next()
        .and_then(|arg| Some(arg.parse().unwrap()))
        .unwrap_or(25);

    let data = read(&mut std::fs::File::open(path).unwrap());

    let x = find_first_invalid(&data, n).unwrap();
    println!("part[1]: {}", x);

    let (i, j) = find_range_totaling(&data, x);
    //println!("{}..{}", i, j);
    println!("part[2]: {}", calc_weakness(&data[i..j]));
}


fn read(stm: &mut impl io::Read) -> Vec<i64> {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}


fn find_first_invalid(data: &[i64], n: usize) -> Option<i64> {
    // sliding window lookup for faster sum checks
    let mut win: HashMap<i64, usize> = data[..n].iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect();

    for (i, &x) in data[n..].iter().enumerate() {
        if data[i..i+n].iter()
            .enumerate()
            .all(|(j, y)| win.get(&(x-y)).cloned().unwrap_or(0) <= i+j)
        {
            return Some(x);
        }

        // maintain sliding window
        let x0 = data[i];
        if win[&x0] == i {
            win.remove(&x0);
        }
        win.insert(x, i + n);
    }
    None
}


fn find_range_totaling(data: &[i64], tgt: i64) -> (usize, usize) {
    // subtract iterator => start of integration window
    let mut subit = data.iter().enumerate();
    let mut i = 0;
    let mut acc = 0;

    // add iteration => end of integration window
    for (j, x) in data.iter().enumerate() {
        assert!(acc < tgt);
        acc += x;			// open window

        while acc > tgt {
            let (k, y) = subit.next().unwrap();
            acc -= y;			// close window
            i = k + 1;
        }

        if acc == tgt && j > i+2 {
            return (i, j+1);
        }
    }

    panic!();
}


fn calc_weakness(data: &[i64]) -> i64 {
    data.iter().min().unwrap()
        + data.iter().max().unwrap()
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_cases0() {
        let mut data: Vec<_> = (1..=25).collect();
        let cases = [
            (26, None),
            (49, None),
            (100, Some(100)),
            (50, Some(50)),
        ];
        for &(next, exp) in &cases {
            data.push(next);
            println!("{:?}", data);
            assert_eq!(find_first_invalid(&data, 25), exp);
            data.pop();
        }
    }

    #[test]
    fn invalid_cases1() {
        let mut data = Vec::new();
        data.extend(1..=19);
        data.extend(21..=25);
        data.push(45);

        let cases = [
            (26, None),
            (65, Some(65)),
            (64, None),
            (66, None),
        ];
        for &(next, exp) in &cases {
            data.push(next);
            println!("{:?}", data);
            assert_eq!(find_first_invalid(&data, 25), exp);
            data.pop();
        }
    }

    #[test]
    fn ex0_invalid() {
        let data = read(&mut EX0.as_bytes());
        assert_eq!(find_first_invalid(&data, 5), Some(127));
    }

    #[test]
    fn ex0_weakness() {
        let data = read(&mut EX0.as_bytes());
        let (i, j) = find_range_totaling(&data, 127);
        assert_eq!(i, 2);
        assert_eq!(j, 6);
        assert_eq!(calc_weakness(&data[i..j]), 62);
    }

    #[test]
    fn answer1() {
        let data = read(&mut INPUT.as_bytes());
        assert_eq!(find_first_invalid(&data, 25), Some(144381670));
    }

    #[test]
    fn answer2() {
        let data = read(&mut INPUT.as_bytes());
        let (i, j) = find_range_totaling(&data, 144381670);
        assert_eq!(i, 451);
        assert_eq!(j, 468);
        assert_eq!(calc_weakness(&data[i..j]), 20532569);
    }

    const EX0: &str = include_str!("../../ex0.txt");
    const INPUT: &str = include_str!("../../input.txt");
}
