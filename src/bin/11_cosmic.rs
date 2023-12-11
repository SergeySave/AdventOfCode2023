
/*
You continue following signs for "Hot Springs" and eventually come across
an observatory. The Elf within turns out to be a researcher studying cosmic
expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only
visiting for this research project. However, he confirms that the hot
springs are the next-closest area likely to have people; he'll even take
you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a
single giant image (your puzzle input). The image includes empty space (.)
and galaxies (#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....

The researcher is trying to figure out the sum of the lengths of the
shortest path between every pair of galaxies. However, there's a catch: the
universe expanded in the time it took the light from those galaxies to
reach the observatory.

Due to something involving gravitational effects, only some space expands.
In fact, the result is that any rows or columns that contain no galaxies
should all actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^

These rows and columns need to be twice as big; the result of cosmic
expansion therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......

Equipped with this expanded universe, the shortest path between every pair
of galaxies can be found. It can help to assign every galaxy a unique
number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......

In these 9 galaxies, there are 36 pairs. Only count each pair once; order
within the pair doesn't matter. For each pair, find any shortest path
between the two galaxies using only steps that move up, down, left, or
right exactly one . or # at a time. (The shortest path between two galaxies
is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......

This path has length 9 because it takes a minimum of nine steps to get from
galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto
galaxy 9 itself). Here are some other example shortest path lengths:

- Between galaxy 1 and galaxy 7: 15
- Between galaxy 3 and galaxy 6: 17
- Between galaxy 8 and galaxy 9: 5

In this example, after expanding the universe, the sum of the shortest path
between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between
every pair of galaxies. What is the sum of these lengths?

--- Part Two ---

The galaxies are much older (and thus much farther apart) than the
researcher initially estimated.

Now, instead of the expansion you did before, make each empty row or column
one million times larger. That is, each empty row should be replaced with
1000000 empty rows, and each empty column should be replaced with 1000000
empty columns.

(In the example above, if each empty row or column were merely 10 times
larger, the sum of the shortest paths between every pair of galaxies would
be 1030. If each empty row or column were merely 100 times larger, the sum
of the shortest paths between every pair of galaxies would be 8410.
However, your universe will need to expand far beyond these values.)

Starting with the same initial image, expand the universe according to
these new rules, then find the length of the shortest path between every
pair of galaxies. What is the sum of these lengths?
 */

use std::collections::BTreeSet;
use std::fs;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./inputs/11_cosmic.txt").unwrap();
    let image = file.lines();
    println!("{}", get_sum_of_lengths(image.clone(), 2));
    println!("{}", get_sum_of_lengths(image, 1000000));
}

/// A structure to store the position data for a star
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Star {
    row: usize,
    column: usize,
}
impl Star {
    /// Manhattan Distance
    fn distance_to(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.column.abs_diff(other.column)
    }
}

/// Extract the important data from the images
/// returns
///  1. the positions of all of the stars (before accounting for gravitational effects)
///  2. the rows with no stars
///  3. the columns with no stars
fn extract<'a>(image: impl Iterator<Item=&'a str>) -> (Vec<Star>, Vec<usize>, Vec<usize>) {
    let mut stars = vec![];
    let mut rows = vec![];
    // Columns are harder to find than rows - so instead we will start with all columns as an option
    // and remove them. A BTreeSet is used to keep things sorted.
    let mut columns = BTreeSet::new();

    image.enumerate().for_each(|(i, row)| {
        if columns.is_empty() {
            // Set the columns to all of them if it isn't yet set
            columns = row.char_indices().map(|(j, _)| j).collect();
        }

        let mut row_empty = true;
        row.char_indices().for_each(|(j, pixel)| {
            if pixel == '#' {
                stars.push(Star {
                    row: i,
                    column: j,
                });
                row_empty = false;
                columns.remove(&j); // Also remove from the column
            }
        });
        if row_empty {
            rows.push(i);
        }
    });

    (stars, rows, columns.into_iter().collect())
}

/// Update the star positions
/// rows and columns are assumed to be sorted in ascending order
fn update_star_positions(stars: &mut Vec<Star>, rows: &Vec<usize>, columns: &Vec<usize>, empty_space_multiplier: usize) {
    let increase_by = empty_space_multiplier.saturating_sub(1); // Saturating just in case
    stars.iter_mut().for_each(|star| {
        // partition point uses a binary search
        // This will tell us how many rows or columns are less than
        let rows_less = rows.partition_point(|&row| row < star.row);
        let cols_less = columns.partition_point(|&column| column < star.column);

        // Each one of these rows or columns increases by a certain amount
        star.row += rows_less * increase_by;
        star.column += cols_less * increase_by;
    });
}

/// Compute the sum of all pairwise distances between stars
fn compute_sum_pair_distances(stars: &Vec<Star>) -> usize {
    // Iterate over each pair
    stars.iter().combinations(2)
        // Compute the distance
        .map(|stars| {
            let star1 = stars[0];
            let star2 = stars[1];
            star1.distance_to(star2)
        })
        // Add it up
        .sum()
}

/// Solves both part 1 and part 2 using different empty space multipliers
/// Part 1 is solved with a multiplier of 2
/// Part 2 is solved with a multiplier of 1000000
/// Other values are only used for unit tests
fn get_sum_of_lengths<'a>(image: impl Iterator<Item=&'a str>, empty_space_multiplier: usize) -> usize {
    let (mut stars, rows, columns) = extract(image);
    update_star_positions(&mut stars, &rows, &columns, empty_space_multiplier);
    compute_sum_pair_distances(&stars)
}

#[test]
fn test_part1() {
    assert_eq!(
        374,
        get_sum_of_lengths(
            r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".lines(),
            2
        )
    )
}

#[test]
fn test_part2a() {
    assert_eq!(
        1030,
        get_sum_of_lengths(
            r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".lines(),
            10
        )
    )
}

#[test]
fn test_part2b() {
    assert_eq!(
        8410,
        get_sum_of_lengths(
            r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".lines(),
            100
        )
    )
}

