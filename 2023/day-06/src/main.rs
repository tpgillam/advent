fn get_input() -> &'static str {
    include_str!("../input.txt")
}

fn part1(input: &str) -> String {
    "moo".to_string()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests{
    use crate::part1;

    const EXAMPLE: &str = "
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "moo"); 
    }
}
