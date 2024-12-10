use crate::custom_error::AocError;
use utils::grid::*;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let trails = parser::parse(input);
    let starts = trails
        .cells
        .all()
        .filter(|(_k, v)| *v == Some(&0))
        .collect::<Vec<_>>();
    let count: usize = starts
        .iter()
        .map(|e| trails.paths(e.0).iter().map(|e| e[0]).count())
        .sum();
    Ok(count.to_string())
}
pub struct Trails {
    cells: Grid<i32>,
}

impl Trails {
    pub fn new() -> Self {
        Self {
            cells: Grid::empty(),
        }
    }

    pub fn exits(&self, xy: Xy) -> Vec<Xy> {
        let mut exits = vec![];
        if let Some(&current) = self.cells.get(xy) {
            for d in Direction4::all() {
                let possible = xy + d;
                if let Some(&next) = self.cells.get(possible) {
                    if next == current + 1 {
                        exits.push(xy + d);
                    }
                }
            }
        }
        exits
    }

    pub fn paths(&self, start: Xy) -> Vec<Vec<Xy>> {
        if self.cells.get(start) == Some(&9) {
            return vec![vec![start]];
        }
        let exits = self.exits(start);
        if exits.is_empty() {
            return vec![vec![]];
        }
        let mut result = vec![];
        for exit in exits {
            let paths = self.paths(exit);
            let paths = paths.iter().filter(|e| !e.is_empty());
            for each in paths {
                let mut v = each.clone();
                v.push(start);
                result.push(v)
            }
        }
        result
    }
}

mod parser {
    use super::*;
    pub fn parse(input: &str) -> Trails {
        let mut trails = Trails::new();
        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                trails
                    .cells
                    .insert((i, j).into(), c.to_string().parse().unwrap());
            }
        }

        trails
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
        assert_eq!("81", process(TRAIL_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let trails = parser::parse(TRAIL_1);
        assert_eq!(4, trails.cells.width());
        assert_eq!(4, trails.cells.height());
        assert_eq!(Some(&8), trails.cells.get((0, 2).into()));
        Ok(())
    }

    #[test]
    fn test_find_exits() -> miette::Result<()> {
        let trails = parser::parse(TRAIL_2);
        let exits = trails.exits((0, 0).into());
        assert_eq!(1, exits.len());
        let exits = trails.exits((1, 0).into());
        assert_eq!(0, exits.len());
        Ok(())
    }
}
