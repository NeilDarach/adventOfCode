pub mod grid {
    use std::fmt::{Debug, Display};
    use std::ops::Add;
    use std::ops::Sub;

    use itertools::Itertools;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Direction4 {
        N,
        E,
        S,
        W,
    }
    impl Direction4 {
        pub fn all() -> Vec<Self> {
            vec![Self::N, Self::E, Self::S, Self::W]
        }

        pub fn clockwise(&self) -> Self {
            match self {
                Self::N => Self::E,
                Self::E => Self::S,
                Self::S => Self::W,
                Self::W => Self::N,
            }
        }

        pub fn anticlockwise(&self) -> Self {
            match self {
                Self::N => Self::W,
                Self::E => Self::N,
                Self::S => Self::E,
                Self::W => Self::S,
            }
        }

        pub fn delta(&self) -> Xy {
            match self {
                Self::N => (0, -1).into(),
                Self::E => (1, 0).into(),
                Self::S => (0, 1).into(),
                Self::W => (-1, 0).into(),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Direction8 {
        N,
        NE,
        E,
        SE,
        S,
        SW,
        W,
        NW,
    }
    impl Direction8 {
        pub fn all() -> Vec<Self> {
            vec![
                Self::N,
                Self::NE,
                Self::E,
                Self::SE,
                Self::S,
                Self::SW,
                Self::W,
                Self::NW,
            ]
        }
        pub fn cardinal() -> Vec<Self> {
            vec![Self::N, Self::E, Self::S, Self::W]
        }

        pub fn diagonal() -> Vec<Self> {
            vec![Self::NE, Self::SE, Self::SW, Self::NW]
        }

        pub fn clockwise(&self) -> Self {
            match self {
                Self::N => Self::NE,
                Self::NE => Self::E,
                Self::E => Self::SE,
                Self::SE => Self::S,
                Self::S => Self::SW,
                Self::SW => Self::W,
                Self::W => Self::NW,
                Self::NW => Self::N,
            }
        }

        pub fn anticlockwise(&self) -> Self {
            match self {
                Self::N => Self::NW,
                Self::NE => Self::N,
                Self::E => Self::NE,
                Self::SE => Self::E,
                Self::S => Self::SE,
                Self::SW => Self::S,
                Self::W => Self::SW,
                Self::NW => Self::W,
            }
        }

        pub fn delta(&self) -> Xy {
            match self {
                Self::N => (0, -1).into(),
                Self::NE => (1, -1).into(),
                Self::E => (1, 0).into(),
                Self::SE => (1, 1).into(),
                Self::S => (0, 1).into(),
                Self::SW => (-1, 1).into(),
                Self::W => (-1, 0).into(),
                Self::NW => (-1, -1).into(),
            }
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
    pub struct Xy {
        pub x: i32,
        pub y: i32,
    }

    impl Add<Xy> for Xy {
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Add<Direction4> for Xy {
        type Output = Self;
        fn add(self, other: Direction4) -> Self::Output {
            Self {
                x: self.x + other.delta().x,
                y: self.y + other.delta().y,
            }
        }
    }

    impl Add<Direction8> for Xy {
        type Output = Self;
        fn add(self, other: Direction8) -> Self::Output {
            Self {
                x: self.x + other.delta().x,
                y: self.y + other.delta().y,
            }
        }
    }

    impl Sub<Xy> for Xy {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Sub<Direction4> for Xy {
        type Output = Self;
        fn sub(self, other: Direction4) -> Self::Output {
            Self {
                x: self.x - other.delta().x,
                y: self.y - other.delta().y,
            }
        }
    }

    impl Sub<Direction8> for Xy {
        type Output = Self;
        fn sub(self, other: Direction8) -> Self::Output {
            Self {
                x: self.x - other.delta().x,
                y: self.y - other.delta().y,
            }
        }
    }

    impl Xy {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    struct IntXy {
        x: usize,
        y: usize,
    }

    impl Display for Xy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }
    impl Debug for Xy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    impl From<(i32, i32)> for Xy {
        fn from(value: (i32, i32)) -> Self {
            Self {
                x: value.0,
                y: value.1,
            }
        }
    }

    impl From<(usize, usize)> for Xy {
        fn from(value: (usize, usize)) -> Self {
            Self {
                x: value.0 as i32,
                y: value.1 as i32,
            }
        }
    }

    #[derive(Debug)]
    pub struct Grid<T> {
        elements: Vec<Vec<Option<T>>>,
        start: Xy,
        end: Xy,
    }

    impl<T> Display for Grid<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "start={}, end={}, rowcount={}, columncount={}",
                self.start,
                self.end,
                self.elements.len(),
                self.elements[0].len(),
            )?;
            for y in 0..=(self.end.y - self.start.y) {
                let y = y as usize;
                for x in 0..=(self.end.x - self.start.x) {
                    let x = x as usize;
                    let c = match &self.elements[y][x] {
                        Some(e) => e.to_string().chars().nth(0).unwrap_or('.'),
                        None => '.',
                    };
                    write!(f, "{} ", c)?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }

    impl<T> Grid<T>
    where
        T: Display + Debug,
    {
        pub fn empty() -> Self {
            Self {
                elements: vec![vec![None]],
                start: (0, 0).into(),
                end: (0, 0).into(),
            }
        }

        pub fn sample() -> Grid<String> {
            let elements = vec![
                vec![Some("x".to_string()), Some("yy".to_string())],
                vec![Some("XX".to_string()), Some("Y".to_string())],
            ];
            Grid::<String> {
                elements,
                start: (0, 0).into(),
                end: (1, 1).into(),
            }
        }

        pub fn all(&self) -> impl Iterator<Item = (Xy, Option<&T>)> {
            (self.start.x..=self.end.x)
                .cartesian_product(self.start.y..=self.end.y)
                .map(|(x, y)| Xy::new(x, y))
                .map(|e| (e, self.get(e)))
        }

        pub fn keys(&self) -> impl Iterator<Item = Xy> {
            (self.start.x..=self.end.x)
                .cartesian_product(self.start.y..=self.end.y)
                .map(|(x, y)| Xy::new(x, y))
        }

        fn extend_x(&mut self, x: i32) {
            if x > self.end.x {
                for i in 0..=(self.end.y - self.start.y) as usize {
                    self.elements[i].resize_with((x - self.start.x + 1) as usize, || None);
                }
                self.end.x = x;
            }
            if x < self.start.x {
                for i in 0..=(self.end.y - self.start.y) as usize {
                    let mut prefix: Vec<Option<T>> = vec![];
                    prefix.resize_with((self.start.x - x as i32) as usize, || None);
                    prefix.append(&mut self.elements[i]);
                    self.elements[i] = prefix;
                }
                self.start.x = x;
            }
        }

        fn extend_y(&mut self, y: i32) {
            if y > self.end.y {
                for _ in 0..(y - self.end.y) as usize {
                    let mut row: Vec<Option<T>> = vec![];
                    row.resize_with((self.end.x - self.start.x + 1) as usize, || None);
                    self.elements.push(row);
                }
                self.end.y = y;
            }

            if y < self.start.y {
                let mut new_rows = vec![];
                dbg!(self.start.y - y);
                for _ in 0..(self.start.y - y) as usize {
                    let mut row: Vec<Option<T>> = vec![];
                    row.resize_with((self.end.x - self.start.x + 1) as usize, || None);
                    new_rows.push(row);
                }
                new_rows.append(&mut self.elements);
                self.elements = new_rows;
                self.start.y = y;
            }
        }

        pub fn insert(&mut self, xy: Xy, element: T) {
            if !(self.start.x..=self.end.x).contains(&xy.x) {
                self.extend_x(xy.x);
            }
            if !(self.start.y..=self.end.y).contains(&xy.y) {
                self.extend_y(xy.y);
            }

            let xy = self.int_xy(xy);

            self.elements[xy.y][xy.x] = Some(element);
        }

        pub fn in_bounds(&self, xy: Xy) -> bool {
            (self.start.x..=self.end.x).contains(&xy.x)
                && (self.start.y..=self.end.y).contains(&xy.y)
        }

        fn int_xy(&self, xy: Xy) -> IntXy {
            let x = (xy.x - self.start.x) as usize;
            let y = (xy.y - self.start.y) as usize;

            IntXy { x, y }
        }

        pub fn get(&self, xy: Xy) -> Option<&T> {
            if !self.in_bounds(xy) {
                return None;
            }
            let xy = self.int_xy(xy);
            match &self.elements[xy.y][xy.x] {
                Some(e) => Some(&e),
                None => None,
            }
        }

        pub fn contains(&self, xy: Xy) -> bool {
            self.get(xy).is_some()
        }
    }
}
