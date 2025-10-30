fn get_input() -> &'static str {
    include_str!("../input.txt")
}

/// Take the difference of adjacent elements, and return as a vector.
/// if the input `sequence` has length n, the result will have length n-1
fn diff(sequence: &[i64]) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

/// Build up a vector of differences, including the parent sequence.
fn make_diff_stack(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut stack: Vec<Vec<i64>> = vec![sequence.to_vec()];
    while !stack.last().unwrap().iter().all(|x| *x == 0) {
        stack.push(diff(stack.last().unwrap()));
    }
    stack
}

fn extrapolate_forward(sequence: &[i64]) -> i64 {
    let stack = make_diff_stack(sequence);

    // To extrapolate, we sum up the last digits of all the series.
    stack.iter().map(|x| x.last().unwrap()).sum::<i64>()
}

fn extrapolate_backward(sequence: &[i64]) -> i64 {
    let stack = make_diff_stack(sequence);

    // To extrapolate backwards, we must sum the _first_ digits multiplied
    // by the alternating sequence +1, -1, +1, ...
    // i.e., if we are given the sequence a_1, a_2, ... , and:
    //  - first differences are b_1, b_2, ... where b_1 = a_2 - a_1.
    //  - second differences are c_1, c_2, ... where c_1 = b_2 - b_1.
    //  etc.
    //
    //  Then the result a_0 = a_1 - b_1 + c_1 - d_1 + ...
    [1, -1]
        .iter()
        .cycle()
        .zip(stack.iter())
        .map(|(coefficient, x)| coefficient * x.first().unwrap())
        .sum()
}

fn sum_extrapolation<F>(input: &str, extrapolate: F) -> i64
where
    F: Fn(&[i64]) -> i64,
{
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

fn part1(input: &str) -> i64 {
    sum_extrapolation(input, extrapolate_forward)
}

fn part2(input: &str) -> i64 {
    sum_extrapolation(input, extrapolate_backward)
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{extrapolate_backward, extrapolate_forward, part1, part2};

    const EXAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_extrapolate_forward() {
        assert_eq!(extrapolate_forward(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate_forward(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate_forward(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 114);
    }

    #[test]
    fn test_extrapolate_backward() {
        assert_eq!(extrapolate_backward(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(extrapolate_backward(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(extrapolate_backward(&[10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 2);
    }
}
