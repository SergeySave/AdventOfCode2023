
/*
You finally reach the hot springs! You can see steam rising from secluded
areas attached to the primary, ornate building.

As you turn to enter, the researcher stops you. "Wait - I thought you were
looking for the hot springs, weren't you?" You indicate that this
definitely looks like hot springs to you.

"Oh, sorry, common mistake! This is actually the onsen! The hot springs are
next door."

You look in the direction the researcher is pointing and suddenly notice
the massive metal helixes towering overhead. "This way!"

It only takes you a few more steps to reach the main gate of the massive
fenced-off area containing the springs. You go through the gate and into a
small administrative building.

"Hello! What brings you to the hot springs today? Sorry they're not very
hot right now; we're having a lava shortage at the moment." You ask about
the missing machine parts for Desert Island.

"Oh, all of Gear Island is currently offline! Nothing is being manufactured
at the moment, not until we get more lava to heat our forges. And our
springs. The springs aren't very springy unless they're hot!"

"Say, could you go up and see why the lava stopped flowing? The springs are
too cold for normal operation, but we should be able to find one springy
enough to launch you up there!"

There's just one problem - many of the springs have fallen into disrepair,
so they're not actually sure which springs would even be safe to use! Worse
yet, their condition records of which springs are damaged (your puzzle
input) are also damaged! You'll need to help them repair the damaged
records.

In the giant field just outside, the springs are arranged into rows. For
each row, the condition records show every spring and whether it is
operational (.) or damaged (#). This is the part of the condition records
that is itself damaged; for some springs, it is simply unknown (?) whether
the spring is operational or damaged.

However, the engineer that produced the condition records also duplicated
some of this information in a different format! After the list of springs
for a given row, the size of each contiguous group of damaged springs is
listed in the order those groups appear in the row. This list always
accounts for every damaged spring, and each number is the entire size of
its contiguous group (that is, groups are always separated by at least one
operational spring: #### would always be 4, never 2,2).

So, condition records with no unknown spring conditions might look like
this:

#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1

However, the condition records are partially damaged; some of the springs'
conditions are actually unknown (?). For example:

???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1

Equipped with this information, it is your job to figure out how many
different arrangements of operational and broken springs fit the given
criteria in each row.

In the first line (???.### 1,1,3), there is exactly one way separate groups
of one, one, and three broken springs (in that order) can appear in that
row: the first three unknown springs must be broken, then operational, then
broken (#.#), making the whole row #.#.###.

The second line is more interesting: .??..??...?##. 1,1,3 could be a total
of four different arrangements. The last ? must always be broken (to
satisfy the final contiguous group of three broken springs), and each ??
must hide exactly one of the two broken springs. (Neither ?? could be both
broken springs or they would form a single contiguous group of two; if that
were true, the numbers afterward would have been 2,3 instead.) Since each
?? can either be #. or .#, there are four possible arrangements of
springs.

The last line is actually consistent with ten different arrangements!
Because the first number is 3, the first and second ? must both be . (if
either were #, the first number would have to be 4 or higher). However, the
remaining run of unknown spring conditions have many different ways they
could hold groups of two and one broken springs:

?###???????? 3,2,1
.###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#

In this example, the number of possible arrangements for each row is:

- ???.### 1,1,3 - 1 arrangement
- .??..??...?##. 1,1,3 - 4 arrangements
- ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
- ????.#...#... 4,1,1 - 1 arrangement
- ????.######..#####. 1,6,5 - 4 arrangements
- ?###???????? 3,2,1 - 10 arrangements

Adding all of the possible arrangement counts together produces a total of
21 arrangements.

For each row, count all of the different arrangements of operational and
broken springs that meet the given criteria. What is the sum of those
counts?

--- Part Two ---

As you look out at the field of springs, you feel like there are way more
springs than the condition records list. When you examine the records, you
discover that they were actually folded up this whole time!

To unfold the records, on each row, replace the list of spring conditions
with five copies of itself (separated by ?) and replace the list of
contiguous groups of damaged springs with five copies of itself (separated
by ,).

So, this row:

.# 1

Would become:

.#?.#?.#?.#?.# 1,1,1,1,1

The first line of the above example would become:

???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3

In the above example, after unfolding, the number of possible arrangements
for some rows is now much larger:

- ???.### 1,1,3 - 1 arrangement
- .??..??...?##. 1,1,3 - 16384 arrangements
- ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
- ????.#...#... 4,1,1 - 16 arrangements
- ????.######..#####. 1,6,5 - 2500 arrangements
- ?###???????? 3,2,1 - 506250 arrangements

After unfolding, adding all of the possible arrangement counts together
produces 525152.

Unfold your condition records; what is the new sum of possible arrangement
counts?
 */

use std::fs;
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./inputs/12_springs.txt").unwrap();
    let condition_records = file.lines();
    println!("{}", get_total_possible_spring_arrangements(condition_records.clone()));
    println!("{}", get_total_possible_folded_spring_arrangements(condition_records));
}

/// An enum to describe the type of a spring/tile/position
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}
impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Unknown Spring State {value}"),
        }
    }
}

