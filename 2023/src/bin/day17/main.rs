use aoc_2023::aoc::*;
use eframe::egui;
use egui::{Color32, Painter, Pos2, Rect, Rounding, Sense, Stroke};
use std::{collections::HashMap, ops::Add, time::Duration};

fn input() -> &'static str {
    include_str!("input.txt")
}

#[derive(Default, Debug)]
pub struct Grid {
    limits: Pos,
    elements: HashMap<Pos, Cell>,
}

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}
impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        Pos {
            x: value.0,
            y: value.1,
        }
    }
}

impl Pos {
    pub fn contained_by(&self, other: Pos) -> bool {
        self.x >= 0 && self.x <= other.x && self.y >= 0 && self.y <= other.y
    }
}

#[derive(Default, Debug)]
pub struct Cell {
    heatloss: i32,
    turned_to_exit_north: bool,
    turned_to_exit_south: bool,
    turned_to_exit_east: bool,
    turned_to_exit_west: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Cart {
    path: Vec<Pos>,
    forward_count: i32,
    heatloss: i32,
    direction: Direction,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

fn part1(data: &str) -> u64 {
    0
}

pub struct App {
    grid: Grid,
    carts: Vec<Cart>,
    best_path: Option<Vec<Pos>>,
    best_heatloss: Option<i32>,
    start: Pos,
    end: Pos,
}

impl Default for App {
    fn default() -> Self {
        let carts = vec![
            Cart {
                direction: Direction::East,
                ..Cart::default()
            },
            Cart {
                direction: Direction::South,
                ..Cart::default()
            },
        ];
        Self {
            grid: Default::default(),
            carts,
            best_path: Default::default(),
            best_heatloss: Default::default(),
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl App {
    pub fn best_path_length(&self) -> i32 {
        match &self.best_path {
            Some(v) => v.len() as i32,
            None => 0,
        }
    }

    pub fn update_state(&mut self) {
        if let Some(next_cart) = self.next_cart() {
        } else {
            self.carts.pop();
        }
    }
    pub fn move_forwards(&mut self, cart: &Cart) -> Option<Cart> {
        if cart.forward_count == 3 {
            return None;
        }
        let mut new_cart = cart.clone();
        let pos = cart.path[cart.path.len() - 1];
        let next_cell = pos
            + (match cart.direction {
                Direction::North => (0, -1),
                Direction::South => (0, 1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            })
            .into();
        if !pos.contained_by(self.grid.limits) {
            return None;
        }
        let entry = self.grid.elements.get(&next_cell).unwrap();
        new_cart.heatloss += entry.heatloss;
        new_cart.path.push(next_cell);
        new_cart.forward_count += 1;
        Some(new_cart)
    }

    pub fn next_cart(&mut self) -> Option<Cart> {
        let len = self.carts.len();
        let cart = self.carts[len - 1].clone();
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if cart.direction == dir && cart.forward_count < 3 {
                if let Some(new_cart) = self.move_forwards(&cart) {
                    return Some(new_cart);
                }
            }
        }
        None
    }
}
fn part1_gui(data: &str) -> u64 {
    let grid = parse::grid(data).unwrap().1;
    let limits = grid.limits;
    let app = App {
        grid,
        start: (0, 0).into(),
        end: limits,
        ..App::default()
    };
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 1600.0)),
        ..Default::default()
    };

    eframe::run_native("AOC 2023 Day 17", options, Box::new(|_cc| Box::new(app)));
    0
}

fn part2(data: &str) -> u64 {
    0
}

fn main() {
    part1_gui(include_str!("sample.txt"));
    /*
    println!("Day x of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
    */
}

mod parse {
    use super::*;
    use nom::character::complete::line_ending;
    use nom::character::complete::one_of;
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::IResult;

    pub fn line_of_numbers(i: &str) -> IResult<&str, Vec<i32>> {
        many1(map(one_of("0123456789"), |e: char| {
            e.to_digit(10).unwrap() as i32
        }))(i)
    }

    pub fn grid(i: &str) -> IResult<&str, Grid> {
        let (i, lines) = separated_list1(line_ending, line_of_numbers)(i)?;
        let mut grid = Grid::default();
        for (y, line) in lines.iter().enumerate() {
            for (x, val) in line.iter().enumerate() {
                let mut cell = Cell::default();
                cell.heatloss = *val;
                grid.elements.insert((x, y).into(), cell);
            }
        }
        grid.limits = (lines.len(), lines[0].len() - 1).into();
        Ok((i, grid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_parse_sample() {
        let grid = parse::grid(sample()).unwrap().1;
        assert_eq!(
            2 as i32,
            grid.elements.get(&(0, 0).into()).unwrap().heatloss
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(102, part1(sample()));
    }

    /*
    #[test]
    fn test_sample() {
        assert_eq!(0, part1(sample()));
    }
    */

    #[test]
    fn test_sample_part_2() {
        assert_eq!(0, part2(sample()));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("summary").show(ctx, |ui| {
            ui.label(format!("best route is {:?} steps", self.best_path_length()));
            ui.label(format!(
                "heatloss is {} steps",
                self.best_heatloss.unwrap_or(-1)
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = egui::vec2(800.0, 800.0);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            const SCALE: f32 = 32.0;
            let white_box = |painter: &Painter, pos: Pos| {
                let min = Pos2 {
                    x: (pos.x + 11) as f32 * SCALE,
                    y: (pos.y + 1) as f32 * SCALE,
                };
                let max = Pos2 {
                    x: (pos.x + 12) as f32 * SCALE,
                    y: (pos.y + 2) as f32 * SCALE,
                };
                let rect = Rect { min, max };
                painter.rect_stroke(rect, Rounding::none(), Stroke::new(1.0, Color32::WHITE))
            };

            let colored_line = |painter: &Painter, start: Pos, end: Pos, color: Color32| {
                let start = Pos2 {
                    x: (start.x as f32 + 11.5) * SCALE,
                    y: (start.y as f32 + 1.5) * SCALE,
                };
                let end = Pos2 {
                    x: (end.x as f32 + 11.5) * SCALE,
                    y: (end.y as f32 + 1.5) * SCALE,
                };
                painter.line_segment([start, end], Stroke::new(3.0, color))
            };

            let yellow_line = |painter: &Painter, start: Pos, end: Pos| {
                colored_line(painter, start, end, Color32::YELLOW)
            };
            let red_line = |painter: &Painter, start: Pos, end: Pos| {
                colored_line(painter, start, end, Color32::RED)
            };

            ui.heading("Grid");
            for (pos, cell) in self.grid.elements.iter() {
                white_box(&painter, *pos);
            }
            yellow_line(&painter, (0, 0).into(), (2, 0).into());
            yellow_line(&painter, (2, 0).into(), (2, 9).into());
            red_line(&painter, (3, 0).into(), (3, 9).into());

            ctx.request_repaint_after(Duration::from_millis(100));
            self.update_state();
        });
    }
}
