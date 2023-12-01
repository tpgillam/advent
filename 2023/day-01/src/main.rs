fn get_input() -> &'static str {
    return include_str!("../input.txt");
}

fn part1(input: &str) -> String {
    let answer: u32 = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|x| {
                if x.is_ascii_digit() {
                    Some(x.to_string().parse::<u32>().unwrap())
                } else {
                    None
                }
            });
            let first_digit = digits.next().unwrap();
            let last_digit = match digits.last() {
                Some(x) => x,
                None => first_digit,
            };
            first_digit * 10 + last_digit
        })
        .sum();

    return answer.to_string();
}

fn part2(input: &str) -> String {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let answer: u32 = input
        .lines()
        .map(|line| {
            // Replace any occurrences of known digits with digits.
            // NOTE: We must always replace the first valid string seen first!
            //  so "eightwo" -> "8wo" (and not "eigh2")
            let mut line_new = line.to_string();
            // FIXME: This replaces stuff in the wrong order.
            for (i, number) in numbers.iter().enumerate() {
                line_new = line_new.replace(number, &(i + 1).to_string());
            }
            dbg!(line_new.clone());

            let mut digits = line_new.chars().filter_map(|x| {
                if x.is_ascii_digit() {
                    Some(x.to_string().parse::<u32>().unwrap())
                } else {
                    None
                }
            });
            let first_digit = digits.next().unwrap();
            let last_digit = match digits.last() {
                Some(x) => x,
                None => first_digit,
            };
            first_digit * 10 + last_digit
        })
        .sum();

    return answer.to_string();
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    fn get_example_1() -> &'static str {
        return include_str!("../example_1.txt");
    }
    fn get_example_2() -> &'static str {
        return include_str!("../example_2.txt");
    }

    #[test]
    fn part_1() {
        assert_eq!(part1(get_example_1()), "142");
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(get_example_2()), "281");
    }
}
