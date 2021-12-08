use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day08::{first, second};

fn first_challenge(c: &mut Criterion) {
    let input = include_str!("../data/challenge.txt");

    c.bench_function("day 08 - challenge 1", |b| {
        b.iter(|| first::challenge(black_box(input)))
    });
}

fn second_challenge(c: &mut Criterion) {
    let input = include_str!("../data/challenge.txt");

    c.bench_function("day 08 - challenge 2", |b| {
        b.iter(|| second::challenge(black_box(input)))
    });
}

criterion_group!(benches, first_challenge, second_challenge);
criterion_main!(benches);
