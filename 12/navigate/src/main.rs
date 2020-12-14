use std::io;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let navs = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", navs);

    println!("part[1]: {}", traverse(&navs).l1_norm());
    println!("part[2]: {}", waypoint(&navs).l1_norm());
}


#[derive(Copy, Clone, PartialEq, Debug)]
struct Instr {
    act: char,
    arg: i32,
}

fn read(stm: &mut impl io::Read) -> Vec<Instr> {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            Instr {
                act: line.chars().next().unwrap(),
                arg: line[1..].parse().unwrap(),
            }
        })
        .collect()
}


fn traverse(navs: &[Instr]) -> Point {
    let mut p = Point(0, 0);
    let mut d = Point(1, 0);

    for &Instr{act, arg} in navs {
        match act {
            'N' => p.translate(0, arg),
            'S' => p.translate(0, -arg),
            'E' => p.translate(arg, 0),
            'W' => p.translate(-arg, 0),
            'L' => d.rotate(-arg),
            'R' => d.rotate(arg),
            'F' => p.translate(arg*d.0, arg*d.1),
            _ => panic!(),
        }
    }

    p
}


fn waypoint(navs: &[Instr]) -> Point {
    let mut p = Point(0, 0);
    let mut d = Point(10, 1);

    for &Instr{act, arg} in navs {
        match act {
            'N' => d.translate(0, arg),
            'S' => d.translate(0, -arg),
            'E' => d.translate(arg, 0),
            'W' => d.translate(-arg, 0),
            'L' => d.rotate(-arg),
            'R' => d.rotate(arg),
            'F' => p.translate(arg*d.0, arg*d.1),
            _ => panic!(),
        }
    }

    p
}


#[derive(Copy, Clone, PartialEq, Debug)]
struct Point(i32, i32);

impl Point {
    fn l1_norm(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.0 += dx;
        self.1 += dy;
    }

    fn rotate(&mut self, deg: i32) {
        let a = (deg/90 + 4) % 4;
        let s = 1 - (1 - a).abs();
        let c = (2 - a).abs() - 1;
        assert!(-1 <= s && s <= 1);
        assert!(-1 <= c && c <= 1);
        *self = Point(c*self.0 + s*self.1, c*self.1 - s*self.0);
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate() {
        use std::f32::consts::PI;
        for i in -3..=3 {
            let deg = 90*i;
            let (s, c) = (PI/180. * deg as f32).sin_cos();
            let mut p = Point(0, 1);
            p.rotate(deg);
            assert_eq!(p, Point(s.round() as i32, c.round() as i32));
        }
    }

    #[test]
    fn ex0_traverse() {
        assert_eq!(traverse(&read(&mut EX0.as_bytes())), Point(17, -8));
    }

    #[test]
    fn answer1() {
        assert_eq!(traverse(&read(&mut INPUT.as_bytes())), Point(-163, -218));
    }

    #[test]
    fn ex0_waypoint() {
        assert_eq!(waypoint(&read(&mut EX0.as_bytes())), Point(214, -72));
    }

    #[test]
    fn answer2() {
        assert_eq!(waypoint(&read(&mut INPUT.as_bytes())), Point(-11946, -16645));
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
