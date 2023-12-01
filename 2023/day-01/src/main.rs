fn get_example() -> &'static str {
    return include_str!("../example.txt");
}

fn part1(input: &str) -> String {
    for line in input.lines() {
        println!("AHA {}", line);
    }
    return "moo".to_string()
}

fn part2(input: &str) -> String {
    for line in input.lines() {
        println!("OHO {}", line);
    }
    return "moo".to_string()
}

fn main() {
    // TODO: Make the example a test case
    println!("EXAMPLE");
    let input = get_example();
    println!("Part1: {}\n", part1(input));
    println!("Part2: {}", part2(input));
}


#[cfg(test)]
mod tests {
    use super::{get_example, part1};

    #[test]
    fn part_1_result() {
        assert_eq!(part1(get_example()), "142");
    }
}
