use std::collections::HashMap;
use super::{Memory, Program, Write};

// BDD - modified binary decision diagram
//
// with multiple leaf/value nodes for write data.  questionable compromise
// between speed and memory with support for arbitrary masks.

pub fn exec_addrmask(prog: &Program) -> impl Memory {
    let mut mem = BDD::new();

    for op in &prog.ops {
        for &Write { addr, data } in &op.writes {
            mem.write(op.maskx, addr & !op.maskx | op.mask1, data);
        }
    }

    mem
}


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Node {
    Var { bit: u32, i0: u32, i1: u32 },
    Const { val: u64 },
}
use Node::*;


struct BDD {
    nodes: Vec<Node>,
    nodemap: HashMap<Node, u32>,
    root: u32,
}


impl BDD {
    fn new() -> BDD {
        let mut bdd = BDD {
            nodes: Vec::new(),
            nodemap: HashMap::new(),
            root: 0,
        };

        bdd.root = bdd.const_(0);
        bdd
    }

    fn const_(&mut self, val: u64) -> u32 {
        self.node(Const{val})
    }

    fn var_(&mut self, bit: u32, i0: u32, i1: u32) -> u32 {
        if i0 == i1 {
            i0
        } else {
            self.node(Var{bit, i0, i1})
        }
    }

    fn node(&mut self, n: Node) -> u32 {
        let nodes = &mut self.nodes;
        *self.nodemap
            .entry(n)
            .or_insert_with(|| {
                let i = nodes.len();
                nodes.push(n);
                i as u32
            })
    }

    fn write(&mut self, mask: u64, addr: u64, data: u64) {
        struct Env<'a> {
            bdd: &'a mut BDD,
            memo: HashMap<(u32, u32), u32>,
            mask: u64,
            addr: u64,
            idata: u32,
        }

        fn wr_r(env: &mut Env, b: u32, i: u32) -> u32 {
            if let Some(&j) = env.memo.get(&(b, i)) {
                return j;
            }
            if b > 35 {
                return env.idata;
            }

            let n = env.bdd.nodes[i as usize];
            let bit = n.bit();
            assert!(bit >= b);
            let j = if bit > b {
                if env.mask>>b & 1 != 0 {
                    wr_r(env, b+1, i)
                } else if env.addr>>b & 1 != 0 {
                    let j1 = wr_r(env, b+1, i);
                    env.bdd.var_(b, i, j1)
                } else {
                    let j0 = wr_r(env, b+1, i);
                    env.bdd.var_(b, j0, i)
                }
            } else if let Var { bit: _, i0, i1 } = n {
                if env.mask>>b & 1 != 0 {
                    let j0 = wr_r(env, b+1, i0);
                    let j1 = wr_r(env, b+1, i1);
                    env.bdd.var_(b, j0, j1)
                } else if env.addr>>b & 1 != 0 {
                    let j1 = wr_r(env, b+1, i1);
                    env.bdd.var_(b, i0, j1)
                } else {
                    let j0 = wr_r(env, b+1, i0);
                    env.bdd.var_(b, j0, i1)
                }
            } else {
                panic!();
            };

            env.memo.insert((b, i), j);
            j
        }

        let idata = self.const_(data);
        let root = self.root;

        self.root = wr_r(&mut Env {
            bdd: self,
            memo: HashMap::new(),
            mask, addr, idata
        }, 0, root);
    }
}


impl Memory for BDD {
    fn read(&self, addr: u64) -> u64 {
        let mut i = self.root;
        loop {
            match self.nodes[i as usize] {
                Const { val } => return val,
                Var { bit, i0, i1 } => {
                    i = if addr>>bit & 1 == 0 { i0 } else { i1 };
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn sum(&self) -> u64 {
        let mut memo = HashMap::new();

        fn sum_r(nodes: &[Node], memo: &mut HashMap<u32, u64>, i: u32, b: u32)
            -> u64
        {
            let n = nodes[i as usize];
            if let Some(&v) = memo.get(&i) {
                return v << (n.bit() - b);
            }

            match n {
                Const { val } => val << (36 - b) as u64,
                Var { bit, i0, i1 } => {
                    let val = sum_r(nodes, memo, i0, bit+1)
                        + sum_r(nodes, memo, i1, bit+1);
                    memo.insert(i, val);
                    val << (bit - b) as u64
                }
            }
        }
        sum_r(&self.nodes, &mut memo, self.root, 0)
    }
}


impl Node {
    fn bit(&self) -> u32 {
        match self {
            Const { val: _ } => 36,
            Var { bit, i0: _, i1: _ } => *bit,
        }
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{*, tests::*};

    addrmask_tests!(ex0, ex1, input);
}
