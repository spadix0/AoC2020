use criterion::{criterion_group, criterion_main, Criterion};

use bitmask::{*, tests::*};

fn run_bench(c: &mut Criterion, name: &str, input: &str, exp: u64) {
    let mut g = c.benchmark_group(name);
    let prog = Program::read(&mut input.as_bytes());

    g.bench_function("bdd", |b| b.iter(|| {
        let mem = bdd::exec_addrmask(&prog);
        assert_eq!(mem.sum(), exp);
    }));
    g.bench_function("split", |b| b.iter(|| {
        let mem = split::exec_addrmask(&prog);
        assert_eq!(mem.sum(), exp);
    }));
    g.bench_function("splat", |b| b.iter(|| {
        let mem = splat::exec_addrmask(&prog);
        assert_eq!(mem.sum(), exp);
    }));
    g.finish();
}


fn bench_small(c: &mut Criterion) {
    run_bench(c, "small", EX1, 208);
}

fn bench_nominal(c: &mut Criterion) {
    run_bench(c, "nominal", INPUT, 2741969047858);
}

criterion_group!(benches, bench_small, bench_nominal);
criterion_main!(benches);
