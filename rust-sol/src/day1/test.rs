use crate::day1::sol::part1;
use crate::day1::sol::part2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "/home/okabe/graveyard/aoc/data/day1.in";
        println!("Part1: {}", part1::driver(input));
    }

    #[test]
    fn test_part2() {
        let input = "/home/okabe/graveyard/aoc/data/day1.in";
        println!("Part1: {}", part2::driver(input));
    }
}
