use luggage::{*, basic::Rules};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let rules = Rules::from_reader(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", rules);

    let contains = rules.contains(MY_BAG);
    //println!("{:?}", contains);

    println!("part[1]: {}", contains.len());
    println!("part[2]: {}", rules.count_contents(MY_BAG));
}
