
/*
The lava starts flowing rapidly once the Lava Production Facility is
operational. As you leave, the reindeer offers you a parachute, allowing
you to quickly reach Gear Island.

As you descend, your bird's-eye view of Gear Island reveals why you had
trouble finding anyone on your way up: half of Gear Island is empty, but
the half below you is a giant factory city!

You land near the gradually-filling pool of lava at the base of your new
lavafall. Lavaducts will eventually carry the lava throughout the city, but
to make use of it immediately, Elves are loading it into large crucibles on
wheels.

The crucibles are top-heavy and pushed by hand. Unfortunately, the
crucibles become very difficult to steer at high speeds, and so it can be
hard to go in a straight line for very long.

To get Desert Island the machine parts it needs as soon as possible, you'll
need to find the best way to get the crucible from the lava pool to the
machine parts factory. To do this, you need to minimize heat loss while
choosing a route that doesn't require the crucible to go in a straight line
for too long.

Fortunately, the Elves here have a map (your puzzle input) that uses
traffic patterns, ambient temperature, and hundreds of other parameters to
calculate exactly how much heat loss can be expected for a crucible
entering any particular city block.

For example:

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533

Each city block is marked by a single digit that represents the amount of
heat loss if the crucible enters that block. The starting point, the lava
pool, is the top-left city block; the destination, the machine parts
factory, is the bottom-right city block. (Because you already start in the
top-left block, you don't incur that block's heat loss unless you leave
that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight
line for very long, it can move at most three blocks in a single direction
before it must turn 90 degrees left or right. The crucible also can't
reverse direction; after entering each city block, it may only turn left,
continue straight, or turn right.

One way to minimize heat loss is this path:

2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>

This path never moves more than three consecutive blocks in the same
direction and incurs a heat loss of only 102.

Directing the crucible from the lava pool to the machine parts factory, but
not moving more than three consecutive blocks in the same direction, what
is the least heat loss it can incur?

--- Part Two ---

The crucibles of lava simply aren't large enough to provide an adequate
supply of lava to the machine parts factory. Instead, the Elves are going
to upgrade to ultra crucibles.

Ultra crucibles are even more difficult to steer than normal crucibles. Not
only do they have trouble going in a straight line, but they also have
trouble turning!

Once an ultra crucible starts moving in a direction, it needs to move a
minimum of four blocks in that direction before it can turn (or even before
it can stop at the end). However, it will eventually start to get wobbly:
an ultra crucible can move a maximum of ten consecutive blocks without
turning.

In the above example, an ultra crucible could follow this path to minimize
heat loss:

2>>>>>>>>1323
32154535v5623
32552456v4254
34465858v5452
45466578v>>>>
143859879845v
445787698776v
363787797965v
465496798688v
456467998645v
122468686556v
254654888773v
432267465553v

In the above example, an ultra crucible would incur the minimum possible
heat loss of 94.

Here's another example:

111111111111
999999999991
999999999991
999999999991
999999999991

Sadly, an ultra crucible would need to take an unfortunate path like this
one:

1>>>>>>>1111
9999999v9991
9999999v9991
9999999v9991
9999999v>>>>

This route causes the ultra crucible to incur the minimum possible heat
loss of 71.

Directing the ultra crucible from the lava pool to the machine parts
factory, what is the least heat loss it can incur?
 */

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs/17_crucible.txt").unwrap();
    let map = file.lines();
    println!("{}", get_minimum_heat_loss(map.clone()));
    println!("{}", get_minimum_heat_loss_ultra(map));
}

/// Preprocess the input into a more useful form
fn preprocess<'a>(layout: impl Iterator<Item=&'a str>) -> Vec<Vec<usize>> {
    layout.map(|row| {
        row.chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect()
    }).collect()
}

/// An enum for the direction of motion
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    /// Turn the direction left
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    /// Turn the direction right
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

/// Represents a state in the problem
#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    x: isize,
    y: isize,
    direction: Direction,
    direction_count: usize,
}
impl State {
    /// Step one unit in the currently set direction
    fn step(&self) -> Self {
        match self.direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
                direction: self.direction,
                direction_count: self.direction_count + 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
                direction: self.direction,
                direction_count: self.direction_count + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
                direction: self.direction,
                direction_count: self.direction_count + 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
                direction: self.direction,
                direction_count: self.direction_count + 1,
            },
        }
    }

    /// Get the state's position and make sure it is a valid position on the map
    fn get_position(&self, width: usize, height: usize) -> Option<(usize, usize)> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        if self.x as usize >= width || self.y as usize >= height {
            return None;
        }
        Some((self.x as usize, self.y as usize))
    }
}

