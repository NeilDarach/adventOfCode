pub mod aoc {
    use nom::character::complete::char;
    use nom::character::complete::digit1;
    use nom::character::complete::space0;
    use nom::combinator::opt;
    use nom::multi::many1;
    use nom::sequence::preceded;
    use nom::sequence::tuple;

    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::ops::Add;
    use std::ops::Index;
    use std::ops::IndexMut;

    use nom::IResult;

    #[derive(Debug)]
    pub struct ParseError;

    pub fn version() -> String {
        "2023".to_owned()
    }

    pub fn number(i: &str) -> IResult<&str, u64> {
        let (i, num) = digit1(i)?;
        Ok((i, num.parse().unwrap()))
    }

    pub fn signed_number(i: &str) -> IResult<&str, i64> {
        let (i, (neg, num)) = tuple((opt(char('-')), digit1))(i)?;
        let num = if neg.is_some() {
            -num.parse::<i64>().unwrap()
        } else {
            num.parse().unwrap()
        };

        Ok((i, num))
    }

    pub fn space_separated_numbers(i: &str) -> IResult<&str, Vec<u64>> {
        many1(preceded(space0, number))(i)
    }

    pub fn space_separated_signed_numbers(i: &str) -> IResult<&str, Vec<i64>> {
        many1(preceded(space0, signed_number))(i)
    }

    pub fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
        (0..input[0].len())
            .map(|i| input.iter().map(|inner| inner[i]).collect::<Vec<_>>())
            .collect()
    }

    #[derive(Default, Copy, Clone, Eq, PartialEq)]
    pub enum Direction {
        #[default]
        North,
        South,
        East,
        West,
    }

    impl Debug for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::North => write!(f, "N"),
                Self::South => write!(f, "S"),
                Self::East => write!(f, "E"),
                Self::West => write!(f, "W"),
            }
        }
    }

    impl Direction {
        pub fn delta(&self) -> Pos {
            match self {
                Self::North => (0, -1).into(),
                Self::South => (0, 1).into(),
                Self::East => (1, 0).into(),
                Self::West => (-1, 0).into(),
            }
        }
        pub fn left(&self) -> Self {
            match self {
                Self::North => Self::West,
                Self::South => Self::East,
                Self::East => Self::North,
                Self::West => Self::South,
            }
        }
        pub fn right(&self) -> Self {
            match self {
                Self::North => Self::East,
                Self::South => Self::West,
                Self::East => Self::South,
                Self::West => Self::North,
            }
        }
        pub fn reverse(&self) -> Self {
            match self {
                Self::North => Self::South,
                Self::South => Self::North,
                Self::East => Self::West,
                Self::West => Self::East,
            }
        }
    }

    #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Pos {
        pub x: i32,
        pub y: i32,
    }

    impl From<(usize, usize)> for Pos {
        fn from(value: (usize, usize)) -> Self {
            Self {
                x: value.0 as i32,
                y: value.1 as i32,
            }
        }
    }
    impl From<(i32, i32)> for Pos {
        fn from(value: (i32, i32)) -> Self {
            Self {
                x: value.0,
                y: value.1,
            }
        }
    }

    impl Add<Pos> for Pos {
        type Output = Pos;

        fn add(self, rhs: Pos) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Pos {
        pub fn zero() -> Self {
            Pos { x: 0, y: 0 }
        }
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn contains(&self, other: &Self) -> bool {
            self.x >= other.x && self.y >= other.y
        }
        pub fn union(&self, other: &Self) -> Pos {
            Self {
                x: self.x.max(other.x),
                y: self.y.max(other.y),
            }
        }
    }

    pub struct Grid<T>
    where
        T: Default,
    {
        pub top_left: Pos,
        pub bottom_right: Pos,
        pub cells: HashMap<Pos, T>,
    }

    impl<T> Grid<T>
    where
        T: Default,
    {
        pub fn ensure(&mut self, index: Pos) {
            if !self.top_left.contains(&index) || self.bottom_right.contains(&index) {
                //New Rows below
                if index.y > self.bottom_right.y {
                    for y in self.bottom_right.y + 1..=index.y {
                        for x in self.top_left.x..=self.bottom_right.x {
                            self.cells.insert((x, y).into(), T::default());
                        }
                    }
                    self.bottom_right.y = index.y;
                }
                //New Rows above
                if index.y < self.top_left.y {
                    for y in index.y..=self.top_left.y - 1 {
                        for x in self.top_left.x..=self.bottom_right.x {
                            println!("New row above {}{}", x, y);
                            self.cells.insert((x, y).into(), T::default());
                        }
                    }
                    self.top_left.y = index.y;
                }

                //New Columns right
                if index.x > self.bottom_right.x {
                    for y in self.top_left.y..=self.bottom_right.y {
                        for x in self.bottom_right.x + 1..=index.x {
                            self.cells.insert((x, y).into(), T::default());
                        }
                    }
                    self.bottom_right.x = index.x;
                }
                //New Columns left
                if index.x < self.top_left.x {
                    for y in self.top_left.y..=self.bottom_right.y {
                        for x in index.x..=self.top_left.x - 1 {
                            self.cells.insert((x, y).into(), T::default());
                        }
                    }
                    self.top_left.x = index.x;
                }
            }
        }
    }

    impl<T> Default for Grid<T>
    where
        T: Default,
    {
        fn default() -> Self {
            let mut cells: HashMap<Pos, T> = Default::default();
            cells.insert(Pos::zero(), Default::default());
            Self {
                top_left: Default::default(),
                bottom_right: Default::default(),
                cells,
            }
        }
    }

    impl<T> Debug for Grid<T>
    where
        T: Debug + Default,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "Limits: {},{} - {},{}",
                self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y
            )?;
            for y in self.top_left.y..=self.bottom_right.y {
                for x in self.top_left.x..=self.bottom_right.x {
                    write!(f, "{:?}", self[(x, y).into()])?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl<T> IndexMut<Pos> for Grid<T>
    where
        T: Default,
    {
        fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
            self.ensure(index);
            self.cells.get_mut(&index).unwrap()
        }
    }

    impl<T> Index<Pos> for Grid<T>
    where
        T: Default,
    {
        type Output = T;

        fn index(&self, index: Pos) -> &Self::Output {
            self.cells.get(&index).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::aoc::*;

    #[test]
    pub fn test_numberlist() {
        assert_eq!(vec![1, 20, 3], space_separated_numbers("1 20 3").unwrap().1);
    }

    #[test]
    pub fn test_signed_numberlist() {
        assert_eq!(
            vec![1, -20, 3],
            space_separated_signed_numbers("1 -20 3").unwrap().1
        );
    }

    #[test]
    pub fn test_number() {
        assert_eq!(20, number("20").unwrap().1);
    }

    #[test]
    pub fn test_grid() {
        let mut grid: Grid<char> = Grid::default();
        assert_eq!(Pos::zero(), grid.top_left);
        assert_eq!(Pos::zero(), grid.bottom_right);
        assert_eq!('\0', grid[Pos::zero()]);
        grid[(4_i32, 3_i32).into()] = 'A';
        assert_eq!(Pos::new(4, 3), grid.bottom_right);
        assert_eq!('A', grid[(4_i32, 3_i32).into()]);
        grid[(-14_i32, -9_i32).into()] = 'B';
        assert_eq!(Pos::new(-14, -9), grid.top_left);
        assert_eq!('A', grid[(-14_i32, -9_i32).into()]);
    }
    #[test]
    pub fn test_directions() {
        let dir: Direction = Default::default();
        assert_eq!(Direction::North, dir);
        assert_eq!(Direction::North, Direction::West.right());
    }
}
