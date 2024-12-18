use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::rc::Rc;

use itertools::Itertools;

#[derive(Eq, PartialEq, Clone)]
pub struct Path<T>(T, Option<Rc<Path<T>>>);
impl<T> Path<T> {
    pub fn new(item: T) -> Self {
        Self(item, None)
    }
    pub fn head(&self) -> T
    where
        T: Clone,
    {
        self.0.clone()
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn len(&self) -> usize {
        match &self.1 {
            None => 1,
            Some(i) => 1 + &i.len(),
        }
    }
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        match &self.1 {
            None => vec![self.0.clone()],
            Some(i) => {
                let mut vec = vec![self.0.clone()];
                vec.append(i.to_vec().as_mut());
                vec
            }
        }
    }
}

impl<T> Add<T> for &Path<T>
where
    T: Clone,
{
    type Output = Path<T>;

    fn add(self, rhs: T) -> Self::Output {
        Path(rhs, Some(Rc::new(self.clone())))
    }
}

impl<T> AddAssign<T> for Path<T>
where
    T: Clone,
{
    fn add_assign(&mut self, rhs: T) {
        *self = Self(rhs, Some(Rc::new(self.clone())))
    }
}

impl<T> Debug for Path<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.1 {
            None => write!(f, "{:?}", &self.0),
            Some(i) => {
                write!(f, "{:?} -> {:?}", &self.0, &i)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction4 {
    N,
    E,
    S,
    W,
}

impl Display for Direction4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::N => write!(f, "N")?,
            Self::E => write!(f, "E")?,
            Self::S => write!(f, "S")?,
            Self::W => write!(f, "W")?,
        }
        Ok(())
    }
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

impl AddAssign<Xy> for Xy {
    fn add_assign(&mut self, rhs: Xy) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl SubAssign<Xy> for Xy {
    fn sub_assign(&mut self, rhs: Xy) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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
            writeln!(f)?;
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

    pub fn width(&self) -> i32 {
        self.end.x - self.start.x + 1
    }

    pub fn height(&self) -> i32 {
        self.end.y - self.start.y + 1
    }

    pub fn all(&self) -> impl Iterator<Item = (Xy, Option<&T>)> {
        self.keys().map(|e| (e, self.get(e)))
    }

    pub fn keys(&self) -> impl Iterator<Item = Xy> + use<T> {
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
                prefix.resize_with((self.start.x - x) as usize, || None);
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

    pub fn is_empty(&self) -> bool {
        self.height() == 1 && self.width() == 1 && self.elements[0][0].is_none()
    }

    pub fn remove(&mut self, xy: Xy) -> Option<T> {
        if !self.in_bounds(xy) {
            return None;
        }
        let xy = self.int_xy(xy);
        self.elements[xy.y][xy.x].take()
    }
    pub fn insert(&mut self, xy: Xy, element: T) {
        if !(self.start.y..=self.end.y).contains(&xy.y) {
            self.extend_y(xy.y);
        }
        if !(self.start.x..=self.end.x).contains(&xy.x) {
            self.extend_x(xy.x);
        }

        let xy = self.int_xy(xy);

        self.elements[xy.y][xy.x] = Some(element);
    }

    pub fn in_bounds(&self, xy: Xy) -> bool {
        (self.start.x..=self.end.x).contains(&xy.x) && (self.start.y..=self.end.y).contains(&xy.y)
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
            Some(e) => Some(e),
            None => None,
        }
    }

    pub fn contains(&self, xy: Xy) -> bool {
        xy.x >= self.start.x && xy.x <= self.end.x && xy.y >= self.start.y && xy.y <= self.end.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_xy() -> Result<(), String> {
        let xy = Xy::new(0, 1);
        assert_eq!(0, xy.x);
        assert_eq!(1, xy.y);
        Ok(())
    }

    #[test]
    fn test_modify_xy() -> Result<(), String> {
        let a = Xy::new(0, 1);
        let b = Xy::new(4, 9);
        assert_eq!(Xy::new(4, 10), a + b);
        assert_eq!(Xy::new(4, 10), b + a);
        assert_eq!(Xy::new(-4, -8), a - b);
        assert_eq!(Xy::new(4, 8), b - a);

        assert_eq!(Xy::new(4, 8), b + Direction4::N);
        assert_eq!(Xy::new(3, 8), b - Direction8::SE);
        Ok(())
    }

    #[test]
    fn test_directions() -> Result<(), String> {
        assert_eq!(Direction4::E, Direction4::N.clockwise());
        assert_eq!(Direction8::E, Direction8::SE.anticlockwise());
        assert_eq!(4, Direction4::all().len());
        assert_eq!(8, Direction8::all().len());
        assert_eq!(4, Direction8::cardinal().len());
        assert_eq!(4, Direction8::diagonal().len());
        assert_eq!(
            Direction8::diagonal(),
            Direction8::cardinal()
                .iter()
                .map(|e| e.clockwise())
                .collect::<Vec<_>>()
        );
        Ok(())
    }

    #[test]
    fn test_grid() -> Result<(), String> {
        let mut grid: Grid<String> = Grid::empty();
        assert_eq!(1, grid.height());
        assert_eq!(1, grid.width());

        grid.insert((3, 4).into(), "One".to_string());
        assert_eq!(5, grid.height());
        assert_eq!(4, grid.width());

        assert_eq!(Some(&"One".to_string()), grid.get((3, 4).into()));
        assert_eq!(None, grid.get((0, 0).into()));
        assert_eq!(None, grid.get((2, 4).into()));
        assert_eq!(None, grid.get((4, 5).into()));
        assert_eq!(None, grid.get((-3, -3).into()));

        grid.insert((-3, -3).into(), "Two".to_string());
        assert_eq!(Some(&"Two".to_string()), grid.get((-3, -3).into()));
        assert_eq!(8, grid.height());
        assert_eq!(7, grid.width());

        assert!(grid.contains((0, 0).into()));
        assert!(grid.contains((2, 2).into()));
        assert!(grid.contains((-3, -3).into()));
        assert!(!grid.contains((-3, -4).into()));
        assert!(!grid.contains((-4, -3).into()));

        assert!(grid.contains((3, 4).into()));
        assert!(!grid.contains((4, 4).into()));
        assert!(!grid.contains((-3, 5).into()));

        assert_eq!(56, grid.keys().count());
        assert_eq!(56, grid.all().count());
        assert_eq!(2, grid.all().filter(|(_k, v)| v.is_some()).count());
        Ok(())
    }

    #[test]
    pub fn test_path() -> Result<(), String> {
        let mut path = Path::new(2);
        path += 3;
        assert_eq!(vec![3, 2], path.to_vec());
        assert!(!path.is_empty());
        assert_eq!(2, path.len());
        dbg!("{:?}", &path);
        assert_eq!("3 -> 2", format!("{:?}", &path));

        let path2 = &path + 8;
        assert_eq!(3, path2.len());
        assert_eq!(2, path.len());
        assert_eq!("8 -> 3 -> 2", format!("{:?}", &path2));

        let mut str_path = Path::new("one".to_string());
        str_path += "two".to_string();
        assert_eq!("\"two\" -> \"one\"", format!("{:?}", &str_path));
        Ok(())
    }
}
