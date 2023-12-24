/*
Enough sand has fallen; it can finally filter water for Snow Island.

Well, almost.

The sand has been falling as large compacted bricks of sand, piling up to
form an impressive stack here near the edge of Island Island. In order to
make use of the sand to filter water, some of the bricks will need to be
broken apart - nay, disintegrated - back into freely flowing sand.

The stack is tall enough that you'll have to be careful about choosing
which bricks to disintegrate; if you disintegrate the wrong brick, large
portions of the stack could topple, which sounds pretty dangerous.

The Elves responsible for water filtering operations took a snapshot of the
bricks while they were still falling (your puzzle input) which should let
you work out which bricks are safe to disintegrate. For example:

1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9

Each line of text in the snapshot represents the position of a single brick
at the time the snapshot was taken. The position is given as two x,y,z
coordinates - one for each end of the brick - separated by a tilde (~).
Each brick is made up of a single straight line of cubes, and the Elves
were even careful to choose a time for the snapshot that had all of the
free-falling bricks at integer positions above the ground, so the whole
snapshot is aligned to a three-dimensional cube grid.

A line like 2,2,2~2,2,2 means that both ends of the brick are at the same
coordinate - in other words, that the brick is a single cube.

Lines like 0,0,10~1,0,10 or 0,0,10~0,1,10 both represent bricks that are
two cubes in volume, both oriented horizontally. The first brick extends in
the x direction, while the second brick extends in the y direction.

A line like 0,0,1~0,0,10 represents a ten-cube brick which is oriented
vertically. One end of the brick is the cube located at 0,0,1, while the
other end of the brick is located directly above it at 0,0,10.

The ground is at z=0 and is perfectly flat; the lowest z value a brick can
have is therefore 1. So, 5,5,1~5,6,1 and 0,2,1~0,2,5 are both resting on
the ground, but 3,3,2~3,3,3 was above the ground at the time of the
snapshot.

Because the snapshot was taken while the bricks were still falling, some
bricks will still be in the air; you'll need to start by figuring out where
they will end up. Bricks are magically stabilized, so they never rotate,
even in weird situations like where a long horizontal brick is only
supported on one end. Two bricks cannot occupy the same position, so a
falling brick will come to rest upon the first other brick it encounters.

Here is the same example again, this time with each brick given a letter so
it can be marked in diagrams:

1,0,1~1,2,1   <- A
0,0,2~2,0,2   <- B
0,2,3~2,2,3   <- C
0,0,4~0,2,4   <- D
2,0,5~2,2,5   <- E
0,1,6~2,1,6   <- F
1,1,8~1,1,9   <- G

At the time of the snapshot, from the side so the x axis goes left to
right, these bricks are arranged like this:

 x
012
.G. 9
.G. 8
... 7
FFF 6
..E 5 z
D.. 4
CCC 3
BBB 2
.A. 1
--- 0

Rotating the perspective 90 degrees so the y axis now goes left to right,
the same bricks are arranged like this:

 y
012
.G. 9
.G. 8
... 7
.F. 6
EEE 5 z
DDD 4
..C 3
B.. 2
AAA 1
--- 0

Once all of the bricks fall downward as far as they can go, the stack looks
like this, where ? means bricks are hidden behind other bricks at that
location:

 x
012
.G. 6
.G. 5
FFF 4
D.E 3 z
??? 2
.A. 1
--- 0

Again from the side:

 y
012
.G. 6
.G. 5
.F. 4
??? 3 z
B.C 2
AAA 1
--- 0
Now that all of the bricks have settled, it becomes easier to tell which
bricks are supporting which other bricks:

- Brick A is the only brick supporting bricks B and C.
- Brick B is one of two bricks supporting brick D and brick E.
- Brick C is the other brick supporting brick D and brick E.
- Brick D supports brick F.
- Brick E also supports brick F.
- Brick F supports brick G.
- Brick G isn't supporting any bricks.

Your first task is to figure out which bricks are safe to disintegrate. A
brick can be safely disintegrated if, after removing it, no other bricks
would fall further directly downward. Don't actually disintegrate any
bricks - just determine what would happen if, for each brick, only that
brick were disintegrated. Bricks can be disintegrated even if they're
completely surrounded by other bricks; you can squeeze between bricks if
you need to.

In this example, the bricks can be disintegrated as follows:

- Brick A cannot be disintegrated safely; if it were disintegrated,
bricks B and C would both fall.
- Brick B can be disintegrated; the bricks above it (D and E) would
still be supported by brick C.
- Brick C can be disintegrated; the bricks above it (D and E) would
still be supported by brick B.
- Brick D can be disintegrated; the brick above it (F) would still be
supported by brick E.
- Brick E can be disintegrated; the brick above it (F) would still be
supported by brick D.
- Brick F cannot be disintegrated; the brick above it (G) would fall.
- Brick G can be disintegrated; it does not support any other bricks.

So, in this example, 5 bricks can be safely disintegrated.

Figure how the blocks will settle based on the snapshot. Once they've
settled, consider disintegrating a single brick; how many bricks could be
safely chosen as the one to get disintegrated?

--- Part Two ---

Disintegrating bricks one at a time isn't going to be fast enough. While it
might sound dangerous, what you really need is a chain reaction.

You'll need to figure out the best brick to disintegrate. For each brick,
determine how many other bricks would fall if that brick were
disintegrated.

Using the same example as above:

- Disintegrating brick A would cause all 6 other bricks to fall.
- Disintegrating brick F would cause only 1 other brick, G, to fall.

Disintegrating any other brick would cause no other bricks to fall. So, in
this example, the sum of the number of other bricks that would fall as a
result of disintegrating each brick is 7.

For each brick, determine how many other bricks would fall if that brick
were disintegrated. What is the sum of the number of other bricks that
would fall?
 */

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./inputs/22_slabs.txt").unwrap();
    let snapshot = file.lines();
    println!("{}", get_disintegration_count(snapshot.clone()));
    println!("{}", get_chain_reaction_count(snapshot));
}

