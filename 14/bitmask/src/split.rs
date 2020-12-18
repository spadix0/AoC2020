use std::mem::swap;
use super::{Memory, Program, mask_combos};

// split – hypercube intersection
//
// tracks masked addresses and only expands overwritten regions.  minimal
// memory usage even w/many Xs but can still explode for other cases.
// brute force intersection checks are too slow to be useful, even with
// nominal input.

pub fn exec_addrmask(prog: &Program) -> impl Memory {
    let mut prev: Vec<Entry> = Vec::new();
    let mut next: Vec<Entry> = Vec::new();

    for op in &prog.ops {
        for wr in &op.writes {
            let addr = wr.addr & !op.maskx | op.mask1;
            next.clear();

            for e in &prev {
                let fixed = !(op.maskx | e.mask);
                if e.addr & fixed != addr & fixed {
                    next.push(*e)
                } else {
                    let keep = e.mask & !op.maskx;
                    mask_combos(0, 0, keep, &mut |off| {
                        let a = e.addr & !keep | off;
                        if a & !op.maskx != addr {
                            next.push(Entry {
                                mask: op.maskx & e.mask,
                                addr: a,
                                data: e.data,
                            })
                        }
                    });
                }
            }

            next.push(Entry {
                mask: op.maskx,
                addr,
                data: wr.data,
            });
            swap(&mut prev, &mut next);
        }
    }

    prev
}


#[derive(Copy, Clone, PartialEq, Debug)]
struct Entry {
    mask: u64,
    addr: u64,
    data: u64,
}


impl Memory for Vec<Entry> {
    fn read(&self, addr: u64) -> u64 {
        self.iter()
            .find_map(|e| {
                if addr & !e.mask == e.addr {
                    Some(e.data)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn sum(&self) -> u64 {
        self.iter()
            .map(|e| e.data << e.mask.count_ones())
            .sum()
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{*, tests::*};

    addrmask_tests!(ex0, ex1, input);
}
