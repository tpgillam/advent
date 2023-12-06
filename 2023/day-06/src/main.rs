fn get_input() -> &'static str {
    include_str!("../input.txt")
}

mod race {

    use std::str::FromStr;

    #[derive(Debug)]
    pub struct Race {
        pub duration: u64,
        pub distance_record: u64,
    }

    #[derive(Debug)]
    pub struct Sheet {
        pub races: Vec<Race>,
    }

    #[derive(Debug)]
    pub struct ParseSheetErr;

    fn parse_line(s: &str) -> Result<Vec<u64>, ParseSheetErr> {
        let (_, rest) = s.split_once(':').ok_or(ParseSheetErr)?;
        rest.split_whitespace()
            .map(|x| x.parse::<u64>().map_err(|_| ParseSheetErr))
            .collect()
    }

    impl FromStr for Sheet {
        type Err = ParseSheetErr;

        fn from_str(s: &str) -> Result<Sheet, Self::Err> {
            let (times_line, distances_line) = s.trim().split_once('\n').ok_or(ParseSheetErr)?;
            let times = parse_line(times_line)?;
            let distances = parse_line(distances_line)?;

            let races: Vec<_> = times
                .into_iter()
                .zip(distances)
                .map(|(duration, distance_record)| Race {
                    duration,
                    distance_record,
                })
                .collect();
            Ok(Sheet { races })
        }
    }
}

fn distance(race: &race::Race, hold_time: u64) -> u64 {
    if hold_time == 0 || hold_time >= race.duration {
        return 0;
    };
    hold_time * (race.duration - hold_time)
}

fn num_ways_to_win(race: &race::Race) -> u64 {
    // PERF: There is definitely a closed-form solution to this...
    let hold_times = 0..race.duration;
    hold_times
        .filter(|&hold_time| distance(race, hold_time) > race.distance_record)
        .count() as u64
}

fn part1(input: &str) -> String {
    let sheet: race::Sheet = input.parse().unwrap();
    let answer: u64 = sheet.races.iter().map(num_ways_to_win).product();
    answer.to_string()
}

fn part2(input: &str) -> String {
    // Slight hack so that we parse correctly...
    let sheet: race::Sheet = input.replace(' ', "").parse().unwrap();
    let answer = if let [race] = sheet.races.as_slice() {
        num_ways_to_win(race)
    } else {
        panic!("Expected exactly one race!");
    };
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
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "288");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "71503");
    }
}
