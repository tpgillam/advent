use std::str::FromStr;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

struct Springs {
    pattern: String,
    required: Vec<usize>,
}

#[derive(Debug)]
struct ParseSpringsErr;

impl FromStr for Springs {
    type Err = ParseSpringsErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, groups_str) = s.split_once(' ').ok_or(ParseSpringsErr)?;
        Ok(Springs {
            pattern: pattern.to_string(),
            required: groups_str
                .split(',')
                .map(|x| x.parse::<usize>().map_err(|_| ParseSpringsErr))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// Assign `m` things to `n` groups, such that the _order_ of the `m` things is
/// unchanged. Groups are allowed to be empty.
///
/// We return a vector of partitions. Each partition is a vector of length `m`.
/// The value at index `i` of this partition is a value in `0..n` -- this represents the
/// index of the group to which thing `i` should be assigned.
// PERF: Could make this return an iterator rather than realise a vector?
fn ordered_partitions(m: usize, n: usize) -> Vec<Vec<usize>> {
    if m == 0 {
        // No input elements, so nothing to do
        return vec![];
    }

    if n == 0 {
        // XXX: This is disgusting.. probably should return a Result?
        panic!()
    }

    let mut result: Vec<Vec<usize>> = Vec::new();

    // For each thing, this records the group index;
    let mut state: Vec<usize> = vec![0; m];

    result.push(state.clone());

    loop {
        if state[0] == n - 1 {
            // We have reached the last state, so there is nothing more to do.
            break;
        }

        // we are aiming for the order:
        // 0 0 0
        // 0 0 1
        // 0 1 1
        // 1 1 1
        // 0 0 2
        // 0 1 2
        // 1 1 2
        // 0 2 2
        // 1 2 2
        // 2 2 2
        // 1 2 2
        // ...

        // We start at the first element of the state which is less than the element
        // to its right, or else the last state.
        let mut i: usize = 0;
        while i < m {
            if i == m - 1 {
                // Last element; nothing more to check.
                break;
            }
            if state[i] < state[i + 1] {
                // This element can be incremented.
                break;
            }
            i += 1;
        }

        // NOTE: We intialize this to 1 + the index, since it will be decremented as
        //  the first step in the while loop. This allows us to have a slightly neater
        //  loop condition, since i is unsigned.
        i += 1;

        while i > 0 {
            // Move to look at the next item.
            // (i is initialized beyond the range of the array, so we should always do this first).
            i -= 1;

            if state[i] < n - 1 {
                // Increment the state at this location.
                state[i] += 1;

                // Set everything before this to zero.
                for j in 0..i {
                    state[j] = 0;
                }
            }
            result.push(state.clone());
        }
    }

    result
}

fn is_valid(is_mandatory: &[bool], required: &[usize], i_starts: &[usize]) -> bool {
    let n = is_mandatory.len();

    let mut is_present: Vec<_> = vec![false; n];
    for (i_group, &i_start) in i_starts.iter().enumerate() {
        for j in i_start..(i_start + required[i_group]) {
            if j >= n {
                // We have gone out of bounds!
                return false;
            }
            is_present[j] = true;
        }
    }

    is_mandatory
        .into_iter()
        .zip(is_present)
        .all(|(mandatory, present)| present || !mandatory)
}

/// Compute the number of ways of fitting the given `required` groups
/// into `pattern`, which is a string comprising only '#' and '?'.
fn num_arrangements_in_group(pattern: &str, required: &[usize]) -> Option<usize> {
    let m = required.len();
    let n = pattern.len();

    let is_mandatory: Vec<bool> = pattern
        .as_bytes()
        .into_iter()
        .map(|x| match x {
            b'?' => false,
            b'#' => true,
            _ => panic!("Unexpected byte: {x}"),
        })
        .collect();

    let available_length = n;
    let mandatory_length = is_mandatory.iter().map(|&x| x as usize).sum();

    if m == 0 {
        // If we are not required to fit anything in, we define there to be exactly one
        // way to do this -- unless we are mandatorily required to fit something here, in
        // which case there are no matches.
        return if mandatory_length == 0 { Some(1) } else { None };
    }

    // The total requested length needs to include gaps in between the groups.
    let requested_length = required.iter().sum::<usize>() + m - 1;

    if (requested_length > available_length) || (requested_length < mandatory_length) {
        // Some obvious cases that will not work.
        return None;
    }

    // Initialise the state to represent all the required groups as far left as
    // possible.
    let i_starts: Vec<usize> = (0..m)
        .map(|i| match i {
            0 => 0,
            _ => i + required[0..i].iter().sum::<usize>(),
        })
        .collect();

    // Now we are going to add offsets to all of the starting points, up to some maximum value.
    let maximum_offset = available_length - requested_length;
    let count = ordered_partitions(m, maximum_offset + 1)
        .into_iter()
        .filter_map(|start_offsets| {
            let this_starts: Vec<_> = i_starts
                .iter()
                .zip(start_offsets)
                .map(|(x, y)| x + y)
                .collect();
            if is_valid(&is_mandatory, required, &this_starts) {
                Some(1)
            } else {
                None
            }
        })
        .sum::<usize>();

    if count == 0 {
        None
    } else {
        Some(count)
    }
}

fn num_arrangements(line: &str) -> usize {
    let springs: Springs = line.parse().unwrap();

    // These are the pattern groups that we have been given.
    let pattern_groups: Vec<_> = springs
        .pattern
        .split('.')
        .filter_map(|x| match x {
            "" => None,
            _ => Some(x),
        })
        .collect();

    // These are the lengths of those groups to use for our first pass.
    let pattern_group_lengths: Vec<_> = pattern_groups.iter().map(|g| g.len()).collect();

    let m = springs.required.len();
    let n = pattern_group_lengths.len();
    let partitions: Vec<_> = ordered_partitions(m, n)
        .into_iter()
        .filter(|partition| {
            // assert_eq!(partition.len(), m)
            // Each element of partition is an index in 0..n

            let mut pattern_totals = vec![0usize; n];
            for (&required_length, &i_pattern_group) in
                springs.required.iter().zip(partition.iter())
            {
                if pattern_totals[i_pattern_group] > 0 {
                    // This is the additional padding required between adjacent spring groups.
                    pattern_totals[i_pattern_group] += 1;
                }
                pattern_totals[i_pattern_group] += required_length as usize;
                if pattern_totals[i_pattern_group] > pattern_group_lengths[i_pattern_group] {
                    // break out as soon as we realise that this possibility is not viable.
                    return false;
                }
            }

            // OK... this is maybe viable! Now we need to check the possibility in more detail.
            return true;
        })
        .collect();

    partitions
        .into_iter()
        .filter_map(|partition| {
            // Store a list of required groups for each pattern
            let mut pattern_required: Vec<Vec<usize>> = vec![vec![]; n];
            for (i_required, &i_pattern) in partition.iter().enumerate() {
                pattern_required[i_pattern].push(springs.required[i_required]);
            }

            let mut count: usize = 1;

            for (i_pattern, required_subset) in pattern_required.into_iter().enumerate() {
                // Determine if the partition works with the precise arrangement that has been given.
                match num_arrangements_in_group(
                    pattern_groups[i_pattern],
                    required_subset.as_slice(),
                ) {
                    None => return None,
                    Some(this_count) => count *= this_count,
                }
            }
            Some(count)
        })
        .sum()
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| num_arrangements(line))
        .sum()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    // println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{num_arrangements, num_arrangements_in_group, ordered_partitions, part1};

    const EXAMPLE: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_ordered_partitions() {
        assert_eq!(ordered_partitions(0, 1), Vec::<Vec<usize>>::new());
        assert_eq!(ordered_partitions(1, 1), vec![vec![0]]);
        assert_eq!(ordered_partitions(2, 1), vec![vec![0, 0]]);
        assert_eq!(ordered_partitions(1, 2), vec![vec![0], vec![1]]);
        assert_eq!(ordered_partitions(1, 3), vec![vec![0], vec![1], vec![2]]);
        assert_eq!(
            ordered_partitions(2, 3),
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 1],
                vec![0, 2],
                vec![1, 2],
                vec![2, 2]
            ]
        );
        assert_eq!(
            ordered_partitions(3, 3),
            vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 1],
                vec![1, 1, 1],
                vec![0, 0, 2],
                vec![0, 1, 2],
                vec![1, 1, 2],
                vec![0, 2, 2],
                vec![1, 2, 2],
                vec![2, 2, 2],
            ]
        );
    }

    #[test]
    fn test_num_arrangements_in_group() {
        assert_eq!(num_arrangements_in_group("?", &[]), Some(1));
        assert_eq!(num_arrangements_in_group("?", &[1]), Some(1));
        assert_eq!(num_arrangements_in_group("?", &[2]), None);

        assert_eq!(num_arrangements_in_group("#", &[]), None);
        assert_eq!(num_arrangements_in_group("#", &[1]), Some(1));
        assert_eq!(num_arrangements_in_group("#", &[2]), None);

        assert_eq!(num_arrangements_in_group("??", &[]), Some(1));
        assert_eq!(num_arrangements_in_group("??", &[1]), Some(2));
        assert_eq!(num_arrangements_in_group("??", &[2]), Some(1));
        assert_eq!(num_arrangements_in_group("??", &[1, 1]), None);

        assert_eq!(num_arrangements_in_group("?#", &[]), None);
        assert_eq!(num_arrangements_in_group("?#", &[1]), Some(1));
        assert_eq!(num_arrangements_in_group("?#", &[2]), Some(1));

        assert_eq!(num_arrangements_in_group("???", &[1, 1]), Some(1));
        assert_eq!(num_arrangements_in_group("???", &[2]), Some(2));
        assert_eq!(num_arrangements_in_group("???", &[2, 1]), None);
        assert_eq!(num_arrangements_in_group("##?", &[2]), Some(1));
        assert_eq!(num_arrangements_in_group("#?#", &[2]), None);
    }

    #[test]
    fn test_num_arrangements() {
        // Custom, slightly more interesting, test.
        assert_eq!(num_arrangements("???.??? 1,1,1"), 6);

        assert_eq!(num_arrangements("???.### 1,1,3"), 1);
        assert_eq!(num_arrangements(".??..??...?##. 1,1,3"), 4);
        assert_eq!(num_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(num_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(num_arrangements("????.######..#####. 1,6,5"), 4);
        assert_eq!(num_arrangements("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 21);
    }
}
