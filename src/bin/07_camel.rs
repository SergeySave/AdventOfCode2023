/*
Your all-expenses-paid trip turns out to be a one-way, five-minute ride in
an airship. (At least it's a cool airship!) It drops you off at the edge of
a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing
goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't
sure what parts she's looking for; you're here to figure out why the sand
stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons
you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look
like very large rocks covering half of the horizon. The Elf explains that
the rocks are all along the part of Desert Island that is directly above
Island Island, making it hard to even get there. Normally, they use big
machines to move the rocks and filter the sand, but the machines have
broken down because Desert Island recently stopped receiving the parts they
need to fix the machines.

You've already assumed it'll be your job to figure out why the parts
stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game
of Camel Cards. Camel Cards is sort of similar to poker except it's
designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them
based on the strength of each hand. A hand consists of five cards labeled
one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of
each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

- Five of a kind, where all five cards have the same label: AAAAA
- Four of a kind, where four cards have the same label and one card has
a different label: AA8AA
- Full house, where three cards have the same label, and the remaining
two cards share a different label: 23332
- Three of a kind, where three cards have the same label, and the
remaining two cards are each different from any other card in the
hand: TTT98
- Two pair, where two cards share one label, two other cards share a
second label, and the remaining card has a third label: 23432
- One pair, where two cards share one label, and the other three cards
have a different label from the pair and each other: A23A4
- High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full house is
stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start
by comparing the first card in each hand. If these cards are different, the
hand with the stronger first card is considered stronger. If the first card
in each hand have the same label, however, then move on to considering the
second card in each hand. If they differ, the hand with the higher second
card wins; otherwise, continue with the third card in each hand, then the
fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger
because its first card is stronger. Similarly, 77888 and 77788 are both a
full house, but 77888 is stronger because its third card is stronger (and
both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding
bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

This example shows five hands; each hand is followed by its bid amount.
Each hand wins an amount equal to its bid multiplied by its rank, where the
weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up
to the strongest hand. Because there are five hands in this example, the
strongest hand will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

- 32T3K is the only one pair and the other hands are all a stronger
type, so it gets rank 1.
- KK677 and KTJJT are both two pair. Their first cards both have the
same label, but the second card of KK677 is stronger (K vs T), so
KTJJT gets rank 2 and KK677 gets rank 3.
- T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first
card, so it gets rank 5 and T55J5 gets rank 4.

Now, you can determine the total winnings of this set of hands by adding up
the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2
+ 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are
6440.

Find the rank of every hand in your set. What are the total winnings?

--- Part Two ---

To make things a little more interesting, the Elf introduces one additional
rule. Now, J cards are jokers - wildcards that can act like whatever card
would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even
than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5,
4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of
determining hand type; for example, QJJQ2 is now considered four of a kind.
However, for the purpose of breaking ties between two hands of the same
type, J is always treated as J, not the card it's pretending to be: JKKK2
is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

- 32T3K is still the only one pair; it doesn't contain any jokers, so
its strength doesn't increase.
- KK677 is now the only two pair, making it the second-weakest hand.
- T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3,
QQQJA gets rank 4, and KTJJT gets rank 5.

With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are
the new total winnings?
 */

use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./inputs/07_camel.txt").unwrap();
    let hands = file.lines();
    println!("{}", get_total_winnings(hands.clone()));
    println!("{}", get_total_winnings_wildcard(hands));
}

/// Represents a hand (and the information extracted from it)
struct Hand {
    hand: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    /// Construct a hand (part 1/normal rules)
    fn normal(hand_input: &str) -> Self {
        Self::construct(hand_input, Card::normal)
    }
    /// Construct a hand (part 2/wildcard rules)
    fn wildcard(hand_input: &str) -> Self {
        Self::construct(hand_input, Card::wildcard)
    }

    /// Common construct logic
    fn construct(hand_input: &str, card_builder: impl Fn(char) -> Card) -> Self {
        let mut parts = hand_input.split(" ");
        let hand = parts.next().unwrap();
        let hand = hand.chars().map(card_builder).collect();
        let bid = usize::from_str(parts.next().unwrap()).unwrap();
        let hand_type = HandType::from(&hand);
        Self {
            hand,
            bid,
            hand_type,
        }
    }
}

/// Represents the type of a hand - this is ordered so that the best hand is the largest value
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    /// Determine the type of hand from counts of each type of card
    fn from(cards: &Vec<Card>) -> Self {
        // Map of the counts of each distinct card type
        let mut counts = cards.iter().counts_by(|c| *c);
        // Get the number of wildcards
        let wildcards = *counts.get(&Card::Wildcard).unwrap_or(&0);
        // Remove them from our counts (essentially remove them from the hand for now)
        counts.remove(&Card::Wildcard);

        // Convert our map into a vector of card counts (because we don't care about the type fo the cards)
        let mut counts = counts.into_values().collect::<Vec<usize>>();
        // Sort these counts in descending order
        counts.sort_by(|right, left| left.cmp(right)); // Sort reversed so that it is descending

        // Get the count of the most used card (in case we don't have any cards just set this to 0)
        let top_card = *counts.get(0).unwrap_or(&0);

        // We will always try to apply our wildcards to our best type
        if top_card + wildcards == 5 {
            HandType::FiveOfAKind
        } else if top_card + wildcards == 4 {
            HandType::FourOfAKind
        } else if top_card + wildcards == 3 {
            if counts[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if top_card + wildcards == 2 {
            if counts[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }
}

/// Represents a type of card
/// Not all of these cards will always be used
/// Most notably both the Joker and a Wildcard will likely not show up in the same game
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
    Wildcard = 0,
}

impl Card {
    /// Convert a character to its card type - assuming a normal game (i.e. J is a J)
    fn normal(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            _ => panic!("Unknown Card")
        }
    }
    /// Convert a character to its card type - assuming a wildcard game (i.e. J is a Wildcard)
    fn wildcard(value: char) -> Self {
        match value {
            'J' => Card::Wildcard,
            _ => Self::normal(value)
        }
    }
}

/// Get the total winnings for a set of hands
fn determine_total_winnings(mut hands: Vec<Hand>) -> usize {
    // Sort the hands in ascending rank order
    hands.sort_by(|left, right| {
        // First sort by hand type
        let hand_compare = left.hand_type.cmp(&right.hand_type);
        if hand_compare != Ordering::Equal {
            hand_compare
        } else {
            // Otherwise sort by hand (starting from the first card
            left.hand.cmp(&right.hand)
        }
    });
    // Finally multiply each hand's rank by its bid and sum the result
    hands.iter().enumerate().map(|(i, hand)| {
        (i + 1) * hand.bid
    }).sum()
}

/// Get the total winnings of a normal game (part 1)
fn get_total_winnings<'a>(hands: impl Iterator<Item=&'a str>) -> usize {
    let hands = hands.map(Hand::normal).collect();
    determine_total_winnings(hands)
}

/// Get the total winnings of a wildcard game (part 2)
fn get_total_winnings_wildcard<'a>(hands: impl Iterator<Item=&'a str>) -> usize {
    let hands = hands.map(Hand::wildcard).collect();
    determine_total_winnings(hands)
}

#[test]
fn test_part1() {
    assert_eq!(
        6440,
        get_total_winnings(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".lines()
        )
    )
}

#[test]
fn test_part2() {
    assert_eq!(
        5905,
        get_total_winnings_wildcard(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".lines()
        )
    )
}
