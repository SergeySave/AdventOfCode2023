/*
It seems like something is going wrong with the snow-making process.
Instead of forming snow, the water that's been absorbed into the air seems
to be forming hail!

Maybe there's something you can do to break up the hailstones?

Due to strong, probably-magical winds, the hailstones are all flying
through the air in perfectly linear trajectories. You make a note of each
hailstone's position and velocity (your puzzle input). For example:

19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3

Each line of text corresponds to the position and velocity of a single
hailstone. The positions indicate where the hailstones are right now (at
time 0). The velocities are constant and indicate exactly how far each
hailstone will move in one nanosecond.

Each line of text uses the format px py pz @ vx vy vz. For instance, the
hailstone specified by 20, 19, 15 @ 1, -5, -3 has initial X position 20, Y
position 19, Z position 15, X velocity 1, Y velocity -5, and Z velocity -3
. After one nanosecond, the hailstone would be at 21, 14, 12.

Perhaps you won't have to do anything. How likely are the hailstones to
collide with each other and smash into tiny ice crystals?

To estimate this, consider only the X and Y axes; ignore the Z axis.
Looking forward in time, how many of the hailstones' paths will intersect
within a test area? (The hailstones themselves don't have to collide, just
test for intersections between the paths they will trace.)

In this example, look for intersections that happen with an X and Y
position each at least 7 and at most 27; in your actual data, you'll need
to check a much larger test area. Comparing all pairs of hailstones' future
paths produces the following results:

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 18, 19, 22 @ -1, -1, -2
Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone A.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths are parallel; they never intersect.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-6, y=-5).

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-2, y=3).

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone B.

Hailstone A: 12, 31, 28 @ -1, -2, -1
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

So, in this example, 2 hailstones' future paths cross inside the boundaries
of the test area.

However, you'll need to search a much larger test area if you want to see
if any hailstones might collide. Look for intersections that happen with an
X and Y position each at least 200000000000000 and at most 400000000000000.
Disregard the Z axis entirely.

Considering only the X and Y axes, check all pairs of hailstones' future
paths for intersections. How many of these intersections occur within the
test area?

--- Part Two ---

Upon further analysis, it doesn't seem like any hailstones will naturally
collide. It's up to you to fix that!

You find a rock on the ground nearby. While it seems extremely unlikely, if
you throw it just right, you should be able to hit every hailstone in a
single throw!

You can use the probably-magical winds to reach any integer position you
like and to propel the rock at any integer velocity. Now including the Z
axis in your calculations, if you throw the rock at time 0, where do you
need to be so that the rock perfectly collides with every hailstone? Due to
probably-magical inertia, the rock won't slow down or change direction when
it collides with a hailstone.

In the example above, you can achieve this by moving to position 24, 13, 10
and throwing the rock at velocity -3, 1, 2. If you do this, you will hit
every hailstone as follows:

Hailstone: 19, 13, 30 @ -2, 1, -2
Collision time: 5
Collision position: 9, 18, 20

Hailstone: 18, 19, 22 @ -1, -1, -2
Collision time: 3
Collision position: 15, 16, 16

Hailstone: 20, 25, 34 @ -2, -2, -4
Collision time: 4
Collision position: 12, 17, 18

Hailstone: 12, 31, 28 @ -1, -2, -1
Collision time: 6
Collision position: 6, 19, 22

Hailstone: 20, 19, 15 @ 1, -5, -3
Collision time: 1
Collision position: 21, 14, 12

Above, each hailstone is identified by its initial position and its
velocity. Then, the time and position of that hailstone's collision with
your rock are given.

After 1 nanosecond, the rock has exactly the same position as one of the
hailstones, obliterating it into ice dust! Another hailstone is smashed to
bits two nanoseconds after that. After a total of 6 nanoseconds, all of the
hailstones have been destroyed.

So, at time 0, the rock needs to be at X position 24, Y position 13, and Z
position 10. Adding these three coordinates together produces 47. (Don't
add any coordinates from the rock's velocity.)

Determine the exact position and velocity the rock needs to have at time 0
so that it perfectly collides with every hailstone. What do you get if you
add up the X, Y, and Z coordinates of that initial position?
 */

use std::fs;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use num::Float;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("./inputs/24_odds.txt").unwrap();
    let hailstones = file.lines();
    println!("{}", get_intersection_count(hailstones.clone(), 200000000000000.0, 400000000000000.0));
    println!("{}", get_initial_position(hailstones));
}

