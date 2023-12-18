use std::{cmp::min, str::FromStr};

use itertools::Itertools;

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

    if (i_last < pattern.len() - 1) && (bytes[i_last + 1] == b'#') {
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
    let mut max_i_last =
        *group_i_starts.last().unwrap().last().unwrap() + (groups.last().unwrap() - 1);

    // NOTE: We iterate through the groups _backwards_
    for (i_starts, group_length) in group_i_starts.iter().zip(groups).rev() {
        let max_i_start = max_i_last - (group_length - 1);

        // Filter this group according to the current maximum.
        let new_i_starts: Vec<_> = i_starts
            .iter()
            .filter(|&&x| x <= max_i_start)
            .map(|&x| x)
            .collect();

        // The new maximum index of the last value is 2 before the current maximum first-value
        // index.
        let new_max_i_start = *new_i_starts.last().unwrap();
        max_i_last = if new_max_i_start >= 2 {
            new_max_i_start - 2
        } else {
            0
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
fn num_arrangements(line: &str) -> usize {
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

            // Translate these gaps into a range for the start indices.
            let i_start_min = gap_before;
            let i_start_max = pattern.len() - (gap_after + group_length);
            let i_start_range = i_start_min..=i_start_max;

            // Now eliminate any start indices that would be invalid according to the pattern.
            i_start_range
                .filter(|&x| is_valid_start(x, group_length, &pattern))
                .collect::<Vec<_>>()
        })
        .collect();

    // We now need to do a little more pruning of the options.
    // One observation: it is possible that a group in the middle of the pack will have more
    // restrictive start options than groups to either side (e.g. due to intersection with the
    // pattern).
    let pruned_max_i_starts = prune_i_starts_from_above(&groups, &group_i_starts);

    // Now do the same thing for a _lower_ bound on i_start.
    let pruned_min_i_starts = prune_i_starts_from_below(&groups, &pruned_max_i_starts);

    // What we're _not_ currently doing is checking whether we are preventing ourselves from
    // covering known springs.
    // We will check this as we iterate over the combinations.
    num_arrangements_from_i_starts(&pattern, &pruned_min_i_starts, &groups, 0)
}

fn num_arrangements_from_i_starts(
    pattern: &str,
    group_i_starts: &[Vec<usize>],
    group_lengths: &[usize],
    offset: usize,
) -> usize {
    if group_i_starts.len() == 0 {
        return if pattern.as_bytes().iter().any(|&x| x == b'#') {
            // We are not matching the pattern, since we need to provide at least one
            // #; return zero.
            0
        } else {
            // There is no requirement to provide any #s, so there is one way to do this.
            1
        };
    } else if group_i_starts.len() == 1 {
        // Only one group to handle.
        // We need to:
        //  - filter out invalid elements given pattern length.
        //  - filter out invalid elements given locations of #
        //
        // The number remaining is our answer.

        let i_starts = &group_i_starts[0];
        let group_length = group_lengths[0];

        // Determine the maximum start & minimum end of the group in order to cover
        // all #s.
        let (max_i_start, min_i_last) = match pattern
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'#')
            .map(|(i, _)| i)
            .minmax()
        {
            itertools::MinMaxResult::NoElements => (pattern.len(), 0usize),
            itertools::MinMaxResult::OneElement(i) => (i, i),
            itertools::MinMaxResult::MinMax(i_min, i_max) => (i_min, i_max),
        };

        let moo = i_starts
            .iter()
            .filter(|&&i_start| {
                if offset > i_start {
                    // This i_start indicates a range that will start before the pattern.
                    return false;
                }
                let i_start_offset = i_start - offset;
                let i_last = i_start_offset + group_length - 1;
                if i_last >= pattern.len() {
                    // This group would finish after the pattern finishes.
                    return false;
                }

                // Apply constraints from # locations.
                if i_start_offset > max_i_start {
                    return false;
                }
                if i_last < min_i_last {
                    return false;
                }
                true
            })
            .count();

        return moo;
    }

    // Bisect the number of groups still available, and then trim to either side.
    let i_group = group_i_starts.len() / 2;

    // This is the total number of combinations that we have found from this bisection.
    let mut total: usize = 0;

    let group_length = group_lengths[i_group];
    for &i_start in group_i_starts[i_group].iter() {
        // NOTE: We do not need to worry about 'not covering' any #s, since this is considered
        // through the union of:
        //  - our initial filtering of the possible start points
        //  - the checks inside the left & right side of our partitions.
        // TODO: refactor this out as it is shared with above
        if offset > i_start {
            // This i_start indicates a range that will start before the pattern.
            continue;
        }
        let i_start_offset = i_start - offset;
        let i_last = i_start_offset + group_length - 1;
        if i_last >= pattern.len() {
            // This group would finish after the pattern finishes.
            continue;
        }

        // XXX: Are these indexings going to go out of range?
        let additional_offset_r = i_start_offset + group_length + 1;

        let pattern_l = if i_start_offset == 0 {
            ""
        } else {
            &pattern[..min(i_start_offset - 1, pattern.len())]
        };
        let pattern_r = if additional_offset_r > pattern.len() {
            ""
        } else {
            &pattern[additional_offset_r..]
        };

        let group_lengths_l = &group_lengths[..i_group];
        let group_lengths_r = &group_lengths[(i_group + 1)..];

        let group_i_starts_l = &group_i_starts[..i_group];
        let group_i_starts_r = &group_i_starts[(i_group + 1)..];

        total +=
            num_arrangements_from_i_starts(pattern_l, group_i_starts_l, group_lengths_l, offset)
                * num_arrangements_from_i_starts(
                    pattern_r,
                    group_i_starts_r,
                    group_lengths_r,
                    offset + additional_offset_r,
                );
    }
    total
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| num_arrangements(&unfold_row(line)))
        .sum()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{num_arrangements, part1, part2, unfold_row};

    const EXAMPLE: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

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
        assert_eq!(num_arrangements("???.??? 1,1,1"), 6);

        assert_eq!(num_arrangements("???.### 1,1,3"), 1);
        assert_eq!(num_arrangements(".??..??...?##. 1,1,3"), 4);
        assert_eq!(num_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(num_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(num_arrangements("????.######..#####. 1,6,5"), 4);
        assert_eq!(num_arrangements("?###???????? 3,2,1"), 10);

        // Another test-case
        assert_eq!(num_arrangements("?#?.??.#?.??? 2,1,1,1"), 14);
    }

    // NOTE: To run just this test:
    //      cargo test tests::test_num_arrangements_2_after_unfolding -- --exact
    #[test]
    fn test_num_arrangements_2_after_unfolding() {
        assert_eq!(num_arrangements(&unfold_row("???.### 1,1,3")), 1);
        assert_eq!(
            num_arrangements(&unfold_row(".??..??...?##. 1,1,3")),
            16384
        );
        assert_eq!(
            num_arrangements(&unfold_row("?#?#?#?#?#?#?#? 1,3,1,6")),
            1
        );
        assert_eq!(num_arrangements(&unfold_row("????.#...#... 4,1,1")), 16);
        assert_eq!(
            num_arrangements(&unfold_row("????.######..#####. 1,6,5")),
            2500
        );
        assert_eq!(
            num_arrangements(&unfold_row("?###???????? 3,2,1")),
            506250
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 525152);
    }
}
