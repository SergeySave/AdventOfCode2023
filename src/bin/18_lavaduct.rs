/*
Thanks to your efforts, the machine parts factory is one of the first
factories up and running since the lavafall came back. However, to catch up
with the large backlog of parts requests, the factory will also need a
large supply of lava for a while; the Elves have already started creating a
large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you
to take a look at the dig plan (your puzzle input). For example:

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)

The digger starts in a 1 meter cube hole in the ground. They then dig the
specified number of meters up (U), down (D), left (L), or right (R),
clearing full 1 meter cubes as they go. The directions are given as seen
from above, so if "up" were north, then "right" would be east, and so on.
Each trench is also listed with the color that the edge of the trench
should be painted as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the
following loop of trench (#) having been dug out from otherwise ground-
level terrain (.):

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######

At this point, the trench could contain 38 cubic meters of lava. However,
this is just the edge of the lagoon; the next step is to dig out the
interior so that it is one meter deep as well:

#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######

Now, the lagoon can contain a much more respectable 62 cubic meters of
lava. While the interior is dug out, the edges are also painted according
to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow
their dig plan, how many cubic meters of lava could it hold?

--- Part Two ---

The Elves were right to be concerned; the planned lagoon would be much too
small.

After a few minutes, someone realizes what happened; someone swapped the
color and instruction parameters when producing the dig plan. They don't
have time to fix the bug; one of them asks if you can extract the correct
instructions from the hexadecimal codes.

Each hexadecimal code is six hexadecimal digits long. The first five
hexadecimal digits encode the distance in meters as a five-digit
hexadecimal number. The last hexadecimal digit encodes the direction to
dig: 0 means R, 1 means D, 2 means L, and 3 means U.

So, in the above example, the hexadecimal codes can be converted into the
true instructions:

#70c710 = R 461937
#0dc571 = D 56407
#5713f0 = R 356671
#d2c081 = D 863240
#59c680 = R 367720
#411b91 = D 266681
#8ceee2 = L 577262
#caa173 = U 829975
#1b58a2 = L 112010
#caa171 = D 829975
#7807d2 = L 491645
#a77fa3 = U 686074
#015232 = L 5411
#7a21e3 = U 500254

Digging out this loop and its interior produces a lagoon that can hold an
impressive 952408144115 cubic meters of lava.

Convert the hexadecimal color codes into the correct instructions; if the
Elves follow this new dig plan, how many cubic meters of lava could the
lagoon hold?
 */

use std::cmp::{max, min};
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./inputs/18_lavaduct.txt").unwrap();
    let dig_plan = file.lines();
    println!("{}", get_total_volume(dig_plan.clone()));
    println!("{}", get_total_volume_swapped(dig_plan));
}

/// Represents a direction
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown Direction {value}")
        }
    }
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn handedness(self, next: Self) -> isize {
        match self {
            Direction::Up => match next {
                Direction::Up => 0,
                Direction::Down => 0,
                Direction::Left => -1,
                Direction::Right => 1,
            },
            Direction::Down => match next {
                Direction::Up => 0,
                Direction::Down => 0,
                Direction::Left => 1,
                Direction::Right => -1
            },
            Direction::Left => match next {
                Direction::Up => 1,
                Direction::Down => -1,
                Direction::Left => 0,
                Direction::Right => 0,
            },
            Direction::Right => match next {
                Direction::Up => -1,
                Direction::Down => 1,
                Direction::Left => 0,
                Direction::Right => 0,
            },
        }
    }
}

/// A data type to hold a single instruction
#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    amount: usize,
    color: u32,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let direction = parts.next().unwrap().into();
        let amount = usize::from_str(parts.next().unwrap()).unwrap();
        let color = parts.next().unwrap();
        let color = &color[2..(color.len() - 1)]; // Shop off the stuff we don't care about
        let color = u32::from_str_radix(color, 16).unwrap();
        Self {
            direction,
            amount,
            color,
        }
    }
}

impl Instruction {
    /// Perform the swap from color to direction & amount
    fn swap(self) -> Self {
        let direction = match self.color % 16 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Unknown direction {}", self.color % 16)
        };
        let amount = (self.color / 16) as usize;
        let color = 0;
        Self {
            direction,
            amount,
            color,
        }
    }
}

/// Preprocess into a more useful form
fn preprocess<'a>(dig_plan: impl Iterator<Item=&'a str>) -> Vec<Instruction> {
    dig_plan.map(|x| x.into()).collect()
}