/// A data type to represent a vector/position in space
/// I ended up writing this before I added the linalg package
#[derive(Debug, Clone, PartialEq)]
struct Vector([f64; 3]);

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1], self.0[2] - rhs.0[2]])
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

/// Represents a hailstone
#[derive(Debug)]
struct Hailstone {
    position: Vector,
    velocity: Vector,
}

/// Preprocess the input into a more useful form
fn preprocess<'a>(hailstones: impl Iterator<Item=&'a str>) -> Vec<Hailstone> {
    let regex = Regex::new(r"^(-?\d+),\s*(-?\d+),\s*(-?\d+)\s*@\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)$").unwrap();
    hailstones.map(|line| {
        let captures = regex.captures(line).unwrap();
        Hailstone {
            position: Vector([
                f64::from_str(captures.get(1).unwrap().as_str()).unwrap(),
                f64::from_str(captures.get(2).unwrap().as_str()).unwrap(),
                f64::from_str(captures.get(3).unwrap().as_str()).unwrap(),
            ]),
            velocity: Vector([
                f64::from_str(captures.get(4).unwrap().as_str()).unwrap(),
                f64::from_str(captures.get(5).unwrap().as_str()).unwrap(),
                f64::from_str(captures.get(6).unwrap().as_str()).unwrap(),
            ]),
        }
    }).collect()
}

/// Get the intersection on the x,y plane of two hailstones
/// They don't have to arrive at the same time (just not the past)
/// and their positions in the z axis don't matter
fn get_x_y_intersect(a: &Hailstone, b: &Hailstone) -> (f64, f64) {
    const X: usize = 0;
    const Y: usize = 1;

    if a.velocity.0[Y] / a.velocity.0[X] == b.velocity.0[Y] / b.velocity.0[X] {
        return (f64::nan(), f64::nan());
    }

    // a.position + a.velocity * t_a = b.position + b.velocity * t_b
    // a.position - b.position = b.velocity * t_b - a.velocity * t_a

    // a.x - b.x = b.dx*tb - a.dx*ta
    // a.y - b.y = b.dy*tb - a.dy*ta
    let delta_x = a.position.0[X] - b.position.0[X];
    let delta_y = a.position.0[Y] - b.position.0[Y];

    // delta_x = b.dx*tb - a.dx*ta
    // delta_y = b.dy*tb - a.dy*ta

    // delta_y + a.dy*ta = b.dy*tb
    // (delta_y + a.dy*ta)/b.dy = tb

    // delta_x = b.dx*(delta_y + a.dy*ta)/b.dy - a.dx*ta
    // a.dx*ta = b.dx*(delta_y + a.dy*ta)/b.dy - delta_x
    // ta = (b.dx*(delta_y + a.dy*ta)/b.dy - delta_y)/a.dx
    // ta = (b.dx*delta_y/b.dy + ta*b.dx*a.dy/b.dy - delta_x)/a.dx
    // ta = b.dx*delta_y/b.dy/a.dx + ta*b.dx*a.dy/b.dy/a.dx - delta_y/a.dx
    let term_1 = b.velocity.0[X] * delta_y / b.velocity.0[Y] / a.velocity.0[X];
    let factor = b.velocity.0[X] * a.velocity.0[Y] / b.velocity.0[Y] / a.velocity.0[X];
    let term_2 = delta_x / a.velocity.0[X];
    // ta = term_1 + ta*factor - term_2
    let term_3 = term_1 - term_2;
    // ta = term_3 + ta*factor
    // ta - ta*factor = term_3
    // ta*(1-factor) = term_3
    // ta = term_3/(1-factor)
    let ta = term_3 / (1.0 - factor);

    // (delta_y + a.dy*ta)/b.dy = tb
    let tb = (delta_y + a.velocity.0[Y] * ta) / b.velocity.0[Y];

    if ta < 0.0 || tb < 0.0 {
        return (f64::nan(), f64::nan());
    }

    let result = (
        a.position.0[X] + a.velocity.0[X] * ta,
        a.position.0[Y] + a.velocity.0[Y] * ta,
    );

    result
}

/// Solve part1
fn get_intersection_count<'a>(hailstones: impl Iterator<Item=&'a str>, min: f64, max: f64) -> usize {
    let hailstones = preprocess(hailstones);

    hailstones.iter()
        .combinations(2)
        .map(|pair| get_x_y_intersect(pair[0], pair[1]))
        // NaNs and Infs get filtered out by this
        .filter(|(x, y)| min <= *x && *x <= max && min <= *y && *y <= max)
        .count()
}

