fn get_input() -> &'static str {
    include_str!("../input.txt")
}

/// Take the difference of adjacent elements, and return as a vector.
/// if the input `sequence` has length n, the result will have length n-1
fn diff(sequence: &[i64]) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn extrapolate(sequence: &[i64]) -> i64 {
    // Build up the differences.
    let mut stack: Vec<Vec<i64>> = vec![sequence.to_vec()];
    while !stack.last().unwrap().iter().all(|x| *x == 0) {
        stack.push(diff(stack.last().unwrap()))
    }

    // To extrapolate, we sum up the last digits of all the series.
    stack.iter().map(|x| x.last().unwrap()).sum::<i64>()
}

fn part1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let sequence: Vec<_> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            extrapolate(&sequence)
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
