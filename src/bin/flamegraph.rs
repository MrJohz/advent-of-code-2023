fn main() {
    let input = aoc_2023::utils::load_real(8);
    loop {
        aoc_2023::day08::day8_part1(&input);
        aoc_2023::day08::day8_part2(&input);
    }
}
