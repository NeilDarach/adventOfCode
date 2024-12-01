use aoc_2023::aoc::*;
use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Network {
    components: HashMap<String, Component>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Signal {
    High(String),
    Low(String),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum State {
    On,
    Off,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Component {
    Broadcaster(String, Vec<String>),
    FlipFlop(String, State, Vec<String>),
    Conjunction(String, HashMap<String, State>, Vec<String>),
    Sink(String, u32, u32),
    Button(String),
}

fn input() -> &'static str {
    include_str!("input.txt")
}

pub fn network_2() -> Network {
    let mut network: HashMap<String, Component> = Default::default();
    network.insert(
        "broadcaster".to_owned(),
        Component::Broadcaster("broadcaster".to_owned(), vec!["a".to_owned()]),
    );
    network.insert(
        "a".to_owned(),
        Component::FlipFlop(
            "a".to_owned(),
            State::Off,
            vec!["inv".to_owned(), "con".to_owned()],
        ),
    );

    let mut invmap = HashMap::default();
    invmap.insert("a".to_owned(), State::Off);
    network.insert(
        "inv".to_owned(),
        Component::Conjunction("inv".to_owned(), invmap, vec!["b".to_owned()]),
    );
    network.insert(
        "b".to_owned(),
        Component::FlipFlop("b".to_owned(), State::Off, vec!["con".to_owned()]),
    );
    let mut conmap = HashMap::default();
    conmap.insert("a".to_owned(), State::Off);
    conmap.insert("b".to_owned(), State::Off);
    network.insert(
        "con".to_owned(),
        Component::Conjunction("con".to_owned(), conmap, vec!["output".to_owned()]),
    );
    network.insert(
        "output".to_owned(),
        Component::Sink("output".to_owned(), 0, 0),
    );
    Network {
        components: network,
    }
}

pub fn network_1() -> Network {
    let mut network: HashMap<String, Component> = Default::default();
    network.insert(
        "broadcaster".to_owned(),
        Component::Broadcaster(
            "broadcaster".to_owned(),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        ),
    );
    network.insert(
        "a".to_owned(),
        Component::FlipFlop("a".to_owned(), State::Off, vec!["b".to_owned()]),
    );
    network.insert(
        "b".to_owned(),
        Component::FlipFlop("b".to_owned(), State::Off, vec!["c".to_owned()]),
    );
    network.insert(
        "c".to_owned(),
        Component::FlipFlop("c".to_owned(), State::Off, vec!["inv".to_owned()]),
    );
    let mut invmap = HashMap::new();
    invmap.insert("c".to_owned(), State::Off);
    network.insert(
        "inv".to_owned(),
        Component::Conjunction("inv".to_owned(), invmap, vec!["a".to_owned()]),
    );
    Network {
        components: network,
    }
}

fn part1(data: &str) -> u64 {
    let mut network = parse::network(data).unwrap().1;
    let mut queue = VecDeque::new();
    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        queue.push_back(("broadcaster".to_owned(), Signal::Low("button".to_owned())));
        while let Some((dest, signal)) = queue.pop_front() {
            if signal.is_high() {
                high += 1;
            } else {
                low += 1;
            }
            //println!("  Processing {:?} -> {:?}", signal, dest);
            if let Some(component) = network.components.get_mut(&dest) {
                for next in component.process(signal) {
                    queue.push_back(next);
                }
            }
        }
        //println!("{:?}\n\n", network);
    }
    println!("{} high, {} low.  result {}", high, low, high * low);
    high * low
}

impl Component {
    pub fn name(&self) -> String {
        match self {
            Self::Broadcaster(name, _) => name.clone(),
            Self::FlipFlop(name, _, _) => name.clone(),
            Self::Conjunction(name, _, _) => name.clone(),
            Self::Sink(name, _, _) => name.clone(),
            Self::Button(name) => name.clone(),
        }
    }

    pub fn targets(&self) -> Vec<String> {
        match self {
            Self::Broadcaster(_, targets) => targets.iter().cloned().collect(),
            Self::FlipFlop(_, _, targets) => targets.iter().cloned().collect(),
            Self::Conjunction(_, _, targets) => targets.iter().cloned().collect(),
            Self::Sink(_, _, _) => vec![],
            Self::Button(_) => vec![],
        }
    }

    pub fn process(&mut self, signal: Signal) -> Vec<(String, Signal)> {
        match self {
            Self::FlipFlop(name, state, connections) => {
                if let Signal::Low(_) = signal {
                    if state == &State::On {
                        *state = State::Off;
                        connections
                            .iter()
                            .map(|e| (e.clone(), Signal::Low(name.clone())))
                            .collect()
                    } else {
                        *state = State::On;
                        connections
                            .iter()
                            .map(|e| (e.clone(), Signal::High(name.clone())))
                            .collect()
                    }
                } else {
                    vec![]
                }
            }
            Self::Conjunction(name, map, connections) => {
                //println!("{} - {:?}", &name, &map);
                if let Signal::High(from) = signal {
                    map.insert(from.clone(), State::On);
                } else if let Signal::Low(from) = signal {
                    map.insert(from.clone(), State::Off);
                }
                if map.iter().all(|e| e.1 == &State::On) {
                    connections
                        .iter()
                        .map(|e| (e.clone(), Signal::Low(name.clone())))
                        .collect()
                } else {
                    connections
                        .iter()
                        .map(|e| (e.clone(), Signal::High(name.clone())))
                        .collect()
                }
            }
            Self::Broadcaster(name, connections) => match signal {
                Signal::High(_) => connections
                    .iter()
                    .map(|e| (e.clone(), Signal::High(name.clone())))
                    .collect(),
                Signal::Low(_) => connections
                    .iter()
                    .map(|e| (e.clone(), Signal::Low(name.clone())))
                    .collect(),
            },
            Self::Button(_) => vec![],
            Self::Sink(name, high, low) => {
                //println!("Got {:?}", signal);
                match signal {
                    Signal::High(_) => *high += 1,
                    Signal::Low(_) => *low += 1,
                }
                vec![]
            }
        }
    }
}

impl Signal {
    pub fn is_high(&self) -> bool {
        match self {
            Self::High(_) => true,
            Self::Low(_) => false,
        }
    }
}

fn part2(data: &str) -> u64 {
    let mut network = parse::network(data).unwrap().1;
    let mut queue = VecDeque::new();
    for i in 0..1000000000 {
        queue.push_back(("broadcaster".to_owned(), Signal::Low("button".to_owned())));
        let mut rx_low = 0;
        let mut rx_high = 0;
        while let Some((dest, signal)) = queue.pop_front() {
            if dest == "rx" {
                if signal.is_high() {
                    rx_high += 1;
                } else {
                    println!("rx low @ {}", i);
                    rx_low += 1;
                }
            }

            //println!("  Processing {:?} -> {:?}", signal, dest);
            if let Some(component) = network.components.get_mut(&dest) {
                for next in component.process(signal) {
                    queue.push_back(next);
                }
            }
        }
        if rx_high == 0 && rx_low == 1 {
            return i;
        }
        //println!("{:?}\n\n", network);
    }
    0
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
    use nom::branch::alt;
    use nom::character::complete::alpha1;
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::pair;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn component(i: &str) -> IResult<&str, Component> {
        alt((broadcaster, flipflop, conjunction))(i)
    }

    pub fn broadcaster(i: &str) -> IResult<&str, Component> {
        let (i, name) = terminated(alpha1, tag(" -> "))(i)?;
        let (i, targets) = targets(i)?;
        let component = Component::Broadcaster(name.to_owned(), targets);
        Ok((i, component))
    }

    pub fn targets(i: &str) -> IResult<&str, Vec<String>> {
        let (i, targets) = separated_list1(tag(", "), alpha1)(i)?;
        Ok((i, targets.iter().map(|&e| e.to_owned()).collect()))
    }

    pub fn conjunction(i: &str) -> IResult<&str, Component> {
        let (i, _) = tag("&")(i)?;
        let (i, name) = terminated(alpha1, tag(" -> "))(i)?;
        let (i, targets) = targets(i)?;
        let component = Component::Conjunction(name.to_owned(), Default::default(), targets);
        Ok((i, component))
    }

    pub fn flipflop(i: &str) -> IResult<&str, Component> {
        let (i, _) = tag("%")(i)?;
        let (i, name) = terminated(alpha1, tag(" -> "))(i)?;
        let (i, targets) = targets(i)?;
        let component = Component::FlipFlop(name.to_owned(), State::Off, targets);
        Ok((i, component))
    }

    pub fn network(i: &str) -> IResult<&str, Network> {
        let (i, items) = separated_list1(line_ending, component)(i)?;
        let mut components: HashMap<String, Component> = Default::default();
        for item in &items[..] {
            components.insert(item.name().clone(), item.clone());
        }
        let mut inputs: HashMap<String, Vec<String>> = Default::default();
        for each in items {
            for target in each.targets() {
                inputs
                    .entry(target)
                    .or_insert(vec![])
                    .push(each.name().clone());
            }
        }

        for (k, v) in inputs.iter() {
            if let Some(Component::Conjunction(_, states, _)) = components.get_mut(k) {
                for input in v {
                    states.insert(input.clone(), State::Off);
                }
            }
        }

        let network = Network { components };
        Ok((i, network))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    fn sample2() -> &'static str {
        include_str!("sample2.txt")
    }

    #[test]
    fn test_parse_sample() {
        let network = parse::network(sample()).unwrap().1;
        println!("{:?}", network);
    }
    #[test]
    fn test_sample() {
        assert_eq!(32000000, part1(sample()));
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(11687500, part1(sample2()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(0, part2(sample()));
    }
}
