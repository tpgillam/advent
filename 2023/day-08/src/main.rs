use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

#[derive(Eq, Hash, PartialEq)]
struct Node {
    name: (u8, u8, u8),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (c1, c2, c3) = self.name;
        let node_str = format!("Node({}{}{})", c1 as char, c2 as char, c3 as char);
        f.write_str(&node_str)
    }
}

#[derive(Debug)]
struct ParseNodeErr;

impl FromStr for Node {
    type Err = ParseNodeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.as_bytes().iter();
        let c1 = iter.next().ok_or(ParseNodeErr)?;
        let c2 = iter.next().ok_or(ParseNodeErr)?;
        let c3 = iter.next().ok_or(ParseNodeErr)?;
        if iter.next().is_some() {
            return Err(ParseNodeErr);
        }
        Ok(Node {
            name: (*c1, *c2, *c3),
        })
    }
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, ()> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

struct Map {
    directions: Vec<Direction>,
    connections: HashMap<Node, (Node, Node)>,
}

#[derive(Debug)]
struct ParseMapErr;

impl FromStr for Map {
    type Err = ParseMapErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().lines();
        let direction_str = it.next().ok_or(ParseMapErr)?;
        if !it.next().ok_or(ParseMapErr)?.is_empty() {
            return Err(ParseMapErr);
        }

        let mut connections: HashMap<Node, (Node, Node)> = HashMap::new();
        for connection_str in it {
            let (node_from_str, rest) = connection_str.split_once(" = ").ok_or(ParseMapErr)?;
            let (node_l_str, node_r_str) = rest
                .strip_prefix('(')
                .ok_or(ParseMapErr)?
                .strip_suffix(')')
                .ok_or(ParseMapErr)?
                .split_once(", ")
                .ok_or(ParseMapErr)?;
            let node_from: Node = node_from_str.parse().map_err(|_| ParseMapErr)?;
            let node_l: Node = node_l_str.parse().map_err(|_| ParseMapErr)?;
            let node_r: Node = node_r_str.parse().map_err(|_| ParseMapErr)?;
            connections.insert(node_from, (node_l, node_r));
        }

        let directions = direction_str
            .chars()
            .map(|c| Direction::from_char(c).map_err(|_| ParseMapErr))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Map {
            directions,
            connections,
        })
    }
}

fn make_move<'a>(map: &'a Map, start: &Node, direction: &Direction) -> &'a Node {
    let (node_l, node_r) = map.connections.get(start).unwrap();
    match direction {
        Direction::Left => node_l,
        Direction::Right => node_r,
    }
}

fn part1(input: &str) -> u32 {
    let map: Map = input.parse().unwrap();

    let start = Node::from_str("AAA").unwrap();
    let finish = Node::from_str("ZZZ").unwrap();

    // The current node in our journey.
    let mut node = &start;

    // An infinitely repeating iterator over directions.
    let mut it_directions = map.directions.iter().cycle();

    // How many steps we have taken
    let mut n: u32 = 0;

    while *node != finish {
        node = make_move(&map, node, it_directions.next().unwrap());
        n += 1
    }

    n
}

fn is_start_node(node: &Node) -> bool {
    matches!(node.name, (_, _, b'A'))
}

fn is_finish_node(node: &Node) -> bool {
    matches!(node.name, (_, _, b'Z'))
}

#[derive(Debug)]
struct Cycle {
    // The steps in at which finishes were identified.
    // NOTE: if any of these are before `cycle_state_first`, then they will be hit
    //  exactly once.
    finish_node_steps: Vec<usize>,

    // Define the "cycle state" to be the (direction index, node) pair that
    // denotes the start of a loop in the process.
    cycle_state_first: usize,

    // This is the step at which we encountered the state for the second time.
    cycle_state_second: usize,
}

fn find_cycle(map: &Map, start: &Node) -> Cycle {
    // An infinitely repeating iterator over directions.
    let mut it_directions = map.directions.iter().enumerate().cycle();

    let mut current_node: &Node = start;

    // Keep track of states that we have seen previously. We also want to know _when_
    // we came across them. Note that we use a HashSet because we are going to have to
    // test for element presence on every step, so need it to be fast.
    // Each state is a tuple of the node & index of the next direction that we will take.
    let mut seen_states: HashMap<(usize, &Node), usize> = HashMap::new();

    // These are the number of steps it took to get to any finish node.
    // Since we may reach multiple finish nodes before completing the cycle (or the same node
    // multiple times), this should end up with length >= 1.
    let mut finish_node_steps: Vec<usize> = Vec::new();

    // This stores the number of steps we have taken so far.
    let mut i_step: usize = 0;

    loop {
        // Determine the next direction
        let (i_direction, direction) = it_directions.next().unwrap();

        let key = (i_direction, current_node);
        match seen_states.get(&key) {
            Some(&cycle_state_first) => {
                // We have completed the cycle! Package everything into a Cycle
                return Cycle {
                    finish_node_steps,
                    cycle_state_first,
                    cycle_state_second: i_step,
                };
            }
            None => {
                // We are at a state that we haven't seen before, so record the state.
                seen_states.insert(key, i_step);

                // Is this a finish node? If so keep track of how many steps it took to get here.
                if is_finish_node(current_node) {
                    finish_node_steps.push(i_step);
                }
            }
        }

        // Advance to the next state.
        current_node = make_move(map, current_node, direction);
        i_step += 1;
    }
}

#[derive(Debug)]
struct SimpleCycle {
    finish_node_step: usize,
    cycle_state_first: usize,
    cycle_state_second: usize,
}

