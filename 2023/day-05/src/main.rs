use almanac::Almanac;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

mod almanac {
    use std::{ops::Range, str::FromStr};

    pub struct RangeMapEntry {
        source_range: Range<i64>,
        destination_offset: i64,
    }

    impl RangeMapEntry {
        pub fn new(destination_start: i64, source_start: i64, length: i64) -> Self {
            Self {
                source_range: source_start..(source_start + length),
                destination_offset: destination_start - source_start,
            }
        }

        /// Lookup the given source id, and return the destination id if we can
        /// determine it from this entry.
        pub fn lookup(&self, id_source: i64) -> Option<i64> {
            if self.source_range.contains(&id_source) {
                Some(id_source + self.destination_offset)
            } else {
                None
            }
        }

        /// Return a tuple
        ///     mapped range       (if any)
        ///     unmapped range(s)  (maybe empty)
        pub fn lookup_range(&self, source: &Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
            if (source.end <= self.source_range.start) || (source.start >= self.source_range.end) {
                // No intersection.
                return (None, [source.clone()].to_vec());
            }

            if (source.start < self.source_range.start) && (source.end > self.source_range.end) {
                // Overlap to both sides.
                return (
                    Some(
                        (self.source_range.start + self.destination_offset)
                            ..(self.source_range.end + self.destination_offset),
                    ),
                    [
                        source.start..self.source_range.start,
                        self.source_range.end..source.end,
                    ]
                    .to_vec(),
                );
            }

            if (source.start >= self.source_range.start) && (source.end <= self.source_range.end) {
                // Fully contained within the map.
                return (
                    Some(
                        (source.start + self.destination_offset)
                            ..(source.end + self.destination_offset),
                    ),
                    Vec::new(),
                );
            }

            // At this point we know that we either overlap over start OR we overlap over the end.
            #[expect(clippy::single_range_in_vec_init)]
            if source.start < self.source_range.start {
                (
                    Some(
                        (self.source_range.start + self.destination_offset)
                            ..(source.end + self.destination_offset),
                    ),
                    [source.start..self.source_range.start].to_vec(),
                )
            } else {
                // Only option left is to overlap to the right
                (
                    Some(
                        (source.start + self.destination_offset)
                            ..(self.source_range.end + self.destination_offset),
                    ),
                    [self.source_range.end..source.end].to_vec(),
                )
            }
        }
    }

    #[derive(Debug)]
    pub struct ParseRangeMapEntryErr;

    impl FromStr for RangeMapEntry {
        type Err = ParseRangeMapEntryErr;

        fn from_str(s: &str) -> Result<RangeMapEntry, Self::Err> {
            let parts: Vec<&str> = s.split_whitespace().collect();

            let get_part = |i: usize| -> Result<i64, ParseRangeMapEntryErr> {
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
        pub fn lookup(&self, id_source: i64) -> i64 {
            for entry in &self.entries {
                if let Some(id_destination) = entry.lookup(id_source) {
                    return id_destination;
                }
            }
            id_source
        }

        pub fn lookup_range(&self, source: &Range<i64>) -> Vec<Range<i64>> {
            let mut all_unprocessed: Vec<Range<i64>> = [source.clone()].to_vec();
            let mut all_processed: Vec<Range<i64>> = Vec::new();

            for entry in &self.entries {
                all_unprocessed = all_unprocessed
                    .iter()
                    .flat_map(|this_source| {
                        let (new_processed, new_unprocessed) = entry.lookup_range(this_source);

                        if let Some(x) = new_processed {
                            all_processed.push(x);
                        }

                        new_unprocessed
                    })
                    .collect();
            }

            // Any unprocessed entries at this point should be considered to be processed.
            all_processed.extend_from_slice(&all_unprocessed);

            all_processed
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
        pub seeds: Vec<i64>,

        /// An ordered list of maps, which map seed -> .... -> location.
        /// The inner levels are not named for now.
        pub maps: Vec<RangeMap>,
    }

    impl Almanac {
        pub fn lookup(&self, seed: i64) -> i64 {
            let mut id = seed;
            for map in &self.maps {
                id = map.lookup(id);
            }
            id
        }

        pub fn lookup_range(&self, seeds: Range<i64>) -> Vec<Range<i64>> {
            let mut ranges: Vec<Range<i64>> = [seeds].to_vec();
            for map in &self.maps {
                // Replace the ranges with the result of applying this layer of mappings.
                ranges = ranges.iter().flat_map(|r| map.lookup_range(r)).collect();
            }
            ranges
        }
    }

    #[derive(Debug)]
    pub struct ParseAlmanacErr;

    impl FromStr for Almanac {
        type Err = ParseAlmanacErr;

        fn from_str(s: &str) -> Result<Almanac, Self::Err> {
            let groups: Vec<_> = s.trim().split("\n\n").collect();

            // Very unexpected if this assert fails! Please let us know...
            assert!(groups.len() == 8);

            let seeds = groups[0]
                .trim()
                .replace("seeds: ", "")
                .split_whitespace()
                .map(|x| x.parse::<i64>().map_err(|_| ParseAlmanacErr))
                .collect::<Result<Vec<_>, _>>()?;

            // Absorb all maps.
            let maps = (1..groups.len())
                .map(|i| {
                    let group = groups[i];

                    // Not using map name for now.
                    let (_map_name_line, rest) = group.split_once('\n').ok_or(ParseAlmanacErr)?;
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

fn part2(input: &str) -> String {
    // Slight hack -- we are going to reinterpret our almanac vector of seeds as pairs
    // denoting ranges, rather than alter the parsing.
    let almanac: Almanac = input.parse().unwrap();

    assert!(
        almanac.seeds.len() % 2 == 0,
        "Should have an even number of seed entries."
    );

    let n = almanac.seeds.len() / 2;
    let answer = (0..n)
        .map(|i| {
            let seed_start = almanac.seeds[2 * i];
            let seed_range_len = almanac.seeds[2 * i + 1];

            let seed_range = seed_start..(seed_start + seed_range_len);

            almanac
                .lookup_range(seed_range)
                .iter()
                .map(|r| r.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    answer.to_string()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::almanac::{Almanac, RangeMap, RangeMapEntry};
    use crate::{part1, part2};

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
        assert_eq!(part1(EXAMPLE), "35");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "46");
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
