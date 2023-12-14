/*
You use the hang glider to ride the hot air from Desert Island all the way
up to the floating metal island. This island is surprisingly cold and there
definitely aren't any thermals to glide on, so you leave your hang glider
behind.

You wander around for a while, but you don't find any people or animals.
However, you do occasionally find signposts labeled "Hot Springs" pointing
in a seemingly consistent direction; maybe you can find someone at the hot
springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal.
As you stop to admire some metal grass, you notice something metallic
scurry away in your peripheral vision and jump into a big pipe! It didn't
look like any animal you've ever seen; if you want a better look, you'll
need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is
densely packed with pipes; it was hard to tell at first because they're the
same metallic silver color as the "ground". You make a quick sketch of all
of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this
tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe
that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....

If the animal had entered this loop in the northwest corner, the sketch
would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....

In the above diagram, the S tile is still a 90-degree F bend: you can tell
because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop!
This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF

In the above diagram, you can still figure out which pipes form the main
loop: they're the ones connected to S, pipes those pipes connect to, pipes
those pipes connect to, and so on. Every pipe in the main loop connects to
its two neighbors (including S, which will have exactly two pipes
connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

If you want to get out ahead of the animal, you should find the tile in the
loop that is farthest from the starting position. Because the animal is in
the pipe, it doesn't make sense to measure this by direct distance.
Instead, you need to find the tile that would take the longest number of
steps along the loop to reach from the starting point - regardless of which
way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....

You can count the distance each tile in the loop is from the starting point
like this:

.....
.012.
.1.3.
.234.
.....

In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...

Find the single giant loop starting at S. How many steps along the loop
does it take to get from the starting position to the point farthest from
the starting position?

--- Part Two ---

You quickly reach the farthest point of the loop, but the animal never
emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a
nest, you should calculate how many tiles are contained within the loop.
For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

The above loop encloses merely four tiles - the two pairs of . in the
southwest and southeast (marked I below). The middle . tiles (marked O
below) are not in the loop. Here is the same loop again with those regions
marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....

In fact, there doesn't even need to be a full tile path to the outside for
tiles to count as outside the loop - squeezing between pipes is also
allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........

In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...

The above sketch has many random bits of ground, some of which are in the
loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO

In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by
the loop. Here's another example with many bits of junk pipe lying around
that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the
area within the loop. How many tiles are enclosed by the loop?


 */

use std::fs;
use std::hash::Hash;

fn main() {
    let file = fs::read_to_string("./inputs/10_pipe.txt").unwrap();
    let pipe_map = file.lines();
    println!("{}", get_furthest_distance(pipe_map.clone()));
    println!("{}", get_enclosed_tiles(pipe_map));
}

/// Describes a position on the map
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    /// Move this position by one unit in a given direction
    fn move_in(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.row -= 1,
            Direction::South => self.row += 1,
            Direction::East => self.column += 1,
            Direction::West => self.column -= 1,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Describes the connections offered by a given tile
#[derive(Eq, PartialEq, Clone, Debug)]
struct Connection {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl From<char> for Connection {
    /// Get the connection from a character
    fn from(value: char) -> Self {
        match value {
            '|' => Connection { north: true, east: false, south: true, west: false },
            '-' => Connection { north: false, east: true, south: false, west: true },
            'L' => Connection { north: true, east: true, south: false, west: false },
            'J' => Connection { north: true, east: false, south: false, west: true },
            '7' => Connection { north: false, east: false, south: true, west: true },
            'F' => Connection { north: false, east: true, south: true, west: false },
            '.' => Connection { north: false, east: false, south: false, west: false },
            'S' => Connection { north: false, east: false, south: false, west: false }, // Will be determined later
            _ => panic!("Unknown Map Tile {value}")
        }
    }
}

/// Metadata for connected components used in part 2
#[derive(Debug, Clone)]
struct ComponentMetadata {
    enclosed: bool,
    count: usize,
}

/// Preprocess the input map into a more useful format
fn preprocess_map<'a>(pipe_map: impl Iterator<Item=&'a str>) -> (Vec<Vec<Connection>>, Position) {
    // We want to convert the characters into lists of our Connection data structures
    // At the same time we want to remember the starting position since this process will wipe that
    // data if we don't
    let mut starting_position = Position { row: 0, column: 0 };
    let mut map: Vec<Vec<Connection>> = pipe_map.enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, char)| {
            if char == 'S' {
                starting_position = Position { row: i, column: j };
            }
            char.into()
        }).collect()
    }).collect();

    // Now compute the connectivity for the starting position
    // Basically if the tile exists in the given direction and that tile is pointing into this one
    // then we say a connection should exist
    let connects_north = starting_position.row > 0 && map[starting_position.row - 1][starting_position.column].south;
    let connects_south = starting_position.row < (map.len() - 1) && map[starting_position.row + 1][starting_position.column].north;
    let connects_west = starting_position.column > 0 && map[starting_position.row][starting_position.column - 1].east;
    let connects_east = starting_position.column < (map[0].len() - 1) && map[starting_position.row][starting_position.column + 1].west;

    // Set the starting tile as we determined it
    map[starting_position.row][starting_position.column] = Connection {
        north: connects_north,
        east: connects_east,
        south: connects_south,
        west: connects_west,
    };

    (map, starting_position)
}

