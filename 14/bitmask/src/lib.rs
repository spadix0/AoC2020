use std::{
    io, fmt,
    collections::HashMap,
    str::FromStr,
};

// 3 alternative implementations for part 2:
pub mod splat;
pub mod split;
pub mod bdd;


pub trait Memory {
    fn read(&self, addr: u64) -> u64;
    fn sum(&self) -> u64;
    fn size(&self) -> usize;
}


#[derive(Clone, Debug)]
pub struct Program {
    ops: Vec<MaskWrites>,
}

#[derive(Clone)]
struct MaskWrites {
    maskx: u64,
    mask1: u64,
    writes: Vec<Write>,
}

#[derive(Copy, Clone)]
struct Write {
    addr: u64,
    data: u64,
}

pub fn exec_datamask(prog: &Program) -> impl Memory {
    prog.ops.iter()
        .flat_map(|op| {
            op.writes.iter()
                .map(move |&wr| (wr.addr, wr.data & op.maskx | op.mask1))
        })
        .collect::<HashMap<_, _>>()
}


impl Memory for HashMap<u64, u64> {
    fn read(&self, addr: u64) -> u64 {
        self.get(&addr).copied().unwrap_or(0)
    }

    fn sum(&self) -> u64 {
        self.values().sum()
    }

    fn size(&self) -> usize {
        self.len()
    }
}


fn mask_combos<F>(mut bit: u64, off: u64, mask: u64, f: &mut F)
where
    F: FnMut(u64)
{
    if mask >> bit == 0 {
        f(off)
    } else {
        while mask>>bit & 1 == 0 {
            bit += 1;
        }
        mask_combos(bit+1, off, mask, f);
        mask_combos(bit+1, off | 1<<bit, mask, f)
    }
}



//----------------------------------------------------------------------------
impl Program {
    pub fn read(stm: &mut impl io::Read) -> Program {
        use io::BufRead;
        let mut ops: Vec<MaskWrites> = Vec::new();

        for line in io::BufReader::new(stm).lines() {
            let line = line.unwrap();
            if let Some(mask) = line.strip_prefix("mask = ") {
                ops.push(mask.parse().unwrap());
            } else {
                ops.last_mut().unwrap().writes.push(line.parse().unwrap())
            }
        }

        Program { ops }
    }
}

impl FromStr for MaskWrites {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 36);
        Ok(MaskWrites {
            maskx: s.chars()
                .enumerate()
                .map(|(i, c)| ((c == 'X') as u64) << 35-i)
                .sum(),
            mask1: s.chars()
                .enumerate()
                .map(|(i, c)| ((c == '1') as u64) << 35-i)
                .sum(),
            writes: Vec::new(),
        })
    }
}

impl FromStr for Write {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s.split(" = ");
        Ok(Write {
            addr: t.next().unwrap()
                .strip_prefix("mem[").unwrap()
                .strip_suffix("]").unwrap()
                .parse().unwrap(),
            data: t.next().unwrap()
                .parse().unwrap(),
        })
    }
}

impl fmt::Debug for MaskWrites {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "&{:036b} |{:036b} {:?}", self.maskx, self.mask1, self.writes))
    }
}

impl fmt::Debug for Write {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "[{}]={}", self.addr, self.data))
    }
}


//----------------------------------------------------------------------------
#[cfg(any(test, feature="bench"))]
pub mod tests {
    use super::*;

    #[test]
    fn ex0_datamask() {
        let prog = Program::read(&mut EX0.as_bytes());
        assert_eq!(prog.ops.len(), 1);
        let n: usize = prog.ops.iter().map(|op| op.writes.len()).sum();
        assert_eq!(n, 3);

        let mem = exec_datamask(&prog);
        assert_eq!(mem.read(7), 101);
        assert_eq!(mem.read(8), 64);
        assert_eq!(mem.sum(), 165);
    }

    #[test]
    fn answer1() {
        let prog = Program::read(&mut INPUT.as_bytes());
        assert_eq!(exec_datamask(&prog).sum(), 12135523360904);
    }

    #[macro_export]
    macro_rules! addrmask_tests {
        (ex0) => {
            #[test]
            pub fn ex0_addrmask() {
                let prog = Program::read(&mut EX0.as_bytes());
                let mem = exec_addrmask(&prog);
                assert_eq!(mem.sum(), 1735166787584);
            }
        };

        (ex1) => {
            #[test]
            pub fn ex1_addrmask() {
                let prog = Program::read(&mut EX1.as_bytes());
                let mem = exec_addrmask(&prog);
                assert_eq!(mem.sum(), 208);
                assert_eq!(mem.read(58), 100);
                assert_eq!(mem.read(17), 1);
                assert_eq!(mem.read(27), 1);
            }
        };

        (input) => {
            #[test]
            pub fn ex1_answer() {
                let prog = Program::read(&mut INPUT.as_bytes());
                let mem = exec_addrmask(&prog);
                assert_eq!(mem.sum(), 2741969047858);
            }
        };

        ($test:ident, $($seq:ident),+) => {
            addrmask_tests! { $test }
            addrmask_tests! { $($seq),+ }
        };
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const EX1: &str = include_str!("../../ex1.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
