use std::fs;

pub fn load_example(day: usize) -> Vec<u8> {
    fs::read(format!("inputs/day{day}-sample.txt")).unwrap()
}

pub fn load_example_with_suffix(day: usize, suffix: &str) -> Vec<u8> {
    fs::read(format!("inputs/day{day}-sample-{suffix}.txt")).unwrap()
}

pub fn load_real(day: usize) -> Vec<u8> {
    fs::read(format!("inputs/day{day}.txt")).unwrap()
}
