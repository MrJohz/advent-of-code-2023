use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day3(c: &mut Criterion) {
    let text = aoc_2023::utils::load_real(2);
    c.bench_function("day 3 part 1", |b| {
        b.iter(|| aoc_2023::day3::day3_part1(black_box(&text)))
    });
    c.bench_function("day 3 part 2", |b| {
        b.iter(|| aoc_2023::day3::day3_part2(black_box(&text)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(1000)
        .measurement_time(Duration::from_secs(60 * 2));
    targets = day3
}
criterion_main!(benches);
