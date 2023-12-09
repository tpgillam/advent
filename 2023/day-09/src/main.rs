fn get_input() -> &'static str {
    include_str!("../input.txt")
}

/// Take the difference of adjacent elements, and return as a vector.
/// if the input `sequence` has length n, the result will have length n-1
fn diff(sequence: &[u64]) -> Vec<u64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn extrapolate(sequence: &[u64]) -> u64 {
    // Build up the differences.
    let mut stack: Vec<Vec<u64>> = Vec::new();
    while stack.len() == 0 || !stack.last().unwrap().iter().all(|x| *x == 0) {
        stack.push(diff(stack.last().unwrap()))
    }

    // Now extrapolate...
    let mut x = 0u64;
    while !stack.is_empty() {
        let moo = stack.pop().unwrap();
        // FIXME:
        // FIXME:
        // FIXME:
        // FIXME:
        // FIXME:
    }
    todo!();
}

fn part1(input: &str) -> u64 {
    todo!()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{extrapolate, part1};

    const EXAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 114);
    }
}
