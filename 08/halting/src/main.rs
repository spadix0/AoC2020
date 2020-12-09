use halting::{read, CPU, exit_search1};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let code = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", code);

    println!("part[1]: {}", CPU::from_executing(&code).acc);
    println!("part[2]: {}", exit_search1(&code).acc);
}
