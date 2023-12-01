use std::fs;

pub fn load_example(day: usize) -> Vec<u8> {
    fs::read_to_string(format!("inputs/day{day}-sample.txt"))
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec()
}

pub fn load_example_with_suffix(day: usize, suffix: &str) -> Vec<u8> {
    fs::read_to_string(format!("inputs/day{day}-sample-{suffix}.txt"))
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec()
}

pub fn load_real(day: usize) -> Vec<u8> {
    fs::read_to_string(format!("inputs/day{day}.txt"))
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec()
}