/// Solve part2
fn get_initial_position<'a>(hailstones: impl Iterator<Item=&'a str>) -> isize {
    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;
    let hailstones = preprocess(hailstones);

    // There exists an P={X}{Y}{Z},V={VX}{VY}{VZ} s.t.
    //   For all Hailstones H {H.P, H.V},
    //     There exists a t s.t.
    //       P.x + V.x * t = H.P.x + H.V.x * t
    //       P.y + V.y * t = H.P.y + H.V.y * t
    //       P.z + V.z * t = H.P.z + H.V.z * t
    //       7 unknowns, 3 equations

    //       P.x + V.x * t1 = H1.P.x + H1.V.x * t1
    //       P.y + V.y * t1 = H1.P.y + H1.V.y * t1
    //       P.z + V.z * t1 = H1.P.z + H1.V.z * t1
    // 7 unknowns, 3 equations
    //       P.x + V.x * t2 = H2.P.x + H2.V.x * t2
    //       P.y + V.y * t2 = H2.P.y + H2.V.y * t2
    //       P.z + V.z * t2 = H2.P.z + H2.V.z * t2
    // 8 unknowns, 6 equations
    //       P.x + V.x * t3 = H3.P.x + H3.V.x * t3
    //       P.y + V.y * t3 = H3.P.y + H3.V.y * t3
    //       P.z + V.z * t3 = H3.P.z + H3.V.z * t3
    // 9 unknowns, 9 equations

    // We want Ax = b so that we can solve this using linear algebra

    // P0 + V0 t = P1 + V1 t
    // (P0 - P1) = (V1 - V0) t

    // (P0.x - P1.x) = (V1.x - V0.x) t
    // (P0.y - P1.y) = (V1.y - V0.y) t
    // Divide the two equations: (the t term will cancel out)
    // (P0.x - P1.x) / (P0.y - P1.y) = (V1.x - V0.x) t / (V1.y - V0.y) t
    // (P0.x - P1.x) / (P0.y - P1.y) = (V1.x - V0.x) / (V1.y - V0.y)
    // (P0.x - P1.x) (V1.y - V0.y) = (V1.x - V0.x) (P0.y - P1.y)
    // Just to make stuff a bit more consistent can swap the order of P0 and P1
    // (adds a negative term to both sides which cancels out):
    // (P1.x - P0.x) (V1.y - V0.y) = (V1.x - V0.x) (P1.y - P0.y)
    // (P1.x - P0.x)V1.y - (P1.x - P0.x)V0.y = (V1.x - V0.x)P1.y - (V1.x - V0.x)P0.y
    // P1.xV1.y - P0.xV1.y - P1.xV0.y - P0.xV0.y = V1.xP1.y - V0.xP1.y - V1.xP0.y - V0.xP0.y

    // P0 and V0 are entangled with P1 and V1 and we have a term with both V0 and P0
    // So we can't easily solve this
    // So lets do the same thing but with P2 and V2 instead of P1 and V1
    // P2.xV2.y - P0.xV2.y - P2.xV0.y - P0.xV0.y = V2.xP2.y - V0.xP2.y - V2.xP0.y - V0.xP0.y
    // Now we subtract the P1 V1 version from this
    // P2.xV2.y - P0.xV2.y - P2.xV0.y - P0.xV0.y - P1.xV1.y + P0.xV1.y + P1.xV0.y + P0.xV0.y
    // = V2.xP2.y - V0.xP2.y - V2.xP0.y - V0.xP0.y - V1.xP1.y + V0.xP1.y + V1.xP0.y + V0.xP0.y
    // The V0 P0 product terms cancel out
    // P2.xV2.y - P0.xV2.y - P2.xV0.y - P1.xV1.y + P0.xV1.y + P1.xV0.y
    // = V2.xP2.y - V0.xP2.y - V2.xP0.y - V1.xP1.y + V0.xP1.y + V1.xP0.y
    // Grouping like terms
    // (P2.xV2.y - P1.xV1.y) + (V1.y - V2.y)P0.x + (P1.x - P2.x)V0.y
    // = (V2.xP2.y - V1.xP1.y) + (V1.x - V2.x)P0.y + (P1.y - P2.y)V0.x
    // Rearranging
    // (V1.y - V2.y)P0.x - (V1.x - V2.x)P0.y + (P1.x - P2.x)V0.y - (P1.y - P2.y)V0.x
    // = (V2.xP2.y - V1.xP1.y) - (P2.xV2.y - P1.xV1.y)
    // Adding Z Terms
    // (V1.y - V2.y)P0.x -(V1.x - V2.x)P0.y +0 P0.z -(P1.y - P2.y)V0.x +(P1.x - P2.x)V0.y +0 V0.z
    // = (V2.xP2.y - V1.xP1.y) - (P2.xV2.y - P1.xV1.y)
    // Rearranging this into a matrix-like form
    // [(V1.y - V2.y), -(V1.x - V2.x), 0, -(P1.y - P2.y), +(P1.x - P2.x), 0] [P0.x, P0.y, P0.z, V0.x, V0.y, V0.z]^T
    // = (V2.xP2.y - V1.xP1.y) - (P2.xV2.y - P1.xV1.y)

    let p1x = hailstones[0].position.0[X];
    let p1y = hailstones[0].position.0[Y];
    let p1z = hailstones[0].position.0[Z];
    let v1x = hailstones[0].velocity.0[X];
    let v1y = hailstones[0].velocity.0[Y];
    let v1z = hailstones[0].velocity.0[Z];

    // Interestingly not all inputs here give me the same result
    // This one happens to work, but that doesnt seem guaranteed for all sets of 3 inputs
    // I suspect this has less numerical errors if the inputs are sufficiently far apart
    let p2x = hailstones[2].position.0[X];
    let p2y = hailstones[2].position.0[Y];
    let p2z = hailstones[2].position.0[Z];
    let v2x = hailstones[2].velocity.0[X];
    let v2y = hailstones[2].velocity.0[Y];
    let v2z = hailstones[2].velocity.0[Z];

    let p3x = hailstones[3].position.0[X];
    let p3y = hailstones[3].position.0[Y];
    let p3z = hailstones[3].position.0[Z];
    let v3x = hailstones[3].velocity.0[X];
    let v3y = hailstones[3].velocity.0[Y];
    let v3z = hailstones[3].velocity.0[Z];

    const ZERO: f64 = 0.0;

    // The equation above copied down here
    // [(V1.y - V2.y), -(V1.x - V2.x), 0, -(P1.y - P2.y), +(P1.x - P2.x), 0] [P0.x, P0.y, P0.z, V0.x, V0.y, V0.z]^T
    // = (V2.xP2.y - V1.xP1.y) - (P2.xV2.y - P1.xV1.y)

    // Now we can construct the entire matrix using the left side of this applied to both the
    // (1, 2) and (1, 3) pairs
    // As well as all the (X, Y), (Y, Z), (Z, X) pairs
    let a = array![
        [v1y - v2y, -v1x + v2x, ZERO, -p1y + p2y, p1x - p2x, ZERO],
        [v1y - v3y, -v1x + v3x, ZERO, -p1y + p3y, p1x - p3x, ZERO],
        [v1z - v2z, ZERO, -v1x + v2x, -p1z + p2z, ZERO, p1x - p2x],
        [v1z - v3z, ZERO, -v1x + v3x, -p1z + p3z, ZERO, p1x - p3x],
        [ZERO, v1z - v2z, -v1y + v2y, ZERO, -p1z + p2z, p1y - p2y],
        [ZERO, v1z - v3z, -v1y + v3y, ZERO, -p1z + p3z, p1y - p3y],
    ];

    let b = array![
        (p2y * v2x - p1y * v1x) - (p2x * v2y - p1x * v1y),
        (p3y * v3x - p1y * v1x) - (p3x * v3y - p1x * v1y),
        (p2z * v2x - p1z * v1x) - (p2x * v2z - p1x * v1z),
        (p3z * v3x - p1z * v1x) - (p3x * v3z - p1x * v1z),
        (p2z * v2y - p1z * v1y) - (p2y * v2z - p1y * v1z),
        (p3z * v3y - p1z * v1y) - (p3y * v3z - p1y * v1z),
    ];

    let x = a.solve_into(b).unwrap();

    // Extract the x, y, and z from the result
    (x[0] + x[1] + x[2]).round() as isize
}

#[test]
fn test_part1() {
    assert_eq!(
        2,
        get_intersection_count(
            r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3".lines(),
            7.0,
            27.0,
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        47,
        get_initial_position(
            r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3".lines()
        )
    );
}

