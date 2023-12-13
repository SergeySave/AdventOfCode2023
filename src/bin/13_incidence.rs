
/*
With your help, the hot springs team locates an appropriate spring which
launches you neatly and precisely up to the edge of Lava Island.

There's just one problem: you don't see any lava.

You do see a lot of ash and igneous rock; there are even what look like
gray mountains scattered around. After a while, you make your way to a
nearby cluster of mountains only to discover that the valley between them
is completely full of large mirrors. Most of the mirrors seem to be aligned
in a consistent way; perhaps you should head in that direction?

As you move through the valley of mirrors, you find that several of them
have fallen from the large metal frames keeping them in place. The mirrors
are extremely flat and shiny, and many of the fallen mirrors have lodged
into the ash at strange angles. Because the terrain is all one color, it's
hard to tell where it's safe to walk or where you're about to run into a
mirror.

You note down the patterns of ash (.) and rocks (#) that you see as you
walk (your puzzle input); perhaps by carefully analyzing these patterns,
you can figure out where the mirrors are!

For example:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

To find the reflection in each pattern, you need to find a perfect
reflection across either a horizontal line between two rows or across a
vertical line between two columns.

In the first pattern, the reflection is across a vertical line between two
columns; arrows on each of the two columns point at the line between the
columns:

123456789
    ><
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><
123456789

In this pattern, the line of reflection is the vertical line between
columns5 and 6. Because the vertical line is not perfectly in the middle
of the pattern, part of the pattern (column 1) has nowhere to reflect onto
and can be ignored; every other column has a reflected column within the
pattern and must match exactly: column 2 matches column 9, column 3 matches
8, 4 matches 7, and 5 matches 6.

The second pattern reflects across a horizontal line instead:

1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7

This pattern reflects across the horizontal line between rows 4 and 5. Row
1 would reflect with a hypothetical row 8, but since that's not in the
pattern, row 1 doesn't need to match anything. The remaining rows match:
row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.

To summarize your pattern notes, add up the number of columns to the left
of each vertical line of reflection; to that, also add 100 multiplied by
the number of rows above each horizontal line of reflection. In the above
example, the first pattern's vertical line has 5 columns to its left and
the second pattern's horizontal line has 4 rows above it, a total of 405.

Find the line of reflection in each of the patterns in your notes. What
number do you get after summarizing all of your notes?

--- Part Two ---

You resume walking through the valley of mirrors and - SMACK! - run
directly into one. Hopefully nobody was watching, because that must have
been pretty embarrassing.

Upon closer inspection, you discover that every mirror has exactly one
smudge: exactly one . or # should be the opposite type.

In each pattern, you'll need to locate and fix the smudge that causes a
different reflection line to be valid. (The old reflection line won't
necessarily continue being valid after the smudge is fixed.)

Here's the above example again:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

The first pattern's smudge is in the top-left corner. If the top-left #
were instead ., it would have a different, horizontal line of reflection:

1 ..##..##. 1
2 ..#.##.#. 2
3v##......#v3
4^##......#^4
5 ..#.##.#. 5
6 ..##..##. 6
7 #.#.##.#. 7

With the smudge in the top-left corner repaired, a new horizontal line of
reflection between rows 3 and 4 now exists. Row 7 has no corresponding
reflected row and can be ignored, but every other row matches exactly: row
1 matches row 6, row 2 matches row 5, and row 3 matches row 4.

In the second pattern, the smudge can be fixed by changing the fifth symbol
on row 2 from . to #:

1v#...##..#v1
2^#...##..#^2
3 ..##..### 3
4 #####.##. 4
5 #####.##. 5
6 ..##..### 6
7 #....#..# 7

Now, the pattern has a different horizontal line of reflection between rows
1 and 2.

Summarize your notes as before, but instead use the new different
reflection lines. In this example, the first pattern's new horizontal line
has 3 rows above it and the second pattern's new horizontal line has 1 row
above it, summarizing to the value 400.

In each pattern, fix the smudge and find the different line of reflection.
What number do you get after summarizing the new reflection line in each
pattern in your notes?
 */

use std::cmp::min;
use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs/13_incidence.txt").unwrap();
    let patterns = file.lines();
    println!("{}", get_note_summaries(patterns.clone()));
    println!("{}", get_note_summaries_smudged(patterns));
}

/// An enum corresponding to the type of tile on the ground
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Ground {
    Ash,
    Rocks
}
impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Ground::Ash,
            '#' => Ground::Rocks,
            _ => panic!("Unknown Ground {value}")
        }
    }
}

/// A struct to hold a pattern/section in the input
#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<Ground>>
}

