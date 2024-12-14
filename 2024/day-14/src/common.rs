use std::collections::HashSet;
use std::fmt::{Debug, Display};
use utils::grid::Xy;

pub struct Zone {
    pub width: i32,
    pub height: i32,
    pub count: i32,
    pub robots: Vec<Robot>,
}

impl Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let positions = self.robots.iter().map(|e| e.pos).collect::<Vec<_>>();
        let mut hash: HashSet<Xy> = HashSet::default();
        for pos in positions {
            hash.insert(pos);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if hash.contains(&(x, y).into()) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Step: {}", self.count)?;
        Ok(())
    }
}
impl Zone {
    pub fn new(width: i32, height: i32, robots: Vec<Robot>) -> Self {
        Self {
            width,
            height,
            robots,
            count: 0,
        }
    }

    pub fn safety(&self) -> i32 {
        self.count().iter().product()
    }
    pub fn count(&self) -> [i32; 4] {
        let lim_x = (self.width - 1) / 2;
        let lim_y = (self.height - 1) / 2;
        dbg!(lim_x, lim_y);
        let q1 = self
            .robots
            .iter()
            .filter(|e| e.pos.x < lim_x && e.pos.y < lim_y)
            .count() as i32;
        let q2 = self
            .robots
            .iter()
            .filter(|e| e.pos.x > lim_x && e.pos.y < lim_y)
            .count() as i32;
        let q3 = self
            .robots
            .iter()
            .filter(|e| e.pos.x < lim_x && e.pos.y > lim_y)
            .count() as i32;
        let q4 = self
            .robots
            .iter()
            .filter(|e| e.pos.x > lim_x && e.pos.y > lim_y)
            .count() as i32;
        [q1, q2, q3, q4]
    }

    pub fn step(&mut self) {
        self.count += 1;
        for i in 0..self.robots.len() {
            self.step_robot(i);
        }
    }

    pub fn step_robot(&mut self, index: usize) {
        let robot = &mut self.robots[index];
        robot.pos.x += robot.vel.x;
        if robot.pos.x < 0 {
            robot.pos.x += self.width
        };
        if robot.pos.x >= self.width {
            robot.pos.x -= self.width
        };
        robot.pos.y += robot.vel.y;
        if robot.pos.y < 0 {
            robot.pos.y += self.height
        };
        if robot.pos.y >= self.height {
            robot.pos.y -= self.height
        };
    }
}
pub struct Robot {
    pub pos: Xy,
    pub vel: Xy,
}
impl Debug for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R({},{})", self.pos.x, self.pos.y)
    }
}

pub mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, line_ending},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        IResult,
    };

    pub fn parse(input: &str) -> Vec<Robot> {
        let (_, robots) = separated_list1(line_ending, robot)(input).unwrap();
        robots
    }

    fn robot(input: &str) -> IResult<&str, Robot> {
        let (input, (p1, p2)) = preceded(tag("p="), separated_pair(i32, tag(","), i32))(input)?;
        let (input, (v1, v2)) = preceded(tag(" v="), separated_pair(i32, tag(","), i32))(input)?;
        Ok((
            input,
            Robot {
                pos: (p1, p2).into(),
                vel: (v1, v2).into(),
            },
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse() -> miette::Result<()> {
        let robots = parser::parse(SAMPLE);
        assert_eq!(12, robots.len());
        assert_eq!(Xy::new(6, 3), robots[1].pos);
        assert_eq!(Xy::new(2, -1), robots[3].vel);
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        let robots = parser::parse(SAMPLE);
        let mut zone = Zone::new(11, 7, robots);
        assert_eq!(Xy::new(2, 4), zone.robots[10].pos);
        zone.step_robot(10);
        assert_eq!(Xy::new(4, 1), zone.robots[10].pos);
        zone.step_robot(10);
        assert_eq!(Xy::new(6, 5), zone.robots[10].pos);
        zone.step_robot(10);
        assert_eq!(Xy::new(8, 2), zone.robots[10].pos);
        zone.step_robot(10);
        assert_eq!(Xy::new(10, 6), zone.robots[10].pos);
        Ok(())
    }

    #[test]
    fn test_count() -> miette::Result<()> {
        let robots = parser::parse(SAMPLE);
        let mut zone = Zone::new(11, 7, robots);
        for _ in 0..100 {
            zone.step();
        }
        assert_eq!(12, zone.safety());
        Ok(())
    }
}
