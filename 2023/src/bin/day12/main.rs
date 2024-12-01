use aoc_2023::aoc::*;
use futures::future::select_all;
use itertools::Itertools;
use tokio::spawn;

pub fn duplicate(
    springs: String,
    spec: Vec<usize>,
    count: usize,
    result: Option<u128>,
) -> (String, Vec<usize>, Option<u128>) {
    let spring_string = (0..count)
        .map(|_| springs.clone())
        .intersperse_with(|| "?".to_owned())
        .fold("".to_owned(), |mut a, e| {
            a.push_str(&e);
            a
        });

    let summary = (0..count).flat_map(|_| spec.clone()).collect::<Vec<_>>();

    (spring_string, summary, result)
}

fn input() -> &'static str {
    include_str!("input.txt")
}
fn partial() -> &'static str {
    include_str!("partial.txt")
}

pub fn longest_run(input: &str) -> (usize, usize) {
    let hashes = input
        .chars()
        .map(|e| if e == '?' { '#' } else { e })
        .collect::<String>();
    let longest_run = hashes
        .split('.')
        .fold("", |a, e| if a.len() > e.len() { a } else { e });
    let position = hashes.find(longest_run).unwrap();
    (position, longest_run.len())
}

async fn count_ways_async(i: usize, input: &str, spec: &[usize]) -> u128 {
    count_ways(i, input, spec)
}

fn count_ways(i: usize, input: &str, spec: &[usize]) -> u128 {
    let rv = count_ways_impl(None, input, spec, false);
    /*
    let poss_hash = input
        .chars()
        .filter(|&e| e == '#' || e == '?')
        .collect::<Vec<_>>()
        .len();
    let poss_dot = input
        .chars()
        .filter(|&e| e == '.' || e == '?')
        .collect::<Vec<_>>()
        .len();
    println!("{} poss_hash: {}, poss_dot: {}", input, poss_hash, poss_dot);
    let rv = count_ways_impl2(
        None,
        input,
        None,
        spec,
        false,
        poss_hash,
        poss_dot,
        longest_run(input),
    );
    */
    println!("{}: {} {:?} = {}", i, input, spec, rv);
    rv
}

fn count_ways_impl(first: Option<&str>, input: &str, spec: &[usize], in_run: bool) -> u128 {
    println!("Checking {:?}/{} {:?}", first, input, spec);
    if input.is_empty() && first.is_none() {
        if spec.is_empty() {
            println!("spec empty - yes");
            return 1;
        }
        if spec == vec![0] {
            println!("spec 0 - yes");
            return 1;
        }
        println!("input empty - no");
        return 0;
    }
    let (first, input) = match first {
        Some(v) => (v, input),
        None => (&input[0..1], &input[1..]),
    };
    match first {
        "#" if spec.is_empty() => 0,
        "#" if spec[0] == 0 => 0,
        "#" => {
            let mut new_spec = spec.to_vec();
            new_spec[0] -= 1;
            count_ways_impl(None, input, &new_spec[..], true)
        }
        "." if spec.is_empty() => count_ways_impl(None, input, spec, false),
        "." if spec[0] == 0 => count_ways_impl(None, input, &spec[1..], false),
        "." if !in_run => count_ways_impl(None, input, spec, false),
        "." => 0,
        "?" => {
            count_ways_impl(Some("#"), input, spec, in_run)
                + count_ways_impl(Some("."), input, spec, in_run)
        }
        _ => panic!("Not possible"),
    }
}

