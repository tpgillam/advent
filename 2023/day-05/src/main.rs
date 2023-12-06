use std::{collections::HashMap, str::FromStr};

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

/// Parsed problem input
struct Almanac {
    /// The ids of the seeds that we need.
    seeds: Vec<u32>,

    /// An ordered list of maps, which map seed -> .... -> location.
    /// The inner levels are not named for now.
    maps: Vec<HashMap<u32, u32>>,
}

struct ParseAlmanacErr;

impl FromStr for Almanac {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Almanac, Self::Err> {
        todo!()
    }
}

// FIXME: The ranges in question are huge; this won't actually scale.
// FIXME: The ranges in question are huge; this won't actually scale.
// FIXME: The ranges in question are huge; this won't actually scale.
fn parse_source_to_destination(s: &str) -> HashMap<u32, u32> {
    let mut result: HashMap<u32, u32> = HashMap::new();
    for line in s.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let destination_start: u32 = parts[0].parse().unwrap();
        let source_start: u32 = parts[1].parse().unwrap();
        let range: u32 = parts[2].parse().unwrap();
        for i in 0..range {
            result.insert(source_start + i, destination_start + i);
        }
    }
    result
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
    use std::collections::HashMap;

    use crate::{parse_source_to_destination, part1};

    const EXAMPLE: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "moo")
    }

    #[test]
    fn test_parse_source_to_destiation() {
        assert_eq!(
            parse_source_to_destination("10 1 1"),
            HashMap::from_iter([(1, 10)])
        );
        assert_eq!(parse_source_to_destination("10 1 0"), HashMap::new());
        assert_eq!(
            parse_source_to_destination("50 98 2\n52 50 3"),
            HashMap::from_iter([(98, 50), (99, 51), (50, 52), (51, 53), (52, 54)])
        );
    }
}
