/*
You manage to catch the airship right as it's dropping someone else off on
their all-expenses-paid trip to Desert Island! It even helpfully drops you
off near the gardener and his massive farm.

"You got the sand flowing again! Great work! Now we just need to wait until
we have enough sand to filter the water for Snow Island and we'll have snow
again in no time."

While you wait, one of the Elves that works with the gardener heard how
good you are at solving problems and would like your help. He needs to get
his steps in for the day, and so he'd like to know which garden plots he
can reach with exactly his remaining 64 steps.

He gives you an up-to-date map (your puzzle input) of his starting position
(S), garden plots (.), and rocks (#). For example:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........

The Elf starts at the starting position (S) which also counts as a garden
plot. Then, he can take one step north, south, east, or west, but only onto
tiles that are garden plots. This would allow him to reach any of the tiles
marked O:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........

Then, he takes a second step. Since at this point he could be at either
tile marked O, his second step would allow him to reach any garden plot
that is one step north, south, east, or west of any tile that he could have
reached after the first step:

...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........

After two steps, he could be at any of the tiles marked O above, including
the starting position (either by going north-then-south or by going west-
then-east).

A single third step leads to even more possibilities:

...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........

He will continue like this until his steps for the day have been exhausted.
After a total of 6 steps, he could reach any of the garden plots marked O:

...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........

In this example, if the Elf's goal was to get exactly 6 more steps today,
he could use them to reach any of 16 garden plots.

However, the Elf actually needs to get 64 steps today, and the map he's
handed you is much larger than the example map.

Starting from the garden plot marked S on your map, how many garden plots
could the Elf reach in exactly 64 steps?

--- Part Two ---

The Elf seems confused by your answer until he realizes his mistake: he was
reading from a list of his favorite numbers that are both perfect squares
and perfect cubes, not his step counter.

The actual number of steps he needs to get today is exactly 26501365.

He also points out that the garden plots and rocks are set up so that the
map repeats infinitely in every direction.

So, if you were to look one additional map-width or map-height out from the
edge of the example map above, you would find that it keeps repeating:

.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................

This is just a tiny three-map-by-three-map slice of the inexplicably-
infinite farm layout; garden plots and rocks repeat as far as you can see.
The Elf still starts on the one middle tile marked S, though - every other
repeated S is replaced with a normal garden plot (.).

Here are the number of reachable garden plots in this new infinite version
of the example map for different numbers of steps:

- In exactly 6 steps, he can still reach 16 garden plots.
- In exactly 10 steps, he can reach any of 50 garden plots.
- In exactly 50 steps, he can reach 1594 garden plots.
- In exactly 100 steps, he can reach 6536 garden plots.
- In exactly 500 steps, he can reach 167004 garden plots.
- In exactly 1000 steps, he can reach 668697 garden plots.
- In exactly 5000 steps, he can reach 16733044 garden plots.

However, the step count the Elf needs is much larger! Starting from the
garden plot marked S on your infinite map, how many garden plots could the
Elf reach in exactly 26501365 steps?
 */

use std::collections::{HashMap, VecDeque};
use std::fs;

use num::Integer;

fn main() {
    let file = fs::read_to_string("./inputs/21_step_counter.txt").unwrap();
    let map = file.lines();
    println!("{}", get_destination_count(map.clone(), 64));
    println!("{}", get_wrapping_destination_count(map, 26501365));
}

/// Represents a tile on the map
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Garden,
    Rocks,
}

/// Represents a position in the world
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    /// Get the tile for this position
    /// If this is not a valid position then None will be returned
    fn get_tile(&self, tiles: &Vec<Vec<Tile>>) -> Option<Tile> {
        if self.x < 0 || self.y < 0 {
            return None;
        } else if self.y as usize >= tiles.len() || self.x as usize >= tiles[0].len() {
            return None;
        }
        return Some(tiles[self.y as usize][self.x as usize]);
    }
    /// Get the list of four-adjacent directions
    const fn adjacent_directions() -> [(isize, isize); 4] {
        [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ]
    }
    /// Get the four-adjacent positions
    fn adjacent(&self) -> [Self; 4] {
        Self::adjacent_directions().map(|(x, y)| {
            Position {
                x: self.x + x,
                y: self.y + y,
            }
        })
    }
}

