use aoc_2023::aoc::*;
use eframe::egui;
use egui::{Color32, Sense, Stroke};
use std::{collections::HashMap, fmt::Display, time::Duration};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct State {
    active: bool,
    seen_north: bool,
    seen_south: bool,
    seen_east: bool,
    seen_west: bool,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Beam {
    location: (usize, usize),
    direction: Direction,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let cell = self.elements.get(&(x, y)).expect("cell");
                if cell.1.active {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Beam {
    pub fn next(&self, max_x: usize, max_y: usize) -> Option<Self> {
        let new = match self.direction {
            Direction::North if self.location.1 == 0 => return None,
            Direction::North => Self {
                location: (self.location.0, self.location.1 - 1),
                direction: self.direction,
            },
            Direction::South if self.location.1 == max_y => return None,
            Direction::South => Self {
                location: (self.location.0, self.location.1 + 1),
                direction: self.direction,
            },
            Direction::East if self.location.0 == max_x => return None,
            Direction::East => Self {
                location: (self.location.0 + 1, self.location.1),
                direction: self.direction,
            },
            Direction::West if self.location.0 == 0 => return None,
            Direction::West => Self {
                location: (self.location.0 - 1, self.location.1),
                direction: self.direction,
            },
        };
        Some(new)
    }

    pub fn rotate(&self, content: Content) -> Vec<Self> {
        let ret = match (self.direction, content) {
            (Direction::North, Content::FMirror) => vec![Beam {
                direction: Direction::East,
                ..*self
            }],
            (Direction::South, Content::FMirror) => vec![Beam {
                direction: Direction::West,
                ..*self
            }],
            (Direction::East, Content::FMirror) => vec![Beam {
                direction: Direction::North,
                ..*self
            }],
            (Direction::West, Content::FMirror) => vec![Beam {
                direction: Direction::South,
                ..*self
            }],
            (Direction::North, Content::BMirror) => vec![Beam {
                direction: Direction::West,
                ..*self
            }],
            (Direction::South, Content::BMirror) => vec![Beam {
                direction: Direction::East,
                ..*self
            }],
            (Direction::East, Content::BMirror) => vec![Beam {
                direction: Direction::South,
                ..*self
            }],
            (Direction::West, Content::BMirror) => vec![Beam {
                direction: Direction::North,
                ..*self
            }],
            (Direction::North, Content::HSplit) => vec![
                Beam {
                    direction: Direction::East,
                    ..*self
                },
                Beam {
                    direction: Direction::West,
                    ..*self
                },
            ],
            (Direction::South, Content::HSplit) => vec![
                Beam {
                    direction: Direction::East,
                    ..*self
                },
                Beam {
                    direction: Direction::West,
                    ..*self
                },
            ],
            (Direction::East, Content::VSplit) => vec![
                Beam {
                    direction: Direction::North,
                    ..*self
                },
                Beam {
                    direction: Direction::South,
                    ..*self
                },
            ],
            (Direction::West, Content::VSplit) => vec![
                Beam {
                    direction: Direction::North,
                    ..*self
                },
                Beam {
                    direction: Direction::South,
                    ..*self
                },
            ],
            _ => vec![*self],
        };
        //println!("Rotated {:?} to {:?} by {:?}", self, &ret, &content);
        ret
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    #[default]
    East,
    West,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Content {
    #[default]
    Empty,
    VSplit,
    HSplit,
    FMirror,
    BMirror,
}

impl Content {
    pub fn label(&self) -> &str {
        match self {
            Self::Empty => ".",
            Self::VSplit => "|",
            Self::HSplit => "-",
            Self::FMirror => "/",
            Self::BMirror => "\\",
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '|' => Self::VSplit,
            '-' => Self::HSplit,
            '/' => Self::FMirror,
            '\\' => Self::BMirror,
            _ => panic!("bad content"),
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Grid {
    max_x: usize,
    max_y: usize,
    elements: HashMap<(usize, usize), (Content, State)>,
    beams: Vec<Beam>,
}

impl Grid {
    pub fn new(content: &str) -> Self {
        let mut elements = HashMap::default();
        let lines = content.lines().collect::<Vec<_>>();
        let max_x = lines.len() - 1;
        let max_y = lines[0].len() - 1;
        //println!("max_x: {}, max_y: {}", max_x, max_y);

        for (y, line) in lines.into_iter().enumerate() {
            line.chars()
                .enumerate()
                .map(|(x, c)| elements.insert((x, y), (Content::from_char(c), State::default())))
                .last();
        }
        let beams = vec![Beam::default()];

        Self {
            max_x,
            max_y,
            elements,
            beams,
        }
    }

    pub fn reset(&mut self) {
        self.elements
            .iter_mut()
            .map(|(_k, v)| v.1 = State::default())
            .last();
    }

    pub fn energized_count(&self) -> usize {
        self.elements.values().filter(|e| e.1.active).count()
    }

    pub fn new_beams(&self, (content, state): &(Content, State), beam: &Beam) -> Vec<Beam> {
        let mut ret = vec![];
        for beam in beam.rotate(*content) {
            if let Some(next_cell) = beam.next(self.max_x, self.max_y) {
                let (_, state) = self
                    .elements
                    .get(&(next_cell.location.0, next_cell.location.1))
                    .unwrap();
                if next_cell.direction == Direction::North && !state.seen_north {
                    ret.push(next_cell)
                };
                if next_cell.direction == Direction::South && !state.seen_south {
                    ret.push(next_cell)
                };
                if next_cell.direction == Direction::East && !state.seen_east {
                    ret.push(next_cell)
                };
                if next_cell.direction == Direction::West && !state.seen_west {
                    ret.push(next_cell)
                };
            }
        }
        ret
    }

    pub fn energize(&mut self, limit: usize) {
        let mut count = 0;
        loop {
            let new_beams = self.energize_impl();
            if new_beams.is_empty() {
                return;
            }
            if limit > 0 {
                if count > limit {
                    return;
                }
            }
            self.beams = new_beams;
            count += 1;
        }
    }

    pub fn energize_impl(&mut self) -> Vec<Beam> {
        //println!("Processing {:?}", beams);
        let mut new_beams = vec![];
        for beam in self.beams.iter() {
            //println!("  Processing {:?}", beam);
            let cell = self
                .elements
                .get(&beam.location)
                .unwrap_or_else(|| panic!("No cell {:?}", &beam.location));
            //println!("  Cell {:?}", cell);
            let mut new = self.new_beams(cell, beam);
            //println!("  new {:?}", new);
            new_beams.append(&mut new);
            self.elements.entry(beam.location).and_modify(|(_c, s)| {
                s.active = true;
                if beam.direction == Direction::North {
                    s.seen_north = true;
                }
                if beam.direction == Direction::South {
                    s.seen_south = true;
                }
                if beam.direction == Direction::East {
                    s.seen_east = true;
                }
                if beam.direction == Direction::West {
                    s.seen_west = true;
                }
            });
        }
        new_beams
    }
}

impl eframe::App for Grid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = egui::vec2(1600.0, 1600.0);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            const SCALE: f32 = 8.0;
            let to_panel_pos = |pos: (usize, usize)| {
                egui::vec2((pos.0 as f32 + 0.9) * SCALE, (pos.1 as f32 + 0.9) * SCALE).to_pos2()
            };

            ui.heading("Grid");
            for beam in &self.beams[..] {
                painter.circle_stroke(
                    to_panel_pos((beam.location.0, beam.location.1)),
                    1.0,
                    Stroke::new(3.0, Color32::YELLOW),
                )
            }
            for y in 0..=self.max_y {
                for x in 0..=self.max_x {
                    let (content, state) = self.elements.get(&(x, y)).unwrap();
                    if state.active {
                        painter.rect_stroke(
                            egui::Rect {
                                min: egui::vec2((x as f32 + 0.6) * SCALE, (y as f32 + 0.6) * SCALE)
                                    .to_pos2(),
                                max: egui::vec2((x as f32 + 1.4) * SCALE, (y as f32 + 1.4) * SCALE)
                                    .to_pos2(),
                            },
                            egui::Rounding::none(),
                            Stroke::new(1.0, Color32::RED),
                        )
                    }
                    match content {
                        Content::Empty => painter.circle_stroke(
                            to_panel_pos((x, y)),
                            1.0,
                            Stroke::new(1.0, Color32::WHITE),
                        ),

                        Content::VSplit => painter.vline(
                            (x as f32 + 1.1) * SCALE,
                            ((y as f32 + 0.6) * SCALE)..=((y as f32 + 1.4) * SCALE),
                            Stroke::new(1.0, Color32::WHITE),
                        ),
                        Content::HSplit => painter.hline(
                            ((x as f32 + 0.6) * SCALE)..=((x as f32 + 1.4) * SCALE),
                            (y as f32 + 1.1) * SCALE,
                            Stroke::new(1.0, Color32::WHITE),
                        ),
                        Content::FMirror => painter.line_segment(
                            [
                                egui::vec2((x as f32 + 0.6) * SCALE, (y as f32 + 1.4) * SCALE)
                                    .to_pos2(),
                                egui::vec2((x as f32 + 1.4) * SCALE, (y as f32 + 0.6) * SCALE)
                                    .to_pos2(),
                            ],
                            Stroke::new(1.0, Color32::WHITE),
                        ),
                        Content::BMirror => painter.line_segment(
                            [
                                egui::vec2((x as f32 + 0.6) * SCALE, (y as f32 + 0.6) * SCALE)
                                    .to_pos2(),
                                egui::vec2((x as f32 + 1.4) * SCALE, (y as f32 + 1.4) * SCALE)
                                    .to_pos2(),
                            ],
                            Stroke::new(1.0, Color32::WHITE),
                        ),
                        _ => painter.circle_stroke(
                            to_panel_pos((x, y)),
                            1.0,
                            Stroke::new(1.0, Color32::DARK_RED),
                        ),
                    }
                }
            }
            self.beams = self.energize_impl();
            //ctx.request_repaint_after(Duration::from_millis(1));
            ctx.request_repaint();
            if self.beams.is_empty() {
                println!("Count {}", self.energized_count());
            }
        });
    }
}

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let mut grid = Grid::new(data);
    grid.beams = vec![Beam {
        location: (96, 105),
        direction: Direction::North,
    }];

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 1600.0)),
        ..Default::default()
    };

    eframe::run_native("AOC 2023 Day 16", options, Box::new(|_cc| Box::new(grid)));
    0
    // grid.energize(0);
    //grid.energized_count() as u64
}