/// A node in the search
#[derive(Eq, PartialEq)]
struct SearchNode {
    state: State,
    cost: usize,
}
/// Since we're using a max-heap flip the ordering so that smaller costs sort higher
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Ordering flipped
    }
}
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Solve part 1
fn get_minimum_heat_loss<'a>(layout: impl Iterator<Item=&'a str>) -> usize {
    // Dijkstra's algorithm
    let heat_map = preprocess(layout);

    let mut queue = BinaryHeap::<SearchNode>::new();
    let mut visited = HashSet::<State>::new();

    queue.push(SearchNode {
        state: State {
            x: 0,
            y: 0,
            direction: Direction::Right,
            direction_count: 0,
        },
        cost: 0,
    });

    while let Some(node) = queue.pop() {
        // Ignore invalid positions
        let Some(position) = node.state.get_position(heat_map[0].len(), heat_map.len()) else { continue; };
        if visited.contains(&node.state) {
            continue; // Ignore if we've been here already
        }

        if position.1 == (heat_map.len() - 1) && position.0 == (heat_map[0].len() - 1) {
            return node.cost;
        }
        // Mark this as visited
        visited.insert(node.state.clone());

        if node.state.direction_count < 3 {
            let state = node.state.step();
            if let Some(position) = state.get_position(heat_map[0].len(), heat_map.len()) {
                queue.push(SearchNode {
                    state,
                    cost: node.cost + heat_map[position.1][position.0],
                });
            }
        }
        {
            let mut state = node.state.clone();
            state.direction = state.direction.turn_left();
            state.direction_count = 0;
            let state = state.step();
            if let Some(position) = state.get_position(heat_map[0].len(), heat_map.len()) {
                queue.push(SearchNode {
                    state,
                    cost: node.cost + heat_map[position.1][position.0],
                });
            }
        }
        {
            let mut state = node.state.clone();
            state.direction = state.direction.turn_right();
            state.direction_count = 0;
            let state = state.step();
            if let Some(position) = state.get_position(heat_map[0].len(), heat_map.len()) {
                queue.push(SearchNode {
                    state,
                    cost: node.cost + heat_map[position.1][position.0],
                });
            }
        }
    }

    panic!("No solution found")
}

/// Part 2
fn get_minimum_heat_loss_ultra<'a>(layout: impl Iterator<Item=&'a str>) -> usize {
    // Dijkstra's algorithm
    let heat_map = preprocess(layout);

    let mut queue = BinaryHeap::<SearchNode>::new();
    let mut visited = HashSet::<State>::new();

    // Both of these are needed as starting positions
    queue.push(SearchNode {
        state: State {
            x: 0,
            y: 0,
            direction: Direction::Right,
            direction_count: 0,
        },
        cost: 0,
    });
    queue.push(SearchNode {
        state: State {
            x: 0,
            y: 0,
            direction: Direction::Down,
            direction_count: 0,
        },
        cost: 0,
    });

    while let Some(node) = queue.pop() {
        // Ignore invalid positions
        let Some(position) = node.state.get_position(heat_map[0].len(), heat_map.len()) else { continue; };
        if visited.contains(&node.state) {
            continue; // Ignore if we've been here already
        }

        // Mark this as visited
        visited.insert(node.state.clone());

        if position.1 == (heat_map.len() - 1)
            && position.0 == (heat_map[0].len() - 1) {
            if node.state.direction_count >= 4 {
                return node.cost;
            } else {
                continue;
            }
        }

        if node.state.direction_count < 10 {
            let state = node.state.step();
            if let Some(position) = state.get_position(heat_map[0].len(), heat_map.len()) {
                queue.push(SearchNode {
                    state,
                    cost: node.cost + heat_map[position.1][position.0],
                });
            }
        }
        if node.state.direction_count >= 4 {
            let mut state = node.state.clone();
            state.direction = state.direction.turn_left();
            state.direction_count = 0;
            let state = state.step();
            if let Some(position) = state.get_position(heat_map[0].len(), heat_map.len()) {
                queue.push(SearchNode {
                    state,
                    cost: node.cost + heat_map[position.1][position.0],
                });
            }
        }
        if node.state.direction_count >= 4 {
            let mut state = node.state.clone();
            state.direction = state.direction.turn_right();
            state.direction_count = 0;
            let state = state.step();
            if let Some(position) = state.get_position(heat_map[0].len(), heat_map.len()) {
                queue.push(SearchNode {
                    state,
                    cost: node.cost + heat_map[position.1][position.0],
                });
            }
        }
    }

    panic!("No solution found")
}

#[test]
fn test_part1() {
    assert_eq!(
        102,
        get_minimum_heat_loss(
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533".lines()
        )
    );
}

#[test]
fn test_part2a() {
    assert_eq!(
        94,
        get_minimum_heat_loss_ultra(
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533".lines()
        )
    );
}

#[test]
fn test_part2b() {
    assert_eq!(
        71,
        get_minimum_heat_loss_ultra(
            r"111111111111
999999999991
999999999991
999999999991
999999999991".lines()
        )
    );
}
