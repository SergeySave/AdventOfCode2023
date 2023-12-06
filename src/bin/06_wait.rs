/*

The ferry quickly brings you across Island Island. After asking around, you
discover that there is indeed normally a large pile of sand somewhere near
here, but you don't see anything besides lots of water and the small island
where the ferry has docked.

As you try to figure out what to do next, you notice a poster on a wall
near the ferry dock. "Boat races! Open to the public! Grand prize is an
all-expenses-paid trip to Desert Island!" That must be where the sand comes
from! Best of all, the boat races are starting in just a few minutes.

You manage to sign up as a competitor in the boat races just in time. The
organizer explains that it's not really a traditional race - instead, you
will get a fixed amount of time during which your boat has to travel as far
as it can, and you win if your boat goes the farthest.

As part of signing up, you get a sheet of paper (your puzzle input) that
lists the time allowed for each race and also the best distance ever
recorded in that race. To guarantee you win the grand prize, you need to
make sure you go farther in each race than the current record holder.

The organizer brings you over to the area where the boat races are held.
The boats are much smaller than you expected - they're actually toy boats,
each with a big button on top. Holding down the button charges the boat,
and releasing the button allows the boat to move. Boats move faster if
their button was held longer, but time spent holding the button counts
against the total race time. You can only hold the button at the start of
the race, and boats don't move until the button is released.

For example:

Time:      7  15   30
Distance:  9  40  200

This document describes three races:

- The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
- The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
- The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.

Your toy boat has a starting speed of zero millimeters per millisecond. For
each whole millisecond you spend at the beginning of the race holding down
the button, the boat's speed increases by one millimeter per millisecond.

So, because the first race lasts 7 milliseconds, you only have a few options:

- Don't hold the button at all (that is, hold it for 0 milliseconds) at
the start of the race. The boat won't move; it will have traveled 0
millimeters by the end of the race.
- Hold the button for 1 millisecond at the start of the race. Then, the
boat will travel at a speed of 1 millimeter per millisecond for 6
milliseconds, reaching a total distance traveled of 6 millimeters.
- Hold the button for 2 milliseconds, giving the boat a speed of 2
millimeters per millisecond. It will then get 5 milliseconds to move,
reaching a total distance of 10 millimeters.
- Hold the button for 3 milliseconds. After its remaining 4 milliseconds
of travel time, the boat will have gone 12 millimeters.
- Hold the button for 4 milliseconds. After its remaining 3 milliseconds
of travel time, the boat will have gone 12 millimeters.
- Hold the button for 5 milliseconds, causing the boat to travel a total
of 10 millimeters.
- Hold the button for 6 milliseconds, causing the boat to travel a total
of 6 millimeters.
- Hold the button for 7 milliseconds. That's the entire duration of the
race. You never let go of the button. The boat can't move until you
let you of the button. Please make sure you let go of the button so
the boat gets to move. 0 millimeters.

Since the current record for this race is 9 millimeters, there are actually
4 different ways you could win: you could hold the button for 2, 3, 4, or
5 milliseconds at the start of the race.

In the second race, you could hold the button for at least 4 milliseconds
and at most 11 milliseconds and beat the record, a total of 8 different
ways to win.

In the third race, you could hold the button for at least 11 milliseconds
and no more than 19 milliseconds and still beat the record, a total of 9
ways you could win.

To see how much margin of error you have, determine the number of ways you
can beat the record in each race; in this example, if you multiply these
values together, you get 288 (4 * 8 * 9).

Determine the number of ways you could beat the record in each race. What
do you get if you multiply these numbers together?

--- Part Two ---

As the race is about to start, you realize the piece of paper with race
times and record distances you got earlier actually just has very bad
kerning. There's really only one race - ignore the spaces between the
numbers on each line.

So, the example from before:

Time:      7  15   30
Distance:  9  40  200

...now instead means this:

Time:      71530
Distance:  940200

Now, you have to figure out how many ways there are to win this single
race. In this example, the race lasts for 71530 milliseconds and the record
distance you need to beat is 940200 millimeters. You could hold the button
anywhere from 14 to 71516 milliseconds and beat the record, a total of
71503 ways!

How many ways can you beat the record in this one much longer race?
 */

use std::fs;
use std::str::FromStr;

fn main() {
    let file = fs::read_to_string("./inputs/06_wait.txt").unwrap();
    let races = file.lines();
    println!("{}", get_margin_product(races.clone()));
    println!("{}", get_combined_race(races));
}

fn get_next_row_numbers<'a>(races: &mut (impl Iterator<Item=&'a str> + Sized)) -> Vec<usize> {
    // Get all of the numbers separated by whitespace on the next row
    races.next().unwrap()
        .split_whitespace()
        .skip(1) // Skip the "header"/name of this variable type
        .map(|time| usize::from_str(time).unwrap())
        .collect::<Vec<usize>>()
}

