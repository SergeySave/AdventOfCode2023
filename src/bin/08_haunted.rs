/*
You're still riding a camel across Desert Island when you spot a sandstorm
quickly approaching. When you turn to warn the Elf, she disappears before
your eyes! To be fair, she had just finished warning you about ghosts a few
minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of
documents (your puzzle input) about how to navigate the desert. At least,
you're pretty sure that's what they are; one of the documents contains a
list of left/right instructions, and the rest of the documents seem to
describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate
the network. Perhaps if you have the camel follow the same instructions,
you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You
feel like AAA is where you are now, and you have to follow the left/right
instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)

Starting with AAA, you need to look up the next element based on the next
left/right instruction in your input. In this example, start with AAA and
go right (R) by choosing the right element of AAA, CCC. Then, L means to
choose the left element of CCC, ZZZ. By following the left/right
instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right
instructions, repeat the whole sequence of instructions as necessary: RL
really means RLRLRLRLRLRLRLRL... and so on. For example, here is a
situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

Starting at AAA, follow the left/right instructions. How many steps are
required to reach ZZZ?

--- Part Two ---

The sandstorm is upon you and you aren't any closer to escaping the
wasteland. You had the camel follow the instructions, but you've barely
left your starting position. It's going to take significantly more steps to
escape!

What if the map isn't for people - what if the map is for ghosts? Are
ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious
fact: the number of nodes with names ending in A is equal to the number
ending in Z! If you were a ghost, you'd probably just start at every node
that ends with A and follow all of the paths at the same time until they
all simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

Here, there are two starting nodes, 11A and 22A (because they both end
with A). As you follow each left/right instruction, use that instruction to
simultaneously navigate away from both nodes you're currently on. Repeat
this process until all of the nodes you're currently on end with Z. (If
only some of the nodes you're on end with Z, they act like any other node
and you continue as normal.) In this example, you would proceed as follows:

- Step 0: You are at 11A and 22A.
- Step 1: You choose all of the left paths, leading you to 11B and 22B.
- Step 2: You choose all of the right paths, leading you to 11Z and 22C.
- Step 3: You choose all of the left paths, leading you to 11B and 22Z.
- Step 4: You choose all of the right paths, leading you to 11Z and 22B.
- Step 5: You choose all of the left paths, leading you to 11B and 22C.
- Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it
take before you're only on nodes that end with Z?
 */

use std::collections::HashMap;
use std::fs;
use std::mem::swap;

use num::Integer;

fn main() {
    let file = fs::read_to_string("./inputs/08_haunted.txt").unwrap();
    let documents = file.lines();
    println!("{}", get_num_steps(documents.clone()));
    println!("{}", get_num_steps_ghost(documents));
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct Name(String);

struct Node {
    left: Name,
    right: Name,
}

fn get_graph<'a>(documents: impl Iterator<Item=&'a str>) -> HashMap<Name, Node> {
    let mut map = HashMap::new();

    documents.for_each(|x| {
        // AAA = (BBB, CCC)
        let name = Name((&x[0..=2]).into());
        let left = Name((&x[7..=9]).into());
        let right = Name((&x[12..=14]).into());

        map.insert(name.clone(), Node {
            left,
            right,
        });
    });

    map
}

/// Solve part1
fn get_num_steps<'a>(mut documents: impl Iterator<Item=&'a str>) -> usize {
    let instructions = documents.next().unwrap();
    let mut instructions = instructions.chars().cycle();
    documents.next();
    let graph = get_graph(documents);

    let start: Name = Name("AAA".into());
    let end: Name = Name("ZZZ".into());

    let mut location = &start;

    let mut step_count = 0;
    while location.0 != end.0 {
        step_count += 1;
        let instruction = instructions.next().unwrap();
        location = match instruction {
            'R' => &graph[&location].right,
            'L' => &graph[&location].left,
            _ => panic!("Unknown Instruction {}", instruction),
        }
    }

    step_count
}

