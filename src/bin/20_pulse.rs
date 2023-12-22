/*
With your help, the Elves manage to find the right parts and fix all of the
machines. Now, they just need to send the command to boot up the machines
and get the sand flowing again.

The machines are far apart and wired together with long cables. The cables
don't connect to the machines directly, but rather to communication modules
attached to the machines that perform various initialization tasks and also
act as communication relays.

Modules communicate using pulses. Each pulse is either a high pulse or a
low pulse. When a module sends a pulse, it sends that type of pulse to each
module in its list of destination modules.

There are several different types of modules:

Flip-flop modules (prefix %) are either on or off; they are initially off.
If a flip-flop module receives a high pulse, it is ignored and nothing
happens. However, if a flip-flop module receives a low pulse, it flips
between on and off. If it was off, it turns on and sends a high pulse. If
it was on, it turns off and sends a low pulse.

Conjunction modules (prefix &) remember the type of the most recent pulse
received from each of their connected input modules; they initially default
to remembering a low pulse for each input. When a pulse is received, the
conjunction module first updates its memory for that input. Then, if it
remembers high pulses for all inputs, it sends a low pulse; otherwise, it
sends a high pulse.

There is a single broadcast module (named broadcaster). When it receives a
pulse, it sends the same pulse to all of its destination modules.

Here at Desert Machine Headquarters, there is a module with a single button
on it called, aptly, the button module. When you push the button, a single
low pulse is sent directly to the broadcaster module.

After pushing the button, you must wait until all pulses have been
delivered and fully handled before pushing it again. Never push the button
if modules are still processing pulses.

Pulses are always processed in the order they are sent. So, if a pulse is
sent to modules a, b, and c, and then module a processes its pulse and
sends more pulses, the pulses sent to modules b and c would have to be
handled first.

The module configuration (your puzzle input) lists each module. The name of
the module is preceded by a symbol identifying its type, if any. The name
is then followed by an arrow and a list of its destination modules. For
example:

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a

In this module configuration, the broadcaster has three destination modules
named a, b, and c. Each of these modules is a flip-flop module (as
indicated by the % prefix). a outputs to b which outputs to c which
outputs to another module named inv. inv is a conjunction module (as
indicated by the & prefix) which, because it has only one input, acts like
an inverter (it sends the opposite of the pulse type it receives); it
outputs to a.

By pushing the button once, the following pulses are sent:

button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a

After this sequence, the flip-flop modules all end up off, so pushing the
button again repeats the same sequence.

Here's a more interesting example:

broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output

This module configuration includes the broadcaster, two flip-flops (named a
and b), a single-input conjunction module (inv), a multi-input conjunction
module (con), and an untyped module named output (for testing purposes).
The multi-input conjunction module con watches the two flip-flop modules
and, if they're both on, sends a low pulse to the output module.

Here's what happens if you push the button once:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output

Both flip-flops turn on and a low pulse is sent to output! However, now
that both flip-flops are on and con remembers a high pulse from each of its
two inputs, pushing the button a second time does something different:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
Flip-flop a turns off! Now, con remembers a low pulse from module a, and so
it sends only a high pulse to output.

Push the button a third time:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output
This time, flip-flop a turns on, then flip-flop b turns off. However,
before b can turn off, the pulse sent to con is handled first, so it
briefly remembers all high pulses for its inputs and sends a low pulse to
output. After that, flip-flop b turns off, which causes con to update its
state and send a high pulse to output.

Finally, with a on and b off, push the button a fourth time:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output

This completes the cycle: a turns off, causing con to remember only low
pulses and restoring all modules to their original states.

To get the cables warmed up, the Elves have pushed the button 1000 times.
How many pulses got sent as a result (including the pulses sent by the
button itself)?

In the first example, the same thing happens every time the button is
pushed: 8 low pulses and 4 high pulses are sent. So, after pushing the
button 1000 times, 8000 low pulses and 4000 high pulses are sent.
Multiplying these together gives 32000000.

In the second example, after pushing the button 1000 times, 4250 low
pulses and 2750 high pulses are sent. Multiplying these together gives
11687500.

Consult your module configuration; determine the number of low pulses and
high pulses that would be sent after pushing the button 1000 times, waiting
for all pulses to be fully handled after each push of the button. What do
you get if you multiply the total number of low pulses sent by the total
number of high pulses sent?

--- Part Two ---

The final machine responsible for moving the sand down to Island Island has
a module attached named rx. The machine turns on when a single low pulse is
sent to rx.

Reset all modules to their default states. Waiting for all pulses to be
fully handled after each button press, what is the fewest number of button
presses required to deliver a single low pulse to the module named rx?
 */

