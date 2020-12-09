use criterion::{criterion_group, criterion_main, Criterion};

use halting::{read, exit_search1, exit_search2};

const EX0: &str = include_str!("../../ex0.txt");
const INPUT: &str = include_str!("../../input.txt");


fn bench_ex0(c: &mut Criterion) {
    let code = read(&mut EX0.as_bytes());
    let mut g = c.benchmark_group("tiny");
    g.bench_function("linear", |b| b.iter(|| {
        assert_eq!(exit_search1(&code).acc, 8);
    }));
    g.bench_function("quadratic", |b| b.iter(|| {
        assert_eq!(exit_search2(&code).acc, 8);
    }));
    g.finish();
}


fn bench_input(c: &mut Criterion) {
    let code = read(&mut INPUT.as_bytes());
    let mut g = c.benchmark_group("nominal");
    g.bench_function("linear", |b| b.iter(|| {
        assert_eq!(exit_search1(&code).acc, 1688);
    }));
    g.bench_function("quadratic", |b| b.iter(|| {
        assert_eq!(exit_search2(&code).acc, 1688);
    }));
    g.finish();
}


criterion_group!(benches, bench_ex0, bench_input);
criterion_main!(benches);
