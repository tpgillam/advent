use std::collections::HashSet;

use ndarray::{concatenate, Array, Axis, Ix2};

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

type Pipes = Array<u8, Ix2>;

/// Convert the input to an ndarray
fn get_pipes(input: &str) -> Pipes {
    // Get a vector of row arrays.
    let rows = input
        .trim()
        .lines()
        .map(|line| Array::from_vec(line.as_bytes().to_vec()).insert_axis(Axis(0)))
        .collect::<Vec<_>>();

    // Concatenate the result into a single array.
    concatenate(
        Axis(0),
        rows.iter().map(|x| x.view()).collect::<Vec<_>>().as_slice(),
    )
    .unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn apply(&self, pipes: &Pipes, location: (usize, usize)) -> Option<(usize, usize)> {
        let (i, j) = location;
        let (ni, nj) = pipes.dim();

        use Direction::*;
        match self {
            North => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            South => {
                if i == ni {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            East => {
                if j == nj {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            West => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
        }
    }

    fn inverse(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(Debug)]
struct Start {
    location: (usize, usize),
}

#[derive(Debug)]
enum Cell {
    Pipe {
        location: (usize, usize),
        directions: (Direction, Direction),
    },
    Start(Start),
    Ground,
}

impl Cell {
    fn new(value: u8, location: (usize, usize)) -> Result<Cell, String> {
        use Direction::{East, North, South, West};
        let directions = match value {
            b'|' => (North, South),
            b'-' => (East, West),
            b'L' => (North, East),
            b'J' => (North, West),
            b'7' => (South, West),
            b'F' => (South, East),
            b'S' => return Ok(Cell::Start(Start { location })),
            b'.' => return Ok(Cell::Ground),
            _ => return Err(format!("Unexpected value: {}", value as char)),
        };
        Ok(Cell::Pipe {
            location,
            directions,
        })
    }
}

/// Find the starting point
fn get_start(pipes: &Pipes) -> Start {
    let location = pipes
        .indexed_iter()
        .filter(|&(_, x)| *x == b'S')
        .map(|(idx, _)| idx)
        .next()
        .unwrap();
    Start { location }
}

fn get_start_directions(pipes: &Pipes, start: &Start) -> (Direction, Direction) {
    // Now infer what piece of pipe this is.

    let all_directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let directions_to_neighbours = all_directions
        .iter()
        .filter(|&&direction| {
            match direction.apply(pipes, start.location) {
                Some(neighbour) => {
                    let cell = Cell::new(pipes[neighbour], neighbour).unwrap();
                    match cell {
                        Cell::Pipe { directions, .. } => {
                            // The direction we took connects back to us if it is a pipe where
                            // either of the ends is pointing in our direction.
                            let (dir0, dir1) = directions;
                            direction == dir0.inverse() || direction == dir1.inverse()
                        }
                        Cell::Start { .. } => panic!("Should not have two starts..."),

                        Cell::Ground { .. } => false,
                    }
                }
                None => false, // Direction led off the map.
            }
        })
        .collect::<Vec<_>>();

    assert!(directions_to_neighbours.len() == 2);
    let dir0 = directions_to_neighbours[0];
    let dir1 = directions_to_neighbours[1];

    (*dir0, *dir1)
}

struct State {
    cell: Cell,
    previous_direction: Direction,
}

fn get_next_state(pipes: &Pipes, state: &State) -> State {
    let ((dir0, dir1), location) = match &state.cell {
        Cell::Pipe {
            directions,
            location,
            ..
        } => (directions, location),
        x => panic!("Cannot get next state for {:?}", x),
    };

    // Pick the 'next' direction so that we don't move back to the previous state.
    let direction = if dir0.inverse() == state.previous_direction {
        dir1
    } else {
        dir0
    };

    let next_location = direction.apply(pipes, *location).unwrap();

    // Unwrap here since we don't expect to _not_ find a pipe connected to this one.
    let cell = Cell::new(pipes[next_location], next_location).unwrap();

    State {
        cell,
        previous_direction: *direction,
    }
}

fn get_loop_length(pipes: &Pipes) -> u32 {
    let start = get_start(pipes);

    let directions = get_start_directions(pipes, &start);

    let mut state = State {
        // Arbitrarily pick a direction to go from the start;
        previous_direction: directions.0.inverse(),
        // Slight hack... we _know_ the pipe under the start now, so use that.
        cell: Cell::Pipe {
            location: start.location,
            directions,
        },
    };

    // Count the number of steps we have taken
    let mut n = 0;
    loop {
        let next_state = get_next_state(pipes, &state);
        n += 1;

        if let Cell::Start(..) = next_state.cell {
            break n;
        }
        state = next_state;
    }
}

fn part1(input: &str) -> u32 {
    let pipes = get_pipes(input);
    let n = get_loop_length(&pipes);

    // n _should_ always be even because we are on a regular grid.
    assert!(n % 2 == 0);

    n / 2
}

fn get_loop_locations(pipes: &Pipes) -> HashSet<(usize, usize)> {
    let start = get_start(pipes);

    let directions = get_start_directions(pipes, &start);

    let mut state = State {
        // Arbitrarily pick a direction to go from the start;
        previous_direction: directions.0.inverse(),
        // Slight hack... we _know_ the pipe under the start now, so use that.
        cell: Cell::Pipe {
            location: start.location,
            directions,
        },
    };

    let mut result: HashSet<(usize, usize)> = HashSet::new();
    // Since we don't have access to the start location in the state, we
    // add it here.
    result.insert(start.location);

    loop {
        let next_state = get_next_state(pipes, &state);

        match next_state.cell {
            Cell::Pipe { location, .. } => result.insert(location),
            Cell::Start(..) => break result,
            Cell::Ground => unreachable!(),
        };
        state = next_state;
    }
}

// Extracting a closure to avoid repetition in the match arms
fn handle_directions(
    loop_locations: &HashSet<(usize, usize)>,
    location: &(usize, usize),
    directions: (Direction, Direction),
    is_inside: &mut bool,
    interior_count: &mut u32,
    north_on_stack: &mut bool,
    south_on_stack: &mut bool,
) {
    let on_loop = loop_locations.contains(location);

    // println!("Thing: {:?}", location);
    // A pipe can be an interior cell if it isn't on the loop.
    if *is_inside && !on_loop {
        // println!("Pipe inside: {:?}", location);
        *interior_count += 1;
    }

    // If this is part of the loop, as we leave we must check whether
    // we need to change the state.
    if on_loop {
        // We _cross_ the pipe if and only if both the directions we came
        // from (West) and are going to (East) are _not_ in the pipe.
        let (dir_0, dir_1) = directions;
        let directions_arr = [dir_0, dir_1];

        // We need to do something to count the number of norths and souths we have come across.
        //  Think about "S bends" (which should change state) vs "U bends" (which should not).
        let has_north = directions_arr.contains(&Direction::North);
        let has_south = directions_arr.contains(&Direction::South);

        if has_north && has_south {
            // We have unambiguously crossed a pipe.
            *is_inside = !(*is_inside);
        } else if has_north {
            if *south_on_stack {
                assert!(!(*north_on_stack));

                // This completes the pipe -- we have crossed!
                *is_inside = !(*is_inside);
                *south_on_stack = false;
            } else if *north_on_stack {
                // We already have a north on the stack -- this resets it, and we do not
                // cross the pipe.
                *north_on_stack = false;
            } else {
                *north_on_stack = true;
            }
        } else if has_south {
            // The inverse logic to the above.
            if *north_on_stack {
                assert!(!(*south_on_stack));

                // This completes the pipe -- we have crossed!
                *is_inside = !(*is_inside);
                *north_on_stack = false;
            } else if *south_on_stack {
                // We already have a south on the stack -- this resets it, and we do not
                // cross the pipe.
                *south_on_stack = false;
            } else {
                *south_on_stack = true;
            }
        }

        // if !directions_arr.contains(&Direction::East) && !directions_arr.contains(&Direction::West)
        // {
        //     *is_inside = !(*is_inside);
        // }
    }
}

fn part2(input: &str) -> u32 {
    let pipes = get_pipes(input);

    // To determine the enclosed area, we must first find which cells are occupied
    // by the loop.
    let loop_locations: HashSet<(usize, usize)> = get_loop_locations(&pipes);

    // We iterate over all locations, recording whether we are inside or outside.
    // Note that we can calculate each row independently.
    let (ni, nj) = pipes.dim();
    (0..ni)
        .map(|i| {
            // As we go along the row, this will record whether or not we are on the 'outside'
            let mut is_inside = false;

            // Internal state for tracking whether we have crossed.
            let mut north_on_stack = false;
            let mut south_on_stack = false;

            // This will record the number of interior cells we have found.
            let mut interior_count: u32 = 0;

            for j in 0..nj {
                let location = (i, j);
                let cell = Cell::new(pipes[location], location).unwrap();

                match cell {
                    Cell::Ground => {
                        if is_inside {
                            // println!("Ground inside: {:?}", location);
                            interior_count += 1
                        }
                    }
                    Cell::Pipe { directions, .. } => handle_directions(
                        &loop_locations,
                        &location,
                        directions,
                        &mut is_inside,
                        &mut interior_count,
                        &mut north_on_stack,
                        &mut south_on_stack,
                    ),
                    Cell::Start(start) => {
                        // The start is just like a pipe, except we need to figure out what the
                        // underlying pipe looks like first.
                        let directions = get_start_directions(&pipes, &start);

                        handle_directions(
                            &loop_locations,
                            &location,
                            directions,
                            &mut is_inside,
                            &mut interior_count,
                            &mut north_on_stack,
                            &mut south_on_stack,
                        )
                    }
                }
            }

            // println!("Interior count: {:?}", interior_count);
            interior_count
        })
        .sum()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1_example1a() {
        let example = "
.....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part1(example), 4);
    }
    #[test]
    fn test_part1_example1b() {
        let example = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(part1(example), 4);
    }
    #[test]
    fn test_part1_example2a() {
        let example = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(example), 8);
    }
    #[test]
    fn test_part1_example2b() {
        let example = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(example), 8);
    }

    #[test]
    fn test_part2_example1() {
        let example = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(example), 4);
    }

    #[test]
    fn test_part2_example2() {
        let example = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(example), 8);
    }

    #[test]
    fn test_part2_example3() {
        let example = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(example), 10);
    }
}
