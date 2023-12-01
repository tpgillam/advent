use std::collections::HashMap;

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

// Replace any occurrences of known digits with digits.
// NOTE: We must always replace the first valid string seen first!
//  so "eightwo" -> "8wo" (and not "eigh2")
fn replace_string_numbers(line: &str) -> String {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut line_new = line.to_string();

    loop {
        // Look for replacements that we can make in the line, and store
        // the index where the substring starts.
        let i_number_to_loc: HashMap<_, _> = numbers
            .iter()
            .enumerate()
            .filter_map(|(i, number)| {
                let loc = line_new.find(number)?;
                Some((i, loc))
            })
            .collect();

        if i_number_to_loc.is_empty() {
            // There are no replacements left to make.
            break;
        }

        // Perform only the first replacement that can be made; we then repeat the process
        // above.
        let (&i_number, _) = i_number_to_loc.iter().min_by_key(|(_, &loc)| loc).unwrap();
        line_new = line_new.replacen(
            numbers.get(i_number).unwrap(),
            &(i_number + 1).to_string(),
            1,
        );
    }

    return line_new;
}

fn part2(input: &str) -> String {
    let answer: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<_> = replace_string_numbers(line).chars().filter_map(|x| {
                if x.is_ascii_digit() {
                    Some(x.to_string().parse::<u32>().unwrap())
                } else {
                    None
                }
            }).collect();
            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();
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