/// Determine the next direction we should move based on connectivity and the last direction we moved
/// We want to prevent going back the way we came
fn get_next_direction(last_direction: Option<Direction>, connection: &Connection) -> Direction {
    if last_direction != Some(Direction::South) && connection.north {
        Direction::North
    } else if last_direction != Some(Direction::North) && connection.south {
        Direction::South
    } else if last_direction != Some(Direction::West) && connection.east {
        Direction::East
    } else {
        Direction::West
    }
}

/// Get the length of the loop
fn get_loop_length(map: Vec<Vec<Connection>>, starting_position: Position) -> usize {
    // The loop length is guaranteed to be even
    // Each step north requires a corresponding step south
    // same argument for east/west

    // This will keep walking along this loop until we get back to the starting position
    let mut position = starting_position;
    let mut length = 0;

    // Pick a direction to start moving in
    let mut last_direction = get_next_direction(None, &map[position.row][position.column]);
    position.move_in(last_direction);
    length += 1;

    while position != starting_position {
        // Get the next direction. This must not be cause us to go the opposite that we just walked
        last_direction = get_next_direction(Some(last_direction), &map[position.row][position.column]);
        position.move_in(last_direction);
        length += 1;
    }

    length
}

/// Part 1
fn get_furthest_distance<'a>(pipe_map: impl Iterator<Item=&'a str>) -> usize {
    let (map, start) = preprocess_map(pipe_map);
    let loop_length = get_loop_length(map, start);
    loop_length / 2
}

/// Generate a map with all pipes which are not part of the loop removed
fn remove_non_loop(map: Vec<Vec<Connection>>, starting_position: Position) -> Vec<Vec<Connection>> {
    // Start out with a blank map and copy the loop into it as we walk along it
    // (Basically part 1 solution but instead
    let mut new_map = vec![vec![Connection {
        north: false,
        east: false,
        south: false,
        west: false,
    }; map[0].len()]; map.len()];
    let mut position = starting_position;

    // Pick a direction to start moving in
    let mut last_direction = get_next_direction(None, &map[position.row][position.column]);
    new_map[position.row][position.column] = map[position.row][position.column].clone(); // This is part of the loop so keep its data
    position.move_in(last_direction);

    while position != starting_position {
        // Get the next direction. This must not be cause us to go the opposite that we just walked
        last_direction = get_next_direction(Some(last_direction), &map[position.row][position.column]);
        new_map[position.row][position.column] = map[position.row][position.column].clone(); // This is part of the loop so keep its data
        position.move_in(last_direction);
    }

    new_map
}

/// Convert the connection map into an "occupation" map
/// The goal here is to go from a map of multiple types of tiles to only one: blocked or free
/// This makes traversal and algorithms significantly easier
/// Unfortunately this occupation map has twice the width and height of the original map
fn occupation_map(map: Vec<Vec<Connection>>) -> Vec<Vec<bool>> {
    let mut new_map = vec![];
    map.into_iter().for_each(|row| {
        let mut row1 = vec![];
        let mut row2 = vec![];
        row.into_iter().for_each(|tile| {
            row1.push(tile.north || tile.south || tile.east || tile.west);
            row1.push(tile.east);
            row2.push(tile.south);
            row2.push(false);
        });
        new_map.push(row1);
        new_map.push(row2);
    });

    new_map
}