/// Solve part 1
fn get_total_volume<'a>(dig_plan: impl Iterator<Item=&'a str>) -> usize {
    let dig_plan = preprocess(dig_plan);
    get_total_volume_reduction(dig_plan)
}

/// Get the corners for the shape
fn get_corners_sorted(dig_plan: Vec<Instruction>) -> Vec<Vec<(isize, isize)>> {
    let mut x = 0_isize;
    let mut y = 0_isize;
    let mut result: Vec<Vec<(isize, isize)>> = dig_plan.into_iter().map(|instruction| {
        match instruction.direction {
            Direction::Up => {
                y -= instruction.amount as isize;
            }
            Direction::Down => {
                y += instruction.amount as isize;
            }
            Direction::Left => {
                x -= instruction.amount as isize;
            }
            Direction::Right => {
                x += instruction.amount as isize;
            }
        }
        (x, y)
    }).into_group_map_by(|x| x.1)
        .into_values()
        .sorted_by_key(|x| x[0].1)
        .collect();

    result.iter_mut().for_each(|x| {
        x.sort_by_key(|x| x.0);
    });

    result
}

/// Get the total size of the subtracted ranges
fn subtract_ranges(subtract_from: Vec<[isize; 2]>, subtract_value: Vec<[isize; 2]>) -> isize {
    let mut result = 0;
    for i in (0..subtract_from.len()).rev() {
        result += subtract_from[i][1] - subtract_from[i][0] + 1;
        for [start, end] in &subtract_value {
            // Intersection test
            if !(subtract_from[i][1] < *start || *end < subtract_from[i][0]) {
                // If we wanted to get the intersection it would be:
                let start_intersection = max(subtract_from[i][0], *start);
                let end_intersection = min(subtract_from[i][1], *end);
                result -= end_intersection - start_intersection + 1;
            }
        }
    }
    result
}

/// Solve both part 1 and part 2 by computing the volume from a given set of instructions
fn get_total_volume_reduction(dig_plan: Vec<Instruction>) -> usize {
    // The general idea here is we can operate on long, identical chunks of the world
    // if we can compute how many interior tiles there are per row and how many
    // rows are needed
    // This is done by looping downward through the corners and adding up the ranges since the
    // last row with corners in it
    let corners = get_corners_sorted(dig_plan);
    let mut interior_ranges = vec![];
    let mut last_y = corners[0][0].1;
    for corner in corners[0].chunks_exact(2) {
        interior_ranges.push([corner[0].0, corner[1].0]);
    }

    let mut volume = 0_usize;
    for corner_row in &corners[1..] {
        // Compute the volume of this section
        let height = (corner_row[0].1 - last_y) as usize;
        last_y = corner_row[0].1;
        volume += height * interior_ranges.iter().map(|range| (range[1] - range[0] + 1) as usize).sum::<usize>();

        // Update the interior ranges
        let mut interior_processing: Vec<_> = interior_ranges.iter().flatten().map(|x| *x).collect();
        interior_processing.extend(corner_row.iter().map(|x| x.0));
        interior_processing.sort_by_key(|x| *x);

        let mut deduped = interior_processing.clone();
        deduped.dedup();
        let x_counts = interior_processing.iter().into_group_map_by(|x| **x);
        // let deleted: Vec<_> = interior_processing.iter().filter(|x| x_counts[&x.0].len() == 2).collect();
        let interior_processing: Vec<_> = interior_processing.iter().filter(|x| x_counts[&x].len() == 1).collect();
        let new_ranges: Vec<[isize; 2]> = interior_processing.chunks_exact(2).map(|x| [*x[0], *x[1]]).collect();

        // We also need to add 1 for each tile which is no longer in the range
        let subtracted = subtract_ranges(interior_ranges.clone(), new_ranges.clone());
        volume += subtracted as usize;

        interior_ranges = new_ranges;
    }
    volume
}

/// Solve part 2
fn get_total_volume_swapped<'a>(dig_plan: impl Iterator<Item=&'a str>) -> usize {
    let dig_plan = preprocess(dig_plan)
        .into_iter()
        .map(Instruction::swap)
        .collect::<Vec<Instruction>>();
    get_total_volume_reduction(dig_plan)
}

#[test]
fn test_part1() {
    assert_eq!(
        62,
        get_total_volume(
            r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        952408144115,
        get_total_volume_swapped(
            r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".lines()
        )
    );
}
