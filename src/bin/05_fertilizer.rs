
/*
You take the boat and find the gardener right where you were told he would
be: managing a giant "garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that
Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it with!
Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand
soon; we only turned off the water a few days... weeks... oh no." His face
sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely
forgot to check why we stopped getting more sand! There's a ferry leaving
soon that is headed over in that direction - it's much faster than your
boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another.
"While you wait for the ferry, maybe you can help us with our food
production problem. The latest Island Island Almanac just arrived and we're
having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be
planted. It also lists what type of soil to use with each kind of seed,
what type of fertilizer to use with each kind of soil, what type of water
to use with each kind of fertilizer, and so on. Every type of seed, soil,
fertilizer and so on is identified with a number, but numbers are reused by
each category - that is, soil 123 and fertilizer 123 aren't necessarily
related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

The almanac starts by listing which seeds need to be planted: seeds 79, 14,
55, and 13.

The rest of the almanac contains a list of maps which describe how to
convert numbers from a source category into numbers in a destination
category. That is, the section that starts with seed-to-soil map: describes
how to convert a seed number (the source) to a soil number (the
destination). This lets the gardener and his team know which soil to use
with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination
number one by one, the maps describe entire ranges of numbers that can be
converted. Each line within a map contains three numbers: the destination
range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48

The first line has a destination range start of 50, a source range start of
98, and a range length of 2. This line means that the source range starts
at 98 and contains two values: 98 and 99. The destination range is the
same length, but it starts at 50, so its two values are 50 and 51. With
this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48
values: 50, 51, ..., 96, 97. This corresponds to a destination range
starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So,
seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination
number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers
looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51

With this map, you can look up the soil number required for each initial
seed number:

- Seed number 79 corresponds to soil number 81.
- Seed number 14 corresponds to soil number 14.
- Seed number 55 corresponds to soil number 57.
- Seed number 13 corresponds to soil number 13.

The gardener and his team want to get started as soon as possible, so
they'd like to know the closest location that needs a seed. Using these
maps, find the lowest location number that corresponds to any of the
initial seeds. To do this, you'll need to convert each seed number through
other categories until you can find its corresponding location number. In
this example, the corresponding types are:

- Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78,
humidity 78, location 82.
- Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42,
humidity 43, location 43.
- Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82,
humidity 82, location 86.
- Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34,
humidity 35, location 35.

So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial
seed numbers?

--- Part Two ---

Everyone will starve if you only plant such a small number of seeds. Re-
reading the almanac, it looks like the seeds: line actually describes
ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the
first value is the start of the range and the second value is the length of
the range. So, in the first line of the example above:

seeds: 79 14 55 13

This line describes two ranges of seed numbers to be planted in the garden.
The first range starts with seed number 79 and contains 14 values: 79, 80,
..., 91, 92. The second range starts with seed number 55 and contains 13
values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a
total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed
number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77
, temperature 45, humidity 46, and location 46. So, the lowest location
number is 46.

Consider all of the initial seed numbers listed in the ranges on the first
line of the almanac. What is the lowest location number that corresponds to
any of the initial seed numbers?
 */

use std::cmp::{max, min};
use std::fs;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("./inputs/05_fertilizer.txt").unwrap();
    let almanac = file.lines();
    println!("{}", get_lowest_location_value(almanac.clone()));
    println!("{}", get_lowest_location_value_range(almanac));
}

