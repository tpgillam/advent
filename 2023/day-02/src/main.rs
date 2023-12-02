use std::str::FromStr;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

// We need to derive from `PartialEq` to support `Eq`.
// We need to derive from `Eq` to support checking in tests.
// We need to derive from `Debug` to support
#[derive(Debug, PartialEq, Eq, Clone)]
struct CubeCount {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    // NOTE: We want Game to be able to own its data. I initially tried
    //  a &[CubeCount], which might sometimes be beneficial, but the problem
    //  is that _something_ needs to own the data when we are parsing from a str.
    cube_counts: Vec<CubeCount>,
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

#[derive(Debug, PartialEq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_str, cube_counts_str) = s.split_once(": ").ok_or(ParseGameError)?;
        let id: u32 = game_str
            .replace("Game ", "")
            .parse()
            .map_err(|_| ParseGameError)?;

        // Some slight magic going on here.
        //  - after the `map` call we get an iterator whose Item is a Result.
        //  - when we do `collect`, we are coercing into a single `Result` whose element
        //      is a `Vec`.
        //  - we can then convert the error type and use the `?`-fast-return operator as usual.
        let cube_counts = cube_counts_str
            .split("; ")
            .map(|tok| CubeCount::from_str(tok))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseGameError)?;

        // Yay happy path.
        Ok(Game { id, cube_counts })
    }
}

fn part1(input: &str) -> String {
    let answer: u32 = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let game = Game::from_str(line).unwrap();
            let is_possible = game
                .cube_counts
                .iter()
                .all(|x| x.red <= 12 && x.green <= 13 && x.blue <= 14);

            if is_possible {
                game.id
            } else {
                0
            }
        })
        .sum();

    return answer.to_string();
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part1, CubeCount, Game, ParseCubeCountError, ParseGameError};

    #[test]
    fn cube_count_from_str() {
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
    fn game_from_str() {
        assert_eq!(
            Game::from_str("Game 42: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(Game {
                id: 42,
                cube_counts: [
                    CubeCount {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    CubeCount {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    CubeCount {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
                .to_vec()
            }),
        );
        assert_eq!(Game::from_str("Game xx: 3 blue"), Err(ParseGameError));
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
        assert_eq!(part1(example), "8");
    }
}
