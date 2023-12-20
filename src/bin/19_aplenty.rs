/*
The Elves of Gear Island are thankful for your help and send you on your
way. They even have a hang glider that someone stole from Desert Island;
since you're already going that direction, it would help them a lot if you
would use it to get down there and return it to them.

As you reach the bottom of the relentless avalanche of machine parts, you
discover that they're already forming a formidable heap. Don't worry,
though - a group of Elves is already here organizing the parts, and they
have a system.

To start, each part is rated in each of four categories:

- x: Extremely cool looking
- m: Musical (it makes a noise when you hit it)
- a: Aerodynamic
- s: Shiny

Then, each part is sent through a series of workflows that will ultimately
accept or reject the part. Each workflow has a name and contains a list of
rules; each rule specifies a condition and where to send the part if the
condition is true. The first rule that matches the part being considered is
applied immediately, and the part moves on to the destination described by
the rule. (The last rule in each workflow has no condition and always
applies if reached.)

Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is
named ex and contains four rules. If workflow ex were considering a
specific part, it would perform the following steps in order:

- Rule "x>10:one": If the part's x is more than 10, send the part to the
workflow named one.
- Rule "m<20:two": Otherwise, if the part's m is less than 20, send the
part to the workflow named two.
- Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is
immediately rejected (R).
- Rule "A": Otherwise, because no other rules matched the part, the part
is immediately accepted (A).

If a part is sent to another workflow, it immediately switches to the start
of that workflow instead and never returns. If a part is accepted (sent to
A) or rejected (sent to R), the part immediately stops any further
processing.

The system works, but it's not keeping up with the torrent of weird metal
shapes. The Elves ask if you can help sort a few parts and give you the
list of workflows and some part ratings (your puzzle input). For example:

px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}

The workflows are listed first, followed by a blank line, then the ratings
of the parts the Elves would like you to sort. All parts begin in the
workflow named in. In this example, the five listed parts go through the
following workflows:

- {x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
- {x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
- {x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
- {x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
- {x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A

Ultimately, three parts are accepted. Adding up the x, m, a, and s rating
for each of the accepted parts gives 7540 for the part with x=787, 4623
for the part with x=2036, and 6951 for the part with x=2127. Adding all of
the ratings for all of the accepted parts gives the sum total of 19114.

Sort through all of the parts you've been given; what do you get if you add
together all of the rating numbers for all of the parts that ultimately get
accepted?

--- Part Two ---

Even with your help, the sorting process still isn't fast enough.

One of the Elves comes up with a new plan: rather than sort parts
individually through all of these workflows, maybe you can figure out in
advance which combinations of ratings will be accepted or rejected.

Each of the four ratings (x, m, a, s) can have an integer value ranging
from a minimum of 1 to a maximum of 4000. Of all possible distinct
combinations of ratings, your job is to figure out which ones will be
accepted.

In the above example, there are 167409079868000 distinct combinations of
ratings that will be accepted.

Consider only your list of workflows; the list of part ratings that the
Elves wanted you to sort is no longer relevant. How many distinct
combinations of ratings will be accepted by the Elves' workflows?
 */

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let file = fs::read_to_string("./inputs/19_aplenty.txt").unwrap();
    let workflows_and_ratings = file.lines();
    println!("{}", get_total_accepted(workflows_and_ratings.clone()));
    println!("{}", get_accept_combinations(workflows_and_ratings));
}

/// Represents a type of rule.
/// This is either one of the four part variables or always (i.e. always apply this rule)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RuleType {
    X,
    M,
    A,
    S,
    Always,
}

impl<'a> From<&'a str> for RuleType {
    fn from(value: &'a str) -> Self {
        match value {
            "x" => RuleType::X,
            "m" => RuleType::M,
            "a" => RuleType::A,
            "s" => RuleType::S,
            _ => panic!("Unknown rule type {value}")
        }
    }
}

/// Get the ordering from a string
fn ordering_from_str(value: &str) -> Ordering {
    match value {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!("Unknown ordering {value}")
    }
}

/// Get the result of a rule
/// If a rule applies this is "executed"
#[derive(Debug, Clone, Eq, PartialEq)]
enum RuleResult {
    Accepted,
    Rejected,
    SendToWorkflow(String),
}

impl<'a> From<&'a str> for RuleResult {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => RuleResult::Accepted,
            "R" => RuleResult::Rejected,
            _ => RuleResult::SendToWorkflow(value.to_string())
        }
    }
}

