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

// FIXME: this blows up because we end up calling it with m=15, n=19 in Part 2 for the second line.
//  The length of the output is O(n^m)... which is MASSIVE.
// FIXME:
// FIXME:
// FIXME:
// The problem is having lots of short groups when performing the initial partitioning.
//
// But then in the last line we get m=15, n=21 ! In this case everything is in one huge group.
//
// We need a more clever way of pruning the search space.

/// Assign `m` things to `n` groups, such that the _order_ of the `m` things is
/// unchanged. Groups are allowed to be empty.
///
/// We return a vector of partitions. Each partition is a vector of length `m`.
/// The value at index `i` of this partition is a value in `0..n` -- this represents the
/// index of the group to which thing `i` should be assigned.
// PERF: Could make this return an iterator rather than realise a vector?
fn ordered_partitions(m: usize, n: usize) -> Vec<Vec<usize>> {
    dbg!(m);
    dbg!(n);
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
    dbg!(line);
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
    dbg!(&pattern_group_lengths);

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
    dbg!(&partitions);

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

fn unfold_row(line: &str) -> String {
    let (pattern, groups) = line.split_once(' ').unwrap();

    let n = 5;
    vec![pattern; n].join("?") + " " + &vec![groups; n].join(",")
}

/// Return `true` iff `i_start` is a potentially valid location to start a group of length `n`.
fn is_valid_start(i_start: usize, n: usize, pattern: &str) -> bool {
    let bytes = pattern.as_bytes();

    // This is the last index of the group.
    let i_last = i_start + n - 1;

    if i_last >= pattern.len() {
        return false;
    }

    if (i_start > 0) && (bytes[i_start - 1] == b'#') {
        // We cannot be adjacent to a spring on our left.
        return false;
    }

    if (i_last < n - 1) && (bytes[i_last + 1] == b'#') {
        // We cannot be adjacent to a spring on our right.
        return false;
    }

    for i in i_start..=i_last {
        if bytes[i] == b'.' {
            // The group cannot overlap with a ground cell.
            return false;
        }
    }

    // No obvious objections at this stage... let it through!
    true
}

fn prune_i_starts_from_below(
    groups: &Vec<usize>,
    group_i_starts: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    // We keep track of the most constraining lower bound as we iterate through, and apply it to
    // the next item.
    let mut min_i_start = 0usize;

    for (i_starts, group_length) in group_i_starts.iter().zip(groups) {
        // Filter this group according to the current minimum.
        let new_i_starts: Vec<_> = i_starts
            .iter()
            .filter(|&&x| x >= min_i_start)
            .map(|&x| x)
            .collect();

        // The new minimum is computed.
        // NOTE: this assumes that i_starts is sorted, which it will be.
        min_i_start = new_i_starts.first().unwrap() + (group_length + 1);

        // We store the filtered group.
        result.push(new_i_starts);
    }

    result
}

fn prune_i_starts_from_above(
    groups: &Vec<usize>,
    group_i_starts: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut reversed_result: Vec<Vec<usize>> = Vec::new();

    // We keep track of the most constraining upper bound as we iterate through, and apply it to
    // the next item.
    let mut max_i_start = *group_i_starts.last().unwrap().last().unwrap();

    // NOTE: We iterate through the groups _backwards_
    for (i_starts, group_length) in group_i_starts.iter().zip(groups).rev() {
        // Filter this group according to the current maximum.
        let new_i_starts: Vec<_> = i_starts
            .iter()
            .filter(|&&x| x <= max_i_start)
            .map(|&x| x)
            .collect();

        // The new maximum is computed.
        let subtractor = group_length + 1;

        // NOTE: this assumes that i_starts is sorted, which it will be.
        let current_max = *new_i_starts.last().unwrap();

        // Avoid underflow!
        max_i_start = if current_max < subtractor {
            0
        } else {
            current_max - subtractor
        };

        // We store the filtered group.
        reversed_result.push(new_i_starts);
    }

    // Reverse the result to get it the correct way around, and return it.
    reversed_result.reverse();
    reversed_result
}


/// Second attempt at computing the number of allowed arrangements.
/// Attempting to have better complexity than `num_arrangements`!
fn num_arrangements_2(line: &str) -> usize {
    dbg!(line);
    let springs: Springs = line.parse().unwrap();

    let pattern = springs.pattern;
    let groups = springs.required;

    // New plan: work out conservative bounds for the allowed starting points for each group.
    //  We will constrain this by:
    //      - Intersections with known parts of the pattern
    //      - Bounds from the other groups that exist.
    let group_i_starts: Vec<_> = groups
        .iter()
        .enumerate()
        .map(|(i_group, &group_length)| {
            let n_groups_before = i_group;
            let n_groups_after = groups.len() - (i_group + 1);

            // This is the number of cells that must be left free before and after this
            // group, simply due to the absolute minimum amount of space that can be left for them.
            let gap_before = groups[0..i_group].iter().sum::<usize>() + n_groups_before;
            let gap_after =
                groups[(i_group + 1)..groups.len()].iter().sum::<usize>() + n_groups_after;

            // dbg!(i_group);
            // dbg!(group_length);
            // dbg!(pattern.len());
            // dbg!(gap_after);
            // dbg!(group_length);

            // Translate these gaps into a range for the start indices.
            let i_start_min = gap_before;
            let i_start_max = pattern.len() - (gap_after + group_length);
            let i_start_range = i_start_min..=i_start_max;

            // dbg!(&i_start_range);

            // Now eliminate any start indices that would be invalid according to the pattern.
            i_start_range
                .filter(|&x| is_valid_start(x, group_length, &pattern))
                .collect::<Vec<_>>()
        })
        .collect();

    for i_starts in &group_i_starts {
        println!("{:?}", &i_starts);
    }

    let moo: usize = group_i_starts.iter().map(|x| x.len()).product();
    dbg!(&moo);

    // We now need to do a little more pruning of the options.
    // One observation: it is possible that a group in the middle of the pack will have more
    // restrictive start options than groups to either side (e.g. due to intersection with the
    // pattern).
    let pruned_max_i_starts = prune_i_starts_from_above(&groups, &group_i_starts);

    println!();
    println!();
    for i_starts in &pruned_max_i_starts {
        println!("{:?}", &i_starts);
    }

    let moo2: usize = pruned_max_i_starts.iter().map(|x| x.len()).product();
    dbg!(&moo2);

    // Now do the same thing for a _lower_ bound on i_start.
    let pruned_min_i_starts = prune_i_starts_from_below(&groups, &pruned_max_i_starts);

    println!();
    println!();
    for i_starts in &pruned_min_i_starts {
        println!("{:?}", &i_starts);
    }

    let moo3: usize = pruned_min_i_starts.iter().map(|x| x.len()).product();
    dbg!(&moo3);

    // These are the group starting points that we will use.
    let group_i_starts = pruned_min_i_starts;

    // Now we must iterate over possible states. A 'state' contains the starting point
    // for every group.
    let mut state = vec![0usize; groups.len()];

    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{
        num_arrangements, num_arrangements_2, num_arrangements_in_group, ordered_partitions, part1,
        part2, unfold_row,
    };

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

    #[test]
    fn test_unfold_row() {
        assert_eq!(unfold_row(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1".to_string());
        assert_eq!(
            unfold_row("???.### 1,1,3"),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".to_string()
        );
    }

    #[test]
    fn test_num_arrangements_2() {
        // Custom, slightly more interesting, test.
        assert_eq!(num_arrangements_2("???.??? 1,1,1"), 6);

        assert_eq!(num_arrangements_2("???.### 1,1,3"), 1);
        assert_eq!(num_arrangements_2(".??..??...?##. 1,1,3"), 4);
        assert_eq!(num_arrangements_2("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(num_arrangements_2("????.#...#... 4,1,1"), 1);
        assert_eq!(num_arrangements_2("????.######..#####. 1,6,5"), 4);
        assert_eq!(num_arrangements_2("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_num_arrangements_2_after_unfolding() {
        // assert_eq!(num_arrangements_2(&unfold_row("???.### 1,1,3")), 1);
        assert_eq!(
            num_arrangements_2(&unfold_row(".??..??...?##. 1,1,3")),
            16384
        );
        assert_eq!(
            num_arrangements_2(&unfold_row("?#?#?#?#?#?#?#? 1,3,1,6")),
            1
        );
        assert_eq!(num_arrangements_2(&unfold_row("????.#...#... 4,1,1")), 16);
        assert_eq!(
            num_arrangements_2(&unfold_row("????.######..#####. 1,6,5")),
            2500
        );
        assert_eq!(
            num_arrangements_2(&unfold_row("?###???????? 3,2,1")),
            506250
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 525152);
    }
}