impl SimpleCycle {
    fn from_cycle(cycle: &Cycle) -> Result<Self, String> {
        if cycle
            .finish_node_steps
            .iter()
            .any(|&x| x < cycle.cycle_state_first)
        {
            return Err(format!(
                "There are finishes before the cycle starts: {cycle:?}",
            ));
        }

        match cycle.finish_node_steps.len() {
            1 => {
                let finish_node_step = cycle.finish_node_steps[0];

                Ok(SimpleCycle {
                    finish_node_step,
                    cycle_state_first: cycle.cycle_state_first,
                    cycle_state_second: cycle.cycle_state_second,
                })
            }
            2 => {
                // We _might_ be able to handle this if the cycle can be simplified.
                let length = cycle.cycle_state_second - cycle.cycle_state_first;
                let x0 = cycle.finish_node_steps[0] - cycle.cycle_state_first;
                let x1 = cycle.finish_node_steps[1] - cycle.cycle_state_first;
                if (x0 == x1 / 2) && (length % 2 == 0) {
                    Ok(SimpleCycle {
                        finish_node_step: cycle.finish_node_steps[0],
                        cycle_state_first: cycle.cycle_state_first,
                        cycle_state_second: cycle.cycle_state_first + length / 2,
                    })
                } else {
                    Err(format!("Can't simplify cycle: {cycle:?}",))
                }
            }
            _ => Err(format!("Can't simplify cycle: {cycle:?}",)),
        }
    }
}

/// Return the number of steps that it would take to finish, given
/// the extracted cycles.
fn completion_steps(cycles: &[Cycle]) -> usize {
    // NOTE: This is a little disgusting, as it required manual inspection of
    //  the problem input, but the following useful additional properties were true:
    //
    //      - no potential finishes are encountered before entering the cycle
    //      - exactly one finish was found in each cycle
    //      - a global rotation can be performed such that ALL cycles have the finish
    //          as their first element.
    //
    //  Therefore, we can convert this into an LCM problem. First we need to assert
    //  that the above is true.
    let simple_cycles: Vec<_> = cycles
        .iter()
        .map(|cycle| SimpleCycle::from_cycle(cycle).unwrap())
        .collect();

    // dbg!(&cycles);
    // dbg!(&simple_cycles);

    // We next re-index the cycles such that they all start simultaneously.
    let offset = simple_cycles
        .iter()
        .map(|cycle| cycle.cycle_state_first)
        .max()
        .unwrap();

    // Each element is a tuple of (finish location, cycle length)
    let finish_lengths: Vec<_> = simple_cycles
        .iter()
        .map(|cycle| {
            // This represents the number of nodes in the cycle. It will be positive.
            let cycle_length = cycle.cycle_state_second - cycle.cycle_state_first;

            let raw_finish_pos = cycle.finish_node_step - cycle.cycle_state_first;

            // This is the amount of extra distance we want to move into the cycle.
            let rotation_distance = offset - cycle.cycle_state_first;

            let finish_step = if rotation_distance > raw_finish_pos {
                raw_finish_pos + cycle_length - rotation_distance
            } else {
                raw_finish_pos - rotation_distance
            };

            (finish_step, cycle_length)
        })
        .collect();
    // dbg!(&finish_lengths);

    // Now we try to to find an additional rotation FORWARDS that would put the end node to the
    // start of the cycle.
    let additional_offsets = finish_lengths
        .iter()
        .map(|(finish_step, cycle_length)| cycle_length - finish_step)
        .collect::<HashSet<_>>();
    assert!(additional_offsets.len() == 1);
    let additional_offset = additional_offsets.iter().next().unwrap();

    // println!("Additional offset: {}", additional_offset);

    // The length of each cycle is now the full set of required information.
    let lengths: Vec<_> = finish_lengths.iter().map(|(_, length)| *length).collect();
    // dbg!(&lengths);

    // The number of steps we need to take now is the Lowest Common Multiple of all the
    // lengths.
    let lowest_common_multiple = lengths
        .into_iter()
        .map(|x| x as i64)
        .reduce(num::integer::lcm)
        .unwrap() as usize;
    // println!("LCM: {}", lowest_common_multiple);

    // Remember that we need to subtract the _additional_ offset to get the actual answer.
    offset + lowest_common_multiple - additional_offset
}

fn part2(input: &str) -> u64 {
    let map: Map = input.parse().unwrap();

    // Find all starting nodes we will then update these until we
    // reach the ending state.
    let starting_nodes: Vec<_> = map
        .connections
        .keys()
        .filter(|&n| is_start_node(n))
        .collect();

    // Whilst technically correct, the brute force approach is VERY slow for the full input.
    //
    //  We can be smarter -- we can trace each individual starting point, and then we have
    //  to identify each _potential_ finishing point before we form a cycle. To be sure that we
    //  have found a cycle, we need to be at the SAME NODE and also at the SAME POINT in the
    //  directions list as one that we have seen previously. This does seem quite fiddly though...
    //
    //  Once you have this, you could perform a bit of prime factor bashing to get the lowest
    //  common multiple of all periods  (the difficulty being that each starting point could have
    //  _multiple_ periods, and also potentially have an offset before the cycle starts).
    //
    // NOTE: Other observations from full input:
    //  - None of the starting nodes are ever referenced in the connectivity graph... so cycles
    //      definitely do NOT include the starting node. The same is true in the example.

    // OK. Let's try to find the cycles.
    let cycles: Vec<_> = starting_nodes.iter().map(|n| find_cycle(&map, n)).collect();

    completion_steps(&cycles) as u64
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE_1: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(EXAMPLE_1), 2);
        assert_eq!(part1(EXAMPLE_2), 6);
    }

    const EXAMPLE_3: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_3), 6)
    }
}