/// This is a depth-first-search visit
/// A breadth-first-search would likely allow for larger maps before causing a stack overflow, but
/// this was sufficient
fn visit(map: &Vec<Vec<bool>>, components: &mut Vec<Vec<usize>>, position: Position, component: usize, metadata: &mut ComponentMetadata) {
    // If this tile is free and it has not been explored yet
    if !map[position.row][position.column] && components[position.row][position.column] == 0 {
        // Mark it as explored
        components[position.row][position.column] = component;

        if position.row % 2 == 0 && position.column % 2 == 0 {
            // If this is a real tile (and since the tile is free)
            // That means this corresponds to a real position in the map
            metadata.count += 1;
        }

        // Continue the search for the 4-adjacent tiles
        // If we hit the edge of the map then mark this component as not enclosed
        if position.row > 0 {
            visit(map, components, Position {
                row: position.row - 1,
                column: position.column,
            }, component, metadata);
        } else {
            metadata.enclosed = false;
        }
        if position.column > 0 {
            visit(map, components, Position {
                row: position.row,
                column: position.column - 1,
            }, component, metadata);
        } else {
            metadata.enclosed = false;
        }
        if position.row < components.len() - 1 {
            visit(map, components, Position {
                row: position.row + 1,
                column: position.column,
            }, component, metadata);
        } else {
            metadata.enclosed = false;
        }
        if position.column < components[0].len() - 1 {
            visit(map, components, Position {
                row: position.row,
                column: position.column + 1,
            }, component, metadata);
        } else {
            metadata.enclosed = false;
        }
    }
}

/// Solve part 2
/// The map is assumed to only contain a single pipe loop
fn get_loop_enclosed(map: Vec<Vec<Connection>>, starting_position: Position) -> usize {
    // Inflate this to an "occupation" map (double width and height to make it easier to traverse)
    let inflated = occupation_map(map);
    // The position needs to be corrected as well
    let starting_position = Position {
        row: 2 * starting_position.row,
        column: 2 * starting_position.column,
    };

    // Some space to store component data
    // Four are used even though only two will ever be present to prevent logic for choosing the
    // next one to use
    let mut components = vec![vec![0_usize; inflated[0].len()]; inflated.len()];
    let mut component_metadata = vec![ComponentMetadata {
        enclosed: true,
        count: 0,
    }; 4];

    // Run a search starting from each guaranteed free spot
    if starting_position.row > 0 {
        if starting_position.column > 0 {
            visit(&inflated, &mut components, Position {
                row: starting_position.row - 1,
                column: starting_position.column - 1,
            }, 1, &mut component_metadata[0]);
        }
        if starting_position.column < inflated[0].len() - 1 {
            visit(&inflated, &mut components, Position {
                row: starting_position.row - 1,
                column: starting_position.column + 1,
            }, 2, &mut component_metadata[1]);
        }
    }
    if starting_position.row < inflated.len() - 1 {
        if starting_position.column > 0 {
            visit(&inflated, &mut components, Position {
                row: starting_position.row + 1,
                column: starting_position.column - 1,
            }, 3, &mut component_metadata[2]);
        }
        if starting_position.column < inflated[0].len() - 1 {
            visit(&inflated, &mut components, Position {
                row: starting_position.row + 1,
                column: starting_position.column + 1,
            }, 4, &mut component_metadata[3]);
        }
    }

    // Get the largest enclosed count
    // Since enclosed is the default state this max is required
    component_metadata.iter()
        .filter(|metadata| metadata.enclosed)
        .map(|metadata| metadata.count)
        .max()// Using max since the default is enclosed (as we discover that we aren't enclosed later)
        .unwrap_or(0)
}

fn get_enclosed_tiles<'a>(pipe_map: impl Iterator<Item=&'a str>) -> usize {
    let (map, start) = preprocess_map(pipe_map);
    let map = remove_non_loop(map, start);
    get_loop_enclosed(map, start)
}

#[test]
fn test_part1a() {
    assert_eq!(
        4,
        get_furthest_distance(
            r".....
.S-7.
.|.|.
.L-J.
.....".lines()
        )
    )
}

#[test]
fn test_part1b() {
    assert_eq!(
        8,
        get_furthest_distance(
            r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...".lines()
        )
    )
}

#[test]
fn test_part2a() {
    assert_eq!(
        4,
        get_enclosed_tiles(
            r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".lines()
        )
    )
}

#[test]
fn test_part2b() {
    assert_eq!(
        4,
        get_enclosed_tiles(
            r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........".lines()
        )
    )
}

#[test]
fn test_part2c() {
    assert_eq!(
        8,
        get_enclosed_tiles(
            r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...".lines()
        )
    )
}

#[test]
fn test_part2d() {
    assert_eq!(
        10,
        get_enclosed_tiles(
            r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".lines()
        )
    )
}
