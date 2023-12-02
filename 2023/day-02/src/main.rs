use std::str::FromStr;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

// We need to derive from `PartialEq` to support `Eq`.
// We need to derive from `Eq` to support checking in tests.
// We need to derive from `Debug` to support
#[derive(Debug, PartialEq, Eq)]
struct CubeCount {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct ParseCubeCountError;

impl FromStr for CubeCount {
    type Err = ParseCubeCountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;

        for tok in s.split(", ") {
            let (count, colour) = tok.split_once(' ').ok_or(ParseCubeCountError)?;
            let i_count: u32 = count.parse().map_err(|_| ParseCubeCountError)?;
            match colour {
                "red" => r += i_count,
                "green" => g += i_count,
                "blue" => b += i_count,
                _ => return Err(ParseCubeCountError),
            }
        }

        Ok(CubeCount {
            red: r,
            green: g,
            blue: b,
        })
    }
}


fn part1(_input: &str) -> String {
    return "moo".to_string();
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part1, CubeCount, ParseCubeCountError};

    #[test]
    fn cube_count_from_string() {
        assert_eq!(
            CubeCount::from_str("3 blue, 4 red"),
            Ok(CubeCount {
                red: 4,
                green: 0,
                blue: 3
            })
        );
        assert_eq!(
            CubeCount::from_str("3 green, 4 blue, 1 red"),
            Ok(CubeCount {
                red: 1,
                green: 3,
                blue: 4
            })
        );
        assert_eq!(CubeCount::from_str("bad"), Err(ParseCubeCountError));
    }

    #[test]
    fn part_1() {
        let example = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(part1(example), "moo");
    }
}
