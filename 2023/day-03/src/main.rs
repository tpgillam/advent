use std::{collections::HashSet, str::FromStr};

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

// Represent a (row, col) location in the schematic.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Extent {
    row: usize,
    col_begin: usize, // Inclusive
    col_end: usize,   // Exclusive
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<(Extent, u32)>,
    symbols: Vec<(Location, char)>,
}

#[derive(Debug)]
struct ParseSchematicError;

// Flush the current number & symbols.
fn flush_current_number(
    row: usize,
    current_number_col: usize,
    current_number: &mut Vec<char>,
    numbers: &mut Vec<(Extent, u32)>,
) -> Result<(), ParseSchematicError> {
    if current_number.is_empty() {
        return Ok(());
    }

    // If there is a problem then this will return.
    let number = current_number
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .map_err(|_| ParseSchematicError)?;

    numbers.push((
        Extent {
            row,
            col_begin: current_number_col,
            col_end: current_number_col + current_number.len(),
        },
        number,
    ));

    // Empty the current number.
    current_number.clear();

    Ok(())
}

impl FromStr for Schematic {
    type Err = ParseSchematicError;
    //

    fn from_str(s: &str) -> Result<Schematic, ParseSchematicError> {
        // First we extract all numbers, and their start and end locations, and all symbols and their
        // locations.
        // NOTE: We don't care about what the symbols are, just their locations.
        let mut numbers: Vec<(Extent, u32)> = Vec::new();
        let mut symbols: Vec<(Location, char)> = Vec::new();

        for (i_line, line) in s.trim().lines().enumerate() {
            // We are looking for contiguous runs of digits, which we will parse
            // as an integer.
            // '.' is a separator.
            // Any other characters are "symbols", which we need to keep track of separately.
            let mut current_number_col: usize = 0;
            let mut current_number: Vec<char> = Vec::new();

            for (col, x) in line.chars().enumerate() {
                if x == '.' {
                    // A dot should flush the current number, but otherwise
                    // be ignored.
                    flush_current_number(
                        i_line,
                        current_number_col,
                        &mut current_number,
                        &mut numbers,
                    )?;
                } else if x.is_ascii_digit() {
                    let start_of_number = current_number.is_empty();
                    if start_of_number {
                        // Set where the number starts iff this is a new integer.
                        current_number_col = col;
                    }
                    // Always append the latest seen digit.
                    current_number.push(x);
                } else {
                    // A symbol should cause the current number to be flushed.
                    flush_current_number(
                        i_line,
                        current_number_col,
                        &mut current_number,
                        &mut numbers,
                    )?;
                    symbols.push((Location { row: i_line, col }, x));
                }
            }

            // Any digits left on the stack should be flushed.
            flush_current_number(
                i_line,
                current_number_col,
                &mut current_number,
                &mut numbers,
            )?;
        }

        return Ok(Schematic { numbers, symbols });
    }
}

// This was used for debugging, so don't warn about the fact that it is unused.
#[allow(dead_code)]
fn render_locations(locations: &HashSet<Location>) -> String {
    let n_rows = locations.iter().map(|x| x.row).max().unwrap() + 1;
    let n_cols = locations.iter().map(|x| x.col).max().unwrap() + 1;

    let mut output_lines: Vec<Vec<char>> = vec![vec!['.'; n_cols]; n_rows];

    for location in locations.iter() {
        output_lines[location.row][location.col] = '*';
    }

    // NOTE: The `collect` is required because `join` requires a collection.
    output_lines
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

// One before x... but x if x == 0
fn one_before<T>(x: T) -> T
where
    T: std::ops::Sub<Output = T> + From<u8> + PartialEq,
{
    if x == 0.into() {
        x
    } else {
        x - 1.into()
    }
}

fn part1(input: &str) -> String {
    let schematic: Schematic = input.parse().unwrap();

    // By iterating over all symbol extents, we build up a set of locations
    // that we should allow.
    let mut allowed_locations: HashSet<Location> = HashSet::new();
    for (location, _) in schematic.symbols {
        // We validate everything around this location "diagonally".
        let row_start = one_before(location.row);
        let col_start = one_before(location.col);

        for row in row_start..(location.row + 2) {
            for col in col_start..(location.col + 2) {
                allowed_locations.insert(Location { row, col });
            }
        }
    }

    // Now we filter out the numbers that are allowed and sum them.
    let answer: u32 = schematic
        .numbers
        .iter()
        .filter(|(extent, _)| {
            (extent.col_begin..extent.col_end).into_iter().any(|col| {
                let location = Location {
                    row: extent.row,
                    col,
                };
                allowed_locations.contains(&location)
            })
        })
        .map(|(_, number)| number)
        .sum();

    answer.to_string()
}

// Return true iff `location` is adjacent to `Extent`.
fn is_adjacent(extent: &Extent, location: &Location) -> bool {
    if one_before(location.row) > extent.row {
        return false;
    }
    if (location.row + 1) < extent.row {
        return false;
    }

    // NOTE: that we include equality in this case because
    //  `col_end` is a non-inclusive upper bound (i.e. indicates the column
    //  after the number has finished.)
    if one_before(location.col) >= extent.col_end {
        return false;
    }
    if (location.col + 1) < extent.col_begin {
        return false;
    }

    true
}

fn part2(input: &str) -> String {
    // Parse the schematic as for part 1.
    let schematic: Schematic = input.parse().unwrap();

    // Now we need to identify any 'gears'; that is a '*' which has exactly two numbers
    // adjacent to it.
    let answer: u32 = schematic
        .symbols
        .iter()
        .filter(|&&(_, c)| c == '*')
        .map(|(location, _)| -> Option<u32> {
            // At this point we have the location of a potential gear symbol.
            // We hope to find exactly two adjacent numbers...
            // PERF:  We really need to have some form of acceleration structure to avoid an O(N)
            //  scan over all known numbers for every symbol.
            let adjacent_numbers: Vec<u32> = schematic
                .numbers
                .iter()
                .filter(|(extent, _)| is_adjacent(extent, location))
                .map(|&(_, number)| number)
                .collect();
            if adjacent_numbers.len() == 2 {
                // This is a gear!
                Some(adjacent_numbers.iter().product::<u32>())
            } else {
                // Incorrect number of adjacent numbers.. not a gear.
                None
            }
        })
        .filter_map(|x| x)
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

    const EXAMPLE: &'static str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), "4361")
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), "467835")
    }
}
