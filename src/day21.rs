use std::{collections::HashMap, str::FromStr, fmt::Display};

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
    Human
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Operator {
    pub fn apply(&self, lhs: Number, rhs: Number) -> Number {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
            _ => panic!("Equal shouldn't happen")
        }
    }

    pub fn get_inverse(&self) -> Operator {
        match self {
            Operator::Add => Operator::Sub,
            Operator::Sub => Operator::Add,
            Operator::Mul => Operator::Div,
            Operator::Div => Operator::Mul,
            Operator::Eq => Operator::Eq,
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Operator::Add => write!(f, " + "),
            Operator::Sub => write!(f, " - "),
            Operator::Mul => write!(f, " * "),
            Operator::Div => write!(f, " / "),
            Operator::Eq => write!(f, " = "),
            
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

    pub fn get_equation(&mut self) -> String {
      
        let Action::Calculation(root) = self.monkeys.get("root").unwrap().clone() else {panic!("Nonono")};
        let lhs_contains_humn = self.contains(&root.lhs, "humn");
        if lhs_contains_humn {
            self.solve(&root.rhs);
        } else {
            self.solve(&root.lhs);
        }
        self.get_equation_part("root")
        
    } 

    pub fn solve_equation(&mut self, root: &str, mut number: Number) -> Number{
        println!("{}", self.get_equation());
        match self.monkeys.get(root).unwrap() {
            Action::Number(x) => return *x,
            Action::Human => return number,
            Action::Calculation(r) => {
                
                if matches!(self.monkeys.get(&r.lhs).unwrap(), Action::Number(_)) && matches!(self.monkeys.get(&r.rhs).unwrap(), Action::Number(_)) {
                    let Action::Number(x) = self.monkeys.get(&r.lhs).unwrap() else {panic!()};
                    return *x;
                }
                let (num, equation_part): (Number, &str) = if let Action::Number(n) = self.monkeys.get(&r.lhs).unwrap() {
                    (*n, &r.rhs)
                } else if let Action::Number(n) = self.monkeys.get(&r.rhs).unwrap() {
                    (*n, &r.lhs)
                } else {
                    panic!("Neither side is a number");
                };
                let inverse = r.op.get_inverse();
                number = inverse.apply(number, num);
                let other_side = self.monkeys.get(equation_part).unwrap().clone();
                self.monkeys.entry(root.to_string()).and_modify(|e| *e = other_side);
                self.solve_equation(root, number)
            },
        }
        
    }

    pub fn solve_equation2(&mut self, root: &str, number: &str) -> Number{
        println!("\n{}", self.get_equation());
        match self.monkeys.get(root).unwrap().clone() {
            Action::Number(x) => return x,
            Action::Human => {
                let Action::Number(n) = self.monkeys.get(number).unwrap() else {panic!()};
                *n
            },
            Action::Calculation(r) => {
                
                if matches!(self.monkeys.get(&r.lhs).unwrap(), Action::Number(_)) && matches!(self.monkeys.get(&r.rhs).unwrap(), Action::Number(_)) {
                    let Action::Number(x) = self.monkeys.get(&r.lhs).unwrap() else {panic!()};
                    return *x;
                }
                let (num, equation_part): (Number, &str) = if let Action::Number(n) = self.monkeys.get(&r.lhs).unwrap() {
                    (*n, &r.rhs)
                } else if let Action::Number(n) = self.monkeys.get(&r.rhs).unwrap() {
                    (*n, &r.lhs)
                } else {
                    panic!("Neither side is a number");
                };
                let inverse = r.op.get_inverse();
                let Action::Number(n) = self.monkeys.get_mut(number).unwrap() else {panic!()};
                println!("{} {}", inverse, num);
                *n = inverse.apply(*n, num);
                let other_side = self.monkeys.get(equation_part).unwrap().clone();
                self.monkeys.entry(root.to_string()).and_modify(|e| *e = other_side);
                self.solve_equation2(root, number)
            },
        }
        
    }


    fn get_equation_part(&mut self, root: &str) -> String {
        let r = self.monkeys.get(root).unwrap().clone();
        match r {
            Action::Number(x) =>  x.to_string(),
            Action::Calculation(calc) => {
                if !self.contains(&calc.lhs, "humn") {
                    self.solve(&calc.lhs);
                }
                if !self.contains(&calc.rhs, "humn") {
                    self.solve(&calc.rhs);
                }
                format!("({} {} {})", self.get_equation_part(&calc.lhs),  calc.op, self.get_equation_part(&calc.rhs))
            }
            Action::Human => root.to_owned(),
        }
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

    #[test]
    fn it_gets_equation() {
        let input = read_input("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        monkeys.monkeys.entry("root".to_string()).and_modify(|m| if let Action::Calculation(c) = m {
            c.op = Operator::Eq
        });
        monkeys.monkeys.entry("humn".to_string()).and_modify(|h| *h = Action::Human);
        let Action::Calculation(root) = monkeys.monkeys.get("root").unwrap().clone() else {panic!()};
        let result = if let Action::Number(_) = monkeys.monkeys.get(&root.lhs).unwrap() {
            monkeys.solve_equation2(&root.rhs, &root.lhs)
        } else {

            monkeys.solve_equation2(&root.lhs, &root.rhs)
        };
        dbg!(result);
        // 5823073937441 is too high
        // TODO: Bug -- if variable is on right side of sub-calculation, the operations are wrong. e.g. 5 -  (2 * humn + 3) = 10, it does +5
    }
}
