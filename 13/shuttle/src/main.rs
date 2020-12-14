use std::io;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let (time, buses, offsets) =
        read(&mut std::fs::File::open(path).unwrap());
    //println!("{} {:?} {:?}", time, buses, offsets);

    let (wait, bus) = next_bus_after(time, &buses);
    println!("part[1]: bus {} after {} minutes => {}", wait, bus, wait*bus);

    println!("part[2]: {}", find_pattern(&buses, &offsets));
}


fn next_bus_after(time: i64, buses: &[i64]) -> (i64, i64) {
    buses.iter()
        .map(|&b| ((-time).rem_euclid(b), b))
        .min_by_key(|&(x, _)| x)
        .unwrap()
}


// advance each bus (b) to target schedule offset (-x) in turn.
//
// time is stepped by product of previously considered buses (m)
// to avoid changing their offsets (m % b_k == 0 for each b_k in m).
//
// each time step by m advances current bus by (m % b),
// so we want to find fewest steps j s.t. dt = j * (m%b) % b
// => j = dt / (m%b) % b (where '/' is modular multiplicative inverse)

fn find_pattern(buses: &[i64], offsets: &[i64]) -> i64 {
    let mut t: BigInt = 0.into();
    let mut m: BigInt = 1.into();

    for (&b, &x) in buses.iter().zip(offsets.iter()) {
        let b: BigInt = b.into();
        let m_b_1 = m.mod_floor(&b).modpow(&(&b-2), &b);
        let j = ((-&x - &t) * m_b_1).mod_floor(&b);
        t += j * &m;
        assert_eq!(t.mod_floor(&b), {
            let x: BigInt = x.into();
            (-x).mod_floor(&b)
        });
        m *= b;
    }

    t.to_i64().unwrap()
}


fn read(stm: &mut impl io::Read) -> (i64, Vec<i64>, Vec<i64>) {
    use io::BufRead;
    let mut lines = io::BufReader::new(stm).lines();

    let time = lines.next().unwrap().unwrap().parse().unwrap();
    let (offsets, buses) = parse_buses(&lines.next().unwrap().unwrap());

    (time, buses, offsets)
}


fn parse_buses(s: &str) -> (Vec<i64>, Vec<i64>) {
    s.split(',')
        .enumerate()
        .filter_map(|(i, b)| {
            b.parse()
                .ok()
                .and_then(|b: i64| Some((i as i64, b)))
        })
        .unzip()
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_next() {
        let (t, bs, _) = read(&mut EX0.as_bytes());
        assert_eq!(next_bus_after(t, &bs), (5, 59));
    }

    #[test]
    fn answer1() {
        let (t, bs, _) = read(&mut INPUT.as_bytes());
        assert_eq!(next_bus_after(t, &bs), (9, 17));
    }

    #[test]
    fn ex0_pattern() {
        let (_, bs, os) = read(&mut EX0.as_bytes());
        assert_eq!(find_pattern(&bs, &os), 1068781);
    }

    #[test]
    fn ex1_pattern() {
        let (os, bs) = parse_buses("17,x,13,19");
        assert_eq!(find_pattern(&bs, &os), 3417);
    }

    #[test]
    fn ex2_pattern() {
        let (os, bs) = parse_buses("67,7,59,61");
        assert_eq!(find_pattern(&bs, &os), 754018);
    }

    #[test]
    fn ex3_pattern() {
        let (os, bs) = parse_buses("67,x,7,59,61");
        assert_eq!(find_pattern(&bs, &os), 779210);
    }

    #[test]
    fn ex4_pattern() {
        let (os, bs) = parse_buses("67,7,x,59,61");
        assert_eq!(find_pattern(&bs, &os), 1261476);
    }

    #[test]
    fn ex5_pattern() {
        let (os, bs) = parse_buses("1789,37,47,1889");
        assert_eq!(find_pattern(&bs, &os), 1202161486);
    }

    #[test]
    fn answer2() {
        let (_, bs, os) = read(&mut INPUT.as_bytes());
        assert_eq!(find_pattern(&bs, &os), 471793476184394);
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
