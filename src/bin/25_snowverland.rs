/*

Still somehow without snow, you go to the last place you haven't checked:
the center of Snow Island, directly below the waterfall.

Here, someone has clearly been trying to fix the problem. Scattered
everywhere are hundreds of weather machines, almanacs, communication
modules, hoof prints, machine parts, mirrors, lenses, and so on.

Somehow, everything has been wired together into a massive snow-producing
apparatus, but nothing seems to be running. You check a tiny screen on one
of the communication modules: Error 2023. It doesn't say what Error 2023
means, but it does have the phone number for a support line printed on it.

"Hi, you've reached Weather Machines And So On, Inc. How can I help you?"
You explain the situation.

"Error 2023, you say? Why, that's a power overload error, of course! It
means you have too many components plugged in. Try unplugging some
components and--" You explain that there are hundreds of components here
and you're in a bit of a hurry.

"Well, let's see how bad it is; do you see a big red reset button
somewhere? It should be on its own module. If you push it, it probably
won't fix anything, but it'll report how overloaded things are." After a
minute or two, you find the reset button; it's so big that it takes two
hands just to get enough leverage to push it. Its screen then displays:

SYSTEM OVERLOAD!

Connected components would require
power equal to at least 100 stars!

"Wait, how many components did you say are plugged in? With that much
equipment, you could produce snow for an entire--" You disconnect the call.

You have nowhere near that many stars - you need to find a way to
disconnect at least half of the equipment here, but it's already Christmas!
You only have time to disconnect three wires.

Fortunately, someone left a wiring diagram (your puzzle input) that shows
how the components are connected. For example:

jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr

Each line shows the name of a component, a colon, and then a list of other
components to which that component is connected. Connections aren't
directional; abc: xyz and xyz: abc both represent the same configuration.
Each connection between two components is represented only once, so some
components might only ever appear on the left or right side of a colon.

In this example, if you disconnect the wire between hfx/pzl, the wire
between bvb/cmg, and the wire between nvd/jqt, you will divide the
components into two separate, disconnected groups:

- 9 components: cmg, frs, lhk, lsr, nvd, pzl, qnr, rsh, and rzs.
- 6 components: bvb, hfx, jqt, ntq, rhn, and xhk.

Multiplying the sizes of these groups together produces 54.

Find the three wires you need to disconnect in order to divide the
components into two separate groups. What do you get if you multiply the
sizes of these two groups together?

--- Part Two ---

You climb over weather machines, under giant springs, and narrowly avoid a
pile of pipes as you find and disconnect the three wires.

A moment after you disconnect the last wire, the big red reset button
module makes a small ding noise:

System overload resolved!
Power required is now 50 stars.

Out of the corner of your eye, you notice goggles and a loose-fitting hard
hat peeking at you from behind an ultra crucible. You think you see a faint
glow, but before you can investigate, you hear another small ding:

Power required is now 49 stars.

Please supply the necessary stars and
push the button to restart the system.

You have enough stars to [Push The Big Red Button].
 */

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

use rand::prelude::IteratorRandom;

fn main() {
    let file = fs::read_to_string("./inputs/25_snowverland.txt").unwrap();
    let wiring_diagram = file.lines();
    println!("{}", get_division_product(wiring_diagram));
    // On my laptop this typically takes under a second when building with optimization flags
}

/// A structure to represent a node in a graph
#[derive(Debug, Clone)]
struct Node {
    name: String,
    edges: HashMap<usize, usize>,
}

/// Represents the graph
#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<usize, Node>,
}

/// Preprocess the input into a more useful form (a graph)
fn preprocess<'a>(wiring_diagram: impl Iterator<Item=&'a str>) -> Graph {
    let wiring_diagram = wiring_diagram.collect::<Vec<_>>();

    let mut graph = Graph {
        nodes: HashMap::new()
    };

    // Helper function
    fn add_node_if_needed<'a>(name: &'a str, graph: &mut Graph, lookup_table: &mut HashMap<&'a str, usize>) {
        if !lookup_table.contains_key(name) {
            lookup_table.insert(name, graph.nodes.len());
            graph.nodes.insert(
                graph.nodes.len(),
                Node {
                    name: name.into(),
                    edges: HashMap::new(),
                }.into(),
            );
        }
    }

    let mut lookup_table = HashMap::<&'a str, usize>::new();
    wiring_diagram.iter().for_each(|entry| {
        let (name, edges) = entry.split_once(':').unwrap();
        let edges = edges.split_whitespace();
        add_node_if_needed(name, &mut graph, &mut lookup_table);
        let this_index = *lookup_table.get(name).unwrap();
        edges.for_each(|edge| {
            add_node_if_needed(edge, &mut graph, &mut lookup_table);
            let edge_index = *lookup_table.get(edge).unwrap();
            graph.nodes.get_mut(&this_index).unwrap().edges.insert(edge_index, 1);
            graph.nodes.get_mut(&edge_index).unwrap().edges.insert(this_index, 1);
        });
    });

    graph
}

