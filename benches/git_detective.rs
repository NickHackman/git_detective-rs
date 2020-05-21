use std::fs::remove_dir_all;

use criterion::{criterion_group, criterion_main, Criterion};

use git_detective::GitDetective;

fn clap_benchmark(c: &mut Criterion) {
    let mut gd = GitDetective::clone("https://github.com/serde-rs/serde", "serde", true).unwrap();
    c.bench_function("Final Contributions - Serde Benchmark", |b| {
        b.iter(|| gd.final_contributions())
    });
    remove_dir_all("serde").unwrap();
}

criterion_group!(name = final_contributions_clap;
                 config = Criterion::default().sample_size(10);
                 targets = clap_benchmark);

criterion_main!(final_contributions_clap);