/// Preprocess the input into a more useful form
fn preprocess<'a>(map: impl Iterator<Item=&'a str>) -> (Vec<Vec<Tile>>, Position) {
    let mut position = Position {
        x: -1,
        y: -1,
    };
    let result = map.enumerate().map(|(y, row)| {
        row.char_indices().map(|(x, c)| match c {
            '.' => Tile::Garden,
            '#' => Tile::Rocks,
            'S' => {
                position = Position { x: x as isize, y: y as isize };
                Tile::Garden
            }
            _ => panic!("Unknown Tile {c}")
        }).collect()
    }).collect();

    (result, position)
}

/// Compute the distances to all positions from a given start position with a given max distance
fn compute_distances(map: &Vec<Vec<Tile>>, start: Position, max_distance: usize) -> HashMap<Position, usize> {
    // Dijkstra's algorithm
    let mut queue = VecDeque::new();
    queue.push_back((start, 0_usize));

    let mut distances = HashMap::<Position, usize>::new();
    while let Some((position, distance)) = queue.pop_front() {
        if distance > max_distance { // Don't care when its greater than max
            continue;
        }
        if distances.contains_key(&position) { // Only visit unvisited tiles
            continue;
        }
        let Some(tile) = position.get_tile(map) else { continue; };
        if tile == Tile::Rocks { // We only care about rocks
            continue;
        }
        position.adjacent().into_iter().for_each(|p| {
            queue.push_back((p, distance + 1));
        });
        distances.insert(position, distance);
    }

    distances
}

/// Solve part1
fn get_destination_count<'a>(map: impl Iterator<Item=&'a str>, max_distance: usize) -> usize {
    let (map, start) = preprocess(map);
    let distances = compute_distances(&map, start, max_distance);

    distances.into_values()
        .filter(|distance| *distance % 2 == 0)
        .count()
}

/// Get the list of total spots for a given position and map which are a valid stopping spot when
/// taking `steps` steps
fn get_total_spots(start: Position, map: &Vec<Vec<Tile>>, steps: usize) -> usize {
    compute_distances(&map, start, steps).into_values()
        .filter(|x| *x <= steps)
        .filter(|x| (steps - *x).is_even())
        .count()
}

/// Solve part2
fn get_wrapping_destination_count<'a>(map: impl Iterator<Item=&'a str>, max_distance: usize) -> usize {
    // For the input we need to solve, the row and column where we start are both completely gardens
    // Additionally the borders to the shape are completely gardens
    // Also the grid is a square with odd size and the start position is in the center of the map
    // Since the grid is mostly sparse most copies of the map are identical/repeated
    // In fact there are only 14 different grids
    // - Ones where we can fully visit and stop in the center/start position
    // - Ones where we can fully visit and cannot stop in the center/start position
    // - The four "corners" (one per direction)
    // - The four "big" border (one for adjacent pairs of directions)
    // - The four "small" border (one for adjacent pairs of directions)
    let (map, start) = preprocess(map);
    let map_size = map.len();

    // The radius of fully explorable repetitions of the area (assuming the area is relatively sparse)
    let radius = max_distance / map_size - 1;

    // Some common travel distances
    let corner_distance = max_distance - radius * map_size - map_size / 2 - 1;
    let big_distance = corner_distance + map_size - 1 - map_size / 2;
    let small_distance = corner_distance - map_size / 2 - 1;

    let even_spots = get_total_spots(start.clone(), &map, map_size * 2);
    let odd_spots = get_total_spots(start.clone(), &map, map_size * 2 + 1);

    let big_top_left_spots = get_total_spots(Position { x: map_size as isize - 1, y: map_size as isize - 1 }, &map, big_distance);
    let small_top_left_spots = get_total_spots(Position { x: map_size as isize - 1, y: map_size as isize - 1 }, &map, small_distance);
    let top_corner_spots = get_total_spots(Position { x: start.x, y: map_size as isize - 1 }, &map, corner_distance);

    let big_top_right_spots = get_total_spots(Position { x: 0, y: map_size as isize - 1 }, &map, big_distance);
    let small_top_right_spots = get_total_spots(Position { x: 0, y: map_size as isize - 1 }, &map, small_distance);
    let right_corner_spots = get_total_spots(Position { x: 0, y: start.y }, &map, corner_distance);

    let big_bottom_right_spots = get_total_spots(Position { x: 0, y: 0 }, &map, big_distance);
    let small_bottom_right_spots = get_total_spots(Position { x: 0, y: 0 }, &map, small_distance);
    let bottom_corner_spots = get_total_spots(Position { x: start.x, y: 0 }, &map, corner_distance);

    let big_bottom_left_spots = get_total_spots(Position { x: map_size as isize - 1, y: 0 }, &map, big_distance);
    let small_bottom_left_spots = get_total_spots(Position { x: map_size as isize - 1, y: 0 }, &map, small_distance);
    let left_corner_spots = get_total_spots(Position { x: map_size as isize - 1, y: start.y }, &map, corner_distance);

    let even_repetition_count = (((radius + 1) / 2) * 2).pow(2);
    let odd_repetition_count = ((radius / 2) * 2 + 1).pow(2);
    let big_border_count = radius;
    let small_border_count = radius + 1;

    even_spots * even_repetition_count
        + odd_spots * odd_repetition_count
        + big_border_count * (big_top_left_spots + big_top_right_spots + big_bottom_right_spots + big_bottom_left_spots)
        + small_border_count * (small_top_left_spots + small_top_right_spots + small_bottom_right_spots + small_bottom_left_spots)
        + top_corner_spots + right_corner_spots + bottom_corner_spots + left_corner_spots
}

