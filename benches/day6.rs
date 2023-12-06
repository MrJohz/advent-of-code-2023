use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day6(c: &mut Criterion) {
    let text = aoc_2023::utils::load_real(6);
    c.bench_function("day 6 part 1", |b| {
        b.iter(|| aoc_2023::day6::day6_part1(black_box(&text)))
    });
    c.bench_function("day 6 part 2", |b| {
        b.iter(|| aoc_2023::day6::day6_part2(black_box(&text)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(500)
        .measurement_time(Duration::from_secs(30));
    targets = day6
}
criterion_main!(benches);
