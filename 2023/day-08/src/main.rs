use std::{collections::HashMap, str::FromStr};

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node {
    name: (u8, u8, u8),
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

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part1;

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
}
