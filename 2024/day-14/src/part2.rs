use crate::common::*;
use crate::custom_error::AocError;
use utils::grid::Xy;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let robots = parser::parse(input);
    let mut zone = Zone::new(101, 103, robots);
    for _i in 0..(zone.height * zone.width) {
        zone.step();
        if zone.has_row() {
            return Ok(zone.count.to_string());
        }
    }
    Ok("".to_string())
}

impl Zone {
    pub fn sort(v: &mut [Xy]) {
        v.sort_by(|a, b| {
            if a.y.cmp(&b.y) == std::cmp::Ordering::Equal {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });
    }
    pub fn has_row(&self) -> bool {
        let mut positions = self.robots.iter().map(|e| e.pos).collect::<Vec<_>>();
        Self::sort(&mut positions);
        positions.windows(8).any(|e| self.is_row(e))
    }

    pub fn is_row(&self, window: &[Xy]) -> bool {
        window[1].x - window[0].x == 1
            && window[2].x - window[1].x == 1
            && window[3].x - window[2].x == 1
            && window[4].x - window[3].x == 1
            && window[5].x - window[4].x == 1
            && window[6].x - window[5].x == 1
            && window[7].x - window[6].x == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let file = include_str!("../input2.txt");
        assert_eq!("6876", process(file)?);
        Ok(())
    }

    #[test]
    fn test_is_row() -> miette::Result<()> {
        let zone = Zone::new(11, 7, vec![]);
        let mut positions = [
            (0, 0).into(),
            (1, 0).into(),
            (2, 0).into(),
            (3, 0).into(),
            (4, 0).into(),
            (5, 0).into(),
            (6, 0).into(),
            (7, 0).into(),
        ];
        assert!(zone.is_row(&positions));
        positions[2] = (3, 0).into();
        assert!(!zone.is_row(&positions));
        Ok(())
    }
    #[test]
    fn test_sort() -> miette::Result<()> {
        let mut positions = vec![
            (1, 0).into(),
            (0, 0).into(),
            (3, 0).into(),
            (4, 0).into(),
            (2, 0).into(),
            (5, 0).into(),
            (6, 0).into(),
            (7, 0).into(),
        ];
        Zone::sort(&mut positions);
        assert_eq!(0, positions[0].x);
        assert_eq!(2, positions[2].x);
        Ok(())
    }
}
