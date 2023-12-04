/*
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to
take a look.The Elves have even given you a map; on it, they've used stars
to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations,
you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on
each day in the Advent calendar; the second puzzle is unlocked when you
complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful
enough") and where they're even sending you ("the sky") and why your map
looks mostly blank ("you sure ask a lot of questions") and hang on did you
just say the sky ("of course, where do you think snow comes from") when you
realize that the Elves are already loading you into a trebuchet ("please
hold still, we need to strap you in").

As they're making the final adjustments, they discover that their
calibration document (your puzzle input) has been amended by a very young
Elf who was apparently just excited to show off her art skills.
Consequently, the Elves are having trouble reading the values on the
document.

The newly-improved calibration document consists of lines of text; each
line originally contained a specific calibration value that the Elves now
need to recover. On each line, the calibration value can be found by
combining the first digit and the last digit (in that order) to form a
single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15,
and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are
actually spelled out with letters: one, two, three, four, five, six, seven,
eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and
last digit on each line. For example:

 */

use std::fs;

fn main() {
    let calibration_file = fs::read_to_string("./inputs/01_trebuchet.txt").unwrap();
    let calibration_document = calibration_file.lines();
    println!("{}", get_calibration_sum_part1(calibration_document.clone()));
    println!("{}", get_calibration_sum_part2(calibration_document));
}

fn get_calibration_sum_part1<'a>(calibration_document: impl Iterator<Item=&'a str>) -> usize {
    // Part 1 solution: use a simple is_numeric to find the first numeric character (and similarly for the last character)
    calibration_document.map(|line| {
        let first_number = line.chars().find(|x| x.is_numeric())
            .map_or(0, |x| (x as usize) - ('0' as usize));
        let last_number = line.chars().rfind(|x| x.is_numeric())
            .map_or(0, |x| (x as usize) - ('0' as usize));
        first_number * 10 + last_number
    }).sum()
}

// Set of numbers used to map for the part 2
const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_calibration_sum_part2<'a>(calibration_document: impl Iterator<Item=&'a str>) -> usize {
    // Part 2 solution: use a more complex function to determine what the "numeric value" of a given character is
    // This is run both forwards and backwards on the line to get the first and last number
    calibration_document.map(|line| {
        let search_fn = |(i, c): (usize, char)| {
            if c.is_numeric() {
                Some((c as usize) - ('0' as usize))
            } else {
                NUMBERS.iter()
                    .enumerate()
                    .find_map(|(j, number)|
                        if line[i..].starts_with(number) {
                            Some(j + 1)
                        } else {
                            None
                        }
                    )
            }
        };
        let first_number = line.char_indices().find_map(search_fn).unwrap_or(0);
        let last_number = line.char_indices().rev().find_map(search_fn).unwrap_or(0);
        first_number * 10 + last_number
    }).sum()
}

#[test]
fn test_part1() {
    assert_eq!(142,
               get_calibration_sum_part1(
                   r"1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet".lines()
               )
    )
}

#[test]
fn test_part2() {
    assert_eq!(281,
               get_calibration_sum_part2(
                   r"two1nine
                   eightwothree
                   abcone2threexyz
                   xtwone3four
                   4nineeightseven2
                   zoneight234
                   7pqrstsixteen".lines()
               )
    )
}
