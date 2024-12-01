use std::{cmp::Ordering, collections::HashMap};

use aoc_2023::aoc::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum Card {
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hand {
    Flush((Card, Card, Card, Card, Card)),
    FourOfAKind((Card, Card, Card, Card, Card)),
    FullHouse((Card, Card, Card, Card, Card)),
    ThreeOfAKind((Card, Card, Card, Card, Card)),
    TwoPair((Card, Card, Card, Card, Card)),
    Pair((Card, Card, Card, Card, Card)),
    High((Card, Card, Card, Card, Card)),
}

impl Card {
    pub fn new_wild(c: char) -> Self {
        match c {
            'J' => Self::Joker,
            _ => Self::new(c),
        }
    }
    pub fn new(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Bad card value: {}", c),
        }
    }
}

impl Hand {
    pub fn new_wild(s: &str) -> Self {
        if s.len() != 5 {
            panic!("Bad hand value: {}", s);
        }
        let mut map: HashMap<Card, usize> = HashMap::new();
        for c in s.chars() {
            map.entry(Card::new_wild(c))
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        let jokers = if let Some(v) = map.remove(&Card::Joker) {
            v
        } else {
            0
        };
        let mut sig: Vec<usize> = map.values().copied().collect();
        sig.sort();
        let mut size = sig.len();
        if size == 0 {
            sig.push(0);
            size = 1;
        }
        sig[size - 1] += jokers;
        let mut chars = s.chars();
        let cards = (
            Card::new_wild(chars.next().unwrap()),
            Card::new_wild(chars.next().unwrap()),
            Card::new_wild(chars.next().unwrap()),
            Card::new_wild(chars.next().unwrap()),
            Card::new_wild(chars.next().unwrap()),
        );
        if sig == vec![5] {
            return Self::Flush(cards);
        }
        if sig == vec![1, 4] {
            return Self::FourOfAKind(cards);
        }
        if sig == vec![2, 3] {
            return Self::FullHouse(cards);
        }
        if sig == vec![1, 1, 3] {
            return Self::ThreeOfAKind(cards);
        }
        if sig == vec![1, 2, 2] {
            return Self::TwoPair(cards);
        }
        if sig == vec![1, 1, 1, 2] {
            return Self::Pair(cards);
        }
        if sig == vec![1, 1, 1, 1, 1] {
            return Self::High(cards);
        }
        panic!("Bad hand signature: {:?}", &sig);
    }
    pub fn new(s: &str) -> Self {
        if s.len() != 5 {
            panic!("Bad hand value: {}", s);
        }
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }
        let mut sig: Vec<usize> = map.values().copied().collect();
        sig.sort();
        let mut chars = s.chars();
        let cards = (
            Card::new(chars.next().unwrap()),
            Card::new(chars.next().unwrap()),
            Card::new(chars.next().unwrap()),
            Card::new(chars.next().unwrap()),
            Card::new(chars.next().unwrap()),
        );
        if sig == vec![5] {
            return Self::Flush(cards);
        }
        if sig == vec![1, 4] {
            return Self::FourOfAKind(cards);
        }
        if sig == vec![2, 3] {
            return Self::FullHouse(cards);
        }
        if sig == vec![1, 1, 3] {
            return Self::ThreeOfAKind(cards);
        }
        if sig == vec![1, 2, 2] {
            return Self::TwoPair(cards);
        }
        if sig == vec![1, 1, 1, 2] {
            return Self::Pair(cards);
        }
        if sig == vec![1, 1, 1, 1, 1] {
            return Self::High(cards);
        }
        panic!("Bad hand signature: {:?}", sig);
    }

    pub fn rank(&self) -> usize {
        match self {
            Self::Flush(_) => 7,
            Self::FourOfAKind(_) => 6,
            Self::FullHouse(_) => 5,
            Self::ThreeOfAKind(_) => 4,
            Self::TwoPair(_) => 3,
            Self::Pair(_) => 2,
            Self::High(_) => 1,
        }
    }

    pub fn cards(&self) -> (Card, Card, Card, Card, Card) {
        match self {
            Self::Flush(c) => c,
            Self::FourOfAKind(c) => c,
            Self::FullHouse(c) => c,
            Self::ThreeOfAKind(c) => c,
            Self::TwoPair(c) => c,
            Self::Pair(c) => c,
            Self::High(c) => c,
        }
        .clone()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank() > other.rank() {
            return Ordering::Greater;
        }
        if self.rank() < other.rank() {
            return Ordering::Less;
        }
        let cards = self.cards();
        let othercards = other.cards();
        if cards.0 != othercards.0 {
            return cards.0.cmp(&othercards.0);
        }
        if cards.1 != othercards.1 {
            return cards.1.cmp(&othercards.1);
        }
        if cards.2 != othercards.2 {
            return cards.2.cmp(&othercards.2);
        }
        if cards.3 != othercards.3 {
            return cards.3.cmp(&othercards.3);
        }
        if cards.4 != othercards.4 {
            return cards.4.cmp(&othercards.4);
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let mut hands = parse::hands(data).unwrap().1;
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, val)| (i + 1) as u64 * val.1)
        .sum()
}

fn part2(data: &str) -> u64 {
    let mut hands = parse::wild_hands(data).unwrap().1;
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, val)| (i + 1) as u64 * val.1)
        .sum()
}

fn main() {
    println!("Day x of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use super::*;
    use nom::character::complete::alphanumeric1;
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::multi::many1;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn hand_and_bid(i: &str) -> IResult<&str, (Hand, u64)> {
        let (i, hand) = alphanumeric1(i)?;
        let (i, _) = many1(tag(" "))(i)?;
        let (i, bid) = digit1(i)?;
        let bid = bid.parse().unwrap();
        let hand = Hand::new(hand);
        Ok((i, (hand, bid)))
    }
    pub fn wild_hand_and_bid(i: &str) -> IResult<&str, (Hand, u64)> {
        let (i, hand) = alphanumeric1(i)?;
        let (i, _) = many1(tag(" "))(i)?;
        let (i, bid) = digit1(i)?;
        let bid = bid.parse().unwrap();
        let hand = Hand::new_wild(hand);
        Ok((i, (hand, bid)))
    }

    pub fn hands(i: &str) -> IResult<&str, Vec<(Hand, u64)>> {
        many1(terminated(hand_and_bid, line_ending))(i)
    }
    pub fn wild_hands(i: &str) -> IResult<&str, Vec<(Hand, u64)>> {
        many1(terminated(wild_hand_and_bid, line_ending))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_sample() {
        let hands = parse::hands(sample()).unwrap().1;
        assert_eq!(5, hands.len());
        assert_eq!(765, hands[0].1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(6440, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(5905, part2(sample()));
    }

    #[test]
    fn test_build_hand() {
        let flush = Hand::new("TTTTT");
        let pair = Hand::new("55678");
        assert!(flush > pair);
    }
    #[test]
    fn test_compare_high() {
        let one = Hand::new("23467");
        let two = Hand::new("23457");
        assert!(one > two);
    }
    #[test]
    fn test_new_wild() {
        let flush = Hand::new_wild("JJAAA");
        assert_eq!(7, flush.rank());
    }
}
