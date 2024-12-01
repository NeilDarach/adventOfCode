use aoc_2023::aoc::*;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct Card {
    id: u64,
    winning: HashSet<u64>,
    chosen: HashSet<u64>,
}

pub struct Game {
    cards: HashMap<u64, (Card, u64)>,
}

impl Game {
    pub fn new(cards: Vec<Card>) -> Self {
        Self {
            cards: cards.into_iter().map(|e| (e.id, (e, 1))).collect(),
        }
    }

    pub fn process_card(&mut self, id: u64) {
        let card = self.cards.get(&id).unwrap();
        let matches = card.0.matches();
        let count = card.1;
        for i in id + 1..=id + matches {
            self.cards.get_mut(&i).unwrap().1 += count;
        }
    }

    pub fn process(&mut self) {
        for i in 1..self.cards.len() as u64 {
            self.process_card(i);
        }
    }

    pub fn total(&self) -> u64 {
        self.cards.iter().map(|e| e.1 .1).sum()
    }
}

impl Card {
    pub fn new(id: u64, winning: HashSet<u64>, chosen: HashSet<u64>) -> Self {
        Self {
            id,
            winning,
            chosen,
        }
    }

    pub fn matches(&self) -> u64 {
        self.winning
            .intersection(&self.chosen)
            .copied()
            .collect::<Vec<_>>()
            .len() as u64
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let (_, game) = parse::game(data).unwrap();

    game.iter()
        .map(|e| e.matches())
        .map(|e| if e == 0 { 0 } else { 1 << (e - 1) })
        .sum()
}

fn part2(data: &str) -> u64 {
    let (_, cards) = parse::game(data).unwrap();
    let mut game = Game::new(cards);
    game.process();
    game.total()
}

fn main() {
    println!("Day 1 of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use super::*;
    use nom::character::complete::line_ending;
    use nom::character::complete::space0;
    use nom::character::complete::space1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::pair;
    use nom::{bytes::complete::tag, IResult};

    pub fn card(i: &str) -> IResult<&str, Card> {
        let (i, id) = delimited(pair(tag("Card"), space1), number, pair(tag(":"), space1))(i)?;
        let (i, winning) = space_separated_numbers(i)?;
        let (i, _) = space0(i)?;
        let (i, _) = tag("|")(i)?;
        let (i, chosen) = space_separated_numbers(i)?;
        Ok((
            i,
            Card::new(
                id,
                winning.into_iter().collect(),
                chosen.into_iter().collect(),
            ),
        ))
    }

    pub fn game(i: &str) -> IResult<&str, Vec<Card>> {
        separated_list1(line_ending, card)(i)
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
        assert_eq!(13, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(30, part2(sample()));
    }

    #[test]
    fn test_one_card() {
        let card = parse::card("Card 2: 1 2 3 | 11 12 13\n").unwrap().1;
        assert_eq!(2, card.id);
        assert_eq!(
            vec![1, 2, 3].into_iter().collect::<HashSet<u64>>(),
            card.winning
        );
        assert_eq!(
            vec![11, 12, 13].into_iter().collect::<HashSet<u64>>(),
            card.chosen
        );
    }

    #[test]
    fn test_input() {
        let games = parse::game(sample()).unwrap().1;
        assert_eq!(6, games.len());
    }
}
