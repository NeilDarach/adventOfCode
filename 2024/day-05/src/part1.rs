use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (rules, orders) = parser::parse(input).unwrap().1;
    let valid = orders
        .into_iter()
        .filter(|e| e.is_valid(&rules))
        .collect::<Vec<_>>();
    let res: i32 = valid.iter().map(|e| e.midpage()).sum();
    Ok(res.to_string())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rule(i32, i32);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Order(Vec<i32>);

impl Rule {
    pub fn is_valid(&self, order: &Order) -> bool {
        let Some(i1) = order.index_of(self.0) else {
            return true;
        };
        let Some(i2) = order.index_of(self.1) else {
            return true;
        };
        i1 < i2
    }
}

impl Order {
    pub fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        rules.iter().all(|r| r.is_valid(self))
    }
    pub fn index_of(&self, val: i32) -> Option<usize> {
        self.0.iter().position(|e| e == &val)
    }

    pub fn midpage(&self) -> i32 {
        self.0[self.0.len() / 2]
    }
}

mod parser {
    use crate::part1::{Order, Rule};
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, newline},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::separated_pair,
        sequence::terminated,
        IResult,
    };
    pub fn parse(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Order>)> {
        separated_pair(many1(rule), newline, separated_list1(newline, order))(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        map(
            terminated(separated_pair(i32, tag("|"), i32), newline),
            |(a, b)| Rule(a, b),
        )(input)
    }

    fn order(input: &str) -> IResult<&str, Order> {
        map(separated_list1(tag(","), i32), |v| Order(v))(input)
    }
}
#[cfg(test)]
mod tests {
    use super::parser;
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13
97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n
75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

        assert_eq!("143", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let input = "47|53
97|13
97|61

75,47,61,53,29
97,61,53,29,13";
        let (rules, orders) = parser::parse(input).unwrap().1;
        assert_eq!(3, rules.len());
        assert_eq!(2, orders.len());
        assert_eq!(Rule(97, 13), rules[1]);
        assert_eq!(Order(vec![75, 47, 61, 53, 29]), orders[0]);
        Ok(())
    }

    #[test]
    fn test_valid_order() -> miette::Result<()> {
        let order = Order(vec![75, 47, 61, 53, 29]);
        let rule = Rule(75, 47);
        assert!(rule.is_valid(&order));
        let rule = Rule(74, 47);
        assert!(rule.is_valid(&order));
        Ok(())
    }

    #[test]
    fn test_invalid_order() -> miette::Result<()> {
        let order = Order(vec![75, 47, 61, 53, 29]);
        let rule = Rule(47, 75);
        assert!(!rule.is_valid(&order));
        Ok(())
    }

    #[test]
    fn test_find_valid_orders() -> miette::Result<()> {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13
97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n
75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

        let (rules, orders) = parser::parse(input).unwrap().1;
        let valid = orders
            .into_iter()
            .filter(|e| e.is_valid(&rules))
            .collect::<Vec<_>>();
        assert_eq!(3, valid.len());
        Ok(())
    }
}