/// Represents a position in 3D space
#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');
        let x = i32::from_str(split.next().unwrap()).unwrap();
        let y = i32::from_str(split.next().unwrap()).unwrap();
        let z = i32::from_str(split.next().unwrap()).unwrap();
        Self { x, y, z }
    }
}

/// Represents a column of 3D space
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Column {
    x: i32,
    y: i32,
}

impl From<&Position> for Column {
    fn from(value: &Position) -> Self {
        Self { x: value.x, y: value.y }
    }
}

/// Data which represents the top of a column
/// If you were to look at the world directly top-down, this is used to describe what each column
/// would look like
#[derive(Debug, Clone, Eq, PartialEq)]
struct ColumnTop {
    z: i32,
    id: usize,
}

/// Represents a block
#[derive(Debug, Clone, Eq, PartialEq)]
struct Block {
    ends: [Position; 2],
    id: usize, // A unique identifier
}

impl Block {
    /// Iterate on the columns which this block occupies
    fn iter_columns(&self) -> impl Iterator<Item=Column> {
        let col1: Column = (&self.ends[0]).into();
        let col2: Column = (&self.ends[1]).into();

        (min(col1.x, col2.x)..=max(col1.x, col2.x))
            .cartesian_product(min(col1.y, col2.y)..=max(col1.y, col2.y))
            .map(|(x, y)| Column { x, y })
    }

    /// Get the lowest Z coordinate of this block
    /// Due to the constraint that this block is a 1x1xN line of blocks in any of the 3 axes,
    /// this lowest coordinate is also the lowest Z coordinate of ALL columns this block falls into
    fn lowest_z(&self) -> i32 {
        min(self.ends[0].z, self.ends[1].z)
    }

    /// Get the highest Z coordinate of this block
    /// Due to the constraint that this block is a 1x1xN line of blocks in any of the 3 axes,
    /// this lowest coordinate is also the highest Z coordinate of ALL columns this block falls into
    fn highest_z(&self) -> i32 {
        max(self.ends[0].z, self.ends[1].z)
    }
}

/// Preprocess the input into a more useful form
fn preprocess<'a>(snapshot: impl Iterator<Item=&'a str>) -> Vec<Block> {
    snapshot.enumerate().map(|(id, line)| {
        let (start, end) = line.split_once('~').unwrap();
        Block {
            ends: [start.into(), end.into(), ],
            id,
        }
    }).collect()
}

/// Get the "floor" of a given column
/// If a 1x1x1 block were to fall from infinitely high on this column, the floor is the z height
/// which the 1x1x1 block would collide with before stopping one block higher
fn get_column_floor(top_layer: &HashMap<Column, ColumnTop>, column: &Column) -> i32 {
    match top_layer.get(&column) {
        Some(top) => top.z,
        None => 0,
    }
}