fn count_ways_impl2(
    first: Option<&str>,
    input: &str,
    first_spec: Option<usize>,
    spec: &[usize],
    in_run: bool,
    poss_hash: usize,
    poss_dot: usize,
    run: (usize, usize),
) -> u128 {
    println!(
        "{:?}/{} {:?}/{:?}  poss_hash: {}, poss_dot: {}",
        first, input, first_spec, spec, poss_hash, poss_dot
    );
    if input.is_empty() && first.is_none() {
        if spec.is_empty() {
            println!("empty spec - yes");
            return 1;
        }
        if spec == vec![0] {
            println!("spec 0 - yes");
            return 1;
        }
        println!("input empty - no");
        return 0;
    }
    if spec.is_empty() && first_spec.is_none() {
        if input.find('#').is_some() {
            println!("empty spec, hashes - no");
            return 0;
        }

        if poss_dot == input.len() + (if first.is_none() { 0 } else { 1 }) {
            println!("empty spec, all dots - yes");
            return 1;
        }
    }

    let spec_len = spec.len() + (if first_spec.is_none() { 0 } else { 1 });
    if poss_dot < spec_len - 1 {
        println!(
            "pos_dot ({}) < spec_len-1 ({}) - no",
            poss_dot,
            spec_len - 1
        );
        return 0;
    }

    if spec.iter().sum::<usize>() + first_spec.unwrap_or(0)
        > input
            .chars()
            .filter(|&e| e == '#')
            .collect::<Vec<_>>()
            .len()
            + (if let Some("#") = first { 1 } else { 0 })
    {
        println!("not enough hashes - no");
        return 0;
    }

    let first_spec_val = if let Some(v) = first_spec { v } else { 0 };
    if spec.iter().sum::<usize>() + first_spec_val > poss_hash {
        return 0;
    }

    let (first, input) = match first {
        Some(v) => (v, input),
        None => (&input[0..1], &input[1..]),
    };

    let (first_spec, spec) = match first_spec {
        Some(v) => (v, spec),
        None => (spec[0], &spec[1..]),
    };

    match first {
        "#" if spec.is_empty() => 0,
        "#" if spec[0] == 0 => 0,
        "#" => {
            let first_spec = if spec[0] > 1 { Some(spec[0] - 1) } else { None };
            let spec = &spec[1..];
            count_ways_impl2(
                None,
                input,
                first_spec,
                spec,
                true,
                poss_hash - 1,
                poss_dot,
                run,
            )
        }
        "." if spec.is_empty() => count_ways_impl2(
            None,
            input,
            Some(first_spec),
            spec,
            false,
            poss_hash,
            poss_dot - 1,
            run,
        ),
        "." if spec[0] == 0 => count_ways_impl2(
            None,
            input,
            None,
            &spec[1..],
            false,
            poss_hash,
            poss_dot - 1,
            run,
        ),
        "." if !in_run => count_ways_impl2(
            None,
            input,
            Some(first_spec),
            spec,
            false,
            poss_hash,
            poss_dot - 1,
            run,
        ),
        "." => 0,
        "?" => {
            count_ways_impl2(
                Some("#"),
                input,
                Some(first_spec),
                spec,
                in_run,
                poss_hash,
                poss_dot - 1,
                run,
            ) + count_ways_impl2(
                Some("."),
                input,
                Some(first_spec),
                spec,
                in_run,
                poss_hash - 1,
                poss_dot,
                run,
            )
        }
        _ => panic!("Not possible"),
    }
}
async fn part1(data: &str) -> u128 {
    let lines = parse::springs(data).unwrap().1;
    let mut tasks = lines
        .into_iter()
        .enumerate()
        .map(|(i, (springs, spec, _result))| {
            spawn(async move { count_ways_async(i, &springs, &spec).await })
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    loop {
        let completed = select_all(tasks).await;
        let result = completed.0.expect("task failed");
        total += result;
        tasks = completed.2;
        if tasks.is_empty() {
            break;
        }
    }
    total
    /*
    lines
        .iter()
        .enumerate()
        .map(|(i, (springs, spec))| {
            //println!("Checking {}", i);
            count_ways(&springs[..], &spec[..])
        })
        .sum()
        */
}

async fn part2(data: &str) -> u128 {
    let lines = parse::springs(data).unwrap().1;
    let mut tasks = lines
        .into_iter()
        .map(|(springs, spec, _result)| duplicate(springs, spec, 5, None))
        .enumerate()
        .map(|(i, (springs, spec, _result))| {
            spawn(async move { count_ways_async(i, &springs, &spec).await })
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    loop {
        let completed = select_all(tasks).await;
        let result = completed.0.expect("task failed");
        total += result;
        tasks = completed.2;
        println!("{} tasks left", tasks.len());
        if tasks.is_empty() {
            break;
        }
    }
    total
    /*
    lines
        .into_iter()
        .map(|(springs, spec)| duplicate(springs, spec, 5))
        .enumerate()
        .map(|(i, (springs, spec))| {
            //println!("Checking {}", i);
            count_ways(i, &springs[..], &spec[..])
        })
        .sum()
        */
}

#[tokio::main()]
async fn main() {
    println!("Day x of {}", version());
    let val = part1(input()).await;
    println!("Part 1 answer is {}", val);
    //let val = part2(input()).await;
    //println!("Part 2 answer is {}", val);
}

mod parse {

    use super::*;
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::character::complete::one_of;
    use nom::combinator::map;
    use nom::combinator::opt;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};

    pub fn inputline(i: &str) -> IResult<&str, (String, Vec<usize>, Option<u128>)> {
        let (i, states) = terminated(many1(one_of("#.?")), tag(" "))(i)?;
        let (i, vals) = separated_list1(tag(","), map(digit1, |s: &str| s.parse().unwrap()))(i)?;
        let (i, result) = opt(preceded(
            tag(" = "),
            map(digit1, |e: &str| e.parse::<u128>().unwrap()),
        ))(i)?;
        Ok((i, (states.iter().collect::<String>(), vals, result)))
    }

    pub fn springs(i: &str) -> IResult<&str, Vec<(String, Vec<usize>, Option<u128>)>> {
        separated_list1(line_ending, inputline)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_parse() {
        let lines = parse::springs(sample()).unwrap().1;
        assert_eq!(6, lines.len());
        assert_eq!("???.###", lines[0].0);
        assert_eq!(vec![1, 1, 3], lines[0].1);
        assert_eq!(None, lines[0].2);
    }
    #[test]
    fn test_parse_partial() {
        let lines = parse::springs(partial()).unwrap().1;
        assert_eq!(837, lines.len());
        assert_eq!(
            "???#??..?#?..????#??..?#?..????#??..?#?..????#??..?#?..????#??..?#?..",
            lines[2].0
        );
        assert_eq!(vec![5, 1, 5, 1, 5, 1, 5, 1, 5, 1], lines[2].1);
        assert_eq!(Some(162), lines[2].2);
    }

    #[test]
    fn test_part_1() {
        let lines = parse::springs(sample()).unwrap().1;
        let results = lines
            .into_iter()
            .enumerate()
            .map(|(i, (springs, spec, _result))| count_ways(i, &springs, &spec))
            .collect::<Vec<_>>();

        assert_eq!(21, results.iter().sum::<u128>());
    }

    #[test]
    fn test_sample_part_2() {
        let lines = parse::springs(sample()).unwrap().1;
        let results = lines
            .into_iter()
            .map(|(springs, spec, _result)| duplicate(springs, spec, 5, None))
            .enumerate()
            .map(|(i, (springs, spec, _result))| count_ways(i, &springs, &spec))
            .collect::<Vec<_>>();

        assert_eq!(525152, results.iter().sum::<u128>());
    }

    /*
    #[test]
    fn test_long() {
        let (input, spec) = duplicate("??..?.??.?.??.".to_owned(), vec![1, 1, 1], 5);
        assert_eq!(0, count_ways(&input, &spec));
    }
    */
    #[test]
    fn test_broken() {
        assert_eq!(1, count_ways(0, "?.#.??.#.#", &[1, 1, 1]));
    }
}
