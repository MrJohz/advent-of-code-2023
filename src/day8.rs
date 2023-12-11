use std::collections::HashMap;

pub fn day8_part1(input: &[u8]) -> u64 {
    let (input, hashmap) = parse(input);

    let mut steps = 0;
    let mut key = b"AAA".as_slice();
    loop {
        let (left, right) = hashmap[key];
        println!(
            "step {}: {:?} -> {:?} {:?}",
            steps,
            String::from_utf8_lossy(key),
            String::from_utf8_lossy(left),
            String::from_utf8_lossy(right),
        );
        match input[steps % input.len()] {
            b'L' => key = left,
            b'R' => key = right,
            _ => unreachable!(
                "invalid input at step {} ({:?})",
                steps, input[steps] as char
            ),
        }
        steps += 1;

        if key == b"ZZZ" {
            break steps.try_into().unwrap();
        }
    }
}

pub fn day8_part2(input: &[u8]) -> u64 {
    0
}

fn parse(input: &[u8]) -> (&[u8], HashMap<&[u8], (&[u8], &[u8])>) {
    let instructions_end = memchr::memchr(b'\n', input).unwrap();

    // TODO: we can probably allocate once by counting the number of lines
    // TODO: we can probably avoid allocation altogether using a fixed-sized array a PHF
    let hashmap = (0_usize..)
        .map(|i| {
            (
                i * 17 + instructions_end + 2,
                i * 17 + 16 + instructions_end + 2,
            )
        })
        .take_while(|(_, end)| *end < input.len())
        .map(|(start, end)| parse_map_line(&input[start..end]))
        .collect();

    (&input[0..instructions_end], hashmap)
}

fn parse_map_line(input: &[u8]) -> (&[u8], (&[u8], &[u8])) {
    let key = &input[0..3];
    let left = &input[7..10];
    let right = &input[12..15];
    (key, (left, right))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    use test_case::test_case;

    #[test]
    fn test_day8_part1_example_1() {
        let input = utils::load_example_with_suffix(8, "1");
        assert_eq!(day8_part1(&input), 2);
    }

    #[test]
    fn test_day8_part1_exampl_2() {
        let input = utils::load_example_with_suffix(8, "2");
        assert_eq!(day8_part1(&input), 6);
    }

    // #[test]
    // fn test_day8_part2_example() {
    //     let input = utils::load_example(8);
    //     assert_eq!(day8_part2(&input), 5905);
    // }

    #[test]
    fn test_day8_part1_real() {
        let input = utils::load_real(8);
        assert_eq!(day8_part1(&input), 13019);
    }

    // #[test]
    // fn test_day8_part2_real() {
    //     let input = utils::load_real(8);
    //     assert_eq!(day8_part2(&input), 253630098);
    // }
}
