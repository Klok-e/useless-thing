#![feature(iter_intersperse)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .map(|s: &str| s.chars().rev())
        .intersperse( " ".chars().rev())
        .flatten()
        .collect::<String>()
}

fn criterion_benchmark(c: &mut Criterion) {
    let text = std::fs::read_to_string("lorem.txt").unwrap();
    c.bench_with_input(BenchmarkId::new("reverse", ""), &text, |b, i| {
        b.iter_batched(
            || i.clone(),
            |i| reverse_words(i),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
