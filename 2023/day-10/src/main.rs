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
    let start = get_start(&pipes);

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
        let next_state = get_next_state(&pipes, &state);
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

fn part2(input: &str) -> u32 {
    let pipes = get_pipes(input);

    todo!()
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
