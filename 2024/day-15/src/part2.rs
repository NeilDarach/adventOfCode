use crate::common::{parser, Item, Warehouse, WarehouseType};
use crate::custom_error::AocError;
use utils::grid::Xy;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut warehouse: Warehouse<Wide> = parser::parse(input);
    while !warehouse.done() {
        warehouse.step();
    }
    Ok(warehouse.score().to_string())
}

pub struct Wide;

impl WarehouseType for Wide {
    fn add_item<T: WarehouseType>(x: i32, y: i32, c: char, warehouse: &mut Warehouse<T>) {
        match c {
            '#' => {
                warehouse.grid.insert((x * 2, y).into(), Item::Wall);
                warehouse.grid.insert(((x * 2) + 1, y).into(), Item::Wall);
            }
            'O' => {
                warehouse.grid.insert((x * 2, y).into(), Item::BoxLeft);
                warehouse
                    .grid
                    .insert(((x * 2) + 1, y).into(), Item::BoxRight);
            }
            '.' => {
                warehouse.grid.insert((x * 2, y).into(), Item::Empty);
                warehouse.grid.insert(((x * 2) + 1, y).into(), Item::Empty);
            }
            '@' => {
                warehouse.grid.insert((x * 2, y).into(), Item::Empty);
                warehouse.grid.insert(((x * 2) + 1, y).into(), Item::Empty);
                warehouse.robot_location = Xy::new(x * 2 as i32, y);
            }
            _ => panic!("Bad map"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::grid::Direction4;
    pub const SAMPLE_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const SAMPLE_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    pub const SAMPLE_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("9021", process(SAMPLE_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let warehouse: Warehouse<Wide> = parser::parse(SAMPLE_1);
        assert_eq!(16, warehouse.grid.width());
        assert_eq!(8, warehouse.grid.height());
        assert_eq!(Xy::new(4, 2), warehouse.robot_location);
        assert_eq!(15, warehouse.instructions.len());
        assert_eq!(&Direction4::W, warehouse.instructions.first().unwrap());
        Ok(())
    }

    #[test]
    fn test_score() -> miette::Result<()> {
        let mut warehouse: Warehouse<Wide> = parser::parse(SAMPLE_2);
        while !warehouse.done() {
            warehouse.step();
        }
        assert_eq!(9021, warehouse.score());
        Ok(())
    }

    #[test]
    fn test_sample3() -> miette::Result<()> {
        let mut warehouse: Warehouse<Wide> = parser::parse(SAMPLE_3);
        while !warehouse.done() {
            warehouse.step();
        }
        assert_eq!(618, warehouse.score());
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        let mut warehouse: Warehouse<Wide> = parser::parse(SAMPLE_1);
        warehouse.instructions = vec![
            Direction4::E,
            Direction4::E,
            Direction4::E,
            Direction4::E,
            Direction4::S,
            Direction4::S,
            Direction4::S,
        ];
        while !warehouse.done() {
            warehouse.step();
        }
        assert_eq!(Xy::new(8, 3), warehouse.robot_location);
        assert_eq!(1949, warehouse.score());
        Ok(())
    }
}