#[test]
fn test_part1() {
    assert_eq!(
        16,
        get_destination_count(
            r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........".lines(),
            6,
        )
    );
}

// #[test]
// fn test_part2a() {
//     assert_eq!(
//         16,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             6
//         )
//     );
// }
//
// #[test]
// fn test_part2b() {
//     assert_eq!(
//         50,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             10
//         )
//     );
// }
//
// #[test]
// fn test_part2c() {
//     assert_eq!(
//         1594,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             50
//         )
//     );
// }
//
// #[test]
// fn test_part2d() {
//     assert_eq!(
//         6536,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             100
//         )
//     );
// }
//
// #[test]
// fn test_part2e() {
//     assert_eq!(
//         167004,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             500
//         )
//     );
// }
//
// #[test]
// fn test_part2f() {
//     assert_eq!(
//         668697,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             1000
//         )
//     );
// }
//
//
// #[test]
// fn test_part2g() {
//     assert_eq!(
//         16733044,
//         get_wrapping_destination_count(
//             r"...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........".lines(),
//             5000
//         )
//     );
// }

/*
An earlier attempt at a part 2 solution

fn get_total_repetitions(start: Position, direction: (isize, isize), distances: &HashMap<Position, usize>, size: usize, max_distance: usize) -> usize {
    if !distances.contains_key(&start) { // If we have no data (its a rock)
        return 0;
    }
    let mut position = start;
    position.x += direction.0 * size as isize;
    position.y += direction.1 * size as isize;
    let mut count = 0_usize;
    let mut last_distance = max_distance;
    loop {
        let Some(distance) = distances.get(&position) else { break; };
        if last_distance + size == *distance { // If we stepped the board size since last time
            break; // No more need to continue looping
        }
        if distance.is_even() == max_distance.is_even() && *distance <= max_distance {
            count += 1;
        }
        position.x += direction.0 * size as isize;
        position.y += direction.1 * size as isize;
        last_distance = *distance;
    }

    // we did count steps and now are at a distance of last_distance
    // additional steps are now size distance each
    // how many more steps can we do while remaining under max_distance?
    let additional_steps = if last_distance.is_even() == max_distance.is_even() {
        if size.is_even() {
            max_distance.saturating_sub(last_distance) / size
        } else {
            max_distance.saturating_sub(last_distance) / (size)
        }
    } else {
        if size.is_even() {
            0
        } else {
            max_distance.saturating_sub(last_distance - size) / (size)
        }
    };
    println!("{}", additional_steps);

    count + additional_steps
}

fn get_wrong_walk_distance(start: Position, direction: (isize, isize), distances: &HashMap<Position, usize>, size: usize) -> usize {
    let mut position = start;
    let mut count = 0;
    let mut last_distance = usize::MAX;
    loop {
        let Some(distance) = distances.get(&position) else { break; };
        if last_distance.saturating_add(size) == *distance { // If we stepped the board size since last time
            break; // No more need to continue looping
        }
        count += 1;
        position.x -= direction.0 * size as isize;
        position.y -= direction.1 * size as isize;
        last_distance = *distance;
    }
    count
}

fn get_real_area(start: Position, distances: &HashMap<Position, usize>, size: (usize, usize)) -> (usize, usize, usize, usize) {
    let left = get_wrong_walk_distance(start.clone(), (-1, 0), distances, size.0);
    let right = get_wrong_walk_distance(start.clone(), (1, 0), distances, size.0);
    let up = get_wrong_walk_distance(start.clone(), (0, -1), distances, size.1);
    let down = get_wrong_walk_distance(start.clone(), (0, 1), distances, size.1);
    let m = max(max(left, right), max(up, down)) + 2;
    (m, m, m, m)
    // (left + 1, right + 1, up + 1, down + 1)
}

fn get_real_area_count(start: Position, distances: &HashMap<Position, usize>, size: (usize, usize), max_distance: usize, real_area: (usize, usize, usize, usize)) -> usize {
    let (left, right, up, down) = real_area;
    (-(left as isize)..=(right as isize))
        .cartesian_product(-(up as isize)..=(down as isize))
        .filter(|(x, y)| {
            distances
                .get(&Position {
                    x: start.x + *x * size.0 as isize,
                    y: start.y + *y * size.1 as isize,
                })
                .map(|d| *d <= max_distance && max_distance.is_even() == d.is_even())
                .unwrap_or(false)
        })
        .count()
}

fn get_fake_repetition_count(start: Position, distances: &HashMap<Position, usize>, size: (usize, usize), max_distance: usize, real_area: (usize, usize, usize, usize)) -> usize {
    let left_right_extent = max_distance / size.0;

    let mut result = 0;
    for left_right in (-(left_right_extent as isize))..=(left_right_extent as isize) {
        let fully_fake = left_right < -(real_area.0 as isize) || (real_area.1 as isize) < left_right;
        let (bottom, top) = if fully_fake {
            let extent = if left_right < 0 { -(real_area.0 as isize) } else { real_area.1 as isize };
            let center_distance = *distances.get(&Position {
                x: start.x + (size.0 as isize) * extent,
                y: start.y,
            }).unwrap();
            let center_distance = center_distance + size.0 * left_right.abs_diff(extent);
            if center_distance <= max_distance {
                result += 1;
            }
            (center_distance, center_distance) // The last real one
        } else {
            (
                *distances.get(&Position {
                    x: start.x + (size.0 as isize) * left_right,
                    y: start.y - (size.1 as isize) * real_area.2 as isize,
                }).unwrap(),
                *distances.get(&Position {
                    x: start.x + (size.0 as isize) * left_right,
                    y: start.y + (size.1 as isize) * real_area.3 as isize,
                }).unwrap()
            )
        };

        for distance in [bottom, top] {
            result += if distance.is_even() == max_distance.is_even() {
                if size.1.is_even() {
                    max_distance.saturating_sub(distance) / size.1
                } else {
                    max_distance.saturating_sub(distance) / (size.1 * 2)
                }
            } else {
                if size.1.is_even() {
                    0
                } else {
                    max_distance.saturating_sub(distance.saturating_sub(size.1)) / (size.1 * 2)
                }
            };
        }
    }
    result
}

fn get_left_right_copies(position: Position, distances: &HashMap<Position, usize>, size: (usize, usize), max_distance: usize) -> usize {
    let (width, height) = size;

    let left_right_extent = max_distance / width;

    let mut copies = 0_usize;
    for extent in 0..left_right_extent {

    }
    copies
}

fn get_copies(position: Position, distances: &HashMap<Position, usize>, size: (usize, usize), max_distance: usize) -> usize {
    let (width, height) = size;

    let left_right_extent = max_distance / width;

    let mut copies = 0_usize;
    let mut position = position;
    let mut last_distance = usize::MAX;
    for extent in 0..left_right_extent {
        let distance = *distances.get(&position).unwrap_or_else(|| &(last_distance + width));

        // If we can get here in the remaining distance
        if distance <= max_distance && (max_distance - distance).is_even() {
            copies += 1;
        }

        last_distance = distance;
    }

    0
}
 */
