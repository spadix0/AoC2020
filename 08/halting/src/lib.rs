use std::{
    io, fmt,
    collections::{HashMap, HashSet},
    str::FromStr,
    error::Error,
};


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Op { NOP, JMP, ACC }
use Op::*;

pub struct Instr {
    pub op: Op,
    pub arg: i32,
}

#[derive(Debug)]
pub struct CPU<'a> {
    pub pc: i32,
    pub acc: i32,
    pub vis: HashSet<i32>,
    pub patch: HashSet<i32>,
    pub code: &'a [Instr],
}


impl CPU<'_> {
    pub fn with_code(code: &[Instr]) -> CPU {
        CPU {
            pc: 0,
            acc: 0,
            vis: HashSet::with_capacity(code.len()),
            patch: HashSet::with_capacity(1),
            code
        }
    }

    pub fn from_executing(code: &[Instr]) -> CPU {
        let mut cpu = CPU::with_code(&code);
        cpu.exec();
        cpu
    }

    pub fn exec(&mut self) {
        loop {
            let pc = self.pc;
            if self.vis.contains(&pc) || !self.step() {
                break;
            }
            self.vis.insert(pc);
        }
    }

    fn step(&mut self) -> bool {
        0 <= self.pc
            && (self.pc as usize) < self.code.len()
            && {
                let ir = &self.code[self.pc as usize];
                self.pc +=
                    match ir.op.patch(self.patch.contains(&self.pc)) {
                        NOP => 1,
                        JMP => ir.arg,
                        ACC => { self.acc += ir.arg; 1 },
                    };
                true
        }
    }
}


impl Op {
    fn patch(&self, patch: bool) -> Op {
        match self {
            NOP if patch => JMP,
            JMP if patch => NOP,
            _ => *self,
        }
    }
}


// brute force search by patching each candidate instruction and executing.
// quadratic in length of program (but, even input.txt is only hundreds
// and this only tries necessary cases)

pub fn exit_search2(code: &[Instr]) -> CPU {
    let init = CPU::from_executing(code);

    for fixpc in init.vis {
        if code[fixpc as usize].op != ACC {
            let mut probe = CPU::with_code(code);
            probe.patch.insert(fixpc);
            probe.exec();
            if (probe.pc as usize) == code.len() {
                return probe;
            }
        }
    }

    panic!();
}


// search by "executing" in reverse from target and testing each instruction.
// linear in length of program (but more memory (still linear))

pub fn exit_search1(code: &[Instr]) -> CPU {
    let mut cpu = CPU::with_code(code);
    cpu.patch.insert(find_patch(code));
    cpu.exec();
    cpu
}


fn find_patch(code: &[Instr]) -> i32 {
    let trace = CPU::from_executing(code).vis;
    let dsts = collect_patch_dsts(code, &trace);
    let srcs = collect_jmp_tgts(code, &trace);

    let mut vis = HashSet::new();
    let mut front = Vec::new();
    front.push(code.len() as i32);

    loop {
        let pc = front.pop().unwrap();
        if let Some(&ppc) = dsts.get(&pc) {
            return ppc;
        }

        let _new = vis.insert(pc);
        assert!(_new);

        if let Some(prev) = srcs.get(&pc) {
            front.extend(
                prev.iter()
                    .filter(|ppc| !vis.contains(ppc))
            );
        }

        let ppc = pc - 1;
        if code[ppc as usize].op != JMP {
            assert!(!vis.contains(&ppc));
            front.push(ppc);
        }
    }
}


// generate map from patched instruction destination to original trace pc
// (reverse edge candidates that could extend initially reachable set)

fn collect_patch_dsts(code: &[Instr], trace: &HashSet<i32>)
    -> HashMap<i32, i32>
{
    trace.iter()
        .filter_map(|&pc| {
            let ir = &code[pc as usize];
            match ir.op.patch(true) {
                NOP => Some((pc + 1, pc)),
                JMP => Some((pc + ir.arg, pc)),
                ACC => None,
            }
        })
        .collect()
}


// generate lookup of jmp targets for reverse execution

fn collect_jmp_tgts(code: &[Instr], trace: &HashSet<i32>)
    -> HashMap<i32, HashSet<i32>>
{
    let mut srcs = HashMap::with_capacity(trace.len());

    for (pc, instr) in code.iter().enumerate() {
        let pc = pc as i32;
        if instr.op == JMP {
            let npc = pc + instr.arg;
            if 0 <= npc && (npc as usize) <= code.len() {
                let ent = srcs.entry(npc)
                    .or_insert_with(|| HashSet::new());
                ent.insert(pc);
            }
        }
    }

    srcs
}


//----------------------------------------------------------------------------
pub fn read(stm: &mut impl io::Read) -> Vec<Instr> {
    use io::BufRead;
    io::BufReader::new(stm)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

impl FromStr for Instr {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s.split_whitespace();
        Ok(Instr {
            op: t.next().unwrap().parse()?,
            arg: t.next().unwrap().parse()?,
        })
    }
}


impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "nop" => NOP,
                "jmp" => JMP,
                "acc" => ACC,
                _ => panic!()
            }
        )
    }
}


impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:?} {}", self.op, self.arg))
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_exec() {
        let code = read(&mut EX0.as_bytes());
        assert_eq!(code.len(), 9);

        let cpu = CPU::from_executing(&code);
        assert_eq!(cpu.acc, 5);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.vis, [
            0, 1, 2, 3, 4, 6, 7
        ].iter().cloned().collect());
    }

    #[test]
    fn ex0_search1() {
        let code = read(&mut EX0.as_bytes());
        let cpu = exit_search1(&code);
        assert_eq!(cpu.acc, 8);
        assert_eq!(cpu.patch, [7].iter().cloned().collect());
    }

    #[test]
    fn ex0_search2() {
        let code = read(&mut EX0.as_bytes());
        let cpu = exit_search2(&code);
        assert_eq!(cpu.acc, 8);
        assert_eq!(cpu.patch, [7].iter().cloned().collect());
    }

    #[test]
    fn answer1() {
        let code = read(&mut INPUT.as_bytes());
        let cpu = CPU::from_executing(&code);
        assert_eq!(cpu.acc, 1930);
        assert_eq!(cpu.pc, 310);
        assert_eq!(cpu.vis.len(), 205);
    }

    #[test]
    fn answer2() {
        let code = read(&mut INPUT.as_bytes());

        let cpu = exit_search1(&code);
        assert_eq!(cpu.acc, 1688);
        assert_eq!(cpu.patch, [217].iter().cloned().collect());

        let cpu = exit_search2(&code);
        assert_eq!(cpu.acc, 1688);
        assert_eq!(cpu.patch, [217].iter().cloned().collect());
    }

    const EX0: &str = include_str!("../../ex0.txt");
    const INPUT: &str = include_str!("../../input.txt");
}
