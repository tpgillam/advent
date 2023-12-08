use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card<const PART2: bool> {
    value: i32,
}

#[derive(Debug)]
struct ParseCardErr {
    msg: String,
}

impl<const PART2: bool> Card<PART2> {
    fn new(label: char) -> Result<Card<PART2>, ParseCardErr> {
        let value = match label {
            '2' => Ok(1),
            '3' => Ok(2),
            '4' => Ok(3),
            '5' => Ok(4),
            '6' => Ok(5),
            '7' => Ok(6),
            '8' => Ok(7),
            '9' => Ok(8),
            'T' => Ok(9),
            'J' => Ok(if PART2 { 0 } else { 10 }),
            'Q' => Ok(11),
            'K' => Ok(12),
            'A' => Ok(13),
            _ => Err(ParseCardErr {
                msg: format!("Unrecognised character {label}").to_string(),
            }),
        }?;

        Ok(Card { value })
    }
}

// These hand types are listed in _increasing_ order of score,
// such that the default ordering implementations are valid.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq)]
struct Hand<const PART2: bool> {
    cards: Vec<Card<PART2>>,
}

fn infer_hand_type<const PART2: bool>(label_to_count: &HashMap<&Card<PART2>, usize>) -> HandType {
    let n = label_to_count.len();

    if n == 1 {
        HandType::FiveOfAKind
    } else if n == 2 {
        if label_to_count.values().any(|&v| v == 4) {
            HandType::FourOfAKind
        } else {
            // This must be the 3 + 2 case
            HandType::FullHouse
        }
    } else if n == 3 {
        // Either three-of-a-kind or two-pair
        if label_to_count.values().any(|&v| v == 3) {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        }
    } else if n == 4 {
        HandType::OnePair
    } else {
        // n == 5
        HandType::HighCard
    }
}

impl<const PART2: bool> Hand<PART2> {
    /// Identify the hand type for this hand.
    fn hand_type(&self) -> HandType {
        let label_to_count = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });

        if !PART2 {
            infer_hand_type(&label_to_count)
        } else {
            let joker = Card::new('J').unwrap();
            // Handle jokers -- assign the joker count to whichever other card has the highest count.
            let n_jokers = match label_to_count.get(&joker) {
                Some(&x) => x,
                _ => 0usize,
            };
            if n_jokers == 5 {
                return HandType::FiveOfAKind;
            }

            let mut new_label_to_count: HashMap<_, _> = label_to_count
                .into_iter()
                .filter(|&(k, _)| *k != joker)
                .collect();

            let (&max_key, _) = new_label_to_count.iter().max_by_key(|&(_, v)| v).unwrap();
            *(new_label_to_count.get_mut(max_key).unwrap()) += n_jokers;

            infer_hand_type(&new_label_to_count)
        }
    }
}

impl<const PART2: bool> PartialOrd for Hand<PART2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Delegate to implementation for Ord
        Some(self.cmp(other))
    }
}

impl<const PART2: bool> Ord for Hand<PART2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        match self_type.cmp(&other_type) {
            Ordering::Equal => {
                // Equality, so define ordering in terms of the cards lexicographically.
                self.cards.cmp(&other.cards)
            }
            // In other cases we are defined purely in terms of type comparison
            x => x,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ParseHandErr {
    msg: String,
}

impl<const PART2: bool> FromStr for Hand<PART2> {
    type Err = ParseHandErr;

    fn from_str(s: &str) -> Result<Hand<PART2>, Self::Err> {
        if s.len() != 5 {
            return Err(ParseHandErr {
                msg: "Length was {s.len()}, should be 5".to_string(),
            });
        }

        let cards: Vec<_> = s
            .chars()
            .map(Card::new)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|x| ParseHandErr { msg: x.msg })?;
        Ok(Hand { cards })
    }
}

struct HandBid<const PART2: bool> {
    hand: Hand<PART2>,
    bid: u64,
}

#[derive(Debug)]
struct ParseHandBidErr;

impl<const PART2: bool> FromStr for HandBid<PART2> {
    type Err = ParseHandBidErr;

    fn from_str(s: &str) -> Result<HandBid<PART2>, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').ok_or(ParseHandBidErr)?;
        let hand = hand_str
            .parse::<Hand<PART2>>()
            .map_err(|_| ParseHandBidErr)?;
        let bid = bid_str.parse::<u64>().map_err(|_| ParseHandBidErr)?;
        Ok(HandBid { hand, bid })
    }
}

fn part<const PART2: bool>(input: &str) -> String {
    let mut hand_bids: Vec<_> = input
        .trim()
        .lines()
        .map(|x| x.parse::<HandBid<PART2>>().unwrap())
        .collect();

    // Sort by the hand, ignoring the bid at this point.
    hand_bids.sort_unstable_by(|x, y| x.hand.cmp(&y.hand));

    let answer: u64 = hand_bids
        .iter()
        .zip(1..=hand_bids.len() as u64)
        .map(|(hand_bid, y)| hand_bid.bid * y)
        .sum();

    answer.to_string()
}

fn part1(input: &str) -> String {
    part::<false>(input)
}

fn part2(input: &str) -> String {
    part::<true>(input)
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part1, part2, Hand};

    const EXAMPLE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "6440");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "5905");
    }

    #[test]
    fn test_ordering_1() {
        let h1 = Hand::<false>::from_str("22222").unwrap();
        let h2 = Hand::<false>::from_str("4AAAA").unwrap();
        let h3 = Hand::<false>::from_str("33332").unwrap();
        let h4 = Hand::<false>::from_str("33332").unwrap();
        let h5 = Hand::<false>::from_str("2AAA2").unwrap();
        let h6 = Hand::<false>::from_str("4AAAK").unwrap();
        assert!(h1 > h2);
        assert!(h2 > h3);
        assert!(h3 == h4);
        assert!(h4 > h5);
        assert!(h5 > h6);
    }
}
