use std::collections::HashMap;

fn get_input() -> &'static str {
    include_str!("../input.txt")
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

    answer.to_string()
}

// Replace any occurrences of known digits with digits.
// NOTE: We always replace ALL digits, even overlapping. To rationalise this,
//  we only replace the first character of the number with the digit, e.g.
//      "eightwo" -> "8igh2wo"
//  This means that the resulting string has the same length as the input.
fn replace_string_numbers(line: &str) -> String {
    // Find all the literal numbers in the string.
    let digit_to_locations = find_literal_digit_occurrences(line);

    // NOTE: For future learning...
    // Initially I came up with the following contorted code:
    //
    //      let line_new: Vec<u8> = line.as_bytes().into_iter().map(|&x| x).collect();
    //
    // What's happening here is that `as_bytes` is referring to (immutable) bytes
    // stored inside `line`. So the weird `map` call is required to actually do a copy.
    //
    // The below is better -- here we do the copy in one go with `to_string`, and then
    // use `into_bytes` to consume the String and get a Vec<u8> (which is owning).
    let mut line_bytes = line.to_string().into_bytes();

    for (&digit, locations) in digit_to_locations.iter() {
        for &loc in locations.iter() {
            line_bytes[loc] = digit;
        }
    }

    String::from_utf8(line_bytes).unwrap()
}

// Return a map from digit (as an ASCII byte) to the locations in `str` at which it occurs.
fn find_literal_digit_occurrences(line: &str) -> HashMap<u8, Vec<usize>> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let result: HashMap<_, _> = numbers
        .iter()
        .enumerate()
        .filter_map(|(i, &number)| {
            let matches: Vec<_> = line.match_indices(number).collect();

            if matches.is_empty() {
                None
            } else {
                let digit_bytes = (i + 1).to_string().into_bytes();
                assert_eq!(digit_bytes.len(), 1);
                return Some((
                    digit_bytes[0],
                    matches.iter().map(|&(loc, _)| loc).collect::<Vec<_>>(),
                ));
            }
        })
        .collect();
    result
}

fn part2(input: &str) -> String {
    let answer: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<_> = replace_string_numbers(line)
                .chars()
                .filter_map(|x| {
                    if x.is_ascii_digit() {
                        Some(x.to_string().parse::<u32>().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();
            first_digit * 10 + last_digit
        })
        .sum();

    answer.to_string()
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
        include_str!("../example_1.txt")
    }
    fn get_example_2() -> &'static str {
        include_str!("../example_2.txt")
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
