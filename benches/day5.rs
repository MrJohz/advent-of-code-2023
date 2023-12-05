use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day5(c: &mut Criterion) {
    let text = aoc_2023::utils::load_real(5);
    c.bench_function("day 5 part 1", |b| {
        b.iter(|| aoc_2023::day5::day5_part1(black_box(&text)))
    });
    c.bench_function("day 5 part 2", |b| {
        b.iter(|| aoc_2023::day5::day5_part2(black_box(&text)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(500)
        .measurement_time(Duration::from_secs(30));
    targets = day5
}
criterion_main!(benches);