/// A condition record line from the input
#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}

/// Preprocess a record from a string to a more useful type
fn preprocess_record(condition_record: &str) -> Record {
    let mut condition_record = condition_record.split(' ');
    // First section is the springs
    let springs = condition_record
        .next()
        .unwrap()
        .chars()
        .map_into()
        .collect();
    // Second section is the damaged groups
    let damaged_groups = condition_record
        .next()
        .unwrap()
        .split(',')
        .map(|group_size| usize::from_str(group_size).unwrap())
        .collect();
    Record {
        springs,
        damaged_groups
    }
}

/// Unfold a record into another with the five copies of itself
fn unfold(record: Record) -> Record {
    let mut springs = vec![];
    springs.extend(record.springs.iter());
    for _ in 0..4 {
        springs.push(Spring::Unknown);
        springs.extend(record.springs.iter());
    }
    Record {
        springs,
        damaged_groups: record.damaged_groups.repeat(5)
    }
}

/// Get the possible arrangements
fn get_possible_arrangements(record: Record) -> usize {
    // Let's use dynamic programming!
    //
    // This is a 2-dimensional dynamic programming problem
    // On the following table let's say the X axis is the input/springs
    //   and the Y axis is the damage groups
    //
    // Example table for input: ???.### 1,1,3
    //   - ? ? ? . # # #
    // - x x x x x x x x
    // 1 x x x x x x x x
    // 1 x x x x x x x x
    // 3 x x x x x x x x
    //
    // Note I've added an additional column for no springs/empty input
    //   and an additional row for no damage group
    // And we're going to fill in the table such that each entry answers the sub-problem with
    //   the spring state up to and including this column and the damaged groups up to and including
    //   this row
    // This layout has a few key/notable points:
    //   The bottom right entry is the answer we are looking for
    // The top left entry is 1 (trivial solution: no springs and no damaged springs - 1 arrangement)
    // The rest of the left side is 0 (no springs but there are damaged springs - impossible)
    // All entries along the top row until the first damaged spring are 1 and 0 after that (some
    //   non-damaged springs and no damaged springs - 1 arrangement).
    //
    // Thus the example table can be quickly filled in with:
    //   - ? ? ? . # # #
    // - 1 1 1 1 1 0 0 0
    // 1 0 x x x x x x x
    // 1 0 x x x x x x x
    // 3 0 x x x x x x x
    //
    // For each location in this table we can then fill it out with the following rules:
    // 1. If that column is an operational spring:
    //      The value is the value in the tile to the left (we can chop off the operational spring
    //        from the end without changing the answer)
    // 2. If that column is a damaged spring:
    //      We will take this to be the last tile of the run of damaged springs
    //      If this is not a valid place for that last tile (check that no adjacent damaged springs
    //        to the left), then set that to 0 (not a valid/impossible location)
    //      Otherwise: the value is the value in the tile one row up, and
    //        (length of damaged springs + 1) to the left. It is +1 to the left since that tile to
    //        the left could be an unknown tile and would then include some values for if it was a
    //        damaged spring. So instead we will go one more to the left (effectively applying the
    //        operational spring rule).
    // 3. If the column is an unknown spring:
    //      Set the value to what the sum of what an operational spring and damaged spring would be
    //        if they were put here
    //
    // These rules should be applied row by row to fill in the entire table.
    // The example table then becomes:
    //   - ? ? ? . # # #
    // - 1 1 1 1 1 0 0 0
    // 1 0 1 2 3 3 1 0 0
    // 1 0 0 0 1 1 3 3 1
    // 3 0 0 0 0 0 0 0 1
    //
    // Notice how some of the tiles are filled with values which are larger or did not contribute to
    // the final solution. This could in theory be avoided by lazily evaluating entries in the
    // table. However, doing so is not required to solve this fast enough.

    let mut table = vec![vec![0_usize; record.damaged_groups.len() + 1]; record.springs.len() + 1];
    table[0][0] = 1; // 1 way to arrange no springs with no damaged groups

    // Fill in the top row of the table
    for spring_index in 1..=record.springs.len() {
        let spring = record.springs[spring_index - 1];
        // The top row is for when no damaged springs
        // So if we find one we are done filling it in
        if spring == Spring::Damaged {
            break;
        }
        // This is just setting it to 1, but I'm doing it this way to be explicit about the transition which we are representing here
        table[spring_index][0] = table[spring_index - 1][0];
    }

    // Fill in the body of the table - looping over spring index first and then over damaged group
    for damaged_group in 1..=record.damaged_groups.len() {
        for spring_index in 1..=record.springs.len() {
            let spring = record.springs[spring_index - 1];
            let damage_group = record.damaged_groups[damaged_group - 1];

            // Compute the amount of arrangements we get if we assign operational to this spring
            let operational_amount = if spring == Spring::Operational || spring == Spring::Unknown {
                // This is just copied from one to the left (by definition of the table)
                table[spring_index - 1][damaged_group]
            } else { 0 };

            // Compute the amount of arrangements we get if we assign damaged to this spring
            let damaged_amount = if spring == Spring::Damaged || spring == Spring::Unknown {
                if spring_index < damage_group {
                    // If the damage group can't fit (i.e. this is impossible)
                    0
                } else {
                    // We want to make sure that this is a valid spot for a damaged spring
                    let mut is_valid = true;
                    let mut final_spring_index;

                    // Make sure the preceding springs in the damage group are non-operational
                    for i in 1..damage_group {
                        is_valid &= record.springs[spring_index - 1 - i] != Spring::Operational;
                    }

                    // If there is a spring right before the damage group
                    if damage_group < spring_index {
                        // Make sure it is not damaged
                        is_valid &= record.springs[spring_index - 1 - damage_group] != Spring::Damaged;
                        // We grab from the spring before final_spring_index as we want to grab
                        // from that last spring AS IF it was operational as it might be unknown
                        final_spring_index = spring_index - 1 - damage_group;
                    } else {
                        final_spring_index = 0; // only way that there is no spring before it is if it is first
                    }

                    // If this is valid for a run of damaged springs, return that
                    // otherwise 0
                    if is_valid {
                        table[final_spring_index][damaged_group - 1]
                    } else {
                        0
                    }
                }
            } else { 0 };

            // Write to the table
            table[spring_index][damaged_group] = operational_amount + damaged_amount;
        }
    }

    // Get the final result from the table
    table[record.springs.len()][record.damaged_groups.len()]
}


