use std::{collections::HashMap, fmt::Debug};

use num::integer::lcm;

pub fn day8_part1(input: &[u8]) -> u64 {
    let (instructions, hm) = parse(input);

    #[inline]
    fn end_node(node: Node) -> bool {
        node == END
    }

    steps_to_end(START, &hm, instructions, end_node)
}

pub fn day8_part2(input: &[u8]) -> u64 {
    let (instructions, hm) = parse(input);

    #[inline]
    fn end_node(node: Node) -> bool {
        node.is_end()
    }

    hm.keys()
        .filter(|key| key.is_start())
        .map(|key| steps_to_end(*key, &hm, instructions, end_node))
        .reduce(lcm)
        .unwrap()
}

fn steps_to_end(
    start: Node,
    hm: &HashMap<Node, (Node, Node)>,
    instructions: &[u8],
    end_node: fn(Node) -> bool,
) -> u64 {
    let mut steps = 0;
    let mut key = start;
    loop {
        let (left, right) = hm[&key];
        match instructions[steps % instructions.len()] {
            b'L' => key = left,
            b'R' => key = right,
            _ => unreachable!(
                "invalid input at step {} ({:?})",
                steps, instructions[steps] as char
            ),
        }
        steps += 1;

        if end_node(key) {
            break steps.try_into().unwrap();
        }
    }
}

const START: Node = Node::new(b"AAA");
const END: Node = Node::new(b"ZZZ");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node(u16);

impl Node {
    const fn new(key: &[u8]) -> Self {
        let mut value = (key[0] - b'A') as u16;
        value *= 26;
        value += (key[1] - b'A') as u16;
        value *= 26;
        value += (key[2] - b'A') as u16;
        Self(value)
    }

    #[inline]
    fn is_start(&self) -> bool {
        self.0 % 26 == 0
    }

    #[inline]
    fn is_end(&self) -> bool {
        self.0 % 26 == 25
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = (self.0 / 26 / 26) as u8 + b'A';
        let second = (self.0 / 26 % 26) as u8 + b'A';
        let third = (self.0 % 26) as u8 + b'A';
        f.debug_tuple("Node")
            .field(&format_args!(
                "{}{}{}",
                first as char, second as char, third as char
            ))
            .finish()
    }
}

fn parse(input: &[u8]) -> (&[u8], HashMap<Node, (Node, Node)>) {
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

fn parse_map_line(input: &[u8]) -> (Node, (Node, Node)) {
    let key = Node::new(&input[0..3]);
    let left = Node::new(&input[7..10]);
    let right = Node::new(&input[12..15]);
    (key, (left, right))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    use test_case::test_case;

    #[test_case(b"AAA" => true)]
    #[test_case(b"AAB" => false)]
    #[test_case(b"ZZA" => true)]
    #[test_case(b"ZZZ" => false)]
    fn test_node_is_start(key: &[u8]) -> bool {
        Node::new(key).is_start()
    }

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

    #[test]
    fn test_day8_part2_example_3() {
        let input = utils::load_example_with_suffix(8, "3");
        assert_eq!(day8_part2(&input), 6);
    }

    #[test]
    fn test_day8_part1_real() {
        let input = utils::load_real(8);
        assert_eq!(day8_part1(&input), 13019);
    }

    #[test]
    fn test_day8_part2_real() {
        let input = utils::load_real(8);
        assert_eq!(day8_part2(&input), 13524038372771);
    }
}