/// Solve part1
fn get_disintegration_count<'a>(snapshot: impl Iterator<Item=&'a str>) -> usize {
    let mut blocks = preprocess(snapshot);
    // First start off with saying we can disintegrate any block
    let mut can_disintegrate = vec![true; blocks.len()];

    // Sort the blocks by ascending height/z-pos
    // The lower blocks will land first and so if we resolve collisions/landing in ascending height
    // order, we will correctly place everything
    blocks.sort_by_key(|block| block.lowest_z());

    let mut top_layer = HashMap::<Column, ColumnTop>::new();
    for block in blocks {
        // Determine how much this block will fall from its starting position
        // This works since we update the top layer with the new positions after the block has fallen
        let mut fall_by = i32::MAX;
        for column in block.iter_columns() {
            fall_by = min(fall_by, block.lowest_z() - get_column_floor(&top_layer, &column) - 1);
        }
        // The new Z positions of this block
        let new_bottom = block.lowest_z() - fall_by;
        let new_top = block.highest_z() - fall_by;

        // The set of blocks which are supporting this block
        let mut supported_by = HashSet::new();
        // Loop over each column again
        for column in block.iter_columns() {
            // If we are touching the block which had previously landed in this column
            if let Some(top) = top_layer.get(&column) {
                if top.z + 1 == new_bottom {
                    // Record that we are supported by that other block
                    supported_by.insert(top.id);
                }
            }

            // Update the top layer based on the location to where we had fallen
            top_layer.insert(column, ColumnTop {
                z: new_top,
                id: block.id,
            });
        }
        // Prevent the sole supporter (if one exists) from disintegrating
        if supported_by.len() == 1 {
            supported_by.into_iter().for_each(|support| can_disintegrate[support] = false);
        }
    }

    // Count the number of blocks which can be disintegrated
    can_disintegrate.into_iter()
        .filter(|x| *x)
        .count()
}

/// Count the number of blocks which would disintegrate if this block were disintegrated
fn get_disintegration_chain_count(block: &Block, supported_by: &Vec<HashSet<usize>>, directly_supports: &Vec<HashSet<usize>>) -> usize {
    // The set of blocks which are disintegrating
    let mut disintegrating = HashSet::<usize>::new();
    disintegrating.insert(block.id);

    // The queue of blocks to check if they will disintegrate
    let mut queue = VecDeque::<usize>::new();
    queue.extend(directly_supports[block.id].iter());

    // Grab a block off of the queue
    while let Some(this) = queue.pop_front() {
        // Skip it if its already disintegrating
        if disintegrating.contains(&this) {
            continue;
        }

        let mut this_supported_by = supported_by[this].clone();
        // Keep the supports which are not yet disintegrating
        this_supported_by.retain(|x| !disintegrating.contains(x));

        // If we are now not supported
        if this_supported_by.len() == 0 {
            // Disintegrate this
            disintegrating.insert(this);
            // And add the stuff it supports to the queue
            queue.extend(directly_supports[this].iter());
        }
    }

    disintegrating.remove(&block.id); // Remove the block from the list of supports
    disintegrating.len()
}

/// Solve part2
fn get_chain_reaction_count<'a>(snapshot: impl Iterator<Item=&'a str>) -> usize {
    // Most of this is similar to part1
    let mut blocks = preprocess(snapshot);
    let mut can_disintegrate = vec![true; blocks.len()];
    let mut supported_by = vec![HashSet::new(); blocks.len()];
    let mut directly_supports = vec![HashSet::new(); blocks.len()];

    // Sort the blocks by ascending height/z-pos
    blocks.sort_by_key(|block| block.lowest_z());

    let mut top_layer = HashMap::<Column, ColumnTop>::new();
    for block in &blocks {
        let mut fall_by = i32::MAX;
        for column in block.iter_columns() {
            fall_by = min(fall_by, block.lowest_z() - get_column_floor(&top_layer, &column) - 1);
        }

        let new_bottom = block.lowest_z() - fall_by;
        let new_top = block.highest_z() - fall_by;
        for column in block.iter_columns() {
            if let Some(top) = top_layer.get(&column) {
                if top.z + 1 == new_bottom {
                    supported_by[block.id].insert(top.id);
                    directly_supports[top.id].insert(block.id);
                }
            }

            top_layer.insert(column, ColumnTop {
                z: new_top,
                id: block.id,
            });
        }
        // Prevent the sole supporter (if one exists) from disintegrating
        if supported_by[block.id].len() == 1 {
            supported_by[block.id].iter().for_each(|support| can_disintegrate[*support] = false);
        }
    }

    // Get blocks which cannot be disintegrated (from part1)
    blocks.into_iter()
        .filter(|block| !can_disintegrate[block.id])
        // Get the number of things which would disintegrate if it disintegrated
        .map(|block| get_disintegration_chain_count(&block, &supported_by, &directly_supports))
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        5,
        get_disintegration_count(
            r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        7,
        get_chain_reaction_count(
            r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9".lines()
        )
    );
}
