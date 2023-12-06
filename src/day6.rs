pub fn day6_part1(input: &[u8]) -> u64 {
    entries(input)
        .map(evaluate_min_maxes)
        .map(|(min, max)| 1 + ((max).floor() as u64) - ((min).ceil() as u64))
        .product()
}

pub fn day6_part2(input: &[u8]) -> u64 {
    let entry = entry(input);
    let (min, max) = evaluate_min_maxes(entry);

    1 + ((max).floor() as u64) - ((min).ceil() as u64)
}

fn evaluate_min_maxes(entry: Entry) -> (f32, f32) {
    let rooted = (entry.time.powf(2.0) - 4.0 * (entry.distance + 1.0)).sqrt();
    let min = (entry.time - rooted) / 2.0;
    let max = (entry.time + rooted) / 2.0;
    (min, max)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Entry {
    time: f32,
    distance: f32,
}

struct EntryIter<'a> {
    input: &'a [u8],
    line2_start: usize,
    pos: usize,
}

impl Iterator for EntryIter<'_> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + self.line2_start >= self.input.len() {
            return None;
        }

        let (time, bytes_read_l1) = parse_number(&self.input[self.pos..]);
        let (distance, bytes_read_l2) = parse_number(&self.input[self.line2_start + self.pos..]);

        debug_assert_eq!(bytes_read_l1, bytes_read_l2);
        self.pos += bytes_read_l1;

        Some(Entry { time, distance })
    }
}

fn entry(input: &[u8]) -> Entry {
    let (time, bytes_read_l1) = parse_number_with_spaces(&input[9..]);
    let (distance, bytes_read_l2) = parse_number_with_spaces(&input[19 + bytes_read_l1..]);

    debug_assert_eq!(bytes_read_l1, bytes_read_l2);

    Entry { time, distance }
}

fn entries(input: &[u8]) -> EntryIter {
    let line2_start = memchr::memchr(b'\n', input).unwrap() + 1;
    EntryIter {
        input,
        line2_start,
        pos: 9, // ignore header text
    }
}

fn parse_number_with_spaces(input: &[u8]) -> (f32, usize) {
    let mut number = 0.0;
    let mut i = 0;
    while input[i] == b' ' {
        i += 1;
    }
    while input[i] != b'\n' {
        if input[i] == b' ' {
            i += 1;
            continue;
        }

        number = number * 10.0 + f32::from(input[i] - b'0');
        i += 1;
    }
    (number, i)
}

fn parse_number(input: &[u8]) -> (f32, usize) {
    let mut number = 0.0;
    let mut i = 0;
    while input[i] == b' ' {
        i += 1;
    }
    while input[i] != b' ' && input[i] != b'\n' {
        number = number * 10.0 + f32::from(input[i] - b'0');
        i += 1;
    }
    (number, i + 1)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_finds_numbers() {
        let input = "Time:      7  15   30\nDistance:  9  40  200\n";
        let mut iter = entries(input.as_bytes());
        assert_eq!(
            iter.next(),
            Some(Entry {
                time: 7.0,
                distance: 9.0
            })
        );
        assert_eq!(
            iter.next(),
            Some(Entry {
                time: 15.0,
                distance: 40.0
            })
        );
        assert_eq!(
            iter.next(),
            Some(Entry {
                time: 30.0,
                distance: 200.0
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_finds_numbers_with_spaces() {
        let input = "Time:      7  15   30\nDistance:  9  40  200\n";
        let entry = entry(input.as_bytes());
        assert_eq!(
            entry,
            Entry {
                time: 71530.0,
                distance: 940200.0
            }
        );
    }

    #[test]
    fn test_day6_part1_example() {
        let input = utils::load_example(6);
        assert_eq!(day6_part1(&input), 288);
    }

    #[test]
    fn test_day6_part2_example() {
        let input = utils::load_example(6);
        assert_eq!(day6_part2(&input), 71503);
    }

    #[test]
    fn test_day6_part1_real() {
        let input = utils::load_real(6);
        assert_eq!(day6_part1(&input), 160816);
    }

    #[test]
    fn test_day6_part2_real() {
        let input = utils::load_real(6);
        assert_eq!(day6_part2(&input), 46561107);
    }
}