fn get_lowest_location_value<'a>(mut almanac: impl Iterator<Item=&'a str>) -> usize {
    // I am taking advantage of the fact that we can handle each mapping row individually
    // Additionally the types of mappings (i.e. seed-to-soil) do not matter
    // I can look at each mapping and apply that mapping to each seed
    // The only thing necessary to recognize is that the mappings may overlap, so within a run
    // of mappings on adjacent lines a given seed can only be mapped once
    // This is handled by a flag for each seed which is reset when a non-mapping line is detected
    // An upside to this implementation is that it is completely agnostic to how many types of mappings
    // there are
    let map_pattern: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let seeds = almanac.next().unwrap();
    // The currently mapped seeds
    let mut seeds = seeds[7..].split(" ").map(|seed| usize::from_str(seed).unwrap()).collect::<Vec<usize>>();
    // Can this seed currently be mapped
    let mut can_map = vec![false; seeds.len()];
    while let Some(map) = almanac.next() { // We will consume perform all of the mappings (in order)
        let Some(mapping) = map_pattern.captures(map) else {
            can_map.iter_mut().for_each(|x| *x = true ); // Reset the can map to true when we fail to match
            continue
        };
        // Get the values in the mapping from the regex
        let destination_start = usize::from_str(mapping.get(1).unwrap().as_str()).unwrap();
        let source_start = usize::from_str(mapping.get(2).unwrap().as_str()).unwrap();
        let length = usize::from_str(mapping.get(3).unwrap().as_str()).unwrap();

        // Update all of the seeds which fall into this mapping
        seeds.iter_mut().enumerate().for_each(|(i, seed)| {
            if can_map[i] && (source_start..(source_start + length)).contains(seed) {
                *seed = *seed - source_start + destination_start;
                can_map[i] = false; // Unset can_map for this run of mappings
                // This prevents us from mapping the same seed twice in a given set of mappings
            }
        });
    }
    // Get the minimum position now that we have completed the mapping
    *seeds.iter().min().unwrap()
}

fn get_lowest_location_value_range<'a>(mut almanac: impl Iterator<Item=&'a str>) -> usize {
    // This is like the above solution but it considers entire ranges of numbers at a time
    // Unfortunately we can't treat each seed individually as that would take far too long and would
    // take too much memory
    // So we start with our initial set of ranges and we try to perform the mapping as a range
    // Since ranges will usually not be fully mapped, we usually need to split ranges
    // So part of a range will be mapped and a new range may need to be created for the unmapped
    // portion
    let map_pattern: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let seeds = almanac.next().unwrap();
    let seeds = seeds[7..].split(" ").map(|seed| usize::from_str(seed).unwrap()).collect::<Vec<usize>>();

    let mut ranges = seeds.chunks_exact(2).map(|slice| (slice[0], slice[1])).collect::<Vec<(usize, usize)>>();
    let mut can_map = vec![false; ranges.len()];
    while let Some(map) = almanac.next() { // We will consume perform all of the mappings (in order)
        let Some(mapping) = map_pattern.captures(map) else {
            can_map.iter_mut().for_each(|x| *x = true ); // Reset the can map to true when we fail to match
            continue
        };
        let destination_start = usize::from_str(mapping.get(1).unwrap().as_str()).unwrap();
        let source_start = usize::from_str(mapping.get(2).unwrap().as_str()).unwrap();
        let map_length = usize::from_str(mapping.get(3).unwrap().as_str()).unwrap();
        let source_end = source_start + map_length; // Excluded

        // Update all of the seeds which fall into this mapping
        let mut new_ranges = vec![];
        let mut new_can_map = vec![];
        ranges.iter_mut().enumerate().for_each(|(i, (start_ref, length_ref))| {
            let start = *start_ref;
            let length = *length_ref;
            let end = start + length; // Excluded

            // They intersect if the region that would be mapped is valid
            let mapped_start = max(start, source_start);
            let mapped_end = min(end, source_end);
            let intersects_mapping_range = mapped_start < mapped_end;

            if can_map[i] && intersects_mapping_range {
                if mapped_start == start { // We are going to map the current range
                    can_map[i] = false; // Prevent this range from being mapped again
                    *start_ref = start - source_start + destination_start;
                    *length_ref = mapped_end - start;
                } else { // mapped_start == source_start (also start < mapped_start)
                    // We'll shorten the current range and produce a new, mapped range
                    // start_ref does not change
                    *length_ref = mapped_start - start;
                    new_ranges.push((destination_start, mapped_end - mapped_start));
                    new_can_map.push(false); // We can't map this new range as its already been mapped
                }
                // Regardless of what we did we might need to handle the stuff after the end of the mapped region
                if mapped_end < end { // We need to split this range
                    new_ranges.push((mapped_end, end - mapped_end)); // The remainder of the range we didn't map
                    new_can_map.push(true); // We can still map this new range as it hasn't been mapped yet
                }
            }
        });
        ranges.extend(new_ranges);
        can_map.extend(new_can_map);
    }

    *ranges.iter().map(|(start, _)| start).min().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(
        35,
        get_lowest_location_value(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".lines()
        )
    )
}


#[test]
fn test_part2() {
    assert_eq!(
        46,
        get_lowest_location_value_range(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".lines()
        )
    )
}
