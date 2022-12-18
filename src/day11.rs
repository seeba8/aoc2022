use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{all_consuming, map_res, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};
use std::collections::VecDeque;

use crate::util::read_input;


pub fn solve() {
    println!("Day 11 part 1: {}", part1(&read_input("day11.txt")));
    println!("Day 11 part 2: {}", part2(&read_input("day11.txt")));
}

fn part2(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input);
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            monkeys.as_mut_slice().monkey_business(i, 1);
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspection_counter)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn part1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys.as_mut_slice().monkey_business(i, 3);
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspection_counter)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
    .trim()
    .replace("\r\n", "\n")
    .split("\n\n")
    .map(|lines| all_consuming(monkey_parser)(lines).finish().unwrap().1)
    .collect()
}

trait Monkeys {
    fn monkey_business(&mut self, monkey: usize, relief_divisor: u64);
}

impl Monkeys for &mut [Monkey] {
    fn monkey_business(&mut self, monkey: usize, relief_divisor: u64) {
        let lcm: u64 = self.iter().map(|monkey| monkey.test_divisor).product();
        // println!("Monkey {monkey}:");
        let monkey_cloned = &mut self[monkey].clone();
        self[monkey].items.clear();
        while let Some(mut item) = monkey_cloned.items.pop_front() {
            self[monkey].inspection_counter += 1;
            // println!("  Monkey inspects an item with a worry level of {item}.");
            monkey_cloned.inspection_counter += 1;
            item = monkey_cloned.operation.apply(item);
            item %= lcm;
            // println!("    Worry level is adjusted to {item}.");
            item /= relief_divisor;
            // println!("    Monkey gets bored with item. Worry level is divided by {RELIEF_DIVISOR} to {item}.");
            if item % monkey_cloned.test_divisor == 0 {
                // println!(
                //     "    Current worry level is divisible by {}.",
                //     monkey_cloned.test_divisor
                // );
                self[monkey_cloned.target_true].items.push_back(item);
                // println!(
                //     "    Item with worry level {item} is thrown to monkey {}.",
                //     monkey_cloned.target_true
                // );
            } else {
                // println!(
                //     "    Current worry level is not divisible by {}.",
                //     monkey_cloned.test_divisor
                // );
                self[monkey_cloned.target_false].items.push_back(item);
                // println!(
                //     "    Item with worry level {item} is thrown to monkey {}.",
                //     monkey_cloned.target_false
                // );
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    operation: Operation,
    test_divisor: u64,
    target_false: usize,
    target_true: usize,
    pub inspection_counter: u64,
}

fn items_parser(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tuple((multispace0, tag("Starting items: "))),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)
}

#[derive(Clone, PartialEq, Debug)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    pub const fn apply(&self, value: u64) -> u64 {
        match (self.operator, self.operand) {
            (Operator::Mul, Operand::Old) => value * value,
            (Operator::Mul, Operand::Value(s)) => value * s,
            (Operator::Add, Operand::Old) => value + value,
            (Operator::Add, Operand::Value(s)) => value + s,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Operator {
    Mul,
    Add,
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Operand {
    Old,
    Value(u64),
}

fn operation_parser(input: &str) -> IResult<&str, Operation> {
    let operand = map_res(digit1, |s: &str| s.parse().map(Operand::Value));
    let (remaining, (operator, operand)) = preceded(
        tuple((multispace0, tag("Operation: new = old"))),
        tuple((
            alt((
                value(Operator::Add, tag(" + ")),
                value(Operator::Mul, tag(" * ")),
            )),
            alt((value(Operand::Old, tag("old")), operand)),
        )),
    )(input)?;
    let f: Operation = Operation { operator, operand };
    IResult::Ok((remaining, f))
}

fn test_parser(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (remaining, test) = preceded(
        tuple((multispace0, tag("Test: divisible by "))),
        nom::character::complete::u64,
    )(input)?;
    let (remaining, true_branch) = preceded(
        tuple((multispace0, tag("If true: throw to monkey "))),
        nom::character::complete::u64,
    )(remaining)?;

    let (remaining, false_branch) = preceded(
        tuple((multispace0, tag("If false: throw to monkey "))),
        nom::character::complete::u64,
    )(remaining)?;
    IResult::Ok((
        remaining,
        (test, true_branch, false_branch),
    ))
}

fn monkey_parser(i: &str) -> IResult<&str, Monkey> {
    let (remaining, (starting_items, operation, (test, true_path, false_path))) = preceded(
        tuple((tag("Monkey "), digit1, tag(":"))),
        tuple((items_parser, operation_parser, test_parser)),
    )(i)?;
    IResult::Ok((
        remaining,
        Monkey {
            items: starting_items.into(),
            operation,
            test_divisor: test,
            target_false: false_path.try_into().unwrap(),
            target_true: true_path.try_into().unwrap(),
            inspection_counter: 0,
        },
    ))
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;

    #[test]
    fn it_parses_starting_items() {
        let input = "Starting items: 79, 98";
        let (remaining, items) = items_parser(input).unwrap();
        assert_eq!("", remaining);
        assert_eq!(vec![79, 98], items);
    }

    #[test]
    fn it_parses_operation() {
        let input = "  Operation: new = old * 19";
        let (remaining, f) = operation_parser(input).unwrap();
        assert_eq!("", remaining);
        assert_eq!(0, f.apply(0));
        assert_eq!(19, f.apply(1));
        assert_eq!(38, f.apply(2));
    }

    #[test]
    fn it_parses_monkey_tests() {
        let input = r"  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let (remainder, x) = test_parser(input).unwrap();
        assert_eq!("", remainder);
        assert_eq!((23, 2, 3), x);
    }

    #[test]
    fn it_parses_single_monkey() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";
        let (remaining, monkey) = monkey_parser(input).unwrap();
        assert_eq!("", remaining);
        let expected = Monkey {
            items: vec![79, 98].into(),
            operation: Operation {
                operator: Operator::Mul,
                operand: Operand::Value(19),
            },
            test_divisor: 23,
            target_false: 3,
            target_true: 2,
            inspection_counter: 0,
        };
        assert_eq!(expected, monkey);
    }

    #[test]
    fn it_solves_part1() {
        let input = read_example("day11.txt");
        assert_eq!(10605, part1(&input));
    }

    #[test]
    fn it_solves_part2() {
        let input = read_example("day11.txt");
        assert_eq!(2_713_310_158, part2(&input));
    }
}