fn part2(data: &str) -> u64 {
    let mut grid = Grid::new(data);
    let mut max = 0;
    let mut beams = vec![];
    for y in 0..=grid.max_y {
        beams.push(Beam {
            location: (0, y),
            direction: Direction::East,
        });
        beams.push(Beam {
            location: (grid.max_x, y),
            direction: Direction::West,
        });
    }
    for x in 0..=grid.max_x {
        beams.push(Beam {
            location: (x, 0),
            direction: Direction::South,
        });
        beams.push(Beam {
            location: (x, grid.max_y),
            direction: Direction::North,
        });
    }
    for beam in beams {
        grid.reset();
        grid.beams = vec![beam];
        grid.energize(0);
        let count = grid.energized_count();
        println!("{:?} - {}", beam, count);
        max = count.max(max);
    }
    max as u64
}

fn main() {
    println!("Day x of {}", version());
    // let val = part1(input());
    //println!("Part 1 answer is {}", val);
    let val = part2(input());
    println!("Part 2 answer is {}", val);
}

mod parse {
    use super::*;
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::pair;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::{bytes::complete::tag, IResult};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_sample_one_step() {
        let mut grid = Grid::new(sample());
        assert_eq!(0, grid.energized_count());
        println!("{:?}", &grid);
        grid.energize_impl();
        println!("{:?}", &grid);
        assert_eq!(1, grid.energized_count());
    }

    #[test]
    fn test_sample() {
        let mut grid = Grid::new(sample());
        assert_eq!(0, grid.energized_count());
        grid.energize(0);
        assert_eq!(46, grid.energized_count());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(51, part2(sample()));
    }
}
