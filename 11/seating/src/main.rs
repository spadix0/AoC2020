use seating::{*, dod::*};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let seats = Seats::read(&mut std::fs::File::open(path).unwrap());

    let graph = Graph::adjacent(&seats);
    println!("part[1]: {}", count_occupied(&graph.run_until_stable(4)));

    let graph = Graph::visible(&seats);
    println!("part[2]: {}", count_occupied(&graph.run_until_stable(5)));
}