use std::collections::{HashMap, VecDeque};
use std::fs;

use num::Integer;
use num::integer::lcm;

fn main() {
    let file = fs::read_to_string("./inputs/20_pulse.txt").unwrap();
    let modules = file.lines();
    println!("{}", get_1000_pulse_product(modules.clone()));
    println!("{}", get_fewest_pushes_to_rx(modules));
}

/// Represents a type of pulse
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

/// A signal sent by one of the modules in this problem
#[derive(Debug, Eq, PartialEq)]
struct Signal {
    source: usize,
    pulse: Pulse,
    destination: usize,
}

/// A structure to store a module
#[derive(Debug, Clone, Eq, PartialEq)]
enum Module {
    Broadcaster {
        outputs: Vec<usize>,
    },
    FlipFlop {
        is_on: bool,
        outputs: Vec<usize>,
    },
    Conjunction {
        received: HashMap<usize, Pulse>,
        outputs: Vec<usize>,
    },
    Null,
}

/// Preprocess the input into a more useful form
fn preprocess<'a>(modules: impl Iterator<Item=&'a str>) -> (Vec<Module>, usize, HashMap<&'a str, usize>) {
    let modules: Vec<_> = modules.collect(); // We need to loop over this twice so store it
    let mut result = vec![];
    let mut broadcast = 0;
    let mut rx = 0;
    result.push(Module::Null); // Keep a null module around in slot 0 just to be safe

    let mut mapping = HashMap::<&'a str, usize>::new();
    modules.iter().for_each(|line| {
        let (module, _) = line.split_once(" -> ").unwrap();
        if module == "broadcaster" {
            mapping.insert(module, result.len());
            broadcast = result.len();
            result.push(Module::Broadcaster { outputs: vec![] });
        } else if module.starts_with('%') {
            mapping.insert(&module[1..], result.len());
            result.push(Module::FlipFlop { is_on: false, outputs: vec![] });
        } else if module.starts_with('&') {
            mapping.insert(&module[1..], result.len());
            result.push(Module::Conjunction { received: HashMap::new(), outputs: vec![] });
        }
    });

    mapping.insert("rx", result.len());
    rx = result.len();
    result.push(Module::Null);

    modules.iter().enumerate().for_each(|(i, line)| {
        let (_, destination) = line.split_once(" -> ").unwrap();
        let module_index = i + 1;

        let mut new_outputs = vec![];
        destination.split(", ").for_each(|recipient| {
            let recipient = *mapping.get(recipient).unwrap_or(&0);

            new_outputs.push(recipient);
            if let Module::Conjunction { received, .. } = &mut result[recipient] {
                received.insert(module_index, Pulse::Low);
            }
        });

        match &mut result[module_index] {
            Module::Broadcaster { outputs } => { *outputs = new_outputs }
            Module::FlipFlop { outputs, .. } => { *outputs = new_outputs }
            Module::Conjunction { outputs, .. } => { *outputs = new_outputs }
            Module::Null => {}
        };
    });

    (result, broadcast, mapping)
}

/// Simulate pushing the button
fn push_button(modules: &mut Vec<Module>, broadcast: usize, mut on_pulse: impl FnMut(Signal) -> ()) {
    let mut signal_queue = VecDeque::<Signal>::new();

    signal_queue.push_back(Signal {
        source: 0, // Coming from the NULL
        pulse: Pulse::Low,
        destination: broadcast,
    });

    while let Some(Signal { source, pulse, destination }) = signal_queue.pop_front() {
        on_pulse(Signal {
            source,
            pulse,
            destination,
        });

        match &mut modules[destination] {
            Module::Broadcaster { outputs } => {
                outputs.iter().for_each(|output| {
                    signal_queue.push_back(Signal { source: destination, pulse, destination: *output });
                });
            }
            Module::FlipFlop { is_on, outputs } => {
                if pulse == Pulse::Low {
                    *is_on = !*is_on;
                    let new_pulse = if *is_on { Pulse::High } else { Pulse::Low };
                    outputs.iter().for_each(|output| {
                        signal_queue.push_back(Signal { source: destination, pulse: new_pulse, destination: *output });
                    });
                }
            }
            Module::Conjunction { received, outputs } => {
                received.insert(source, pulse);
                let new_pulse = if received.values().all(|p| *p == Pulse::High) { Pulse::Low } else { Pulse::High };
                outputs.iter().for_each(|output| {
                    signal_queue.push_back(Signal { source: destination, pulse: new_pulse, destination: *output });
                });
            }
            Module::Null => {} // NoOp
        }
    }
}

