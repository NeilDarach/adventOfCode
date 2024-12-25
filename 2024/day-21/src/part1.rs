use crate::custom_error::AocError;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use utils::grid::{Direction4, Grid, Path, Xy};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let codes = input.lines().collect::<Vec<_>>();
    let mut instructions = vec![];
    for code in codes {
        let value = &code[0..code.len() - 1].parse::<i64>().unwrap();
        let i1 = instructions_for(&numpad(), code.to_string());
        let i2 = instructions_for(&dirpad(), i1);
        let i3 = instructions_for(&dirpad(), i2);
        instructions.push((*value, i3));
    }
    let score = instructions
        .iter()
        .map(|(v, i)| dbg!(*v * i.len() as i64))
        .sum::<i64>();
    Ok(score.to_string())
}

pub fn dirpad() -> Grid<char> {
    let mut grid = Grid::<char>::empty();
    grid.insert(Xy::new(1, 0), '^');
    grid.insert(Xy::new(2, 0), 'A');
    grid.insert(Xy::new(0, 1), '<');
    grid.insert(Xy::new(1, 1), 'v');
    grid.insert(Xy::new(2, 1), '>');
    grid
}

pub fn numpad() -> Grid<char> {
    let mut grid = Grid::<char>::empty();
    grid.insert(Xy::new(0, 0), '7');
    grid.insert(Xy::new(1, 0), '8');
    grid.insert(Xy::new(2, 0), '9');
    grid.insert(Xy::new(0, 1), '4');
    grid.insert(Xy::new(1, 1), '5');
    grid.insert(Xy::new(2, 1), '6');
    grid.insert(Xy::new(0, 2), '1');
    grid.insert(Xy::new(1, 2), '2');
    grid.insert(Xy::new(2, 2), '3');

    grid.insert(Xy::new(1, 3), '0');
    grid.insert(Xy::new(2, 3), 'A');
    grid
}

pub fn decode_path(path: &Path<(Xy, Direction4, i64)>) -> String {
    //println!("Decoding {:?}", path);
    let mut v = path.to_vec();
    v.reverse();
    let mut r = "".to_string();
    for t in &v[1..] {
        r.push(match &t.1 {
            Direction4::N => '^',
            Direction4::E => '>',
            Direction4::S => 'v',
            Direction4::W => '<',
        })
    }
    r
}
pub fn shortest_routes<T>(grid: &Grid<T>, start: T, end: T) -> Vec<String>
where
    T: Debug + Display + Eq,
{
    let start = grid
        .all()
        .find(|(_k, v)| v == &Some(&start))
        .map(|(k, _v)| k)
        .unwrap();
    let end = grid
        .all()
        .find(|(_k, v)| v == &Some(&end))
        .map(|(k, _v)| k)
        .unwrap();
    let mut queue = VecDeque::new();
    let mut best = HashMap::new();
    let mut visited = HashSet::new();
    let mut best_len = i64::MAX;
    queue.push_back(Path::new((start, Direction4::W, 0)));
    while let Some(p) = queue.pop_front() {
        let head = p.head();
        let current = head.0;
        if head.2 > best_len {
            //println!("not as good");
            continue;
        }
        if current == end {
            //println!("found a match");
            let route = decode_path(&p);
            best_len = route.len() as i64;
            best.entry(best_len)
                .and_modify(|v: &mut Vec<String>| v.push(route.clone()))
                .or_insert(vec![route.clone()]);
            continue;
        };
        if visited.contains(&current) {
            //println!("been here");
            continue;
        }
        visited.insert(current);
        for d in Direction4::all() {
            let turn_penalty = if head.2 == 0 { 0 } else { 11 };
            if grid.get(current + d).is_some() {
                queue.push_back(
                    &p + (
                        current + d,
                        d,
                        head.2 + (if d == head.1 { 1 } else { turn_penalty }),
                    ),
                );
            }
        }
    }
    best.get(&best_len).unwrap().clone()
}

fn instructions_for(grid: &Grid<char>, output: String) -> String {
    let mut r = String::new();
    let mut current = 'A';
    for c in output.chars() {
        r.push_str(&shortest_routes(grid, current, c)[0]);
        r.push('A');
        current = c;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("126384", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_dirpad_shortest() -> miette::Result<()> {
        assert_eq!(vec![">"], shortest_routes(&dirpad(), '^', 'A'));
        assert_eq!(vec![">^>", ">>^"], shortest_routes(&dirpad(), '<', 'A'));
        Ok(())
    }

    #[test]
    fn test_numpad_shortest() -> miette::Result<()> {
        assert_eq!(vec!["^<<"], shortest_routes(&numpad(), 'A', '1'));
        Ok(())
    }
    #[test]
    fn test_instructions_for() -> miette::Result<()> {
        assert_eq!(
            "^<<A^^A>>AvvvA",
            instructions_for(&numpad(), "179A".to_string())
        );
        assert_eq!(
            "<A^A^^>AvvvA",
            instructions_for(&numpad(), "029A".to_string())
        );
        Ok(())
    }
    #[test]
    fn test_179a() -> miette::Result<()> {
        let i1 = instructions_for(&numpad(), "179A".to_string());
        let i2 = instructions_for(&dirpad(), i1);
        let i3 = instructions_for(&dirpad(), i2);
        assert_eq!(68, i3.len());
        Ok(())
    }
}
