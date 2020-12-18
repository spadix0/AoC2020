use std::collections::HashMap;
use super::{Memory, Program, mask_combos};

// splat – combinatorial expansion
//
// initial, obvious approach that expands and writes all possible combinations
// of masked address bits to sparse memory map.  simple and effective for
// provided problem input.  explodes and runs out of memory if any mask has
// too many Xs, such as example from part 1 (ex0)

pub fn exec_addrmask(prog: &Program) -> impl Memory {
    let mut mem = HashMap::new();

    for op in &prog.ops {
        mask_combos(0, 0, op.maskx, &mut |a| {
            for wr in op.writes.iter() {
                mem.insert(wr.addr & !op.maskx | op.mask1 | a, wr.data);
            }
        })
    }

    mem
}


//----------------------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{*, tests::*};

    addrmask_tests!(ex1, input);
}