/// Solve part 1
fn get_1000_pulse_product<'a>(modules: impl Iterator<Item=&'a str>) -> usize {
    // Brute force - push the button 1000 times
    let (mut modules, broadcast, _) = preprocess(modules);

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    // Push the button 1000 times
    for _ in 0..1000 {
        push_button(&mut modules, broadcast, |Signal { pulse, .. }| {
            match pulse {
                Pulse::Low => low_pulse_count += 1,
                Pulse::High => high_pulse_count += 1,
            }
        });
    }

    low_pulse_count * high_pulse_count
}

/// Solve part2
fn get_fewest_pushes_to_rx<'a>(modules: impl Iterator<Item=&'a str>) -> usize {
    // I hate this solution as it relies on a lot of domain knowledge
    // However, my original solution (brute force) was taking too long
    // And my next solution (by attempting to reduce the problem to binary cycles) failed
    // As 3197 steps into the problem whatever was causing the binary cycles suddenly stops working
    // So I ended up looking some tips as to what others had done and it seemed like
    // applying the domain knowledge given the known structure of the input is the solution
    let (mut modules, broadcast, mapping) = preprocess(modules);

    // rx gets its signal only from &dn
    // All four of these are send signals to &dn
    // So we'll use the domain knowledge that &dn goes low (and thus rx gets a low pulse)
    // only if all four of these signals go high
    // While this could be improved by programmatically finding these
    // since I got this path to a solution from the internet I choose not to further improve it
    let dd = *mapping.get("dd").unwrap();
    let fh = *mapping.get("fh").unwrap();
    let xp = *mapping.get("xp").unwrap();
    let fc = *mapping.get("fc").unwrap();

    // We will figure out when each of these nodes sends a high pulse
    let mut pushes = 0_usize;
    let mut dd_pushes = vec![];
    let mut fh_pushes = vec![];
    let mut xp_pushes = vec![];
    let mut fc_pushes = vec![];

    // We need at least two pulses from each of them
    while dd_pushes.len() < 2 || fh_pushes.len() < 2 || xp_pushes.len() < 2 || fc_pushes.len() < 2 {
        pushes += 1;
        let _ = push_button(&mut modules, broadcast, |Signal { source, pulse, .. }| {
            // Record the pulse
            if pulse == Pulse::High {
                if source == dd {
                    dd_pushes.push(pushes);
                } else if source == fh {
                    fh_pushes.push(pushes);
                } else if source == xp {
                    xp_pushes.push(pushes);
                } else if source == fc {
                    fc_pushes.push(pushes);
                }
            }
        });
    }

    // Figure out the amount of time between the first pulse and the second pulse
    let dd_pushes = dd_pushes[1] - dd_pushes[0];
    let fh_pushes = fh_pushes[1] - fh_pushes[0];
    let xp_pushes = xp_pushes[1] - xp_pushes[0];
    let fc_pushes = fc_pushes[1] - fc_pushes[0];

    // All of these will pulse together at the lcm of the above times
    [dd_pushes, fh_pushes, xp_pushes, fc_pushes]
        .into_iter()
        .reduce(lcm)
        .unwrap()
}

#[test]
fn test_part1a() {
    assert_eq!(
        32000000,
        get_1000_pulse_product(
            r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a".lines()
        )
    );
}

#[test]
fn test_part1b() {
    assert_eq!(
        11687500,
        get_1000_pulse_product(
            r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output".lines()
        )
    );
}

