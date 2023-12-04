use std::collections::HashSet;
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
            Some((_, contents)) => Ok(contents),
            None => Err(ParseCardError),
        }?;

        let (winning_numbers_str, our_numbers_str) = match contents.split_once(" | ") {
            Some(x) => x,
            None => return Err(ParseCardError),
        };

        let winning_numbers = parse_numbers(winning_numbers_str).map_err(|_| ParseCardError)?;
        let our_numbers = parse_numbers(our_numbers_str).map_err(|_| ParseCardError)?;
        Ok(Card {
            winning_numbers,
            our_numbers,
        })
    }
}

// Some new syntax here!
//   'a represents a lifetime that we have labelled "a".
//   It must appear in `parse_cards<'a>` to declare the label.
//   It then appears in `&'a str` to indicate that we are using it to label the lifetime
//      of the argument.
//   We then want to indicate that the lifetime of the result is the same as this. The
//      notation was introduced in this RFC:
//
//      https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
//
// Here is the explicit line:
//
//  fn parse_cards<'a>(input: &'a str) -> impl Iterator<Item = Card> + 'a {
//
// But then clippy points out that we can simplify it to the following using
//  the placeholder lifetime `'_`. This will match the lifetime of the argument.
fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.trim().lines().map(|x| x.parse().unwrap())
}

fn num_winning(card: &Card) -> u32 {
    let winning: HashSet<u32> = card.winning_numbers.iter().cloned().collect();
    let ours: HashSet<u32> = card.our_numbers.iter().cloned().collect();
    ours.intersection(&winning).count() as u32
}

fn part1(input: &str) -> String {
    let answer: u32 = parse_cards(input)
        .map(|card| {
            let n = num_winning(&card);
            match n {
                0 => 0,
                _ => 2u32.pow(n - 1),
            }
        })
        .sum::<u32>();
    answer.to_string()
}

struct Replicator {
    /// The number of copies that should be made.
    num_copies: u32,

    /// The number of remaining cards to which this applies.
    remaining_lifetime: u32,
}

fn part2(input: &str) -> String {
    // We keep track of a stack of replicators; each entry has a 'time-to-live',
    // which is decremented as we go through the pack. It also indicates how many
    // copies should be made.
    let mut replicators: Vec<Replicator> = Vec::new();

    let answer: u32 = parse_cards(input)
        .map(|card| {
            // First we score this card; i.e. figure out how many of it we have.
            // One for the initial copy, and then we sum up the number of copies to make
            let num_copies = 1 + replicators.iter().map(|x| x.num_copies).sum::<u32>();

            // Update the existing replicators...
            // PERF: An alternative design could probably reduce all these allocations.
            replicators = replicators
                .iter()
                .filter_map(|x| {
                    match x.remaining_lifetime {
                        0 => unreachable!(), // We should never create a replicator with a lifetime of 0.
                        1 => None,
                        n => Some(Replicator {
                            num_copies: x.num_copies,
                            remaining_lifetime: n - 1,
                        }),
                    }
                })
                .collect();

            // ... and add a new replicator if required.
            match num_winning(&card) {
                0 => {}
                n => replicators.push(Replicator {
                    num_copies,
                    remaining_lifetime: n,
                }),
            }

            // We return the number of copies; we will aggregate these.
            num_copies
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

    const EXAMPLE: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "13")
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "30")
    }
}
