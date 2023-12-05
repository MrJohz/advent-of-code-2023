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

#[derive(Debug)]
struct MapLine {
    destination: i64,
    source: i64,
    size: i64,
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

fn parse_map_line(input: &[u8]) -> (MapLine, usize) {
    let (destination, len) = parse_number(input);
    let (source, len2) = parse_number(&input[len..]);
    let (size, len3) = parse_number(&input[(len + len2)..]);
    (
        MapLine {
            destination,
            source,
            size,
        },
        len + len2 + len3,
    )
}

fn parse_and_apply_maps(input: &[u8], stage: u8, seeds: &mut [Seed]) -> usize {
    let mut pos = 0;
    while input.len() > pos && input[pos] != b'\n' {
        let (map_line, len) = parse_map_line(&input[pos..]);
        dbg!(&map_line);
        pos += len;
        for seed in seeds.iter_mut() {
            if seed.stage != stage {
                continue;
            }

            if seed.id >= map_line.source && seed.id < map_line.source + map_line.size {
                seed.id = map_line.destination + seed.id - map_line.source;
                seed.stage += 1;
            }
        }
    }
    for seed in seeds.iter_mut() {
        seed.stage = stage + 1;
    }

    dbg!(pos)
}

pub fn day5_part1(input: &[u8]) -> u32 {
    let (mut seeds, mut pos) = dbg!(seeds(input));
    let mut stage = 0;
    while input.len() > pos && input[pos] == b'\n' {
        pos += 1;
        dbg!(String::from_utf8_lossy(&input[pos..]));
        pos += memchr::memchr(b'\n', &input[pos..]).unwrap() + 1;
        pos += parse_and_apply_maps(&input[pos..], stage, &mut seeds);
        stage += 1;
        dbg!(&seeds);
    }

    seeds.into_iter().map(|seed| seed.id).min().unwrap() as u32
}

pub fn day5_part2(input: &[u8]) -> u32 {
    0
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
    fn test_day5_part1_example() {
        let input = utils::load_example(5);
        assert_eq!(day5_part1(&input), 35);
    }

    // #[test]
    // fn test_day5_part2_example() {
    //     let input = utils::load_example(5);
    //     assert_eq!(day5_part2(&input), 30);
    // }

    #[test]
    fn test_day5_part1_real() {
        let input = utils::load_real(5);
        assert_eq!(day5_part1(&input), 227653707);
    }

    // #[test]
    // fn test_day5_part2_real() {
    //     let input = utils::load_real(5);
    //     assert_eq!(day5_part2(&input), 5329815);
    // }
}
