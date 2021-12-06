use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day06::{first, second};

fn first_challenge(c: &mut Criterion) {
    let input = include_str!("../data/challenge.txt");
    let days = 256;

    c.bench_function("day 06 - challenge 1", |b| {
        b.iter(|| first::challenge(black_box(input), black_box(days)))
    });
}

fn second_challenge(c: &mut Criterion) {
    let input = include_str!("../data/challenge.txt");
    let days = 256;

    c.bench_function("day 06 - challenge 2", |b| {
        b.iter(|| second::challenge(black_box(input), black_box(days)))
    });
}

criterion_group!(benches, first_challenge, second_challenge);
criterion_main!(benches);