/// Implements Karger's Algorithm
/// https://en.wikipedia.org/wiki/Karger%27s_algorithm
fn kargers_algorithm(mut graph: Graph) -> Option<Vec<usize>> {
    // Used to remember which of the initial nodes got merged into this node
    let mut partition_memory = HashMap::<usize, Vec<usize>>::new();
    for node in graph.nodes.keys() {
        partition_memory.insert(*node, vec![*node]);
    }

    // Random object to be reused by in the loop
    let mut random = rand::thread_rng();

    // Until we've reduced the graph to just two nodes
    while graph.nodes.len() > 2 {
        // Picking a random edge (note - this is not uniform as each node is picked separately)
        // This makes the algorithm less optimal, but it still works
        let s = *graph.nodes.keys().choose(&mut random).unwrap();
        let t = *graph.nodes.get(&s).unwrap().edges.iter().choose(&mut random).unwrap().0;

        // Combine t's partition into s's partition
        let t_partition = partition_memory.remove(&t).unwrap();
        partition_memory.get_mut(&s).unwrap().extend(t_partition);

        // Collapse the edges of t into s
        let t_node = graph.nodes.remove(&t).unwrap();
        for (to, weight) in t_node.edges {
            // Remove edges to this node from all other nodes
            graph.nodes.get_mut(&to).unwrap().edges.remove(&t);

            // Add this edge's weight to s (or move it over otherwise)
            if to != s {
                let existing_weight = *graph.nodes.get_mut(&s).unwrap().edges.get(&to).unwrap_or(&0);
                let new_weight = existing_weight + weight;
                graph.nodes.get_mut(&s).unwrap().edges.insert(to, new_weight);
                graph.nodes.get_mut(&to).unwrap().edges.insert(s, new_weight);
            }
        }
    }

    // Get the count of how many edges remain in the reduced graph
    let count = *graph.nodes.iter().next().unwrap().1.edges.values().next().unwrap();
    if count != 3 {
        // If there is not three original edges - we have failed to solve this problem correctly
        // This is how many edges in the original graph we would cut if we separated these two
        return None;
    }

    // Return one of the partitions
    let node = *partition_memory.keys().next().unwrap();
    Some(partition_memory.remove(&node).unwrap())
}

/// Solve part1
fn get_division_product<'a>(wiring_diagram: impl Iterator<Item=&'a str>) -> usize {
    // This is asking us to find a minimum cut of the input graph

    // So process the input into a graph
    let graph = preprocess(wiring_diagram);

    // Keep looping until we successfully find a partition which cuts only 3 edges
    let mut partition = None;
    while partition.is_none() {
        partition = kargers_algorithm(graph.clone());
    }
    let partition = partition.unwrap();

    // Take the product of the size of both partitions
    (partition.len()) * (graph.nodes.len() - partition.len())
}

#[test]
fn test_part1() {
    assert_eq!(
        54,
        get_division_product(
            r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr".lines()
        )
    );
}

/*
Earlier version I tried to make work.
Unfortunately this was too much slower than Karger's Algorithm (though it wouldn't have been random)

// /// A modified version of stoer-wagner
// fn stoer_wagner(mut graph: Graph) -> Vec<usize> {
//     fn phase(graph: &Graph) -> (usize, usize, usize) {
//         let mut visited = vec![ *graph.nodes.keys().next().unwrap() ];
//         let mut cut_weights = vec![];
//         let mut candidates = graph.nodes.keys().map(|x| *x).collect::<HashSet<_>>();
//         candidates.remove(&visited[0]);
//
//         while !candidates.is_empty() {
//             let mut max_candidate: Option<usize> = None;
//             let mut max_weight = usize::MIN;
//             // Find the most tightly connected vertex
//             for candidate in &candidates {
//                 let mut weight = 0_usize;
//                 for visited in &visited {
//                     let edge = graph.nodes.get(candidate).unwrap().edges.get(visited);
//                     if let Some(edge_weight) = edge {
//                         weight += *edge_weight;
//                     }
//                 }
//                 if weight > max_weight {
//                     max_candidate = Some(*candidate);
//                     max_weight = weight;
//                 }
//             }
//
//             candidates.remove(&max_candidate.unwrap());
//             visited.push(max_candidate.unwrap());
//             cut_weights.push(max_weight);
//         }
//
//         // The "cut-of-the-phase"
//         (
//             visited[visited.len() - 2],
//             visited[visited.len() - 1],
//             cut_weights[cut_weights.len() - 1]
//         )
//     }
//
//     let mut minimum_cut = usize::MAX;
//     let mut partition = vec![];
//     let mut best_partition = vec![];
//     while graph.nodes.len() > 1 {
//         let (s, t, w) = phase(&graph);
//         partition.push(t);
//
//         if w == 3 && minimum_cut > 3 {
//             minimum_cut = w;
//             best_partition = partition.clone();
//         }
//
//         // Now we merge vertices s and t
//         let t_node = graph.nodes.remove(&t).unwrap();
//         for (to, weight) in t_node.edges {
//             // Remove edges to this node from all other nodes
//             graph.nodes.get_mut(&to).unwrap().edges.remove(&t);
//
//             // Add this edge's weight to s (or move it over otherwise)
//             if to != s {
//                 let existing_weight = *graph.nodes.get_mut(&s).unwrap().edges.get(&to).unwrap_or(&0);
//                 let new_weight = existing_weight + weight;
//                 graph.nodes.get_mut(&s).unwrap().edges.insert(to, new_weight);
//                 graph.nodes.get_mut(&to).unwrap().edges.insert(s, new_weight);
//             }
//         }
//     }
//
//     best_partition
// }
 */
