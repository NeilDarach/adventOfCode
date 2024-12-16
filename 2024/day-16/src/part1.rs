use crate::custom_error::AocError;
use std::collections::{HashMap, HashSet};

use utils::grid::{Direction4, Grid, Xy};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parser::parse(input);
    let start = grid
        .all()
        .filter(|(_k, v)| v == &Some(&'S'))
        .map(|(k, _v)| k)
        .nth(0)
        .expect("Didn't find the start position");
    let end = grid
        .all()
        .filter(|(_k, v)| v == &Some(&'E'))
        .map(|(k, _v)| k)
        .nth(0)
        .expect("Didn't find the end position");

    let mut seats = HashMap::new();
    let result = step(
        &grid,
        start,
        Direction4::E,
        end,
        None,
        0,
        &mut HashSet::new(),
        &mut HashMap::new(),
        &mut seats,
    );
    let seat_count = seats.get(&result.unwrap()).unwrap().len() + 1 /* for the start square */;
    println!(
        "shortest path: {}, seat count: {}",
        result.unwrap_or(0),
        seat_count
    );
    Ok(result.unwrap_or(0).to_string())
}

pub fn step(
    grid: &Grid<char>,
    pos: Xy,
    dir: Direction4,
    end: Xy,
    best_score: Option<i32>,
    current_score: i32,
    in_path: &mut HashSet<Xy>,
    seen: &mut HashMap<(Xy, Direction4), i32>,
    seats: &mut HashMap<i32, HashSet<Xy>>,
) -> Option<i32> {
    if grid.get(pos) == Some(&'#') {
        return None;
    }
    if best_score.is_some() && current_score > best_score.unwrap() {
        return None;
    }
    if pos == end {
        let entry = seats.entry(current_score).or_default();

        for xy in in_path.clone() {
            entry.insert(xy);
        }
        return Some(current_score);
    }
    if in_path.contains(&pos) {
        return None;
    }
    if let Some(&val) = seen.get(&(pos, dir)) {
        if val < current_score {
            return None;
        }
    }
    in_path.insert(pos);
    seen.insert((pos, dir), current_score);
    let mut best_score = best_score;
    for d in Direction4::all() {
        let n = if d == dir { 1 } else { 1001 };
        if let Some(s) = step(
            grid,
            pos + d,
            d,
            end,
            best_score,
            current_score + n,
            &mut in_path.clone(),
            seen,
            seats,
        ) {
            if best_score.is_none() {
                best_score = Some(s);
            } else {
                best_score = Some(best_score.unwrap().min(s));
            }
        }
    }
    best_score
}

pub mod parser {
    use super::*;
    pub fn parse(input: &str) -> Grid<char> {
        let mut grid = Grid::empty();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.insert(Xy::new(x as i32, y as i32), c);
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MAP: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("7036", process(MAP)?);
        Ok(())
    }
}
