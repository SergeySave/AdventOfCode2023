# Advent of Code 2023

## My Solutions

- [Day 1: Trebuchet?!](src/bin/01_trebuchet.rs)
- [Day 2: Cube Conundrum](src/bin/02_cube_conundrum.rs)
- [Day 3: Gear Ratios](src/bin/03_gear_ratios.rs)
- [Day 4: Scratchcards](src/bin/04_scratchcards.rs)
- [Day 5: If You Give A Seed A Fertilizer](src/bin/05_fertilizer.rs)
- [Day 6: Wait For It](src/bin/06_wait.rs)
- [Day 7: Camel Cards](src/bin/07_camel.rs)
- [Day 8: Haunted Wasteland](src/bin/08_haunted.rs)
- [Day 9: Mirage Maintenance](src/bin/09_mirage.rs)
- [Day 10: Pipe Maze](src/bin/10_pipe.rs)
- [Day 11: Cosmic Expansion](src/bin/11_cosmic.rs)
- [Day 12: Hot Springs](src/bin/12_springs.rs)
- [Day 13: Point of Incidence](src/bin/13_incidence.rs)
- [Day 14: Parabolic Reflector Dish](src/bin/14_parabolic.rs)
- [Day 15: Lens Library](src/bin/15_lens.rs)
- [Day 16: The Floor Will Be Lava](src/bin/16_lava.rs)
- [Day 17: Clumsy Crucible](src/bin/17_crucible.rs)
- [Day 18: Lavaduct Lagoon](src/bin/18_lavaduct.rs)
- [Day 19: Aplenty](src/bin/19_aplenty.rs)
- [Day 20: Pulse Propagation](src/bin/20_pulse.rs)
- [Day 21: Step Counter](src/bin/21_step_counter.rs)
- [Day 22: Sand Slabs](src/bin/22_slabs.rs)
- [Day 23: A Long Walk](src/bin/23_long.rs)
- [Day 24: Never Tell Me The Odds](src/bin/24_odds.rs)

## Retrospective

While I did quite enjoy doing all the problems, I don't think that I'll try
to be as serious about doing them when they come out next year.
I felt as though it was fairly stressful trying to do them as soon as they
came out.
For the first ~18 this was mostly due to having other things going on in my
life or other plans I had.
As I live in Pacific Time, the problems would come out at 21:00 and I would
be able to get most of them done by 22:00 or 23:00.
However, when I wasn't able to get around to finishing the previous problem
or I wasn't able to be home at that time, I would be stressed thinking about
it and it would put a ticking clock on getting the problem done before the
next one came out.
After those first few problems, some of the later problems seemed to rely
on inspecting the input and applying domain knowledge derived from that
in order to actually solve it - i.e. if you just went off of the
restrictions on the input given to you in the problem statement, you would
not be able to solve the answer in a reasonable amount of time.
I'll probably continue next year, but I will probably take my time and be
significantly less diligent.

A lot of these could probably make good interview questions.

### Individual Problems

1. Simple problem where the second part required me to rethink/redo how
   I was reading in the input.
   3/5

2. Once again just a problem where I had to change how I was processing
   the input. At this point I realized that it'll be useful to write
   functions which accept the input and extract the important information
   in an appropriate data structure which can be reused by parts 1 and 2.
   3/5

3. Tried to do what I mentioned above with having a common extract
   function, but I didn't really take it far enough and had some actual
   logic in the preprocess/extract function.
   3/5

4. Nothing interesting to mention.
   3/5

5. My solution wasn't particularly elegant, but I found the range operations
   interesting.
   3/5

6. I originally solved this using binary search on the left half of the
   parabola, but then I realized I could just solve this algebraically by
   looking for the roots of the parabola.
   3/5

7. I finally started using structs and enums to represent the problem. I
   was also able to make both use the same algorithm so the only difference
   was what happened when a 'J' was ingested by the solution.
   3/5

8. Had some trouble with this at first since I was assuming that each
   individual ghost would be able to get into a loop with multiple solutions.
   So the steps between solutions would look like: A, B, C, D, B, C, D, ...
   So the domain knowledge of the input that the loops would look like:
   A, B, B, B, ... had to be discovered and applied. Luckily I was able to
   notice this myself and this also occurred in the test cases so it was
   easy to discover. Then it was just a matter of applying some of the number
   theory that I once learned. Though I don't remember the solution will be
   equal to the period so not entirely sure why that is the case.
   3.5/5

9. Nothing interesting to mention.
   3/5

10. I personally liked this one. I was originally going to go with a
    solution with "quasi-spots/states" but ended up going with the simpler
    solution of expanding the original grid to allow passing between two
    adjacent vertical pipes.
    4/5

11. I went with a more generic solution and part 2 was trivial to
    implement.
    4/5

12. This one required dynamic programming and thats not something that
    I've had to do in a while. It was a lot of fun to do that and to
    remember all the dynamic programming techniques. Though I guess I'm
    lucky that it is a 2D DP problem since I always found those easier than
    1D or higher dimension DP problems.
    5/5

13. Nothing interesting to mention.
    4/5

14. Nothing interesting to mention.
    4/5

15. Nothing interesting to mention.
    3/5

16. Nothing interesting to mention.
    3/5

17. Dijkstra's algorithm
    3/5

18. My original solution involved trying to simplify the shape of the
    input and continuously reduce the complexity to end up with a simple
    square. My final solution involved computing ranges and then expanding
    those ranges into blocks.
    4/5

19. Once again this one required operating on ranges which I quite
    enjoyed.
    4/5

20. Required domain knowledge on the structure of the input.
    I tried to do a solution based on "binary cycles". I.e. for many of the
    first few thousand cycles the states of the states of the flip flops
    appeared to act "binary". I.e. you if you got the state at cycle 1, 2, 4,
    8, 16, 32, 64, 128; you could compute the states at any cycles 0 to 255
    by taking an xor of the states after each of those. That would then let
    me efficiently compute future states. I ended up having to look up what
    other people had done for solutions to find out that they examined the
    input to find that this it has a structure where the answer is the lcm
    of four more reasily computable cycles.
    1/5

21. Wanted to implement a solution that wasn't dependent on the structure
    of the input. I had a way to do this in mind but it would really only
    work for the straight edges and not diagonals. I think it would work on
    diagonals but it would require more thought and planning than I had
    originally put into it. So I ended up going with the solution that it
    seemed everyone else was doing where they counted the different types
    of repetitions and multiplied it by the number of end positions on that
    repetition. Maybe I'll try to write the solution I wanted to at first.
    1/5

22. Nothing interesting to mention.
    3/4

23. Part 2 resulted in brute force on a reduced graph. As for part 1
    as it is a DAG the result is very easy to compute. Part 2 was just done
    by brute force. There might be a better way but I didn't bother as brute
    force on ~35 nodes only took a few seconds (with release flags (O3)
    enabled).
    3.5/5

24. Just a bunch of math. While I liked the math, it was unexpected.
    It also required thinking of a new solution to this problem. By biggest
    complaint is that the numbers are so big that the f64 in the problem have
    troubles representing intermediate states during the calculation. So very
    specific choices of the vectors from the inputs were necessary to ensure
    that the results didn't lose precision.
    2/5

25. This was kind of fun. I instantly recognized it as a minimum cut
    problem, so found some algorithms. Originally tried to implement
    Stoer-Wagner as it wouldn't be random but that was taking too long so
    I implemented Karger's Algorithm (though a flawed implementation as the
    random edge is a bad random). Fortunately this let the problem get solved
    in about a second (with release flags enabled). I was also pleasantly
    surprised that it only had a single part.
    4/5