/*
Stuff from a previous attempt

fn recurse_possible_states(modules: &Vec<Module>, index: usize, output: Pulse) -> Vec<HashMap<usize, bool>> {
    let module = &modules[index];
    let Module::Conjunction { received, .. } = (match module {
        Module::Broadcaster { .. } => return vec![],
        Module::FlipFlop { .. } => {
            let mut map = HashMap::new();
            map.insert(index, output == Pulse::High);
            return vec![map];
        },
        Module::Conjunction { .. } => module,
        Module::Null => return vec![],
    }) else { panic!("Can't happen") };
    match output {
        Pulse::Low => {
            // All incoming must be HIGH
            let mut result = vec![];
            for (input, _) in received {
                let new_states = recurse_possible_states(modules, *input, Pulse::High);
                if result.is_empty() {
                    result = new_states
                } else {
                    let mut new_result = vec![];
                    for new in &new_states {
                        for old in &result {
                            // If they don't conflict
                            if old.iter().all(|(index, value)| new.get(index).unwrap_or(value) == value) {
                                let mut entry = new.clone();
                                entry.extend(old);
                                new_result.push(entry);
                            } else {
                                println!();
                            }
                        }
                    }
                    result = new_result;
                }
            }

            result
        }
        Pulse::High => {
            let lows = received.keys().powerset().filter(|x| x.len() >= 1);
            // At least one incoming must be LOW
            let mut full_result = vec![];
            for low in lows {
                let mut result = vec![];
                for (input, _) in received {
                    let new_states = recurse_possible_states(modules, *input, if low.contains(&input) { Pulse::Low } else { Pulse::High });
                    if result.is_empty() {
                        result = new_states
                    } else {
                        let mut new_result = vec![];
                        for new in &new_states {
                            for old in &result {
                                // If they don't conflict
                                if old.iter().all(|(index, value)| new.get(index).unwrap_or(value) == value) {
                                    let mut entry = new.clone();
                                    entry.extend(old);
                                    new_result.push(entry);
                                } else {
                                    println!();
                                }
                            }
                        }
                        result = new_result;
                    }
                }
                full_result.extend(result);
            }

            full_result
        }
    }
}

fn propagate_state(modules: &mut Vec<Module>) {
    let mut signal_queue = VecDeque::<Signal>::new();

    for (module_index, module) in modules.iter_mut().enumerate() {
        let Module::FlipFlop { is_on, outputs } = module else { continue; };
        for output in outputs {
            signal_queue.push_back(Signal {
                source: module_index,
                pulse: if *is_on { Pulse::High } else { Pulse::Low },
                destination: *output,
            });
        }
    }

    while let Some(Signal { source, pulse, destination }) = signal_queue.pop_front() {
        match &mut modules[destination] {
            Module::Conjunction { received, outputs } => {
                received.insert(source, pulse);
                let new_pulse = if received.values().all(|p| *p == Pulse::High) { Pulse::Low } else { Pulse::High };
                outputs.iter().for_each(|output| {
                    signal_queue.push_back(Signal { source: destination, pulse: new_pulse, destination: *output });
                });
            }
            _  => {} // NoOp
        }
    }
}

fn get_possible_states(modules: &Vec<Module>, rx: usize) -> Vec<HashMap<usize, bool>> {
    // The problem only has one CONJUNCTION module output to rx
    let Some((index, _)) = modules.iter().find_position(|x| match x {
        Module::Broadcaster { .. } => false,
        Module::FlipFlop { .. } => false,
        Module::Conjunction { outputs, .. } => outputs.contains(&rx),
        Module::Null => false,
    }) else { panic!("No way to write to rx") };

    recurse_possible_states(modules, index, Pulse::Low)
}

fn reset(modules: &mut Vec<Module>) {
    for module in modules.iter_mut() {
        match module {
            Module::Broadcaster { .. } => {}
            Module::FlipFlop { is_on, .. } => *is_on = false,
            Module::Conjunction { received, .. } => received.values_mut().for_each(|x| *x = Pulse::Low),
            Module::Null => {}
        }
    }
}

fn apply_partial_state(modules: &mut Vec<Module>, partial_state: &Vec<usize>) {
    for x in partial_state {
        let Module::FlipFlop { is_on, .. } = &mut modules[*x] else { continue };
        *is_on = !*is_on;
    }
}
 */
