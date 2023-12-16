
/*

With the beam of light completely focused somewhere, the reindeer leads you
deeper still into the Lava Production Facility. At some point, you realize
that the steel facility walls have been replaced with cave, and the
doorways are just cave, and the floor is cave, and you're pretty sure this
is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a
bright light in a cavern up ahead. There, you discover that the beam of
light you so carefully focused is emerging from the cavern wall closest to
the facility and pouring all of its energy into a contraption on the
opposite side.

Upon closer inspection, the contraption appears to be a flat, two-
dimensional square grid containing empty space (.), mirrors (/ and \), and
splitters (| and -).

The contraption is aligned so that most of the beam bounces around the
grid, but each tile on the grid converts some of the beam's light into heat
to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....

The beam enters in the top-left corner from the left and heading to the
right. Then, its behavior depends on what it encounters as it moves:

- If the beam encounters empty space (.), it continues in the same
direction.
- If the beam encounters a mirror (/ or \), the beam is reflected 90
degrees depending on the angle of the mirror. For instance, a
rightward-moving beam that encounters a / mirror would continue upward
in the mirror's column, while a rightward-moving beam that encounters
a \ mirror would continue downward from the mirror's column.
- If the beam encounters the pointy end of a splitter (| or -), the beam
passes through the splitter as if the splitter were empty space. For
instance, a rightward-moving beam that encounters a - splitter would
continue in the same direction.
- If the beam encounters the flat side of a splitter (| or -), the beam
is split into two beams going in each of the two directions the
splitter's pointy ends are pointing. For instance, a rightward-moving
beam that encounters a | splitter would split into two beams: one that
continues upward from the splitter's column and one that continues
downward from the splitter's column.

Beams do not interact with other beams; a tile can have many beams passing
through it at the same time. A tile is energized if that tile has at least
one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the
contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..

Beams are only shown on empty tiles; arrows indicate the direction of the
beams. If a tile contains beams moving in multiple directions, the number
of distinct directions is shown instead. Here is the same diagram but
instead only showing whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..

Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the
contraption, you need to start by analyzing the current situation. With the
beam starting in the top-left heading right, how many tiles end up being
energized?

--- Part Two ---

As you try to work out what might be wrong, the reindeer tugs on your shirt
and leads you to a nearby control panel. There, a collection of buttons
lets you align the contraption so that the beam enters from any edge tile
and heading away from that edge. (You can choose either of two directions
for the beam if it starts on a corner; for instance, if the beam starts in
the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any
tile in the bottom row (heading upward), any tile in the leftmost column
(heading right), or any tile in the rightmost column (heading left). To
produce lava, you need to find the configuration that energizes as many
tiles as possible.

In the above example, this can be achieved by starting the beam in the
fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..

Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..

Find the initial beam configuration that energizes the largest number of
tiles; how many tiles are energized in that configuration?
 */

use std::cmp::max;
use std::collections::HashSet;
use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs/16_lava.txt").unwrap();
    let layout = file.lines();
    println!("{}", get_tile_energized_count(layout.clone()));
    println!("{}", get_max_tile_energized_count(layout));
}

/// An enum to describe the type of a mirror
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MirrorType {
    TopLeftBottomRight,
    TopRightBottomLeft,
}
impl MirrorType {
    /// Reflect a beam traveling in a certain direction based on this mirror type
    fn reflect(self, direction: Direction) -> Direction {
        match self {
            MirrorType::TopLeftBottomRight => match direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            }
            MirrorType::TopRightBottomLeft => match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            }
        }
    }
}

/// An enum representing an axis
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Axis {
    Vertical,
    Horizontal,
}

/// An enum representing a tile on the layout
#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Mirror(MirrorType),
    Splitter(Axis)
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::Mirror(MirrorType::TopRightBottomLeft),
            '\\' => Tile::Mirror(MirrorType::TopLeftBottomRight),
            '-' => Tile::Splitter(Axis::Vertical),
            '|' => Tile::Splitter(Axis::Horizontal),
            _ => panic!("Unknown Tile {value}")
        }
    }
}

/// A beam representing a direction
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    /// Get the axis which this direction is traveling along
    fn to_axis(self) -> Axis {
        match self {
            Direction::Up => Axis::Vertical,
            Direction::Right => Axis::Horizontal,
            Direction::Down => Axis::Vertical,
            Direction::Left => Axis::Horizontal,
        }
    }
}

