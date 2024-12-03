use crate::day1::sol::part1::driver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "/home/okabe/graveyard/aoc/data/day1.in";
        println!("Part1: {}", driver(input));
    }
}
