/*
You and the Elf eventually reach a gondola lift station; he says the
gondola lift will take you up to the water source, but this is as far as he
can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem:
they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of
surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the
engine, but nobody can figure out which one. If you can add up all the part
numbers in the engine schematic, it should be easy to work out which part
is missing.

The engine schematic (your puzzle input) consists of a visual
representation of the engine. There are lots of numbers and symbols you
don't really understand, but apparently any number adjacent to a symbol,
even diagonally, is a "part number" and should be included in your sum.
(Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not
adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of
all of the part numbers in the engine schematic?

--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the
engine springs to life, you jump in the closest gondola, finally ready to
ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still
wrong? Fortunately, the gondola has a phone labeled "help", so you pick it
up and the engineer answers.

Before you can explain the situation, she suggests that you look out the
window. There stands the engineer, holding a phone in one hand and waving
with the other. You're going so slowly that you haven't even left the
station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is
wrong. A gear is any * symbol that is adjacent to exactly two part numbers.
Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all
up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, there are two gears. The first is in the top left; it
has part numbers 467 and 35, so its gear ratio is 16345. The second gear is
in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not
a gear because it is only adjacent to one part number.) Adding up all of
the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
 */

use std::cmp::min;
use std::fs;
use std::str::FromStr;

fn main() {
    let file = fs::read_to_string("./inputs/03_gear_ratios.txt").unwrap();
    let games = file.lines();
    println!("{}", get_part_number_sum(games.clone()));
    println!("{}", get_gear_ratio_sum(games));
}

fn preprocess_schematic<'a>(schematic: impl Iterator<Item=&'a str>, width: usize, height: usize) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
    // The total sum of adjacent numbers for each cell
    // Doing this preprocessing means that we can simply look up the sum for each symbol and add
    // them all together
    let mut result = vec![vec![0; height]; width];
    // A list of (x, y) coordinates of all of the symbols
    let mut symbols = vec![];

    let mut accumulator = String::new();
    let mut add_number = |accumulator: &mut String, end_x: usize, y: usize| {
        if !accumulator.is_empty() {
            let start_x = end_x - accumulator.len();
            let value = usize::from_str(&accumulator).unwrap_or(0);
            for x in start_x.saturating_sub(1)..=min(end_x, width - 1) {
                for y in y.saturating_sub(1)..=min(y + 1, height - 1) {
                    result[x][y] += value;
                }
            }
            accumulator.clear();
        }
    };
    for (y, row) in schematic.enumerate() {
        for (x, char) in row.char_indices() {
            if char.is_numeric() {
                accumulator.push(char);
            } else {
                add_number(&mut accumulator, x, y);
                if char != '.' {
                    symbols.push((x, y));
                }
            }
        }
        add_number(&mut accumulator, width, y);
    }

    (result, symbols)
}


fn get_part_number_sum<'a>(schematic: impl Iterator<Item=&'a str> + Clone) -> usize {
    let width = schematic.clone().next().map_or(0, |x| x.len());
    let height = schematic.clone().count();
    let (preprocessed, symbols) = preprocess_schematic(schematic, width, height);
    symbols.iter()
        .map(|(x, y)| preprocessed[*x][*y])
        .sum()
}

fn preprocess_gear<'a>(schematic: impl Iterator<Item=&'a str>, width: usize, height: usize) -> (Vec<Vec<(usize, usize)>>, Vec<(usize, usize, char)>) {
    // The total product of adjacent numbers for each cell as well as the count of numbers
    // which were multiplied to arrive at that product
    // Doing this preprocessing means that we can simply look up the sum for each valid gear symbol
    // and add them all together
    let mut result = vec![vec![(1, 0); height]; width];
    // A list of (x, y, symbol) coordinate and symbol of all of the symbols
    let mut symbols = vec![];

    let mut accumulator = String::new();
    let mut add_number = |accumulator: &mut String, end_x: usize, y: usize| {
        if !accumulator.is_empty() {
            let start_x = end_x - accumulator.len();
            let value = usize::from_str(&accumulator).unwrap_or(0);
            for x in start_x.saturating_sub(1)..=min(end_x, width - 1) {
                for y in y.saturating_sub(1)..=min(y + 1, height - 1) {
                    result[x][y].0 *= value;
                    result[x][y].1 += 1;
                }
            }
            accumulator.clear();
        }
    };
    for (y, row) in schematic.enumerate() {
        for (x, char) in row.char_indices() {
            if char.is_numeric() {
                accumulator.push(char);
            } else {
                add_number(&mut accumulator, x, y);
                if char != '.' {
                    symbols.push((x, y, char));
                }
            }
        }
        add_number(&mut accumulator, width, y);
    }

    (result, symbols)
}

fn get_gear_ratio_sum<'a>(schematic: impl Iterator<Item=&'a str> + Clone) -> usize {
    let width = schematic.clone().next().map_or(0, |x| x.len());
    let height = schematic.clone().count();
    let (preprocessed, symbols) = preprocess_gear(schematic, width, height);
    symbols.iter()
        .filter(|(_, _, symbol)| *symbol == '*')
        .filter(|(x, y, _)| preprocessed[*x][*y].1 == 2)
        .map(|(x, y, _)| preprocessed[*x][*y].0)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        4361,
        get_part_number_sum(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".lines()
        )
    )
}

#[test]
fn test_part2() {
    assert_eq!(
        467835,
        get_gear_ratio_sum(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".lines()
        )
    )
}

