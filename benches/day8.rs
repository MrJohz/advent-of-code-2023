use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day8(c: &mut Criterion) {
    let text = aoc_2023::utils::load_real(8);
    c.bench_function("day 8 part 1", |b| {
        b.iter(|| aoc_2023::day8::day8_part1(black_box(&text)))
    });
    c.bench_function("day 8 part 2", |b| {
        b.iter(|| aoc_2023::day8::day8_part2(black_box(&text)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(500)
        .measurement_time(Duration::from_secs(30));
    targets = day8
}
criterion_main!(benches);
