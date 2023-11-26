use std::{collections::HashMap, str::FromStr};

use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use crate::util::read_input;

pub fn solve() {
    let input = read_input("day21.txt");
    let mut monkeys: Monkeys = input.parse().unwrap();
    println!("Day 21 part 1: {}", monkeys.solve("root").unwrap());
}

type Number = isize;

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    name: String,
    action: Action,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    Number(Number),
    Calculation(Calculation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn apply(&self, lhs: Number, rhs: Number) -> Number {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Calculation {
    lhs: String,
    op: Operator,
    rhs: String,
}

impl FromStr for Operator {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            x => Err(color_eyre::eyre::eyre!("Cannot parse operator {x}")),
        }
    }
}


#[derive(Debug, Clone)]
struct Monkeys {
    monkeys: HashMap<String, Action>,
}

impl FromStr for Monkeys {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Vec<Monkey> = s
            .trim()
            .lines()
            .map(|line| line.parse())
            .collect::<color_eyre::Result<Vec<_>>>()?;
        let mut result = HashMap::new();
        for monkey in res {
            result.insert(monkey.name, monkey.action);
        }
        Ok(Self { monkeys: result })
    }
}

impl FromStr for Monkey {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\w+): (\w+)(?: ([\+\-\*\/]) (\w+))?$").unwrap());
        let cap = RE
            .captures(s)
            .ok_or_else(|| color_eyre::eyre::eyre!("Cannot parse line {s}"))?;
        if cap.iter().filter(|grp| grp.is_some()).count() > 3 {
            Ok(Self {
                name: cap[1].to_owned(),
                action: Action::Calculation(Calculation {
                    lhs: cap[2].to_owned(),
                    op: cap[3].parse()?,
                    rhs: cap[4].to_owned(),
                }),
            })
        } else {
            Ok(Self {
                name: cap[1].to_owned(),
                action: Action::Number(cap[2].parse()?),
            })
        }
    }
}

impl Monkeys {
    pub fn solve(&mut self, name: &str) -> Option<Number> {
        if let Action::Calculation(calculation) = self.monkeys.get(name)?.clone() {
            let lhs = self.solve(&calculation.lhs)?;
            let rhs = self.solve(&calculation.rhs)?;
            let v = calculation.op.apply(lhs, rhs);
            self.monkeys.insert(name.to_owned(), Action::Number(v));
        }
        if let Action::Number(v) = self.monkeys.get(name)? {
            return Some(*v);
        }
        return None;
    }

    pub fn contains(&self, root: &str, child: &str) -> bool {
        if root == child {
            return true;
        }
        let Action::Calculation(calc) = self.monkeys.get(root).unwrap() else {return false};
        self.contains(&calc.lhs, child) || self.contains(&calc.rhs, child)
    }

    pub fn human_solve(&mut self) -> Number {
        //TODO: solve tree side without reference to HUMN
        let Action::Calculation(root) = self.monkeys.get("root").unwrap().clone() else {panic!("Nonono")};
        let lhs_contains_humn = self.contains(&root.lhs, "humn");
        if lhs_contains_humn {
            self.solve(&root.rhs);
        } else {
            self.solve(&root.lhs);
        }
        let mut c = 0;
        loop {
            if c > 0 && c % 50_000 == 0 {
                println!("{c}");
            }
            if c == 301 {
                dbg!(c);
            }
            let mut inner_monkeys = self.clone();
            let Some(m) = inner_monkeys.monkeys.get_mut("humn") else {panic!("No human found")};
            *m = Action::Number(c);
            let lhs = inner_monkeys.solve(&root.lhs).unwrap();
            let rhs = inner_monkeys.solve(&root.rhs).unwrap();
            if lhs == rhs {
                return lhs;
            }
            c += 1;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::{read_example, read_input};

    use super::*;

    #[test]
    fn it_parses_monkey() -> color_eyre::Result<()> {
        let input = "root: pppw + sjmn";
        let monkey: Monkey = input.parse()?;
        assert_eq!("root", monkey.name);
        assert_eq!(
            Action::Calculation(Calculation {
                lhs: "pppw".to_owned(),
                op: Operator::Add,
                rhs: "sjmn".to_owned()
            }),
            monkey.action
        );

        let input = "dbpl: 5";
        let monkey: Monkey = input.parse()?;
        assert_eq!(
            Monkey {
                name: "dbpl".to_owned(),
                action: Action::Number(5)
            },
            monkey
        );
        Ok(())
    }

    #[test]
    fn it_solves() {
        let input = read_example("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        assert_eq!(Some(5), monkeys.solve("dbpl"));
        assert_eq!(Some(32), monkeys.solve("hmdt"));
        assert_eq!(Some(30), monkeys.solve("drzm"));
        assert_eq!(Some(152), monkeys.solve("root"));
    }
    #[test]
    fn it_solves_part1() {
        let input = read_input("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        assert_eq!(Some(194058098264286), monkeys.solve("root"));
    }

    #[test]
    fn it_human_solves() {
        let input = read_input("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
      
        println!("{}", monkeys.human_solve());

    }

    #[test]
    fn it_finds_node_in_subtree() {
        let input = read_example("day21.txt");
        let monkeys: Monkeys = input.parse().unwrap();
        assert!(monkeys.contains("pppw", "humn"));
        assert!(!monkeys.contains("sjmn", "humn"));
    }

    #[test]
    fn that_real_input_does_not_have_humn_on_both_sides() {
        let input = read_input("day21.txt");
        let monkeys: Monkeys = input.parse().unwrap();
        let Action::Calculation(root) = monkeys.monkeys.get("root").unwrap() else {panic!("Nonono")};
        assert_ne!(monkeys.contains(&root.lhs, "humn"), monkeys.contains(&root.rhs,"humn"));

    }
}
