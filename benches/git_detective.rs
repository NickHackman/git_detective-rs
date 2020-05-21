use criterion::{criterion_group, criterion_main, Criterion};

use git_detective::GitDetective;

fn gd_benchmark(c: &mut Criterion) {
    let gd = GitDetective::open(".").unwrap();
    c.bench_function("Final Contributions - GitDetective Benchmark", |b| {
        b.iter(|| gd.final_contributions())
    });
}

fn gd_lib_benchmark(c: &mut Criterion) {
    let gd = GitDetective::open(".").unwrap();
    c.bench_function("Final Contributions Lib.rs - GitDetective Benchmark", |b| {
        b.iter(|| gd.final_contributions_file("src/lib.rs"))
    });
}

criterion_group!(final_contributions_file, gd_lib_benchmark);
criterion_group!(final_contributions_gd, gd_benchmark);

criterion_main!(final_contributions_file, final_contributions_gd);
