use crate::custom_error::AocError;
use std::collections::{HashSet, VecDeque};
use utils::grid::{Direction4, Grid, Path, Xy};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    local_process(input, 1024, Xy::new(70, 70))
}

pub fn local_process(input: &str, limit: usize, exit: Xy) -> miette::Result<String, AocError> {
    let map = parser::parse(input, limit);
    let p = map.shortest_path2(Xy::new(0, 0), exit);
    Ok(p.unwrap().to_string())
}

pub struct Map {
    pub grid: Grid<char>,
    pub remaining: VecDeque<Xy>,
}

impl Map {
    pub fn shortest_path(&self, start: Xy, end: Xy) -> i32 {
        let shortest = self.step(start, &vec![], end, None);
        shortest.unwrap().len() as i32
    }

    pub fn shortest_path2(&self, start: Xy, end: Xy) -> Option<i32> {
        let mut visited = HashSet::<Xy>::new();
        let mut queue = VecDeque::<(Path<Xy>, i32)>::new();
        queue.push_back((Path::new(start), 0));
        while let Some((path, distance)) = queue.pop_front() {
            let cell = path.head();
            if cell == end {
                return Some(distance);
            }
            if visited.contains(&cell) {
                continue;
            }
            if !self.grid.in_bounds(cell) {
                continue;
            }
            visited.insert(cell);
            if Some(&'#') == self.grid.get(cell) {
                continue;
            }
            for d in Direction4::all() {
                queue.push_back((&path + (cell + d), distance + 1));
            }
        }
        None
    }

    pub fn step(
        &self,
        current: Xy,
        visited: &[Xy],
        goal: Xy,
        best: Option<usize>,
    ) -> Option<Vec<Xy>> {
        if !self.grid.in_bounds(current) {
            return None;
        }
        if best.is_some() && best.unwrap() <= visited.len() {
            return None;
        }
        if visited.contains(&current) {
            return None;
        }
        if current == goal {
            return Some(visited.to_vec());
        }
        if self.grid.get(current) == Some(&'#') {
            return None;
        }
        let mut b = best;
        let mut p = None;
        let mut visited = visited.to_vec();
        visited.push(current);
        for d in Direction4::all() {
            if let Some(r) = self.step(current + d, &visited, goal, b) {
                b = Some(r.len());
                p = Some(r);
            }
        }
        p
    }
}

mod parser {
    use super::*;

    pub fn parse(input: &str, limit: usize) -> Map {
        let mut grid: Grid<char> = Grid::empty();
        let mut remaining = VecDeque::new();

        for (i, line) in input.lines().enumerate() {
            let coords = line
                .split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let xy = Xy::new(coords[0], coords[1]);
            if i >= limit {
                remaining.push_back(Xy::new(coords[0], coords[1]));
            } else {
                grid.insert(xy, '#');
            }
        }
        Map { grid, remaining }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_process() -> miette::Result<()> {
        let p = local_process(SAMPLE, 12, Xy::new(6, 6))?;
        assert_eq!(p, "22");
        Ok(())
    }
    #[test]
    fn test_parse() -> miette::Result<()> {
        let map = parser::parse(SAMPLE, 12);
        assert_eq!(7, map.grid.height());
        assert_eq!(7, map.grid.width());
        println!("{}", map.grid);
        Ok(())
    }
}
