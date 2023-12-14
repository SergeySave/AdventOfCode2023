/*

You reach the place where all of the mirrors were pointing: a massive
parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves
are roughly in the shape of a parabolic reflector dish, each individual
mirror seems to be pointing in slightly the wrong direction. If the dish is
meant to focus light, all it's doing right now is sending it in a vague
direction.

This system must be what provides the energy for the lava! If you focus the
reflector dish, maybe you can go where it's pointing and use the light to
fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected
via an elaborate system of ropes and pulleys to a large metal platform
below the dish. The platform is covered in large rocks of various shapes.
Depending on their position, the weight of the rocks deforms the platform,
and the shape of the platform controls which ropes move and ultimately the
focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even
has a control panel on the side that lets you tilt it in one of four
directions! The rounded rocks (O) will roll when the platform is tilted,
while the cube-shaped rocks (#) will stay in place. You note the positions
of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....

Start by tilting the lever so all of the rocks will slide north as far as
they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....

You notice that the support beams along the north side of the platform are
damaged; to ensure the platform doesn't collapse, you should calculate the
total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the
number of rows from the rock to the south edge of the platform, including
the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.)
So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1

The total load is the sum of the load caused by all of the rounded rocks.
In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what
is the total load on the north support beams?

--- Part Two ---

The parabolic reflector dish deforms, but not in a way that focuses the
beam. To do that, you'll need to move the rocks to the edges of the
platform. Fortunately, a button on the side of the control panel labeled
"spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the rounded rocks roll
north, then west, then south, then east. After each tilt, the rounded rocks
roll as far as they can before the platform tilts in the next direction.
After one cycle, the platform will have finished rolling the rounded rocks
in those four directions in that order.

Here's what happens in the example above after each of the first few
cycles:

After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O

This process should work if you leave it running long enough, but you're
still worried about the north support beams. To make sure they'll survive
for a while, you need to calculate the total load on the north support
beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load on the north
support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the total load
on the north support beams?
 */

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs/14_parabolic.txt").unwrap();
    let input = file.lines();
    println!("{}", get_total_load_north_tilted(input.clone()));
    println!("{}", get_cycled_load_north(input));
}

/// Represents a tile in the world
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Cube,
    Rounded,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Cube,
            'O' => Tile::Rounded,
            _ => panic!("Unknown tile {value}"),
        }
    }
}

/// Represents the input to the problem i.e. the map
#[derive(Clone, Eq, PartialEq, Hash)]
struct Input {
    columns: Vec<Vec<Tile>>,
}

/// Preprocess the string input into a more useful form
fn preprocess<'a>(input: impl Iterator<Item=&'a str>) -> Input {
    let mut result = Input {
        columns: vec![]
    };
    for row in input {
        for (col, tile) in row.char_indices() {
            while result.columns.len() <= col {
                result.columns.push(vec![]);
            }
            result.columns[col].push(tile.into());
        }
    }
    result
}

/// Compute the total load for a given input
fn compute_total_load(input: Input) -> usize {
    input.columns.into_iter().map(|column| {
        let column_length = column.len();
        column.into_iter().enumerate().map(|(i, tile)| {
            match tile {
                Tile::Empty => 0, // NoOp
                Tile::Cube => 0,
                Tile::Rounded => {
                    // column_length is 1 greater than the index of the last column so this
                    // is the total amount of rows including this one from the bottom
                    column_length - i
                }
            }
        }).sum::<usize>()
    }).sum()
}

/// Slide the rounded stones north
fn slide_north(input: &mut Input) {
    input.columns.iter_mut().for_each(|column| {
        let mut slide_to = 0;
        for i in 0..column.len() {
            match column[i] {
                Tile::Empty => {}
                Tile::Cube => slide_to = i + 1,
                Tile::Rounded => {
                    column[i] = Tile::Empty;
                    column[slide_to] = Tile::Rounded;
                    slide_to += 1;
                }
            }
        }
    });
}

