use itertools::Itertools;
use ndarray::{concatenate, Array, Axis, Ix2};

type Galaxies = Array<u32, Ix2>;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

/// Convert the input to an ndarray
fn get_galaxies(input: &str) -> Galaxies {
    // Get a vector of row arrays.
    let rows = input
        .trim()
        .lines()
        .map(|line| {
            Array::from_iter(line.as_bytes().iter().map(|x| match x {
                b'.' => 0,
                b'#' => 1,
                _ => panic!(),
            }))
            .insert_axis(Axis(0))
        })
        .collect::<Vec<_>>();

    // Concatenate the result into a single array.
    concatenate(
        Axis(0),
        rows.iter().map(|x| x.view()).collect::<Vec<_>>().as_slice(),
    )
    .unwrap()
}

fn double_axis(galaxies: &Galaxies, axis: Axis) -> Galaxies {
    // NOTE: Using "rows" here to correspond to the Axis(0) case, but this
    //  generalised to columns for axis=Axis(1).

    // Iterate over rows, and build output with additional rows as necessary.
    let mut new_rows = Vec::new();
    galaxies.axis_iter(axis).into_iter().for_each(|row| {
        let reshaped_row = row.insert_axis(axis);
        new_rows.push(reshaped_row);
        if row.sum() == 0 {
            new_rows.push(reshaped_row);
        }
    });
    concatenate(axis, new_rows.as_slice()).unwrap()
}

fn get_expanded_galaxies(galaxies: &Galaxies) -> Galaxies {
    double_axis(&double_axis(&galaxies, Axis(0)), Axis(1))
}

type Location = (usize, usize);

fn get_galaxy_locations(galaxies: &Galaxies) -> Vec<Location> {
    galaxies
        .indexed_iter()
        .filter_map(|(i, &x)| if x == 0 { None } else { Some(i) })
        .collect()
}

fn manhattan_distance(a: &Location, b: &Location) -> u32 {
    let x_diff = ((a.0 as i32) - (b.0 as i32)).abs();
    let y_diff = ((a.1 as i32) - (b.1 as i32)).abs();
    // Both x_diff and y_diff will be non-negative due to taking the absolute value.
    (x_diff + y_diff) as u32
}

fn part1(input: &str) -> u32 {
    let galaxies = get_galaxies(input);
    let expanded_galaxies = get_expanded_galaxies(&galaxies);
    let locations = get_galaxy_locations(&expanded_galaxies);

    // Compute the sum of distances between all pairs of galaxies.
    locations
        .iter()
        .combinations(2)
        .map(|x| {
            assert!(x.len() == 2);
            manhattan_distance(x[0], x[1])
        })
        .sum()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const EXAMPLE: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 374);
    }
}
