use criterion::{criterion_group, criterion_main, Criterion};

use git_detective::GitDetective;

fn gd_benchmark(c: &mut Criterion) {
    let gd = GitDetective::open(".").unwrap();
    c.bench_function("Final Contributions - GitDetective Benchmark", |b| {
        b.iter(|| gd.final_contributions())
    });
}
criterion_group!(benches, gd_benchmark);
criterion_main!(benches);