/// Slide the rounded stones south
fn slide_south(input: &mut Input) {
    input.columns.iter_mut().for_each(|column| {
        let mut slide_to = column.len() - 1;
        for i in 0..column.len() {
            let i = column.len() - 1 - i;
            match column[i] {
                Tile::Empty => {}
                Tile::Cube => slide_to = i.saturating_sub(1),
                Tile::Rounded => {
                    column[i] = Tile::Empty;
                    column[slide_to] = Tile::Rounded;
                    slide_to = slide_to.saturating_sub(1);
                }
            }
        }
    });
}

/// Slide the rounded stones west
fn slide_west(input: &mut Input) {
    for row in 0..input.columns[0].len() {
        let mut slide_to = 0;
        for col in 0..input.columns.len() {
            match input.columns[col][row] {
                Tile::Empty => {}
                Tile::Cube => slide_to = col + 1,
                Tile::Rounded => {
                    input.columns[col][row] = Tile::Empty;
                    input.columns[slide_to][row] = Tile::Rounded;
                    slide_to += 1;
                }
            }
        }
    }
}

/// Slide the rounded stones east
fn slide_east(input: &mut Input) {
    for row in 0..input.columns[0].len() {
        let mut slide_to = input.columns[0].len() - 1;
        for col in 0..input.columns.len() {
            let col = input.columns.len() - 1 - col;
            match input.columns[col][row] {
                Tile::Empty => {}
                Tile::Cube => slide_to = col.saturating_sub(1),
                Tile::Rounded => {
                    input.columns[col][row] = Tile::Empty;
                    input.columns[slide_to][row] = Tile::Rounded;
                    slide_to = slide_to.saturating_sub(1);
                }
            }
        }
    }
}

/// Perform a single cycle of the input
fn cycle(input: &mut Input) {
    slide_north(input);
    slide_west(input);
    slide_south(input);
    slide_east(input);
}

/// Cycle the input 1000000000 times
fn cycle_long(input: &mut Input) {
    // The main realization here is that after a certain amount of time this cyclic input results
    // in the state cycling
    // Once it gets into a cycle we know it is trapped in that cycle and cannot leave
    // Whenever we discover the cycle, we can use our knowledge of the cycle start and period to
    // figure out which of the states we have already seen we will see at at an arbitrary time in
    // the future
    const GOAL: usize = 1000000000;

    let mut visited: HashMap<Input, usize> = HashMap::new();
    let mut cycle_memory: Vec<Input> = vec![];
    for i in 0..GOAL {
        if visited.contains_key(&input) {
            // We've found a cycle
            let cycle_start = *visited.get(&input).unwrap();
            let cycle_period = i - cycle_start; // Both i and cycle_start

            // i == 0 means cycle has not been called
            // i == 1 means cycle has been called once
            // i == 1000000000 means cycle has been called 1000000000 times (our goal)
            // where n is a non-negative integer we want to solve the following for j:
            // 1000000000 = cycle_start + cycle_period * n + j
            //
            // 1000000000 - cycle_start = cycle_period * n + j
            // (1000000000 - cycle_start) % cycle_period = j
            // Then we look up whatever is in our memory for i == cycle_start + j
            let j = (GOAL - cycle_start) % cycle_period;

            *input = cycle_memory[cycle_start + j].clone();
            return;
        }
        visited.insert(input.clone(), i);
        cycle_memory.push(input.clone());
        cycle(input);
    }
    // If we somehow managed to get here then we found the answer as to where w are after 1000000000
    // cycles. Though getting here means we got here the hard way
}

/// Solve part1
fn get_total_load_north_tilted<'a>(input: impl Iterator<Item=&'a str>) -> usize {
    let mut input = preprocess(input);
    slide_north(&mut input);
    compute_total_load(input)
}

/// Solve part2
fn get_cycled_load_north<'a>(input: impl Iterator<Item=&'a str>) -> usize {
    let mut input = preprocess(input);
    cycle_long(&mut input);
    compute_total_load(input)
}

#[test]
fn test_part1() {
    assert_eq!(
        136,
        get_total_load_north_tilted(
            r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        64,
        get_cycled_load_north(
            r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".lines()
        )
    );
}
