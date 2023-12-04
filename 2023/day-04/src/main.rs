use std::num::ParseIntError;
use std::str::FromStr;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

#[derive(Debug)]
struct ParseCardError;

fn parse_numbers(s: &str) -> Result<Vec<u32>, ParseIntError> {
    s.split_whitespace().map(|x| x.parse::<u32>()).collect()
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Card, ParseCardError> {
        let contents = match s.split_once(": ") {
            Some((_, contents)) => contents,
            None => return Err(ParseCardError),
        };

        let (winning_numbers_str, our_numbers_str) = match contents.split_once(" | ") {
            Some(x) => x,
            None => return Err(ParseCardError),
        };

        let winning_numbers = parse_numbers(winning_numbers_str).map_err(|_| ParseCardError)?;
        let our_numbers = parse_numbers(our_numbers_str).map_err(|_| ParseCardError)?;
        return Ok(Card {
            winning_numbers,
            our_numbers,
        });
    }
}

fn part1(input: &str) -> String {
    let cards: Vec<Card> = input.trim().lines().map(|x| x.parse().unwrap()).collect();
    dbg!(cards);
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
    fn part1_example() {
        let example = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(example), "13")
    }
}