fn binary_search_required_time(total_time: usize, optimal_time: usize, distance: usize) -> usize {
    // Uses a binary search to determine the minimum required amount of time to achieve at least
    // distance on this race

    // This probably isn't needed but its just a poison pill for if a race can't be won
    let d = optimal_time * (total_time - optimal_time);
    if d < distance {
        return 0;
    }

    // Simple binary search
    let mut min_time = 0;
    let mut max_time = optimal_time;

    while min_time < max_time - 1 {
        let guess = (min_time + max_time) / 2;
        let guess_distance = guess * (total_time - guess);
        if guess_distance < distance {
            min_time = guess;
        } else {
            max_time = guess;
        }
    }

    max_time
}

fn get_margin_product<'a>(mut races: impl Iterator<Item=&'a str>) -> usize {
    // Get all of the times
    let times = get_next_row_numbers(&mut races);
    // Get all of the distance
    let distances = get_next_row_numbers(&mut races);

    // Loop over each time, distance pair
    times.into_iter().zip(distances).map(|(time, distance)|  {
        // The optimal time is half of the total time
        // Proof:
        // distance = button_time * (total_time - button_time)
        // distance = total_time * button_time - button_time^2
        // We want argmax(distance), so compute the derivative:
        // d distance / d button_time = total_time - 2 * button_time
        // 0 = total_time - 2 * button_time
        // 2 * button_time = total_time
        // button_time = total_time / 2

        // // If time is odd the optimal time isn't an integer
        // // However, since this parabola is symmetric, we can round down by one and the value is one of the two max values
        // let optimal_button_time = time / 2;
        // // Find the smallest time which is greater than the distance using a binary search
        // let min_time = binary_search_required_time(time, optimal_button_time, distance + 1); // want to beat distance
        // // Compute the amount of times are at least as much as the time we found
        // // This is taking advantage of the symmetry of the parabola
        // // Get the amount of times between (and including both) optimal and min
        // // Double this amount (the parabola is symmetric)
        // // And if we happened to have an even number (where there is a center to the parabola)
        // // this would have double counted the optimal_button_time - so we need to subtract 1 to fix
        // // the answer
        // let time_amount = (optimal_button_time - min_time + 1) * 2 - if time % 2 == 0 { 1 } else { 0 };

        math_time_count(time, distance)
    }).product() // Get the product of all of the values
}

fn get_next_row_as_number<'a>(races: &mut (impl Iterator<Item=&'a str> + Sized)) -> usize {
    usize::from_str(&races.next().unwrap()
        .chars() // Get all of the characters
        .filter(|x| x.is_numeric()) // grab only the numbers
        .collect::<String>() // Join it all into a string
    ).unwrap()
}

fn get_combined_race<'a>(mut races: impl Iterator<Item=&'a str>) -> usize {
    // Get the number from the first row
    let time = get_next_row_as_number(&mut races);
    // Get the number from the second row
    let distance = get_next_row_as_number(&mut races);

    // // Perform the same operation as we did above for the same reasons but this time only using
    // // this single long race
    // // Due to the fact that this is using a binary search this is very efficient this takes almost
    // // no time at all even with very large inputs
    // let optimal_button_time = time / 2;
    // let min_time = binary_search_required_time(time, optimal_button_time, distance + 1); // want to beat distance
    // let time_amount = (optimal_button_time - min_time + 1) * 2 - if time % 2 == 0 { 1 } else { 0 };
    //
    // time_amount

    math_time_count(time, distance)
}

/// A mathematical/analytical solution to this (rather than the numerical/iterative approach
/// originally used to solve this via binary search)
fn math_time_count(total_time: usize, distance_to_beat: usize) -> usize {
    // Parabola for the total travel distance based on time the button is held
    // distance = time * (total_time - time)
    // distance = time * total_time - time^2
    // y = time^2 - total_time * time

    // Let's find the intersection with
    // y = -distance_to_beat

    // distance_to_beat = time^2 - total_time * time
    // 0 = time^2 - total_time * time + distance_to_beat

    // Applying the quadratic formula
    // time = (total_time +- sqrt(total_time^2 - 4 * distance_to_beat )) / 2

    // So we have two times where these lines cross:
    // t1 = (total_time - sqrt(total_time^2 - 4 * distance_to_beat )) / 2
    // t1 = (total_time / 2) - sqrt(total_time^2 - 4 * distance_to_beat ) / 2
    // t2 = (total_time + sqrt(total_time^2 - 4 * distance_to_beat )) / 2
    // t2 = (total_time / 2) + sqrt(total_time^2 - 4 * distance_to_beat ) / 2

    // We care about the number of integers between these two values
    // number_of_integers = floor(t2) - ceil(t1) + 1
    let a = (total_time as f64) / 2.0;
    let b = ((total_time.pow(2) - 4 * distance_to_beat) as f64).sqrt() / 2.0;

    let t1 = a - b;
    let t2 = a + b;

    let t2_floor = t2.floor();

    (t2_floor as usize) - (t1.floor() as usize) - if t2 == t2_floor { 1 } else { 0 }
}

#[test]
fn test_part1() {
    assert_eq!(
        288,
        get_margin_product(
            r"Time:      7  15   30
Distance:  9  40  200".lines()
        )
    )
}


#[test]
fn test_part2() {
    assert_eq!(
        71503,
        get_combined_race(
            r"Time:      7  15   30
Distance:  9  40  200".lines()
        )
    )
}