struct Solution {
    solution: usize,
    period: usize,
}

fn solve_equation(mut a: Solution, mut b: Solution) -> Solution {
    if a.period < b.period {
        swap(&mut a, &mut b);
    }
    // a has the longer period of the two

    // We want to solve the following system of equations
    // y = a_solution + a_period * a_n
    // y = b_solution + b_period * b_n

    // We will do this by stepping solutions from the equation of longer period until we find one that
    // solves the other equation
    let mut solution = a.solution;
    // Given a_n solve for b_n
    // a_solution + a_period * a_n = b_solution + b_period * b_n
    // a_solution + a_period * a_n - b_solution = b_period * b_n
    // (a_solution + a_period * a_n - b_solution) / b_period = b_n
    // i.e. we've found a solution to both when b_period divides a_solution + a_period * a_n - b_solution
    // keep looping until we've found one
    while (solution < b.solution) || ((solution - b.solution) % b.period) != 0 {
        solution += a.period; // Step by a's period and try again
    }
    // Knowing the inputs to this problem, the periods are somwhere around ~550
    // Thus, a more efficient solution to this is not needed, although one certainly exists

    // This should take less than b.period steps
    // Once we have a solution, lets compute the period
    // The period is just the Least Common Multiple of both periods
    let period = a.period.lcm(&b.period);

    Solution {
        solution,
        period,
    }
}

/// Solve part2
fn get_num_steps_ghost<'a>(mut documents: impl Iterator<Item=&'a str>) -> usize {
    // My original solution to this was significantly more complicated and solved a much harder
    // superset of this problem. However, that was way too slow to get to the necessary answer.
    //
    // The important thing to note here is that ALL of the ghosts take X steps to first
    // reach finish location M and then they will be in a loop of Y steps in length where the only
    // finish they hit is location M.
    // This is a MUCH easier problem to solve than the problem seems to be as written.
    //
    // Essentially we are trying to solve a set of linear Diophantine equations
    let instructions = documents.next().unwrap();
    documents.next();
    let graph = get_graph(documents);

    // Get the initial start locations
    let initial = graph.keys().filter(|node| node.0.ends_with('A')).collect::<Vec<&Name>>();

    // Find the first solution (X) and the length of the loop (Y) for each initial start location
    let solutions = initial.iter().map(|start| {
        let mut instructions = instructions.chars().cycle();

        let mut location = *start;

        let mut step_count = 0;
        let mut finishes = [0; 2];
        // We want to find the first two finishes
        for finish in finishes.iter_mut() {
            while !location.0.ends_with('Z') {
                step_count += 1;
                let instruction = instructions.next().unwrap();
                location = match instruction {
                    'R' => &graph[&location].right,
                    'L' => &graph[&location].left,
                    _ => panic!("Unknown Instruction {}", instruction),
                }
            }

            *finish = step_count;

            // Do an extra step
            step_count += 1;
            let instruction = instructions.next().unwrap();
            location = match instruction {
                'R' => &graph[&location].right,
                'L' => &graph[&location].left,
                _ => panic!("Unknown Instruction {}", instruction),
            };
        }

        Solution {
            solution: finishes[0],
            period: finishes[1] - finishes[0],
        }
    }).collect::<Vec<Solution>>();

    // Find the overall solution by combining the individual solutions
    let solution = solutions.into_iter()
        .reduce(|accumulator, solution| solve_equation(accumulator, solution))
        .unwrap();
    // solution.solution == solution.period
    solution.solution
}

#[test]
fn test_part1() {
    assert_eq!(
        2,
        get_num_steps(
            r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".lines()
        )
    )
}

#[test]
fn test_part1b() {
    assert_eq!(
        6,
        get_num_steps(
            r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".lines()
        )
    )
}

#[test]
fn test_part2() {
    assert_eq!(
        6,
        get_num_steps_ghost(
            r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".lines()
        )
    )
}
