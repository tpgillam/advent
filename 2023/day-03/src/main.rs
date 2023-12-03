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
mod tests {
    use crate::part1;

    #[test]
    fn example_part1() {
        let example = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(example), "4361")
    }
}
