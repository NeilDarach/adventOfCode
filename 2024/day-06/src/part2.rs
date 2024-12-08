use crate::common::*;
use crate::custom_error::AocError;
use std::collections::HashMap;
use utils::grid::*;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut count = 0;
    let mut map = parser::parse(input);
    let guard = map.guard.clone();
    let visited = map.visited();
    map.guard = guard;
    for c in map.grid.keys() {
        let content = map.grid.get(c).cloned().unwrap();
        let guard = map.guard.clone();
        if visited.contains_key(&c) {
            if process_candidate(&mut map, 0, 0, c) {
                count += 1;
            }
            map.grid.insert(c, content);
            map.guard = guard;
        }
    }

    Ok(count.to_string())
}

pub fn process_candidate(map: &mut Map, _count: usize, _total: usize, pos: Xy) -> bool {
    if let Content::Guard(p, _) = map.guard {
        if p == pos {
            return false;
        }
    }
    if let Some(&Content::Obstacle) = map.grid.get(pos) {
        return false;
    }

    map.grid.insert(pos, Content::Obstacle);
    let mut visited = HashMap::new();
    loop {
        while map.next_cell() == Some(&Content::Empty) {
            map.guard.move_cell();
        }
        let next = map.next_cell();
        if next.is_none() {
            return false;
        }
        if visited.contains_key(&map.guard) {
            return true;
        }
        visited.insert(map.guard, ());
        map.guard.turn_right();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_process_candidate() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        assert!(process_candidate(&mut map, 0, 0, Xy::new(3, 6)));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        assert_eq!(map.grid.get((0, 0).into()), Some(&Content::Empty));
        assert_eq!(map.grid.get((2, 3).into()), Some(&Content::Obstacle));
        Ok(())
    }

    #[test]
    fn test_look() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        assert_eq!(map.next_cell(), Some(&Content::Empty));
        map.guard = Content::Guard((4, 1).into(), Direction4::N);
        assert_eq!(map.next_cell(), Some(&Content::Obstacle));
        map.guard = Content::Guard((3, 0).into(), Direction4::E);
        assert_eq!(map.next_cell(), Some(&Content::Obstacle));
        map.guard = Content::Guard((3, 0).into(), Direction4::W);
        assert_eq!(map.next_cell(), Some(&Content::Empty));
        map.guard.turn_right();
        assert_eq!(map.next_cell(), None);

        map.guard.turn_right();
        map.guard.turn_right();
        map.guard.move_cell();
        if let Content::Guard(c, v) = map.guard {
            assert_eq!(Direction4::S, v);
            assert_eq!(Xy::new(3, 1), c);
        } else {
            panic!("Not a guard");
        }

        Ok(())
    }
}
