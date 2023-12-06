use almanac::Almanac;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

mod almanac {
    use std::{ops::Range, str::FromStr};

    pub struct RangeMapEntry {
        source_range: Range<u64>,
        destination_start: u64,
    }

    impl RangeMapEntry {
        pub fn new(destination_start: u64, source_start: u64, length: u64) -> Self {
            Self {
                source_range: source_start..(source_start + length),
                destination_start,
            }
        }

        /// Lookup the given source id, and return the destination id if we can
        /// determine it from this entry.
        pub fn lookup(self: &Self, id_source: u64) -> Option<u64> {
            if self.source_range.contains(&id_source) {
                let offset = id_source - self.source_range.start;
                Some(self.destination_start + offset)
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    pub struct ParseRangeMapEntryErr;

    impl FromStr for RangeMapEntry {
        type Err = ParseRangeMapEntryErr;

        fn from_str(s: &str) -> Result<RangeMapEntry, Self::Err> {
            let parts: Vec<&str> = s.split_whitespace().collect();

            let get_part = |i: usize| -> Result<u64, ParseRangeMapEntryErr> {
                parts
                    .get(i)
                    .ok_or(ParseRangeMapEntryErr)?
                    .parse()
                    .map_err(|_| ParseRangeMapEntryErr)
            };

            let destination_start = get_part(0)?;
            let source_start = get_part(1)?;
            let length = get_part(2)?;
            Ok(RangeMapEntry::new(destination_start, source_start, length))
        }
    }

    pub struct RangeMap {
        entries: Vec<RangeMapEntry>,
    }

    impl RangeMap {
        pub fn lookup(self: &Self, id_source: u64) -> u64 {
            for entry in &self.entries {
                match entry.lookup(id_source) {
                    Some(id_destination) => return id_destination,
                    None => continue,
                }
            }
            id_source
        }
    }

    #[derive(Debug)]
    pub struct ParseRangeMapErr;

    impl FromStr for RangeMap {
        type Err = ParseRangeMapErr;

        fn from_str(s: &str) -> Result<RangeMap, Self::Err> {
            let entries: Vec<_> = s
                .lines()
                .map(|line| line.parse::<RangeMapEntry>().map_err(|_| ParseRangeMapErr))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(RangeMap { entries })
        }
    }

    pub struct Almanac {
        /// The ids of the seeds that we need.
        pub seeds: Vec<u64>,

        /// An ordered list of maps, which map seed -> .... -> location.
        /// The inner levels are not named for now.
        pub maps: Vec<RangeMap>,
    }

    impl Almanac {
        pub fn lookup(&self, seed: u64) -> u64 {
            let mut id = seed;
            for map in &self.maps {
                id = map.lookup(id);
            }
            id
        }
    }

    #[derive(Debug)]
    pub struct ParseAlmanacErr;

    impl FromStr for Almanac {
        type Err = ParseAlmanacErr;

        fn from_str(s: &str) -> Result<Almanac, Self::Err> {
            let groups: Vec<_> = s.trim().split("\n\n").collect();
            if groups.len() != 8 {
                // Very unexpected! Please let us know...
                panic!();
            }

            let seeds = groups[0]
                .trim()
                .replace("seeds: ", "")
                .split_whitespace()
                .map(|x| x.parse::<u64>().map_err(|_| ParseAlmanacErr))
                .collect::<Result<Vec<_>, _>>()?;

            // Absorb all maps.
            let maps = (1..groups.len())
                .map(|i| {
                    let group = groups[i];

                    // Not using map name for now.
                    let (_map_name_line, rest) = group.split_once("\n").ok_or(ParseAlmanacErr)?;
                    rest.parse::<RangeMap>().map_err(|_| ParseAlmanacErr)
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Almanac { seeds, maps })
        }
    }
}

fn part1(input: &str) -> String {
    let almanac: Almanac = input.parse().unwrap();

    let answer = almanac
        .seeds
        .iter()
        .map(|&seed| almanac.lookup(seed))
        .min()
        .unwrap();

    answer.to_string()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::almanac::{Almanac, RangeMap, RangeMapEntry};
    use crate::part1;

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
        assert_eq!(part1(EXAMPLE), "35")
    }

    #[test]
    fn test_parse_range_map_entry_trivial() {
        let entry: RangeMapEntry = "10 1 0".parse().unwrap();
        assert_eq!(entry.lookup(1), None);
        assert_eq!(entry.lookup(2), None);
        assert_eq!(entry.lookup(10), None);
    }

    #[test]
    fn test_parse_range_map_entry() {
        let entry: RangeMapEntry = "50 98 2".parse().unwrap();
        assert_eq!(entry.lookup(97), None);
        assert_eq!(entry.lookup(98), Some(50));
        assert_eq!(entry.lookup(99), Some(51));
        assert_eq!(entry.lookup(100), None);
    }

    #[test]
    fn test_parse_range_map() {
        let entry: RangeMap = "50 98 2\n52 50 48".parse().unwrap();
        assert_eq!(entry.lookup(98), 50);
        assert_eq!(entry.lookup(99), 51);
        assert_eq!(entry.lookup(100), 100);

        assert_eq!(entry.lookup(49), 49);
        assert_eq!(entry.lookup(50), 52);
        assert_eq!(entry.lookup(60), 62);
        assert_eq!(entry.lookup(76), 78);
        assert_eq!(entry.lookup(97), 99);
    }

    #[test]
    fn test_parse_almanac() {
        let almanac: Almanac = EXAMPLE.parse().unwrap();
        assert_eq!(almanac.seeds, [79, 14, 55, 13]);
        assert_eq!(almanac.maps.len(), 7);
    }
}
