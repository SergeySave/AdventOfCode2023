/*
The Elves resume water filtering operations! Clean water starts flowing
over the edge of Island Island.

They offer to help you go over the edge of Island Island, too! Just hold on
tight to one end of this impossibly long rope and they'll lower you down a
safe distance from the massive waterfall you just created.

As you finally reach Snow Island, you see that the water isn't really
reaching the ground: it's being absorbed by the air itself. It looks like
you'll finally have a little downtime while the moisture builds up to snow-
producing levels. Snow Island is pretty scenic, even without any snow; why
not take a walk?

There's a map of nearby hiking trails (your puzzle input) that indicates
paths (.), forest (#), and steep slopes (^, >, v, and <).

For example:

#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#

You're currently on the single path tile in the top row; your goal is to
reach the single path tile in the bottom row. Because of all the mist from
the waterfall, the slopes are probably quite icy; if you step onto a slope
tile, your next step must be downhill (in the direction the arrow is
pointing). To make sure you have the most scenic hike possible, never step
onto the same tile twice. What is the longest hike you can take?

In the example above, the longest hike you can take is marked with O, and
your starting position is marked S:

#S#####################
#OOOOOOO#########...###
#######O#########.#.###
###OOOOO#OOO>.###.#.###
###O#####O#O#.###.#.###
###OOOOO#O#O#.....#...#
###v###O#O#O#########.#
###...#O#O#OOOOOOO#...#
#####.#O#O#######O#.###
#.....#O#O#OOOOOOO#...#
#.#####O#O#O#########v#
#.#...#OOO#OOO###OOOOO#
#.#.#v#######O###O###O#
#...#.>.#...>OOO#O###O#
#####v#.#.###v#O#O###O#
#.....#...#...#O#O#OOO#
#.#########.###O#O#O###
#...###...#...#OOO#O###
###.###.#.###v#####O###
#...#...#.#.>.>.#.>O###
#.###.###.#.###.#.#O###
#.....###...###...#OOO#
#####################O#

This hike contains 94 steps. (The other possible hikes you could have taken
were 90, 86, 82, 82, and 74 steps long.)

Find the longest hike you can take through the hiking trails listed on your
map. How many steps long is the longest hike?

--- Part Two ---

As you reach the trailhead, you realize that the ground isn't as slippery
as you expected; you'll have no problem climbing up the steep slopes.

Now, treat all slopes as if they were normal paths (.). You still want to
make sure you have the most scenic hike possible, so continue to ensure
that you never step onto the same tile twice. What is the longest hike you
can take?

In the example above, this increases the longest hike to 154 steps:

#S#####################
#OOOOOOO#########OOO###
#######O#########O#O###
###OOOOO#.>OOO###O#O###
###O#####.#O#O###O#O###
###O>...#.#O#OOOOO#OOO#
###O###.#.#O#########O#
###OOO#.#.#OOOOOOO#OOO#
#####O#.#.#######O#O###
#OOOOO#.#.#OOOOOOO#OOO#
#O#####.#.#O#########O#
#O#OOO#...#OOO###...>O#
#O#O#O#######O###.###O#
#OOO#O>.#...>O>.#.###O#
#####O#.#.###O#.#.###O#
#OOOOO#...#OOO#.#.#OOO#
#O#########O###.#.#O###
#OOO###OOO#OOO#...#O###
###O###O#O###O#####O###
#OOO#OOO#O#OOO>.#.>O###
#O###O###O#O###.#.#O###
#OOOOO###OOO###...#OOO#
#####################O#

Find the longest hike you can take through the surprisingly dry hiking
trails listed on your map. How many steps long is the longest hike?
 */

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./inputs/23_long.txt").unwrap();
    let map = file.lines();
    println!("{}", get_longest_hike(map.clone()));
    println!("{}", get_longest_hike_no_slopes(map));
    // Part2 is solved using a brute force (on the reduced graph)
    // Timing the performance on my laptop:
    // cargo run --bin 23_long  36.20s user 0.03s system 98% cpu 36.793 total
    // cargo run --release --bin 23_long  2.49s user 0.02s system 96% cpu 2.583 total
    // So I am perfectly happy with brute force
}

/// A structure to hold a node in the graph
struct Node {
    x: usize,
    y: usize,
    edges: HashMap<usize, usize>,
}

