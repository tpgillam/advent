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
    symbols: Vec<Location>,
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
        let mut symbols: Vec<Location> = Vec::new();

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
                    symbols.push(Location { row: i_line, col });
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

fn part1(input: &str) -> String {
    let schematic: Schematic = input.parse().unwrap();

    // By iterating over all symbol extents, we build up a set of locations
    // that we should allow.
    let mut allowed_locations: HashSet<Location> = HashSet::new();
    for extent in schematic.symbols {
        // We validate everything around this extent "diagonally".
        let row_start = if extent.row == 0 { 0 } else { extent.row - 1 };
        let col_start = if extent.col == 0 { 0 } else { extent.col - 1 };

        for row in row_start..(extent.row + 2) {
            for col in col_start..(extent.col + 2) {
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

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example_part1() {
        let example = "
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
        assert_eq!(part1(example), "4361")
    }
}
