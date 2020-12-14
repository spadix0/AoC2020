use criterion::{criterion_group, criterion_main, Criterion};

use seating::{*, tests::*, dod::*};

fn run_bench(c: &mut Criterion, name: &str, input: &str, exp: u32) {
    let mut g = c.benchmark_group(name);
    let seats = Seats::read(&mut input.as_bytes());
    let graph = Graph::adjacent(&seats);

    g.bench_function("graph", |b| b.iter(|| {
        let occ = graph.run_until_stable(4);
        assert_eq!(count_occupied(&occ), exp);
    }));
    g.bench_function("grid", |b| b.iter(|| {
        let occ = basic::run_adjacent(&seats);
        assert_eq!(count_occupied(&occ), exp);
    }));
    g.finish();
}


fn bench_small(c: &mut Criterion) {
    run_bench(c, "adjacent-small", EX0, 37);
}

fn bench_nominal(c: &mut Criterion) {
    run_bench(c, "adjacent-nominal", INPUT, 2368);
}

criterion_group!(benches, bench_small, bench_nominal);
criterion_main!(benches);