/// Represents a line of reflection
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}
impl Reflection {
    /// Get the value for a line of reflection
    fn value(self) -> usize {
        match self {
            Reflection::Vertical(x) => x,
            Reflection::Horizontal(x) => x * 100,
        }
    }
}

/// Preprocess the input into a vector of patterns
fn preprocess<'a>(patterns: impl Iterator<Item=&'a str>) -> Vec<Pattern> {
    let mut result = vec![];
    let mut next_pattern = Pattern {
        pattern: vec![],
    };

    for pattern_line in patterns {
        if pattern_line.is_empty() {
            result.push(next_pattern);
            next_pattern = Pattern {
                pattern: vec![]
            };
        } else {
            next_pattern.pattern.push(pattern_line.chars().map(|char| char.into()).collect())
        }
    }
    result.push(next_pattern);

    result
}

/// Search for a horizontal reflection line for a given pattern with a certain amount of expected errors
///
/// `error_count` is used to allow for part1 and part2 to be solved with the same algorithm
/// part1 is called with `error_count == 0`
/// part2 is called with `error_count == 1`
fn search_horizontal_line_reflection(pattern: &Pattern, error_count: usize) -> Option<Reflection> {
    // A label is added here to allow us to quickly break out of all inner loops when we determine that
    // a given location cannot be a valid line of reflection
    'outer: for second_row in 1..pattern.pattern.len() {
        let first_row = second_row - 1;
        // We are going to loop over each spot between rows
        // first row is the one on top, second row is the one on the bottom
        let mut errors = 0_usize;
        // Now we loop outwards from this pair of rows until we reach the edge
        for distance in 0..=min(first_row, pattern.pattern.len() - second_row - 1) {
            // Now we loop across those rows
            for i in 0..pattern.pattern[0].len() {
                // If a tile doesn't match, count it as an error
                if pattern.pattern[first_row - distance][i] != pattern.pattern[second_row + distance][i] {
                    errors += 1;
                    // Short circuit this row if we already know it cannot be a reflection
                    // Ideally almost every time we go around the outer loop we hit this very quickly
                    // This is a valid assumption to make since we know there is only one reflection/line of symmetry
                    // Therefore, we can assume that the pattern is approximately random
                    if errors > error_count {
                        continue 'outer;
                    }
                }
            }
        }
        // Finally if we didn't short circuit we need to make sure that we have the right number of errors
        // This is for part2
        // If part2 had a higher error_count than 1, this would be much more computationally intensive
        if errors == error_count {
            return Some(Reflection::Horizontal(second_row));
        }
    }
    None
}

fn search_vertical_line_reflection(pattern: &Pattern, error_count: usize) -> Option<Reflection> {
    // This is basically the same thing as horizontal, but transposed
    'outer: for second_column in 1..pattern.pattern[0].len() {
        let first_column = second_column - 1;
        let mut errors = 0_usize;
        for distance in 0..=min(first_column, pattern.pattern[0].len() - second_column - 1) {
            for i in 0..pattern.pattern.len() {
                if pattern.pattern[i][first_column - distance] != pattern.pattern[i][second_column + distance] {
                    errors += 1;
                    if errors > error_count {
                        continue 'outer;
                    }
                }
            }
        }
        if errors == error_count {
            return Some(Reflection::Vertical(second_column));
        }
    }
    None
}

/// Find a normal reflection
fn find_reflection(pattern: Pattern) -> Reflection {
    // Try to get a horizontal line
    search_horizontal_line_reflection(&pattern, 0)
        // Otherwise we know its a vertical line (by the nature of the problem)
        .or_else(|| search_vertical_line_reflection(&pattern, 0))
        .unwrap()
}

/// Solve part1
fn get_note_summaries<'a>(patterns: impl Iterator<Item=&'a str>) -> usize {
    let patterns = preprocess(patterns);
    patterns.into_iter()
        .map(find_reflection)
        .map(Reflection::value)
        .sum()
}

/// Find a smudged reflection
fn find_reflection_smudged(pattern: Pattern) -> Reflection {
    // Same as part1 but with error_count of 1
    search_horizontal_line_reflection(&pattern, 1)
        .or_else(|| search_vertical_line_reflection(&pattern, 1))
        .unwrap()
}

/// Solve part2
fn get_note_summaries_smudged<'a>(patterns: impl Iterator<Item=&'a str>) -> usize {
    let patterns = preprocess(patterns);
    patterns.into_iter()
        .map(find_reflection_smudged)
        .map(Reflection::value)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        405,
        get_note_summaries(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        400,
        get_note_summaries_smudged(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".lines()
        )
    );
}

