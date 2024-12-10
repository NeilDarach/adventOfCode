use crate::custom_error::AocError;
use itertools::Itertools;
use utils::grid::*;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parser::parse(input);
    let count: usize = grid
        .keys()
        .filter(|&xy| grid.get(xy) == Some(&0))
        .map(|e| paths(&grid, e).iter().map(|e| e[0]).unique().count())
        .sum();
    Ok(count.to_string())
}

pub fn exits(grid: &Grid<i32>, xy: Xy) -> Vec<Xy> {
    match grid.get(xy) {
        None => vec![],
        Some(val) => Direction4::all()
            .iter()
            .map(|&d| xy + d)
            .filter(|&xy| grid.get(xy).copied() == Some(val + 1))
            .collect::<Vec<_>>(),
    }
}

pub fn paths(grid: &Grid<i32>, xy: Xy) -> Vec<Vec<Xy>> {
    if grid.get(xy) == Some(&9) {
        return vec![vec![xy]];
    }
    let mut result = exits(grid, xy)
        .iter()
        .flat_map(|&exit| {
            paths(grid, exit)
                .iter()
                .filter(|e| !e.is_empty())
                .cloned()
                .map(|mut p| {
                    p.push(xy);
                    p
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if result.is_empty() {
        result.push(vec![]);
    }
    result
}

mod parser {
    use super::*;
    pub fn parse(input: &str) -> Grid<i32> {
        let mut grid = Grid::empty();
        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                grid.insert((i, j).into(), c.to_string().parse().unwrap());
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRAIL_1: &str = "0123\n1234\n8765\n9876";
    const TRAIL_2: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("36", process(TRAIL_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let grid = parser::parse(TRAIL_1);
        assert_eq!(4, grid.width());
        assert_eq!(4, grid.height());
        assert_eq!(Some(&8), grid.get((0, 2).into()));
        Ok(())
    }

    #[test]
    fn test_find_exits() -> miette::Result<()> {
        let grid = parser::parse(TRAIL_2);
        let all_exits = exits(&grid, (0, 0).into());
        assert_eq!(1, all_exits.len());
        let all_exits = exits(&grid, (1, 0).into());
        assert_eq!(0, all_exits.len());
        Ok(())
    }
}