fn get_total_possible_spring_arrangements<'a>(condition_records: impl Iterator<Item=&'a str>) -> usize {
    condition_records
        .map(preprocess_record)
        .map(get_possible_arrangements)
        .sum()
}

fn get_total_possible_folded_spring_arrangements<'a>(condition_records: impl Iterator<Item=&'a str>) -> usize {
    condition_records
        .map(preprocess_record)
        .map(unfold)
        .map(get_possible_arrangements)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        21,
        get_total_possible_spring_arrangements(
            r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".lines()
        )
    )
}

#[test]
fn test_part2() {
    assert_eq!(
        525152,
        get_total_possible_folded_spring_arrangements(
            r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".lines()
        )
    )
}

#[test]
fn test_part2_input_debug() {
    // I had an issue with this one
    assert_eq!(
        103175004, // Computed using the non-dynamic version
        get_total_possible_folded_spring_arrangements(
            r"#??????.??.??? 4,1,1,1".lines()
        )
    )
}


/*
Code for an old implementation which was implemented without dynamic programming

struct RecordSlice<'a> {
    springs: &'a [Spring],
    damaged_groups: &'a [usize],
}

fn compute_possible_arrangements(record_slice: RecordSlice, required: usize) -> usize {
    // Base case: we have no more damaged groups - make sure we don't have any more damaged springs
    if record_slice.damaged_groups.len() == 0 {
        return if record_slice.springs.contains(&Spring::Damaged) {
            0
        } else {
            1
        }
    }

    // Base case: we can't fit this group in the remaining space
    if record_slice.damaged_groups[0] > record_slice.springs.len() {
        return 0;
    }

    // Base case: we know we won't be able to fit everything else in
    if record_slice.springs.len() < required {
        return 0;
    }

    let mut total = 0;

    // Loop over each possible position where we can place the first damaged group
    for i in 0..(record_slice.springs.len() - required + 1) {
        // If we've found an unknown spot, we can try to put the damaged group there
        // Additionally if we've found a damaged spot, then this is the last spot we can attempt to put the damaged group
        if record_slice.springs[i] == Spring::Unknown || record_slice.springs[i] == Spring::Damaged {
            // Check if this is long enough for the damaged group
            let end_index = i + record_slice.damaged_groups[0];
            let fits_group = record_slice.springs[i..end_index].iter()
                .all(|x| *x != Spring::Operational);
            let next_tile_undamaged = if end_index < record_slice.springs.len() {
                record_slice.springs[end_index] != Spring::Damaged
            } else {
                true
            };
            if fits_group && next_tile_undamaged {
                total += compute_possible_arrangements(RecordSlice {
                    springs: &record_slice.springs[(end_index + 1).min(record_slice.springs.len())..],
                    damaged_groups: &record_slice.damaged_groups[1..],
                }, required.saturating_sub(record_slice.damaged_groups[0] + 1));
            }
            // Stop looping if this was a damaged spot - we can't postpone this damaged group any further so we must have found where it goes (or failed)
            if record_slice.springs[i] == Spring::Damaged {
                break;
            }
        }
    }

    total
}

fn get_possible_arrangements_non_dynamic(record: Record) -> usize {
    let damaged_required = record.damaged_groups.iter().sum::<usize>() + record.damaged_groups.len() - 1;
    return compute_possible_arrangements(RecordSlice {
        springs: &record.springs,
        damaged_groups: &record.damaged_groups,
    }, damaged_required);
}

fn get_total_possible_folded_spring_arrangements_non_dynamic<'a>(condition_records: impl Iterator<Item=&'a str>) -> usize {
    condition_records
        .map(preprocess_record)
        .map(unfold)
        .map(get_possible_arrangements_non_dynamic)
        .sum()
}

 */
