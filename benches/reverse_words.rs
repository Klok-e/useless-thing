use std::str::from_utf8_unchecked;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};



pub fn reverse_words(mut s: String) -> String {
    // s.split_whitespace()
    //     .map(|s: &str| s.chars().rev().chain(std::iter::once(' ')))
    //     .flatten()
    //     .take(s.len())
    //     .collect::<String>()

    let mut res = String::with_capacity(s.len());
    let mut words = s.as_bytes()
        //  Split bytes
        .split(|&b| b == b' ')
        .map(|bytes| unsafe { 
            //  Convert the slice to a string wihtout checking
           from_utf8_unchecked(bytes)
    });
    //  This reverses each individual words
    if let Some(word) = words.next() { 
        res.extend(word.chars().rev())
    }
    //  Collect reversed words into a collection
    for word in words { 
        res.push(' ');
        res.extend(word.chars().rev())
    }
    res
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