/// Preprocess the input into a more useful form
/// If `treat_slopes_as_path` then slopes are treated identically to paths
fn preprocess<'a>(snapshot: impl Iterator<Item=&'a str>, treat_slopes_as_path: bool) -> (Vec<Node>, usize, usize) {
    let snapshot = snapshot.map(|row| row.chars().collect_vec()).collect_vec();
    let height = snapshot.len();
    let width = snapshot[0].len();
    let start_col = snapshot[0].iter().position(|x| *x == '.').unwrap();
    let exit_col = snapshot[height - 1].iter().position(|x| *x == '.').unwrap();

    let mut nodes: Vec<Node> = vec![];

    let mut stack: Vec<((usize, usize), (usize, usize))> = vec![];
    stack.push(((start_col, 0), (0, 0)));
    let mut visited = HashSet::new();

    while let Some((pos, (last, distance))) = stack.pop() {
        let (x, y) = pos;
        let adjacent = get_adjacent(&snapshot, x, y)
            .filter(|(c, _, _)| *c != '#')
            .collect::<Vec<_>>();
        let this = snapshot[y][x];

        if visited.contains(&pos) {
            // This might also be a conjunction
            // Find this tile's node (this is how we will know it is a conjunction)
            if let Some((node_index, _)) = nodes.iter().find_position(|node| node.x == x && node.y == y) {
                if node_index != last { // No self-references
                    nodes[last].edges.insert(node_index, distance);
                    if treat_slopes_as_path {
                        nodes[node_index].edges.insert(last, distance);
                    }
                }
            }
            continue;
        }
        visited.insert(pos);

        if this == '.' || treat_slopes_as_path {
            if adjacent.len() == 2 {
                for (_, pos, d) in adjacent {
                    if snapshot[pos.1][pos.0] == '.' || snapshot[pos.1][pos.0] == d || treat_slopes_as_path {
                        stack.push((pos, (last, distance + 1)));
                    }
                }
            } else {
                let new_node = nodes.len();

                nodes.push(Node {
                    x,
                    y,
                    edges: HashMap::new(),
                });

                if distance != 0 {
                    nodes[last].edges.insert(new_node, distance);
                    if treat_slopes_as_path {
                        nodes[new_node].edges.insert(last, distance);
                    }
                }
                for (_, pos, d) in adjacent {
                    if snapshot[pos.1][pos.0] == '.' || snapshot[pos.1][pos.0] == d || treat_slopes_as_path {
                        stack.push((pos, (new_node, 1)));
                    }
                }
            }
        } else if this == '^' {
            if y > 0 {
                stack.push(((x, y - 1), (last, distance + 1)));
            }
        } else if this == 'v' {
            if y < height - 1 {
                stack.push(((x, y + 1), (last, distance + 1)));
            }
        } else if this == '<' {
            if x > 0 {
                stack.push(((x - 1, y), (last, distance + 1)));
            }
        } else if this == '>' {
            if x < width - 1 {
                stack.push(((x + 1, y), (last, distance + 1)));
            }
        }
    }

    let end_node = nodes.iter().find_position(|node| node.x == exit_col && node.y == height - 1).unwrap().0;

    (nodes, 0, end_node)
}

/// Get the information for adjacent tiles in this map for a given tile
fn get_adjacent<'a>(map: &'a Vec<Vec<char>>, x: usize, y: usize) -> impl Iterator<Item=(char, (usize, usize), char)> + 'a {
    [(-1, 0, '<'), (1, 0, '>'), (0, -1, '^'), (0, 1, 'v')]
        .iter()
        .map(move |(dx, dy, c)| (x as isize + dx, y as isize + dy, *c))
        .filter(|(x, y, _)| *x >= 0 && *y >= 0 && *y < map.len() as isize && *x < map[0].len() as isize)
        .map(|(x, y, c)| (map[y as usize][x as usize], (x as usize, y as usize), c))
}

/// Create a topological ordering for the given DAG
fn create_topological_ordering(graph: &Vec<Node>) -> Vec<usize> {
    /// Helper function for performing the ordering
    fn recurse_topological_ordering(graph: &Vec<Node>, node: usize, result: &mut Vec<usize>, marked: &mut HashSet<usize>) {
        if marked.contains(&node) {
            return;
        }
        marked.insert(node);
        for to in graph[node].edges.keys() {
            recurse_topological_ordering(graph, *to, result, marked);
        }
        result.push(node);
    }

    let mut result = vec![];
    let mut marked = HashSet::new();

    for node in 0..graph.len() {
        recurse_topological_ordering(graph, node, &mut result, &mut marked);
    }

    result.reverse();
    result
}

/// Solve part 1
fn get_longest_hike<'a>(map: impl Iterator<Item=&'a str>) -> usize {
    // Preprocess the input map into a reduced graph (paths with no forks are reduced to edges)
    let (nodes, _, end) = preprocess(map, false);

    // The map is a DAG so we can efficiently find the longest path by traversing it in topological
    // order
    let ordering = create_topological_ordering(&nodes);
    let mut distances = vec![0_usize; nodes.len()];
    for node in ordering {
        for (to, distance) in nodes[node].edges.iter() {
            distances[*to] = max(distances[*to], distances[node] + *distance);
        }
    }

    distances[end]
}

/// Recursive brute-force solution to find the longest path
fn recurse_brute_force(nodes: &Vec<Node>, node: usize, end: usize, visited: &mut HashSet<usize>) -> (usize, bool) {
    if node == end {
        // For a solution to be valid it must include the end node
        return (0, true);
    }
    let mut result = 0_usize;
    let mut solved = false;
    visited.insert(node);
    for (to, distance) in &nodes[node].edges {
        if !visited.contains(to) {
            let (sub_problem, valid) = recurse_brute_force(&nodes, *to, end, visited);
            if valid {
                result = max(result, sub_problem + distance);
                solved = true
            }
        }
    }
    visited.remove(&node);

    return (result, solved);
}

/// Solve part 2
fn get_longest_hike_no_slopes<'a>(map: impl Iterator<Item=&'a str>) -> usize {
    // Preprocess the input map into a reduced graph (paths with no forks are reduced to edges)
    let (nodes, start, end) = preprocess(map, true);

    recurse_brute_force(&nodes, start, end, &mut HashSet::new()).0
}

#[test]
fn test_part1() {
    assert_eq!(
        94,
        get_longest_hike(
            r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        154,
        get_longest_hike_no_slopes(
            r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#".lines()
        )
    );
}