/// Represents a position on the world/layout
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    /// Step this position in a given direction
    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    /// Check if this is a valid position on a world with the given size
    fn is_valid(&self, width: usize, height: usize) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }
        if (self.x as usize) >= width {
            return false;
        }
        if (self.y as usize) >= height {
            return false;
        }
        true
    }
}

/// Represents the state of a beam
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Beam {
    position: Position,
    direction: Direction,
}
impl Beam {
    /// Step this beam forward
    fn step(self) -> Self {
        Self {
            position: self.position.step(self.direction),
            direction: self.direction,
        }
    }
}

/// Preprocess the string input into a set of tiles
fn preprocess<'a>(layout: impl Iterator<Item=&'a str>) -> Vec<Vec<Tile>> {
    layout.map(|row| {
        row.chars()
            .map(|char| char.into())
            .collect()
    }).collect()
}

/// Compute the amount of energized tiles in a given layout with a given starting beam
fn compute_energized_count(layout: &Vec<Vec<Tile>>, start: Beam) -> usize {
    // The set of tiles which has been energized
    let mut energized = vec![vec![false; layout[0].len()]; layout.len()];
    // The states we gave visited (used to prune the search from revisiting states)
    let mut visited: HashSet<Beam> = HashSet::new();

    // Used as a stack of tiles which still need to be visited
    let mut to_visit = vec![start];

    // Grab the next state which has not been visited
    while let Some(mut beam) = to_visit.pop() {
        if !beam.position.is_valid(layout[0].len(), layout.len()) {
            continue; // If the beam is off the map we can ignore it
        }
        if visited.contains(&beam) {
            continue; // Don't revisit the same spot twice
        }
        visited.insert(beam.clone()); // Prevent this tile from being revisited
        energized[beam.position.y as usize][beam.position.x as usize] = true; // Energize this tile

        // Propagate the beam forward
        match layout[beam.position.y as usize][beam.position.x as usize] {
            Tile::Empty => {
                to_visit.push(beam.step());
            }
            Tile::Mirror(mirror_type) => {
                beam.direction = mirror_type.reflect(beam.direction);
                to_visit.push(beam.step());
            }
            Tile::Splitter(mirror_type) => {
                if mirror_type == beam.direction.to_axis() {
                    for direction in match mirror_type {
                        Axis::Vertical => [Direction::Left, Direction::Right],
                        Axis::Horizontal => [Direction::Up, Direction::Down],
                    } {
                        let mut new_beam = beam.clone();
                        new_beam.direction = direction;
                        to_visit.push(new_beam.step());
                    }
                } else {
                    to_visit.push(beam.step());
                }
            }
        }
    }

    // count the number of true/energized states
    energized.iter().flatten().filter(|x| **x).count()
}

/// Solve part 1
fn get_tile_energized_count<'a>(layout: impl Iterator<Item=&'a str>) -> usize {
    let layout = preprocess(layout);
    compute_energized_count(&layout, Beam {
        position: Position { x: 0, y: 0 },
        direction: Direction::Right,
    })
}

/// Solve part 2
fn get_max_tile_energized_count<'a>(layout: impl Iterator<Item=&'a str>) -> usize {
    // Just try all start states to find the best
    let layout = preprocess(layout);
    let width = layout[0].len();
    let height = layout.len();
    let right = (0..height).map(|y| compute_energized_count(&layout, Beam {
        position: Position { x: 0, y: y as isize },
        direction: Direction::Right,
    })).max().unwrap();
    let left = (0..height).map(|y| compute_energized_count(&layout, Beam {
        position: Position { x: width as isize - 1, y: y as isize },
        direction: Direction::Left,
    })).max().unwrap();
    let down = (0..width).map(|x| compute_energized_count(&layout, Beam {
        position: Position { x: x as isize, y: 0 },
        direction: Direction::Down,
    })).max().unwrap();
    let up = (0..width).map(|x| compute_energized_count(&layout, Beam {
        position: Position { x: x as isize, y: height as isize - 1 },
        direction: Direction::Up,
    })).max().unwrap();
    max(max(right, left), max(down, up))
}

#[test]
fn test_part1() {
    assert_eq!(
        46,
        get_tile_energized_count(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        51,
        get_max_tile_energized_count(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....".lines()
        )
    );
}
