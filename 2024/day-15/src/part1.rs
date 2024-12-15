use crate::common::{parser, Item, Warehouse, WarehouseType};
use crate::custom_error::AocError;
use utils::grid::Xy;

pub struct Standard;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut warehouse: Warehouse<Standard> = parser::parse(input);
    while !warehouse.done() {
        warehouse.step();
    }
    println!("{}", warehouse);
    Ok(warehouse.score().to_string())
}

impl WarehouseType for Standard {
    fn add_item<T: WarehouseType>(x: i32, y: i32, c: char, warehouse: &mut Warehouse<T>) {
        match c {
            '#' => {
                warehouse.grid.insert((x, y).into(), Item::Wall);
            }
            'O' => {
                warehouse.grid.insert((x, y).into(), Item::Box);
            }
            '.' => {
                warehouse.grid.insert((x, y).into(), Item::Empty);
            }
            '@' => {
                warehouse.grid.insert((x, y).into(), Item::Empty);
                warehouse.robot_location = Xy::new(x, y);
            }
            _ => panic!("Bad map"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::grid::Direction4;
    const SAMPLE_1: &str = "########
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

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("10092", process(SAMPLE_2)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let warehouse: Warehouse<Standard> = parser::parse(SAMPLE_1);
        assert_eq!(8, warehouse.grid.width());
        assert_eq!(8, warehouse.grid.height());
        assert_eq!(Xy::new(2, 2), warehouse.robot_location);
        assert_eq!(15, warehouse.instructions.len());
        assert_eq!(&Direction4::W, warehouse.instructions.first().unwrap());
        Ok(())
    }

    #[test]
    fn test_score() -> miette::Result<()> {
        let mut warehouse: Warehouse<Standard> = parser::parse(SAMPLE_2);
        while !warehouse.done() {
            warehouse.step();
        }
        println!("{}", warehouse);
        assert_eq!(10092, warehouse.score());
        Ok(())
    }

    #[test]
    fn test_step() -> miette::Result<()> {
        let mut warehouse: Warehouse<Standard> = parser::parse(SAMPLE_1);
        assert_eq!(Xy::new(2, 2), warehouse.robot_location);
        println!("{}", warehouse);
        warehouse.step();
        assert_eq!(Xy::new(2, 2), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(2, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(2, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(3, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(4, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(4, 1), warehouse.robot_location);
        warehouse.step();
        assert_eq!(Xy::new(4, 2), warehouse.robot_location);
        println!("{}", warehouse);
        while !warehouse.done() {
            warehouse.step();
        }
        println!("{}", warehouse);
        assert_eq!(Xy::new(4, 4), warehouse.robot_location);
        assert_eq!(2028, warehouse.score());
        Ok(())
    }
}
