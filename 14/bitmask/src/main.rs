use bitmask::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let prog = Program::read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", prog);

    println!("part[1]: {}", exec_datamask(&prog).sum());

    let mem = bdd::exec_addrmask(&prog);
    let sum = mem.sum();
    println!("part[2]: {}", mem.sum());
    println!("  bdd: {} nodes", mem.size());

    let mem = split::exec_addrmask(&prog);
    assert_eq!(sum, mem.sum());
    println!("  split: {} entries", mem.size());

    let mem = splat::exec_addrmask(&prog);
    assert_eq!(sum, mem.sum());
    println!("  splat: {} entries", mem.size());
}