/// Represents a rule
#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    rule_type: RuleType,
    comparison: Ordering,
    value: usize,
    result: RuleResult,
}

/// Represents a workflow (i.e. a list of steps)
#[derive(Debug, Clone, Eq, PartialEq)]
struct Workflow {
    steps: Vec<Rule>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

/// Preprocess the input into a more useful form
fn preprocess<'a>(mut workflows_and_ratings: impl Iterator<Item=&'a str>) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = vec![];

    let rule_regex = Regex::new(r"^(\w+)([<>])(\d+):(\w+)$").unwrap();
    let part_regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}$").unwrap();

    while let Some(workflow) = workflows_and_ratings.next() {
        if workflow.is_empty() {
            break;
        }

        let Some((name, rules)) = workflow.split_once('{') else { break; };
        let rules = &rules[..(rules.len() - 1)]; // Chop off the closing brace
        workflows.insert(name.into(), Workflow {
            steps: rules.split(',').map(|rule| {
                let captures = rule_regex.captures(rule);
                if let Some(captures) = captures {
                    let rule_type = captures.get(1).unwrap().as_str().into();
                    let comparison = ordering_from_str(captures.get(2).unwrap().as_str());
                    let value = usize::from_str(captures.get(3).unwrap().as_str()).unwrap();
                    let result = captures.get(4).unwrap().as_str().into();
                    Rule {
                        rule_type,
                        comparison,
                        value,
                        result,
                    }
                } else {
                    let result = rule.into();
                    Rule {
                        rule_type: RuleType::Always,
                        comparison: Ordering::Equal, // This does not matter
                        value: 0, // This does not matter
                        result,
                    }
                }
            }).collect(),
        });
    }

    while let Some(rating) = workflows_and_ratings.next() {
        let captures = part_regex.captures(rating).unwrap();
        parts.push(Part {
            x: usize::from_str(captures.get(1).unwrap().as_str()).unwrap(),
            m: usize::from_str(captures.get(2).unwrap().as_str()).unwrap(),
            a: usize::from_str(captures.get(3).unwrap().as_str()).unwrap(),
            s: usize::from_str(captures.get(4).unwrap().as_str()).unwrap(),
        });
    }

    (workflows, parts)
}

/// Evaluate a part using a given workflow
fn evaluate_part<'a>(part: &Part, workflow: &'a Workflow) -> &'a RuleResult {
    for step in &workflow.steps {
        let part_value = match step.rule_type {
            RuleType::X => part.x,
            RuleType::M => part.m,
            RuleType::A => part.a,
            RuleType::S => part.s,
            RuleType::Always => return &step.result,
        };
        let comparison = part_value.cmp(&step.value);
        // If we watched the step then do the result of the step
        if comparison == step.comparison {
            return &step.result;
        }
    }
    // If we failed then lets just reject - we shouldn't hit this as all workflows end with an
    // ALWAYS rule
    &RuleResult::Rejected
}

/// Check if a part is accepted by the set of workflows
/// Used to solve part 1
fn is_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow = &"in".to_string();

    loop {
        let Some(current_workflow) = workflows.get(workflow) else { return false; };

        match evaluate_part(part, current_workflow) {
            RuleResult::Accepted => return true,
            RuleResult::Rejected => return false,
            RuleResult::SendToWorkflow(next_workflow) => workflow = next_workflow,
        }
    }
}

/// Part 1
fn get_total_accepted<'a>(workflows_and_ratings: impl Iterator<Item=&'a str>) -> usize {
    let (workflows, parts) = preprocess(workflows_and_ratings);

    parts.into_iter()
        // Get the parts which are accepted
        .filter(|part| is_accepted(part, &workflows))
        // Compute the sum of their stats
        .map(|part| part.x + part.m + part.a + part.s)
        // Sum it all up
        .sum()
}

/// Represents a range of parts
#[derive(Debug, Clone, Eq, PartialEq)]
struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl PartRange {
    /// Get the total size of this part range
    fn get_total_size(&self) -> usize {
        self.x.clone().count() * self.m.clone().count() * self.a.clone().count() * self.s.clone().count()
    }
}

