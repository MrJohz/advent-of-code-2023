#![allow(clippy::single_range_in_vec_init)]

use std::ops::Range;
#[derive(Debug, PartialEq, Eq)]
struct Seed {
    id: i64,
    stage: u8,
}

impl Seed {
    fn new(id: i64) -> Self {
        Self { id, stage: 0 }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Seeds {
    seeds: Vec<Range<i64>>,
    next_stage: Vec<Range<i64>>,
}

impl Seeds {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            next_stage: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct MapLine {
    source_start: i64,
    source_end: i64,
    modifier: i64,
}

fn parse_number(input: &[u8]) -> (i64, usize) {
    let mut number = 0;
    let mut i = 0;
    while input[i] != b' ' && input[i] != b'\n' {
        number = number * 10 + (input[i] - b'0') as i64;
        i += 1;
    }
    (number, i + 1)
}

fn seeds(input: &[u8]) -> (Vec<Seed>, usize) {
    let mut seeds = Vec::new();
    let mut pos = 7;
    while input[pos] != b'\n' {
        let (seed, len) = parse_number(&input[pos..]);
        seeds.push(Seed::new(seed));
        pos += len;
    }
    (seeds, pos)
}

fn seed_ranges(input: &[u8]) -> (Seeds, usize) {
    let mut seeds = Seeds::new();
    let mut pos = 7;
    while input[pos] != b'\n' {
        let (start, len) = parse_number(&input[pos..]);
        pos += len;
        let (range, len) = parse_number(&input[pos..]);
        seeds.seeds.push(start..start + range);
        pos += len;
    }
    (seeds, pos)
}

fn parse_map_line(input: &[u8]) -> (MapLine, usize) {
    let (destination, len) = parse_number(input);
    let (source, len2) = parse_number(&input[len..]);
    let (size, len3) = parse_number(&input[(len + len2)..]);
    (
        MapLine {
            source_start: source,
            source_end: source + size,
            modifier: destination - source,
        },
        len + len2 + len3,
    )
}

fn apply_seedmap(map_line: MapLine, seeds: &mut Seeds) {
    let mut to_add = Vec::new();
    seeds.seeds.retain_mut(|seed| {
        if seed.end <= map_line.source_start || seed.start >= map_line.source_end {
            true
        } else if seed.start >= map_line.source_start && seed.end <= map_line.source_end {
            seeds
                .next_stage
                .push(seed.start + map_line.modifier..seed.end + map_line.modifier);
            false
        } else if seed.start < map_line.source_start && seed.end <= map_line.source_end {
            seeds
                .next_stage
                .push(map_line.source_start + map_line.modifier..seed.end + map_line.modifier);
            seed.end = map_line.source_start;
            true
        } else if seed.start >= map_line.source_start && seed.end >= map_line.source_end {
            seeds
                .next_stage
                .push(seed.start + map_line.modifier..map_line.source_end + map_line.modifier);
            seed.start = map_line.source_end;
            true
        } else if seed.start < map_line.source_start && seed.end >= map_line.source_end {
            seeds.next_stage.push(
                map_line.source_start + map_line.modifier..map_line.source_end + map_line.modifier,
            );
            to_add.push(map_line.source_end..seed.end);
            seed.end = map_line.source_start;
            true
        } else {
            unreachable!("All cases should be covered")
        }
    });

    seeds.seeds.append(&mut to_add);
}

fn parse_and_apply_maps_to_seed_ranges(input: &[u8], seeds: &mut Seeds) -> usize {
    let mut pos = 0;
    while input.len() > pos && input[pos] != b'\n' {
        let (map_line, len) = parse_map_line(&input[pos..]);
        pos += len;
        apply_seedmap(map_line, seeds);
    }
    seeds.seeds.append(&mut seeds.next_stage);

    pos
}

fn parse_and_apply_maps(input: &[u8], stage: u8, seeds: &mut [Seed]) -> usize {
    let mut pos = 0;
    while input.len() > pos && input[pos] != b'\n' {
        let (map_line, len) = parse_map_line(&input[pos..]);
        pos += len;
        for seed in seeds.iter_mut() {
            if seed.stage != stage {
                continue;
            }

            if seed.id >= map_line.source_start && seed.id < map_line.source_end {
                seed.id += map_line.modifier;
                seed.stage += 1;
            }
        }
    }
    for seed in seeds.iter_mut() {
        seed.stage = stage + 1;
    }

    pos
}

pub fn day5_part1(input: &[u8]) -> u64 {
    let (mut seeds, mut pos) = seeds(input);
    let mut stage = 0;
    while input.len() > pos && input[pos] == b'\n' {
        pos += 1;
        pos += memchr::memchr(b'\n', &input[pos..]).unwrap() + 1;
        pos += parse_and_apply_maps(&input[pos..], stage, &mut seeds);
        stage += 1;
    }

    seeds.into_iter().map(|seed| seed.id).min().unwrap() as u64
}

pub fn day5_part2(input: &[u8]) -> u64 {
    let (mut seeds, mut pos) = seed_ranges(input);
    while input.len() > pos && input[pos] == b'\n' {
        pos += 1;
        pos += memchr::memchr(b'\n', &input[pos..]).unwrap() + 1;
        pos += parse_and_apply_maps_to_seed_ranges(&input[pos..], &mut seeds);
    }

    seeds
        .seeds
        .into_iter()
        .map(|seed| seed.start)
        .min()
        .unwrap()
        .try_into()
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_finds_seeds() {
        let input = utils::load_example(5);
        assert_eq!(
            seeds(&input).0,
            vec![Seed::new(79), Seed::new(14), Seed::new(55), Seed::new(13)]
        );
    }

    #[test]
    fn test_finds_seed_ranges() {
        let input = utils::load_example(5);
        assert_eq!(
            seed_ranges(&input).0,
            Seeds {
                seeds: vec![79..93, 55..68],
                next_stage: Vec::new(),
            }
        );
    }

    #[test]
    fn test_applies_map_to_out_of_range_seeds() {
        let mut seeds = Seeds {
            seeds: vec![79..93, 55..68],
            next_stage: Vec::new(),
        };
        let map_line = MapLine {
            source_start: 68,
            source_end: 77,
            modifier: 99,
        };

        apply_seedmap(map_line, &mut seeds);
        assert_eq!(
            seeds,
            Seeds {
                seeds: vec![79..93, 55..68],
                next_stage: Vec::new(),
            }
        );
    }

    #[test]
    fn test_applies_map_to_seeds_fully_within_range() {
        let mut seeds = Seeds {
            seeds: vec![79..88, 55..100],
            next_stage: Vec::new(),
        };
        let map_line = MapLine {
            source_start: 55,
            source_end: 100,
            modifier: 10,
        };

        apply_seedmap(map_line, &mut seeds);
        assert_eq!(
            seeds,
            Seeds {
                seeds: vec![],
                next_stage: vec![89..98, 65..110],
            }
        );
    }

    #[test]
    fn test_applies_map_to_seeds_overlapping_initial_boundary_of_map() {
        let mut seeds = Seeds {
            seeds: vec![10..30, 10..50],
            next_stage: Vec::new(),
        };
        let map_line = MapLine {
            source_start: 25,
            source_end: 50,
            modifier: 10,
        };

        apply_seedmap(map_line, &mut seeds);
        assert_eq!(
            seeds,
            Seeds {
                seeds: vec![10..25, 10..25],
                next_stage: vec![35..40, 35..60],
            }
        );
    }

    #[test]
    fn test_applies_map_to_seeds_overlapping_final_boundary_of_map() {
        let mut seeds = Seeds {
            seeds: vec![40..60, 20..60],
            next_stage: Vec::new(),
        };
        let map_line = MapLine {
            source_start: 20,
            source_end: 50,
            modifier: 5,
        };

        apply_seedmap(map_line, &mut seeds);
        assert_eq!(
            seeds,
            Seeds {
                seeds: vec![50..60, 50..60],
                next_stage: vec![45..55, 25..55],
            }
        );
    }

    #[test]
    fn test_applies_map_to_seeds_overlapping_both_boundaries_of_map() {
        let mut seeds = Seeds {
            seeds: vec![10..60],
            next_stage: Vec::new(),
        };
        let map_line = MapLine {
            source_start: 20,
            source_end: 50,
            modifier: 5,
        };

        apply_seedmap(map_line, &mut seeds);
        assert_eq!(
            seeds,
            Seeds {
                seeds: vec![10..20, 50..60],
                next_stage: vec![25..55],
            }
        );
    }

    #[test]
    fn test_day5_part1_example() {
        let input = utils::load_example(5);
        assert_eq!(day5_part1(&input), 35);
    }

    #[test]
    fn test_day5_part2_example() {
        let input = utils::load_example(5);
        assert_eq!(day5_part2(&input), 46);
    }

    #[test]
    fn test_day5_part1_real() {
        let input = utils::load_real(5);
        assert_eq!(day5_part1(&input), 227653707);
    }

    #[test]
    fn test_day5_part2_real() {
        let input = utils::load_real(5);
        assert_eq!(day5_part2(&input), 5329815);
    }
}
