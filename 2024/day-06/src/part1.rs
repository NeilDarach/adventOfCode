use crate::common::*;
use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut map = parser::parse(input);
    let visited = map.visited();
    Ok((visited.keys().len()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("41", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let map = parser::parse(SAMPLE);
        println!("{}", map.grid);
        //assert_eq!(map.grid.get((0, 0).into()), Some(&Content::Empty));
        assert_eq!(map.grid.get((2, 3).into()), Some(&Content::Obstacle));
        Ok(())
    }

    #[test]
    fn test_look() -> miette::Result<()> {
        let mut map = parser::parse(SAMPLE);
        assert_eq!(map.next_cell(), Some(&Content::Empty));
        map.guard = Content::Guard((4, 1).into(), Direction4::N);
        assert_eq!(map.next_cell(), Some(&Content::Obstacle));
        map.guard = Content::Guard((3, 0).into(), Direction4::E);
        assert_eq!(map.next_cell(), Some(&Content::Obstacle));
        map.guard = Content::Guard((3, 0).into(), Direction4::W);
        assert_eq!(map.next_cell(), Some(&Content::Empty));
        map.guard.turn_right();
        assert_eq!(map.next_cell(), None);

        map.guard.turn_right();
        map.guard.turn_right();
        map.guard.move_cell();
        if let Content::Guard(c, v) = map.guard {
            assert_eq!(Direction4::S, v);
            assert_eq!(Xy::new(3, 1), c);
        } else {
            panic!("Not a guard");
        }

        Ok(())
    }
}