/// Get the amount of combinations which will be accepted by a given workflow for a given range
/// Used to recursively solve part 2
fn get_accept_combinations_for_range<'a>(part_range: PartRange, workflow: &'a String, workflows: &'a HashMap<String, Workflow>) -> usize {
    if part_range.get_total_size() == 0 { // Base case - nothing
        return 0;
    }
    let Some(workflow) = workflows.get(workflow) else { return 0; };

    let mut total = 0;
    let mut remaining_part_range = part_range;

    for step in &workflow.steps {
        // Each rule results in a part which matches the rule step and another part which doesn't
        // match
        // Either or none of these ranges can be empty
        let (applies, fails) = match step.rule_type {
            RuleType::X => {
                let (applies, fails) = match step.comparison {
                    Ordering::Less => (
                        (*remaining_part_range.x.start())..=(step.value - 1),
                        step.value..=(*remaining_part_range.x.end()),
                    ),
                    Ordering::Greater => (
                        (step.value + 1)..=(*remaining_part_range.x.end()),
                        (*remaining_part_range.x.start())..=step.value,
                    ),
                    _ => panic!("Not allowed")
                };
                (
                    PartRange {
                        x: applies,
                        ..remaining_part_range.clone()
                    },
                    PartRange {
                        x: fails,
                        ..remaining_part_range.clone()
                    }
                )
            }
            RuleType::M => {
                let (applies, fails) = match step.comparison {
                    Ordering::Less => (
                        (*remaining_part_range.m.start())..=(step.value - 1),
                        step.value..=(*remaining_part_range.m.end()),
                    ),
                    Ordering::Greater => (
                        (step.value + 1)..=(*remaining_part_range.m.end()),
                        (*remaining_part_range.m.start())..=step.value,
                    ),
                    _ => panic!("Not allowed")
                };
                (
                    PartRange {
                        m: applies,
                        ..remaining_part_range.clone()
                    },
                    PartRange {
                        m: fails,
                        ..remaining_part_range.clone()
                    }
                )
            }
            RuleType::A => {
                let (applies, fails) = match step.comparison {
                    Ordering::Less => (
                        (*remaining_part_range.a.start())..=(step.value - 1),
                        step.value..=(*remaining_part_range.a.end()),
                    ),
                    Ordering::Greater => (
                        (step.value + 1)..=(*remaining_part_range.a.end()),
                        (*remaining_part_range.a.start())..=step.value,
                    ),
                    _ => panic!("Not allowed")
                };
                (
                    PartRange {
                        a: applies,
                        ..remaining_part_range.clone()
                    },
                    PartRange {
                        a: fails,
                        ..remaining_part_range.clone()
                    }
                )
            }
            RuleType::S => {
                let (applies, fails) = match step.comparison {
                    Ordering::Less => (
                        (*remaining_part_range.s.start())..=(step.value - 1),
                        step.value..=(*remaining_part_range.s.end()),
                    ),
                    Ordering::Greater => (
                        (step.value + 1)..=(*remaining_part_range.s.end()),
                        (*remaining_part_range.s.start())..=step.value,
                    ),
                    _ => panic!("Not allowed")
                };
                (
                    PartRange {
                        s: applies,
                        ..remaining_part_range.clone()
                    },
                    PartRange {
                        s: fails,
                        ..remaining_part_range.clone()
                    }
                )
            }
            RuleType::Always => {
                (remaining_part_range.clone(), PartRange { x: 1..=0, m: 1..=0, a: 1..=0, s: 1..=0 })
            }
        };
        match &step.result {
            RuleResult::Accepted => {
                // If we accept the range add its size to our total
                total += applies.get_total_size();
            }
            RuleResult::Rejected => {
                // If we reject the range - delete it from further consideration and ignore it
            }
            RuleResult::SendToWorkflow(workflow) => {
                // Get the amount of entries in the other workflow which are accepted
                total += get_accept_combinations_for_range(applies, workflow, workflows);
            }
        }
        // Continue trying to apply future steps on the range which did not match the current step
        remaining_part_range = fails;
        if remaining_part_range.get_total_size() == 0 { // If there is nothing left to do, stop
            break;
        }
    }

    total
}

/// Part 2
fn get_accept_combinations<'a>(workflows_and_ratings: impl Iterator<Item=&'a str>) -> usize {
    let (workflows, _) = preprocess(workflows_and_ratings);

    get_accept_combinations_for_range(PartRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    }, &"in".to_string(), &workflows)
}

#[test]
fn test_part1() {
    assert_eq!(
        19114,
        get_total_accepted(
            r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".lines()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        167409079868000,
        get_accept_combinations(
            r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".lines()
        )
    );
}
